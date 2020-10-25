use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::gen_lock::{self, GenLock, LockStorage};
use crate::strategy::sealed::{CaS, InnerStrategy};
use crate::RefCnt;

#[derive(Copy, Clone, Default)]
pub struct GenLockStrategy<L>(pub(crate) L);

impl<T: RefCnt, L: LockStorage> InnerStrategy<T> for GenLockStrategy<L> {
    type Protected = T;

    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        let lock = GenLock::new(&self.0);

        let ptr = storage.load(Ordering::Acquire);
        let result = T::from_ptr(ptr);
        T::inc(&result);

        drop(lock);

        result
    }

    unsafe fn wait_for_readers(&self, _: *const T::Base) {
        gen_lock::wait_for_readers(&self.0);
    }
}

impl<T: RefCnt, L: LockStorage> CaS<T> for GenLockStrategy<L> {
    unsafe fn compare_and_swap<C: crate::as_raw::AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        let cur_ptr = current.as_raw();
        let new = T::into_ptr(new);

        // As noted above, this method has either semantics of load or of store. We don't know
        // which ones upfront, so we need to implement safety measures for both.
        let lock = GenLock::new(&self.0);

        let previous_ptr = storage.compare_and_swap(cur_ptr, new, Ordering::SeqCst);
        let swapped = ptr::eq(cur_ptr, previous_ptr);

        // Drop it here, because:
        // * We can't drop it before the compare_and_swap ‒ in such case, it could get recycled,
        //   put into the pointer by another thread with a different value and create a fake
        //   success (ABA).
        // * We drop it before waiting for readers, because it could have been a Guard with a
        //   generation lock. In such case, the caller doesn't have it any more and can't check if
        //   it succeeded, but that's OK.
        drop(current);

        let previous = T::from_ptr(previous_ptr);

        if swapped {
            drop(lock);
            gen_lock::wait_for_readers(&self.0);
        } else {
            // We didn't swap. Therefore, we need to bump the count on the old one and release the
            // new one (blackhole it).
            T::inc(&previous);
            T::dec(new);
        }

        previous
    }
}

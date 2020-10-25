use std::borrow::Borrow;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use super::gen_lock::GenLockStrategy;
use super::sealed::{CaS, InnerStrategy, Protected};
use crate::debt::Debt;
use crate::gen_lock::{GenLock, LockStorage};
use crate::ref_cnt::RefCnt;

pub struct HybridProtection<T: RefCnt> {
    debt: Option<&'static Debt>,
    ptr: ManuallyDrop<T>,
}

impl<T: RefCnt> HybridProtection<T> {
    #[inline]
    unsafe fn new(ptr: *const T::Base, debt: Option<&'static Debt>) -> Self {
        Self {
            debt,
            ptr: ManuallyDrop::new(T::from_ptr(ptr)),
        }
    }
    #[inline]
    fn attempt(storage: &AtomicPtr<T::Base>) -> Option<Self> {
        // Relaxed is good enough here, see the Acquire below
        let ptr = storage.load(Ordering::Relaxed);
        // Try to get a debt slot. If not possible, fail.
        let debt = Debt::new(ptr as usize)?;

        let confirm = storage.load(Ordering::Acquire);
        if ptr == confirm {
            // Successfully got a debt
            Some(unsafe { Self::new(ptr, Some(debt)) })
        } else if debt.pay::<T>(ptr) {
            // It changed in the meantime, we return the debt (that is on the outdated pointer,
            // possibly destroyed) and fail.
            None
        } else {
            // It changed in the meantime, but the debt for the previous pointer was already paid
            // for by someone else, so we are fine using it.
            Some(unsafe { Self::new(ptr, None) })
        }
    }
}

impl<T: RefCnt> Drop for HybridProtection<T> {
    #[inline]
    fn drop(&mut self) {
        match self.debt.take() {
            // We have our own copy of Arc, so we don't need a protection. Do nothing (but release
            // the Arc below).
            None => (),
            // If we owed something, just return the debt. We don't have a pointer owned, so
            // nothing to release.
            Some(debt) => {
                let ptr = T::as_ptr(&self.ptr);
                if debt.pay::<T>(ptr) {
                    return;
                }
                // But if the debt was already paid for us, we need to release the pointer, as we
                // were effectively already in the Unprotected mode.
            }
        }
        // Equivalent to T::dec(ptr)
        unsafe { ManuallyDrop::drop(&mut self.ptr) };
    }
}

impl<T: RefCnt> Protected<T> for HybridProtection<T> {
    #[inline]
    fn from_inner(ptr: T) -> Self {
        Self {
            debt: None,
            ptr: ManuallyDrop::new(ptr),
        }
    }

    #[inline]
    fn into_inner(mut self) -> T {
        // Drop any debt and release any lock held by the given guard and return a
        // full-featured value that even can outlive the ArcSwap it originated from.
        match self.debt.take() {
            None => (), // We have a fully loaded ref-counted pointer.
            Some(debt) => {
                let ptr = T::inc(&self.ptr);
                if !debt.pay::<T>(ptr) {
                    unsafe { T::dec(ptr) };
                }
            }
        }

        // The ptr::read & forget is something like a cheating move. We can't move it out, because
        // we have a destructor and Rust doesn't allow us to do that.
        let inner = unsafe { ptr::read(self.ptr.deref()) };
        mem::forget(self);
        inner
    }
}

impl<T: RefCnt> Borrow<T> for HybridProtection<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ptr
    }
}

#[derive(Clone, Default)]
pub struct HybridStrategy<F> {
    fallback: F,
}

impl<T, F> InnerStrategy<T> for HybridStrategy<F>
where
    T: RefCnt,
    F: InnerStrategy<T, Protected = T>,
{
    type Protected = HybridProtection<T>;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        HybridProtection::attempt(storage).unwrap_or_else(|| {
            let loaded = self.fallback.load(storage);
            HybridProtection {
                debt: None,
                ptr: ManuallyDrop::new(loaded),
            }
        })
    }
    unsafe fn wait_for_readers(&self, old: *const T::Base) {
        self.fallback.wait_for_readers(old);
        Debt::pay_all::<T>(old);
    }
}

impl<T: RefCnt, L: LockStorage> CaS<T> for HybridStrategy<GenLockStrategy<L>> {
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
        let gen = GenLock::new(&self.fallback.0);

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

        let debt = if swapped {
            // New went in, previous out, but their ref counts are correct. So nothing to do here.
            None
        } else {
            // Previous is a new copy of what is inside (and it stays there as well), so bump its
            // ref count. New is thrown away so dec its ref count (but do it outside of the
            // gen-lock).
            //
            // We try to do that by registering a debt and only if that fails by actually bumping
            // the ref.
            let debt = Debt::new(previous_ptr as usize);
            if debt.is_none() {
                let previous = T::from_ptr(previous_ptr);
                T::inc(&previous);
                T::into_ptr(previous);
            }
            debt
        };

        drop(gen);

        if swapped {
            // We swapped. Before releasing the (possibly only) ref count of previous to user, wait
            // for all readers to make sure there are no more untracked copies of it.
            //
            // Why is rustc confused about self.wait_for_readers???
            InnerStrategy::<T>::wait_for_readers(self, previous_ptr);
        } else {
            // We didn't swap, so new is black-holed.
            T::dec(new);
        }

        HybridProtection::new(previous_ptr, debt)
    }
}

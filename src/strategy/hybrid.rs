use std::borrow::Borrow;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::process;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::debt::Debt;
use crate::ref_cnt::RefCnt;
use crate::gen_lock::{self, LockStorage, GEN_CNT};
use super::{Protected, Sealed, Strategy};

const MAX_GUARDS: usize = (isize::MAX) as usize;

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

impl<T: RefCnt> Sealed for HybridProtection<T> {}
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
pub struct HybridStrategy<L> {
    lock: L,
}

impl<L> Sealed for HybridStrategy<L> {}
impl<T: RefCnt, L: LockStorage> Strategy<T> for HybridStrategy<L> {
    type Protected = HybridProtection<T>;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        HybridProtection::attempt(storage).unwrap_or_else(|| {
            let shard = self.lock.choose_shard();
            let gen = self.lock.gen_idx().load(Ordering::Relaxed) % GEN_CNT;
            // TODO: Is this still needed? Is the other SeqCst needed, in the writer? Is *there* any?
            // Or should it be Release in there and SeqCst barrier as part of wait_for_readers?
            // SeqCst: Acquire, so the dangerous section stays in. SeqCst to sync timelines with the
            // swap on the ptr in writer thread.
            let slot = &self.lock.shards().as_ref()[shard].0[gen];
            let old = slot.fetch_add(1, Ordering::SeqCst);
            // The trick is taken from Arc.
            if old > MAX_GUARDS {
                process::abort();
            }

            let ptr = storage.load(Ordering::Acquire);
            let result = HybridProtection::new(ptr, None);
            T::inc(result.borrow());

            // Release, so the dangerous section stays in. Acquire to chain the operations.
            // Do not drop the inner (maybe we should do into_raw for proper measures?)
            slot.fetch_sub(1, Ordering::AcqRel);

            result
        })
    }
    unsafe fn wait_for_readers(&self, old: *const T::Base) {
        gen_lock::wait_for_readers(&self.lock);
        Debt::pay_all::<T>(old);
    }
}

use std::borrow::Borrow;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::*;

use super::sealed::{CaS, InnerStrategy, Protected};
use crate::debt::{Debt, LocalNode};
use crate::ref_cnt::RefCnt;

pub struct HybridProtection<T: RefCnt> {
    debt: Option<&'static Debt>,
    ptr: ManuallyDrop<T>,
}

impl<T: RefCnt> HybridProtection<T> {
    pub(super) unsafe fn new(ptr: *const T::Base, debt: Option<&'static Debt>) -> Self {
        Self {
            debt,
            ptr: ManuallyDrop::new(T::from_ptr(ptr)),
        }
    }

    fn attempt(node: &LocalNode, storage: &AtomicPtr<T::Base>) -> Option<Self> {
        // Relaxed is good enough here, see the Acquire below
        let ptr = storage.load(Relaxed);
        // Try to get a debt slot. If not possible, fail.
        let debt = node.new_fast(ptr as usize)?;

        let confirm = storage.load(Acquire);
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

    fn fallback(node: &LocalNode, storage: &AtomicPtr<T::Base>) -> Self {
        // First, we claim a debt slot and store the address of the atomic pointer there, so the
        // writer can optionally help us out with loading and protecting something.
        let gen = node.new_helping(storage as *const _ as usize);
        // We already synchronized the start of the sequence by SeqCst in the new_helping vs swap on
        // the pointer. We just need to make sure to bring the pointee in (this can be newer than
        // what we got in the Debt)
        let candidate = storage.load(Acquire);

        // Try to replace the debt with our candidate.
        match node.confirm_helping(gen, candidate as usize) {
            Ok(debt) => {
                // The fast path -> we got the debt confirmed alright.
                Self::from_inner(unsafe { Self::new(candidate, Some(debt)).into_inner() })
            }
            Err((unused_debt, replacement)) => {
                // The debt is on the candidate we provided and it is unused, we so we just pay it
                // back right away.
                unused_debt.pay::<T>(candidate);
                // We got a (possibly) different pointer out. But that one is already protected and
                // the slot is paid back.
                unsafe { Self::new(replacement as *mut _, None) }
            }
        }
    }

    fn as_ptr(&self) -> *const T::Base {
        T::as_ptr(self.ptr.deref())
    }
}

impl<T: RefCnt> Drop for HybridProtection<T> {
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
    fn from_inner(ptr: T) -> Self {
        Self {
            debt: None,
            ptr: ManuallyDrop::new(ptr),
        }
    }

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
    fn borrow(&self) -> &T {
        &self.ptr
    }
}

#[derive(Clone, Default)]
pub struct HybridStrategy;

impl<T> InnerStrategy<T> for HybridStrategy
where
    T: RefCnt,
{
    type Protected = HybridProtection<T>;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        LocalNode::with(|node| {
            HybridProtection::attempt(node, storage)
                .unwrap_or_else(|| HybridProtection::fallback(node, storage))
        })
    }
    unsafe fn wait_for_readers(&self, old: *const T::Base, storage: &AtomicPtr<T::Base>) {
        // The pay_all may need to provide fresh replacement values if someone else is loading from
        // this particular storage. We do so by the exact same way, by `load` ‒ it's OK, a writer
        // does not hold a slot and the reader doesn't recurse back into writer, so we won't run
        // out of slots.
        let replacement = || self.load(storage).into_inner();
        Debt::pay_all::<T, _>(old, storage as *const _ as usize, replacement);
    }
}

impl<T: RefCnt> CaS<T> for HybridStrategy {
    unsafe fn compare_and_swap<C: crate::as_raw::AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        loop {
            let old = <Self as InnerStrategy<T>>::load(self, storage);
            // Observation of their inequality is enough to make a verdict
            if old.as_ptr() != current.as_raw() {
                return old;
            }
            // If they are still equal, put the new one in.
            let new_raw = T::as_ptr(&new);
            if storage
                .compare_exchange_weak(current.as_raw(), new_raw, SeqCst, Relaxed)
                .is_ok()
            {
                // We successfully put the new value in. The ref count went in there too.
                T::into_ptr(new);
                <Self as InnerStrategy<T>>::wait_for_readers(self, old.as_ptr(), storage);
                // We just got one ref count out of the storage and we have one in old. We don't
                // need two.
                T::dec(old.as_ptr());
                return old;
            }
        }
    }
}

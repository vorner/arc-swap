//! A strategy where the writer is helping the reader in case of collision.

// FIXME: Port parts of this comment somewhere.
// # How does this work.
//
// This is a bit modified version of hazard pointers. The readers put the currently leased pointers
// into debt slots. That means the reader owes one reference count. But as long as there's one
// reference inside the storage, everything is fine.
//
// When a writer takes the pointer out, it walks all the old debt slots that got created before the
// change and pays them all by incrementing the reference count as many times, and erases the debts
// so the readers knows they need to decrement it after the fact.
//
// ## Reader, the fast path
//
// * Publish an active address ‒ the address we'll be loading stuff from.
// * Picks a debt slot (note that we always keep at least one free; if we used the last one, we
//   immediately pay the debt by bumping the reference count and release it and we never
//   recursively call load). Mark it with a generation (this is to prevent ABA in writers).
// * Load from the address.
// * CaS-replace the generation with the address of the destination, reserving the debt. At this
//   point, we are good to do our stuff.
//
// * Later, we pay it back by CaS-replacing it with the NO_DEPT.
//
// ## Writer, the non-colliding path
//
// * Replaces the pointer in the storage.
// * The writer walks over all debts. It pays each debt that it is concerned with by bumping the
//   reference and replacing the dept with NO_DEPT. The relevant reader will fail in the CaS
//   (either because it finds NO_DEPT or other pointer in there) and knows the reference was
//   bumped, so it needs to decrement it. Note that it is possible that someone also reuses the
//   slot for the _same_ pointer. In that case that reader will set it to NO_DEPT and the newer
//   reader will have a pre-paid debt, which is fine.
//
// ## The collision path
//
// The reservation of a slot is not atomic, therefore a writer can observe the reservation in
// progress. But it doesn't want to wait for it to complete (it wants to be lock-free, which means
// it needs to be able to resolve the situation on its own).
//
// The way it knows it is in progress of the reservation is by seeing a generation in there (it has
// a distinct tag). In that case it'll try to:
//
// * First verify that the reservation is being done for the same address it modified, by reading
//   and re-confirming the active_addr slot corresponding to the currently handled node. If it is
//   for some other address, the writer doesn't have to be concerned and proceeds to the next slot.
// * It does a full load. That is fine, because the writer must be on a different thread than the
//   reader and therefore there is at least one free slot. Full load means paying the debt right
//   away by incrementing the reference count.
// * Then it tries to pass the already fully protected/paid pointer to the reader, by CaS-replacing
//   the previously observed generation with the pointer. If it fails, it decrements the count
//   again and retries on the same slot, as it might now contain a debt for the same pointer it is
//   being concerned.
// * The reader then finds the generation got replaced by a pointer (tagged with the right tag, so
//   some other writer doesn't try to "pay" a debt that is actually a negative debt). It can then
//   proceed to using that one instead of what it loaded (and not ever retry anything).
//
// ## ABA protection
//
// The generation as pre-reserving the slot allows the writer to make sure it is offering the
// loaded pointer to the same reader and that the read value is new enough (and of the same type).
//
// This solves the general case, but there's also much less frequent but theoretical ABA problem
// that could lead to UB, if left unsolved:
//
// * There is a collision on generation G.
// * The writer loads a pointer, bumps it.
// * In the meantime, all the 2^30 or 2^62 generations (depending on the usize width) generations
//   wrap around.
// * The writer stores the outdated and possibly different-typed pointer in there and the reader
//   uses it.
//
// To mitigate that, every time the counter overflows we take the current node and un-assign it
// from our current thread. We mark it as in "cooldown" and let it in there until there are no
// writers messing with that node any more (if they are not on the node, they can't experience the
// ABA problem on it). After that, we are allowed to use it again.
//
// This doesn't block the reader, it'll simply find *a* node next time ‒ this one, or possibly a
// different (or new) one.
//
// # Orderings
//
// The debt linked list is the easy part:
// * The head is used to synchronize the non-atomic content of it through trivial Acquire /
//   Release. The non-atomic parts are never written to after publishing the node (publish
//   pattern).
// * The in_use field is used as a form of lock, to claim exclusive use / write privilege of some
//   of the fields, also with Acquire / Release (mutex pattern).
//
// The actual debt juggling is a bit more interesting. We need to ensure that:
//
// * We don't start using anything before the debt is installed.
// * We don't release the swapped-out pointer to the caller before all debts acquired from
//   previously loading the old value are paid (and none are somewhere in process).
//
// We use SeqCst both on the change of the pointer and on the initial setting of the generation
// into the debt slot. That way we can be sure both threads agree on what happened first.
//
// Then all things (with the small exception about the current address, see the comments inline)
// happen on the same debt slot, therefore the time line is well established on that. Everything
// that synchronizes anything is Acquire/Release as needed there.
//
// There's a gotcha in the destructor ‒ that one doesn't have the write with SeqCst on it. But at
// that time other things must have assured that there's an exclusive access to the ArcSwap
// (because drop takes &mut self) and that also assures nobody is currently loading anything ‒ all
// the debts that are there must have existed before that exclusive ownership was acquired by
// whoever drops it.

use std::borrow::Borrow;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::*;

use super::sealed::{CaS, InnerStrategy, Protected};
use crate::debt::{Debt, REPLACEMENT_TAG, TAG_MASK};
use crate::{AsRaw, RefCnt};

pub struct Protection<T: RefCnt> {
    debt: Option<&'static Debt>,
    ptr: ManuallyDrop<T>,
}

impl<T: RefCnt> Protection<T> {
    #[inline]
    unsafe fn new(ptr: *const T::Base, debt: Option<&'static Debt>) -> Self {
        Self {
            debt,
            ptr: ManuallyDrop::new(T::from_ptr(ptr)),
        }
    }
}

impl<T: RefCnt> Drop for Protection<T> {
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

impl<T: RefCnt> Protected<T> for Protection<T> {
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
        // (it's basically ManuallyDrop::take, but that one is only since 1.42.0 and that's too new
        // for us).
        let inner = unsafe { ptr::read(self.ptr.deref()) };
        mem::forget(self);
        inner
    }
}

impl<T: RefCnt> Borrow<T> for Protection<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.ptr
    }
}

/// A strategy where a writer helps the reader in case there's a collision.
/// Which makes it possible for the writer to move forward. We don't use the generation lock here,
/// so ever writers are lock-free.
///
/// This is inspired (but not an exact copy) of
/// <https://pvk.ca/Blog/2020/07/07/flatter-wait-free-hazard-pointers/>. The debts are mostly
/// copies of the ones used by the hybrid strategy, but modified a bit. Just like in the hybrid
/// strategy, in case the slots run out or when the writer updates the value, the debts are paid by
/// incrementing the ref count (which is a little slower, but still wait-free/lock-free and still
/// in order of nanoseconds).
///
/// The strategy is:
///
/// * Wait-free on readers (except every usize::MAX/4 accesses, when it is only lock-free).
/// * Lock free on writers.
/// * Precise (data is dropped as soon as possible).
#[derive(Copy, Clone, Debug, Default)]
pub struct Helping;

impl<T: RefCnt> InnerStrategy<T> for Helping {
    type Protected = Protection<T>;

    /// Check that the pointer conforms to our idea how it should be aligned.
    ///
    /// Even if the alignment of T::Base is smaller, we assume this holds because it comes from
    /// some kind of Arc and that one has usize-sized fields before it.
    ///
    /// But as that is an internal implementation and users might provide their own implementation
    /// of RefCnt (is it unsealed?), we must protect from the ones that don't satisfy it.
    #[inline]
    fn assert_ptr_supported(&self, ptr: *const T::Base) {
        assert_eq!(ptr as usize & TAG_MASK, 0);
    }

    #[inline]
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        // First, we claim a debt slot and store the address of the atomic pointer there, so the
        // writer can optionally help us out with loading and protecting something.
        let (debt, last, gen) = Debt::new_helping(storage as *const _ as usize);
        // We already synchronized the start of the sequence by SeqCst in the Debt::new vs swap on
        // the pointer. We just need to make sure to bring the pointee in (this can be newer than
        // what we got in the Debt)
        let candidate = storage.load(Acquire);

        let ptr_addr = candidate as usize;
        // Try to replace the debt with our candidate.
        match debt.confirm(gen, ptr_addr) {
            Ok(()) => {
                // The fast path -> we got the debt confirmed alright.
                let result = Protection::new(candidate, Some(debt));
                if last {
                    // If we got the last debt slot, we must pay it back before proceeding, so
                    // others can get a slot too.
                    Protection::from_inner(Protection::into_inner(result))
                } else {
                    result
                }
            }
            Err(replacement) => {
                // We got a (possibly) different pointer out. But that one is already protected and
                // the slot is paid back. It's not possible someone could have changed our gen to
                // something but a replacement.
                debug_assert_eq!(
                    replacement & TAG_MASK,
                    REPLACEMENT_TAG,
                    "Missing tag on replacement",
                );
                // TODO
                //debug_assert_eq!(NO_DEBT, debt.0.load(Relaxed));
                let replacement = replacement & !TAG_MASK; // Remove the tag
                Protection::new(replacement as *const _, None)
            }
        }
    }

    unsafe fn wait_for_readers(&self, old: *const T::Base, storage: &AtomicPtr<T::Base>) {
        // The pay_all may need to provide fresh replacement values if someone else is loading from
        // this particular storage. We do so by the exact same way, by `load` ‒ it's OK, a writer
        // does not hold a slot and the reader doesn't recurse back into writer, so we won't run
        // out of slots.
        let replacement = || Protection::into_inner(self.load(storage));
        Debt::pay_all_helping::<T, _>(old, storage as *const _ as usize, replacement);
    }
}

impl<T: RefCnt> CaS<T> for Helping {
    unsafe fn compare_and_swap<C: AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        loop {
            let old = <Self as InnerStrategy<T>>::load(self, storage);
            // Observation of their inequality is enough to make a verdict
            if old.ptr.deref().as_raw() != current.as_raw() {
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
                <Self as InnerStrategy<T>>::wait_for_readers(
                    self,
                    old.ptr.deref().as_raw(),
                    storage,
                );
                // We just got one ref count out of the storage and we have one in old. We don't
                // need two.
                T::dec(old.ptr.deref().as_raw());
                return old;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /* FIXME: Where do these tests belong?
    /// Just plain load, no contention or whatever.
    #[test]
    fn load_bare() {
        // Check everything after loading stuff
        type T = Arc<usize>;
        let a = T::new(42);
        let s = AtomicPtr::new(T::as_ptr(&a) as *mut usize);
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        let pa: &Arc<usize> = p.borrow();
        assert_eq!(42, **pa);
        assert_eq!(T::as_ptr(&a), p.ptr.deref().deref());
        assert_eq!(s.load(Relaxed) as usize, p.debt.unwrap().0.load(Relaxed));
        assert!(!Node::get_thread().is_empty());
        assert_eq!(1, Arc::strong_count(&a));
        // Now we drop the protection and everything shall clean up.
        drop(p);
        assert!(Node::get_thread().is_empty());
    }

    /// A smoke test exercising the generation overflow thing.
    #[test]
    fn gen_overflow() {
        type T = Arc<usize>;
        let a = T::new(42);
        let s = AtomicPtr::new(T::as_ptr(&a) as *mut usize);
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        // Force it to overflow during the next one.
        THREAD_HEAD.with(|h| h.generation.set(usize::MAX / 4 * 4));
        drop(p);
        // When it overflows, the node is released back to the pool (no good way to check it got
        // into cooldown, because some other thread might claim it in between).
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        THREAD_HEAD.with(|h| assert!(h.node.get().is_none()));
        drop(p);
        // But next round will pick some node again.
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        THREAD_HEAD.with(|h| assert!(h.node.get().is_some()));
        drop(p);
    }
    */
}

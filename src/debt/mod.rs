//! Debt handling.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::*;

pub(crate) use self::helping::{GEN_TAG, REPLACEMENT_TAG, TAG_MASK};
use self::list::{LocalNode, Node};
use super::RefCnt;

mod fast;
mod helping;
mod list;

/// One debt slot.
///
/// It may contain an „owed“ reference count.
pub(crate) struct Debt(AtomicUsize);

impl Debt {
    /// The value of pointer `3` should be pretty safe, for two reasons:
    ///
    /// * It's an odd number, but the pointers we have are likely aligned at least to the word size,
    ///   because the data at the end of the `Arc` has the counters.
    /// * It's in the very first page where NULL lives, so it's not mapped.
    const NONE: usize = 0b11;
}

impl Default for Debt {
    fn default() -> Self {
        Debt(AtomicUsize::new(Self::NONE))
    }
}

impl Debt {
    /// Creates a new debt.
    ///
    /// This stores the debt of the given pointer (untyped, casted into an usize) and returns a
    /// reference to that slot, or gives up with `None` if all the slots are currently full.
    ///
    /// This is technically lock-free on the first call in a given thread and wait-free on all the
    /// other accesses.
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub(crate) fn new_fast(ptr: usize) -> Option<&'static Self> {
        LocalNode::with(|local| local.new_fast(ptr))
    }

    // TODO: Unify
    pub(crate) fn new_helping(ptr: usize) -> (&'static Self, bool, usize) {
        LocalNode::with(|local| local.new_helping(ptr))
    }

    /// Confirms the debt.
    ///
    /// Replaces the pre-debt mark (the address we are reading from) with the actual debt and
    /// confirm it that way.
    ///
    /// If it got changed in the meantime, returns the (already protected) replacement value.
    ///
    /// This is for the helping strategy (allocated with new_helping).
    #[inline]
    pub(crate) fn confirm(&self, gen: usize, ptr_addr: usize) -> Result<(), usize> {
        // AcqRel -> Release to publish everything (do we need to publish anything?)/make sure
        // sutff stays "inside" the lock.
        // Acquire -> we read a potentially new address, need to bring the pointee in.
        match self.0.compare_exchange(gen, ptr_addr, AcqRel, Acquire) {
            Ok(_) => Ok(()),
            Err(replacement) => {
                // Nothing synchronized by this, we just clear the slot for future reuse. Release
                // just to make sure, but should Relaxed be enough?
                self.0.store(Self::NONE, Release);
                Err(replacement)
            }
        }
    }

    /// Tries to pay the given debt.
    ///
    /// If the debt is still there, for the given pointer, it is paid and `true` is returned. If it
    /// is empty or if there's some other pointer, it is not paid and `false` is returned, meaning
    /// the debt was paid previously by someone else.
    ///
    /// # Notes
    ///
    /// * It is possible that someone paid the debt and then someone else put a debt for the same
    ///   pointer in there. This is fine, as we'll just pay the debt for that someone else.
    /// * This relies on the fact that the same pointer must point to the same object and
    ///   specifically to the same type ‒ the caller provides the type, it's destructor, etc.
    /// * It also relies on the fact the same thing is not stuffed both inside an `Arc` and `Rc` or
    ///   something like that, but that sounds like a reasonable assumption. Someone storing it
    ///   through `ArcSwap<T>` and someone else with `ArcSwapOption<T>` will work.
    #[inline]
    pub(crate) fn pay<T: RefCnt>(&self, ptr: *const T::Base) -> bool {
        self.0
            // If we don't change anything because there's something else, Relaxed is fine.
            //
            // The Release works as kind of Mutex. We make sure nothing from the debt-protected
            // sections leaks below this point.
            .compare_exchange(ptr as usize, Self::NONE, Release, Relaxed)
            .is_ok()
    }

    /// Pays all the debts on the given pointer.
    pub(crate) fn pay_all_fast<T: RefCnt>(ptr: *const T::Base) {
        let val = unsafe { T::from_ptr(ptr) };
        // Pre-pay one ref count that can be safely put into a debt slot to pay it.
        T::inc(&val);
        Node::traverse::<(), _>(|node| {
            for slot in node.fast_slots() {
                if slot
                    .0
                    // TODO: Do we actually need that AcqRel, or is Release enough? Then we could
                    // call into the above pay.
                    .compare_exchange(ptr as usize, Self::NONE, AcqRel, Relaxed)
                    .is_ok()
                {
                    // Pre-pay one more, for another future slot
                    T::inc(&val);
                }
            }
            None
        });
        // Implicit dec by dropping val in here, pair for the above
    }

    // TODO: Unify
    pub(crate) fn pay_all_helping<T, R>(ptr: *const T::Base, storage_addr: usize, replacement: R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        let val = unsafe { T::from_ptr(ptr) };
        // Pre-pay one ref count that can be safely put into a debt slot to pay it.
        T::inc(&val);

        Node::traverse::<(), _>(|node| {
            let _reservation = node.reserve_writer();
            for slot in node.helping_slots() {
                loop {
                    match slot
                        .0
                        // We don't need to release anything in here. The writes to the reference
                        // counts take care of themselves before they drop to 0 (they are all to
                        // the same atomic, and we are doing the decrements later on). We need to
                        // acquire any increments done by the other threads though and we need to
                        // acquire the active_addr
                        .compare_exchange(ptr as usize, Self::NONE, Acquire, Acquire)
                    {
                        Ok(_) => {
                            // We just paid the debt by the pre-paid ref count from before.
                            // Pre-pay one more, for another future slot
                            T::inc(&val);
                            break;
                        }
                        Err(gen) if gen & TAG_MASK == GEN_TAG => {
                            // The reader is trying to claim the slot right now. Let's have a look
                            // at the address where the data should come from and help the reader
                            // out.
                            let active_addr = node.active_addr().load(Acquire);
                            if active_addr != storage_addr {
                                // Re-confirm the gen matches. That way with the above active_addr
                                // load and Acquire we make sure the active_addr is not newer than
                                // the gen and therefore we are not missing a place where we need
                                // to help (eg. that Acquire makes sure the gen catches up with
                                // it).
                                if slot.0.load(Relaxed) == gen {
                                    // OK, it is really doing something with some other ArcSwap,
                                    // not interested in that.
                                    break;
                                } else {
                                    // The value changed and we are not sure how useful the
                                    // active_addr is for us. So retry.
                                    continue;
                                }
                            }
                            // Get a replacement value and try to donate it.
                            let replacement = replacement();
                            let replace_addr = T::as_ptr(&replacement) as usize;
                            let replace_addr = replace_addr | REPLACEMENT_TAG;
                            if slot
                                .0
                                // Release the value we send there. TODO: Do we need the Acquire
                                // there?
                                //
                                // Relaxed on failure: Basically, nothing happened anywhere, all
                                // data stayed with us and we are going to retry this loop from
                                // scratch.
                                .compare_exchange_weak(gen, replace_addr, AcqRel, Relaxed)
                                .is_ok()
                            {
                                // OK, it went it
                                T::into_ptr(replacement);
                                break;
                            }
                            // else -> replacement is dropped.
                            // Also, loop once more because the current slot did *not* get
                            // resolved. Retry and see if the reader already got what it wanted or
                            // try creating a new replacement.
                        }
                        // OK, not interested in this slot. Either no debt or different one.
                        Err(_) => break,
                    }
                }
            }
            None
        });
        // Implicit dec by dropping val in here, pair for the above T::inc
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    /// Checks the assumption that arcs to ZSTs have different pointer values.
    #[test]
    fn arc_zst() {
        struct A;
        struct B;

        let a = Arc::new(A);
        let b = Arc::new(B);

        let aref: &A = &a;
        let bref: &B = &b;

        let aptr = aref as *const _ as usize;
        let bptr = bref as *const _ as usize;
        assert_ne!(aptr, bptr);
    }
}

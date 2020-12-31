//! Slots and global/thread local data for the Helping strategy.

use std::cell::Cell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::*;

use super::Debt;
use crate::RefCnt;

pub(crate) const REPLACEMENT_TAG: usize = 0b01;
pub(crate) const GEN_TAG: usize = 0b10;
pub(crate) const TAG_MASK: usize = 0b11;

/// Thread local data for the helping strategy.
#[derive(Default)]
pub(super) struct Local {
    // The generation counter.
    generation: Cell<usize>,
}

/// The slots for the helping strategy.
#[derive(Default)]
pub(super) struct Slots {
    slot: Debt,
    active_addr: AtomicUsize,
}

impl Slots {
    #[inline]
    pub(super) fn slot(&self) -> &Debt {
        &self.slot
    }

    #[inline]
    pub(super) fn get_debt(&self, ptr: usize, local: &Local) -> (usize, bool) {
        // Incrementing by 4 ensures we always have enough space for 2 bit of tags.
        let gen = local.generation.get().wrapping_add(4);
        debug_assert_eq!(gen & GEN_TAG, 0);
        local.generation.set(gen);
        let discard = gen == 0;
        let gen = gen | GEN_TAG;
        // We will sync by the write to the debt. But we also sync the value of the previous
        // generation/released slot. That way we may re-confirm in the writer that the reader is
        // not in between here and the compare_exchange below with a stale gen (eg. if we are in
        // here, the re-confirm there will load the NO_DEPT and we are fine).
        self.active_addr.store(ptr, Release);

        // We could do load and store separately as we are the only ones allowed to
        // overwrite a NO_DEPT, but we actually need the SeqCst to be a read-write
        // operation in here (we need both the release and acquire part of it).
        let ok = self
            .slot
            .0
            .compare_exchange(Debt::NONE, gen, SeqCst, Relaxed)
            .is_ok();
        assert!(ok, "Run out of slots");
        (gen, discard)
    }

    #[inline]
    pub(super) fn help<R, T>(&self, gen: usize, storage_addr: usize, replacement: &R) -> bool
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        // The reader is trying to claim the slot right now. Let's have a look at the address where
        // the data should come from and help the reader out.
        let active_addr = self.active_addr.load(Acquire);
        if active_addr != storage_addr {
            // Re-confirm the gen matches. That way with the above active_addr
            // load and Acquire we make sure the active_addr is not newer than
            // the gen and therefore we are not missing a place where we need
            // to help (eg. that Acquire makes sure the gen catches up with
            // it).

            // -> The same means it really is bothering some other ArcSwap than the one we are
            // interested in, which means it is solved.
            //
            // -> different: Something changed under our hands and we are not sure what happened.
            // Not solved, retry the whole thing.
            return self.slot.0.load(Relaxed) == gen;
        }
        // Get a replacement value and try to donate it.
        let replacement = replacement();
        let replace_addr = T::as_ptr(&replacement) as usize;
        let replace_addr = replace_addr | REPLACEMENT_TAG;
        if self
            .slot
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
            return true;
        }
        // else -> replacement is dropped.
        // Also, loop once more because the current slot did *not* get
        // resolved. Retry and see if the reader already got what it wanted or
        // try creating a new replacement.
        false
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::Arc;

    use super::*;

    /// Check some alignment assumptions.
    ///
    /// Note that we also check them at runtime, in case someone doesn't run the tests.
    #[test]
    fn alignments() {
        // We don't need _exactly_ this, but that will ensure that the pointer to data is also
        // aligned to that. Or at least always unaligned to that.
        assert!(mem::align_of::<Arc<u8>>() >= 4);
        assert_eq!(Arc::as_ptr(&Arc::new(0u8)) as usize % 4, 0);
        assert!(mem::align_of::<AtomicUsize>() >= 4);
    }
}

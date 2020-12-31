//! Slots and global/thread local data for the Helping strategy.

use std::cell::Cell;
use std::slice::Iter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::*;

use super::Debt;

const DEBT_SLOT_CNT: usize = 8;
pub(crate) const REPLACEMENT_TAG: usize = 0b01;
pub(crate) const GEN_TAG: usize = 0b10;
pub(crate) const TAG_MASK: usize = 0b11;

/// Thread local data for the helping strategy.
#[derive(Default)]
pub(super) struct Local {
    // The generation counter.
    generation: Cell<usize>,

    // Rotate the slots that are tried first.
    slot_pos: Cell<usize>,
}

/// The slots for the helping strategy.
#[derive(Default)]
pub(super) struct Slots {
    slots: [Debt; DEBT_SLOT_CNT],
    pub(super) active_addr: AtomicUsize,
}

impl Slots {
    pub(super) fn get_debt(&self, ptr: usize, local: &Local) -> (&Debt, bool, usize, bool) {
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
        let offset = local.slot_pos.get();

        let len = self.slots.len();
        for raw_i in 0..len {
            let i = (raw_i + offset) % len;
            let slot = &self.slots[i];

            // We could do load and store separately as we are the only ones allowed to
            // overwrite a NO_DEPT, but we actually need the SeqCst to be a read-write
            // operation in here (we need both the release and acquire part of it).
            if slot
                .0
                .compare_exchange(Debt::NONE, gen, SeqCst, Relaxed)
                .is_ok()
            {
                let mut last = true;
                for raw_j in raw_i + 1..len {
                    let j = (raw_j + offset) % len;
                    if self.slots[j].0.load(Relaxed) == Debt::NONE {
                        last = false;
                        // We already discovered this one will be empty, so store the info for
                        // next time.
                        local.slot_pos.set(j);
                        break;
                    }
                }
                if last {
                    // OK, store _this_ one for next time because we are going to free it up in
                    // a moment.
                    local.slot_pos.set(i);
                }
                return (slot, last, gen, discard);
            }
        }
        unreachable!("Run out of slots");
    }
}

impl<'a> IntoIterator for &'a Slots {
    type Item = &'a Debt;

    type IntoIter = Iter<'a, Debt>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter()
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

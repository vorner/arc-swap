//! Slots and global/thread local data for the Helping strategy.

use std::cell::Cell;
use std::ptr;
use std::sync::atomic::Ordering::*;
use std::sync::atomic::{AtomicPtr, AtomicUsize};

use super::Debt;
use crate::RefCnt;

pub(crate) const REPLACEMENT_TAG: usize = 0b01;
pub(crate) const GEN_TAG: usize = 0b10;
pub(crate) const TAG_MASK: usize = 0b11;
pub(crate) const IDLE: usize = 0;

/// Thread local data for the helping strategy.
#[derive(Default)]
pub(super) struct Local {
    // The generation counter.
    generation: Cell<usize>,
}

// Make sure the pointers have 2 empty bits. Always.
#[derive(Default)]
#[repr(align(4))]
struct Handover(AtomicUsize);

/// The slots for the helping strategy.
pub(super) struct Slots {
    control: AtomicUsize,
    slot: Debt,
    active_addr: AtomicUsize,
    handover: Handover,
    space_offer: AtomicPtr<Handover>,
}

impl Default for Slots {
    fn default() -> Self {
        Slots {
            control: AtomicUsize::new(IDLE),
            slot: Debt::default(),
            // Doesn't matter yet
            active_addr: AtomicUsize::new(0),
            // Also doesn't matter
            handover: Handover::default(),
            // Here we would like it to point to our handover. But for that we need to be in place
            // in RAM (effectively pinned, though we use older Rust than Pin, possibly?), so not
            // yet. See init().
            space_offer: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

impl Slots {
    pub(super) fn slot(&self) -> &Debt {
        &self.slot
    }

    pub(super) fn get_debt(&self, ptr: usize, local: &Local) -> (usize, bool) {
        // Incrementing by 4 ensures we always have enough space for 2 bit of tags.
        let gen = local.generation.get().wrapping_add(4);
        debug_assert_eq!(gen & GEN_TAG, 0);
        local.generation.set(gen);
        let discard = gen == 0;
        let gen = gen | GEN_TAG;
        // We will sync by the write to the control. But we also sync the value of the previous
        // generation/released slot. That way we may re-confirm in the writer that the reader is
        // not in between here and the compare_exchange below with a stale gen (eg. if we are in
        // here, the re-confirm there will load the NO_DEPT and we are fine).
        self.active_addr.store(ptr, Release);

        // We are the only ones allowed to do the IDLE -> * transition and we never leave it in
        // anything else after an transaction, so this is OK. But we still need a load-store SeqCst
        // operation here :-(.
        let prev = self.control.swap(gen, SeqCst);
        debug_assert_eq!(IDLE, prev, "Left control in wrong state");

        (gen, discard)
    }

    pub(super) fn help<R, T>(&self, who: &Self, storage_addr: usize, replacement: &R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        debug_assert_eq!(IDLE, self.control.load(Relaxed));
        let mut control = who.control.load(Acquire);
        loop {
            match control & TAG_MASK {
                // Nothing to help with
                IDLE if control == IDLE => break,
                // Someone has already helped out with that, so we have nothing to do here
                REPLACEMENT_TAG => break,
                // Something is going on, let's have a better look.
                GEN_TAG => {
                    debug_assert!(
                        !ptr::eq(self, who),
                        "Refusing to help myself, makes no sense"
                    );
                    // Get the address that other thread is trying to load from. By that acquire,
                    // we also sync the control into our thread once more and reconfirm that the
                    // value of the active_addr is in between two same instances, therefore up to
                    // date to it.
                    let active_addr = who.active_addr.load(Acquire);
                    if active_addr != storage_addr {
                        let new_control = who.control.load(Acquire);
                        if new_control == control {
                            // The other thread is doing something, but to some other ArcSwap, so
                            // we don't care.
                            break;
                        } else {
                            // The control just changed under our hands, we don't know what to
                            // trust, so retry.
                            control = new_control;
                            continue;
                        }
                    }

                    // Now we know this work is for us. Try to create a replacement and offer it.
                    // This actually does a full-featured load under the hood, but we are currently
                    // idle and the load doesn't re-enter write.
                    let replacement = replacement();
                    let replace_addr = T::as_ptr(&replacement) as usize;
                    let their_space = who.space_offer.load(Acquire);
                    // Relaxed is fine, our own thread and nobody but us writes in here.
                    let my_space = self.space_offer.load(Relaxed);
                    // Relaxed is fine, we'll sync by the next compare-exchange. If we don't, the
                    // value won't ever be read anyway.
                    unsafe {
                        (*my_space).0.store(replace_addr, Relaxed);
                    }
                    let space_addr = (my_space as usize) | REPLACEMENT_TAG;
                    match who
                        .control
                        .compare_exchange(control, space_addr, AcqRel, Acquire)
                    {
                        Ok(_) => {
                            // We have successfully sent our replacement out (Release) and got
                            // their space in return (Acquire on that load above).
                            self.space_offer.store(their_space, Release);
                            // The ref count went with it, so forget about it here.
                            T::into_ptr(replacement);
                            // We have successfully helped out, so we are done.
                            break;
                        }
                        Err(new_control) => {
                            // Something has changed in between. Let's try again, nothing changed
                            // (the replacement will get dropped at the end of scope, we didn't do
                            // anything with the spaces, etc.
                            control = new_control;
                        }
                    }
                }
                _ => unreachable!("Invalid control value {:X}", control),
            }
        }
    }

    pub(super) fn init(&mut self) {
        *self.space_offer.get_mut() = &mut self.handover;
    }

    pub(super) fn confirm(&self, gen: usize, ptr: usize) -> Result<(), usize> {
        // Put the slot there and consider it acquire of a „lock“. For that we need swap, not store
        // only.
        let prev = self.slot.0.swap(ptr as usize, AcqRel);
        debug_assert_eq!(Debt::NONE, prev);

        // Confirm by writing to the control (or discover that we got helped). We stop anyone else
        // from helping by setting it to IDLE.
        let control = self.control.swap(IDLE, AcqRel);
        if control == gen {
            // Nobody interfered, we have our debt in place and can proceed.
            Ok(())
        } else {
            // Someone put a replacement in there.
            debug_assert_eq!(control & TAG_MASK, REPLACEMENT_TAG);
            let handover = (control & !TAG_MASK) as *mut Handover;
            let replacement = unsafe { &*handover }.0.load(Acquire);
            self.space_offer.store(handover, Release);
            // Note we've left the debt in place. The caller should pay it back (without ever
            // taking advantage of it) to make sure any extra is actually dropped (it is possible
            // someone provided the replacement *and* paid the debt and we need just one of them).
            Err(replacement)
        }
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

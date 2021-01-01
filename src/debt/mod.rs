//! Debt handling.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::*;

pub(crate) use self::list::{LocalNode, Node};
use super::RefCnt;

mod fast;
mod helping;
mod list;

/// One debt slot.
///
/// It may contain an „owed“ reference count.
#[derive(Debug)]
pub(crate) struct Debt(pub(crate) AtomicUsize);

impl Debt {
    /// The value of pointer `3` should be pretty safe, for two reasons:
    ///
    /// * It's an odd number, but the pointers we have are likely aligned at least to the word size,
    ///   because the data at the end of the `Arc` has the counters.
    /// * It's in the very first page where NULL lives, so it's not mapped.
    pub(crate) const NONE: usize = 0b11;
}

impl Default for Debt {
    fn default() -> Self {
        Debt(AtomicUsize::new(Self::NONE))
    }
}

impl Debt {
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
    pub(crate) fn pay_all<T, R>(ptr: *const T::Base, storage_addr: usize, replacement: R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        LocalNode::with(|local| {
            let val = unsafe { T::from_ptr(ptr) };
            // Pre-pay one ref count that can be safely put into a debt slot to pay it.
            T::inc(&val);

            Node::traverse::<(), _>(|node| {
                // Make the cooldown trick know we are poking into this node.
                let _reservation = node.reserve_writer();

                let all_slots = node
                    .fast_slots()
                    .chain(std::iter::once(node.helping_slot()));
                for slot in all_slots {
                    if slot
                        .0
                        // TODO: Do we actually need that AcqRel, or is Release enough? Then we
                        // could call into the above pay.
                        .compare_exchange(ptr as usize, Self::NONE, AcqRel, Relaxed)
                        .is_ok()
                    {
                        // Pre-pay one more, for another future slot
                        T::inc(&val);
                    }
                }

                local.help(node, storage_addr, &replacement);

                None
            });
            // Implicit dec by dropping val in here, pair for the above
        })
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

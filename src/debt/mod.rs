//! Debt handling.

use std::sync::atomic::{AtomicUsize, Ordering};

use self::list::{LocalNode, Node};
use super::RefCnt;

mod fast;
mod list;

pub(crate) const NO_DEBT: usize = 0b11;

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
    pub(crate) fn new(ptr: usize) -> Option<&'static Self> {
        LocalNode::with(|local| local.new_fast(ptr))
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
            .compare_exchange(ptr as usize, NO_DEBT, Ordering::Release, Ordering::Relaxed)
            .is_ok()
    }

    /// Pays all the debts on the given pointer.
    pub(crate) fn pay_all<T: RefCnt>(ptr: *const T::Base) {
        let val = unsafe { T::from_ptr(ptr) };
        // Pre-pay one ref count that can be safely put into a debt slot to pay it.
        T::inc(&val);
        Node::traverse::<(), _>(|node| {
            for slot in node.fast_slots() {
                if slot
                    .0
                    // TODO: Do we actually need that AcqRel, or is Release enough? Then we could
                    // call into the above pay.
                    .compare_exchange(ptr as usize, NO_DEBT, Ordering::AcqRel, Ordering::Relaxed)
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

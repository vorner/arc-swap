use std::cell::Cell;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};

use super::RefCnt;

const DEBT_SLOT_CNT: usize = 6;

/// One debt slot.
pub(crate) struct Debt(AtomicUsize);

impl Default for Debt {
    fn default() -> Self {
        Debt(AtomicUsize::new(NO_DEBT))
    }
}

/// One thread-local node for debts.
#[repr(align(64))]
struct Node {
    next: Option<&'static Node>,
    in_use: AtomicBool,
    slots: [Debt; DEBT_SLOT_CNT],
}

impl Default for Node {
    fn default() -> Self {
        Node {
            next: None,
            in_use: AtomicBool::new(true),
            slots: Default::default(),
        }
    }
}

impl Node {
    fn get() -> &'static Self {
        // Try to find an unused one in the chain and reuse it.
        traverse(|node| {
            // Try to claim this node. Nothing is synchronized through this atomic, we only
            // track if someone claims ownership of it.
            if !node.in_use.compare_and_swap(false, true, Ordering::Relaxed) {
                Some(node)
            } else {
                None
            }
        })
        // If that didn't work, create a new one and prepend to the list.
        .unwrap_or_else(|| {
            let node = Box::leak(Box::new(Node::default()));
            // Not shared between threads yet, so ordinary write would be fine too.
            node.in_use.store(true, Ordering::Relaxed);
            // We don't want to read any data in addition to the head, Relaxed is fine
            // here.
            //
            // We do need to release the data to others, but for that, we acquire in the
            // compare_exchange below.
            let mut head = DEBT_HEAD.load(Ordering::Relaxed);
            loop {
                node.next = unsafe { head.as_ref() };
                if let Err(old) = DEBT_HEAD.compare_exchange_weak(
                    head,
                    node,
                    // We need to release *the whole chain* here. For that, we need to
                    // acquire it first.
                    Ordering::AcqRel,
                    Ordering::Relaxed, // Nothing changed, go next round of the loop.
                ) {
                    head = old;
                } else {
                    return node;
                }
            }
        })
    }
}

/// The value of pointer `1` should be pretty safe, for two reasons:
///
/// * It's an odd number, but the pointers we have are likely aligned at least to the word size,
///   because the data at the end of the `Arc` has the counters.
/// * It's in the very first page where NULL lives, so it's not mapped.
pub(crate) const NO_DEBT: usize = 1;

/// The head of the debt chain.
static DEBT_HEAD: AtomicPtr<Node> = AtomicPtr::new(ptr::null_mut());

/// A wrapper around a node pointer, to un-claim the node on thread shutdown.
struct DebtHead(Cell<Option<&'static Node>>);

impl Drop for DebtHead {
    fn drop(&mut self) {
        if let Some(node) = self.0.get() {
            // Nothing synchronized by this atomic.
            assert!(node.in_use.swap(false, Ordering::Relaxed));
        }
    }
}

thread_local! {
    /// A debt node assigned to this thread.
    static THREAD_HEAD: DebtHead = DebtHead(Cell::new(None));
}

/// Goes through the debt linked list.
///
/// This traverses the linked list, calling the closure on each node. If the closure returns
/// `Some`, it terminates with that value early, otherwise it runs to the end.
fn traverse<R, F: FnMut(&'static Node) -> Option<R>>(mut f: F) -> Option<R> {
    // Acquire ‒ we want to make sure we read the correct version of data at the end of the
    // pointer. Any write to the DEBT_HEAD is with Release.
    //
    // Note that the other pointers in the chain never change and are *ordinary* pointers. The
    // whole linked list is synchronized through the head.
    let mut current = unsafe { DEBT_HEAD.load(Ordering::Acquire).as_ref() };
    while let Some(node) = current {
        let result = f(node);
        if result.is_some() {
            return result;
        }
        current = node.next;
    }
    None
}

impl Debt {
    /// Creates a new debt.
    ///
    /// This stores the debt of the given pointer (untyped, casted into an usize) and returns a
    /// reference to that slot, or gives up with `None` if all the slots are currently full.
    ///
    /// This is technically lock-free on the first call in a given thread and wait-free on all the
    /// other accesses.
    // Turn the lint off in clippy, but don't complain anywhere else. clippy::new_ret_no_self
    // doesn't work yet, that thing is not stabilized.
    #[allow(unknown_lints, new_ret_no_self)]
    #[inline]
    pub(crate) fn new(ptr: usize) -> Option<&'static Self> {
        THREAD_HEAD
            .try_with(|head| {
                // Already have my own node?
                if let Some(node) = head.0.get() {
                    return node;
                }
                let new_node = Node::get();
                head.0.set(Some(new_node));
                new_node
            })
            .ok()
            .and_then(|node| {
                debug_assert!(node.in_use.load(Ordering::Relaxed)); // Check it is in use by *us*
                node.slots.iter().find(|slot| {
                    slot.0
                        // Try to claim one of the slots. If it doesn't work and we change nothing,
                        // Relaxed is enough.
                        //
                        // This Acquire is like locking a Mutex ‒ the dangerous stuff stays after
                        // this and can't be reordered before it. The acquire works, part of this
                        // is a load.
                        .compare_exchange(NO_DEBT, ptr, Ordering::SeqCst, Ordering::Relaxed)
                        .is_ok()
                })
            })
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
        T::inc(&val);
        traverse::<(), _>(|node| {
            for slot in &node.slots {
                if slot
                    .0
                    .compare_exchange(ptr as usize, NO_DEBT, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    T::inc(&val);
                }
            }
            None
        });
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

//! A linked list of debt nodes.
//!
//! A node may or may not be owned by a thread. Reader debts are allocated in its owned node,
//! writer walks everything (but may also use some owned values).
//!
//! The list is prepend-only ‒ if thread dies, the node lives on (and can be claimed by another
//! thread later on). This makes the implementation much simpler, since everything here is
//! `'static` and we don't have to care about knowing when to free stuff.
//!
//! The nodes contain both the fast primary slots and a secondary fallback ones.
//!
//! # Synchronization
//!
//! We synchronize several things here.
//!
//! The addition of nodes is synchronized through the head (Load on each read, AcqReal on each
//! attempt to add another node). Note that certain parts never change after that (they aren't even
//! atomic) and other things that do change take care of themselves (the debt slots have their own
//! synchronization, etc).
//!
//! The ownership is acquire-release lock pattern.
//!
//! Similar, the counting of active writers is an acquire-release lock pattern.
//!
//! We also do release-acquire "send" from the start-cooldown to check-cooldown to make sure we see
//! at least as up to date value of the writers as when the cooldown started. That we if we see 0,
//! we know it must have happened since then.

use core::cell::Cell;
use core::ptr;
use alloc::slice::Iter;
use core::sync::atomic::Ordering::*;
use core::sync::atomic::{AtomicPtr, AtomicUsize};
use alloc::boxed::Box;

use super::fast::{Local as FastLocal, Slots as FastSlots};
use super::helping::{Local as HelpingLocal, Slots as HelpingSlots};
use super::Debt;
use crate::RefCnt;

const NODE_UNUSED: usize = 0;
const NODE_USED: usize = 1;
const NODE_COOLDOWN: usize = 2;

/// The head of the debt linked list.
static LIST_HEAD: AtomicPtr<Node> = AtomicPtr::new(ptr::null_mut());

pub struct NodeReservation<'a>(&'a Node);

impl Drop for NodeReservation<'_> {
    fn drop(&mut self) {
        self.0.active_writers.fetch_sub(1, Release);
    }
}

/// One thread-local node for debts.
#[repr(C, align(64))]
pub(crate) struct Node {
    fast: FastSlots,
    helping: HelpingSlots,
    in_use: AtomicUsize,
    next: Option<&'static Node>,
    active_writers: AtomicUsize,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            fast: FastSlots::default(),
            helping: HelpingSlots::default(),
            in_use: AtomicUsize::new(NODE_USED),
            next: None,
            active_writers: AtomicUsize::new(0),
        }
    }
}

impl Node {
    /// Goes through the debt linked list.
    ///
    /// This traverses the linked list, calling the closure on each node. If the closure returns
    /// `Some`, it terminates with that value early, otherwise it runs to the end.
    pub(crate) fn traverse<R, F: FnMut(&'static Node) -> Option<R>>(mut f: F) -> Option<R> {
        // Acquire ‒ we want to make sure we read the correct version of data at the end of the
        // pointer. Any write to the DEBT_HEAD is with Release.
        //
        // Note that the other pointers in the chain never change and are *ordinary* pointers. The
        // whole linked list is synchronized through the head.
        let mut current = unsafe { LIST_HEAD.load(Acquire).as_ref() };
        while let Some(node) = current {
            let result = f(node);
            if result.is_some() {
                return result;
            }
            current = node.next;
        }
        None
    }

    /// Put the current thread node into cooldown
    fn start_cooldown(&self) {
        // Trick: Make sure we have an up to date value of the active_writers in this thread, so we
        // can properly release it below.
        let _reservation = self.reserve_writer();
        assert_eq!(NODE_USED, self.in_use.swap(NODE_COOLDOWN, Release));
    }

    /// Perform a cooldown if the node is ready.
    ///
    /// See the ABA protection at the [helping].
    fn check_cooldown(&self) {
        // Check if the node is in cooldown, for two reasons:
        // * Skip most of nodes fast, without dealing with them.
        // * More importantly, sync the value of active_writers to be at least the value when the
        //   cooldown started. That way we know the 0 we observe happened some time after
        //   start_cooldown.
        if self.in_use.load(Acquire) == NODE_COOLDOWN {
            // The rest can be nicely relaxed ‒ no memory is being synchronized by these
            // operations. We just see an up to date 0 and allow someone (possibly us) to claim the
            // node later on.
            if self.active_writers.load(Relaxed) == 0 {
                let _ = self
                    .in_use
                    .compare_exchange(NODE_COOLDOWN, NODE_UNUSED, Relaxed, Relaxed);
            }
        }
    }

    /// Mark this node that a writer is currently playing with it.
    pub fn reserve_writer(&self) -> NodeReservation {
        self.active_writers.fetch_add(1, Acquire);
        NodeReservation(self)
    }

    /// "Allocate" a node.
    ///
    /// Either a new one is created, or previous one is reused. The node is claimed to become
    /// in_use.
    fn get() -> &'static Self {
        // Try to find an unused one in the chain and reuse it.
        Self::traverse(|node| {
            node.check_cooldown();
            if node
                .in_use
                // We claim a unique control over the generation and the right to write to slots if
                // they are NO_DEPT
                .compare_exchange(NODE_UNUSED, NODE_USED, Acquire, Relaxed)
                .is_ok()
            {
                Some(node)
            } else {
                None
            }
        })
        // If that didn't work, create a new one and prepend to the list.
        .unwrap_or_else(|| {
            let node = Box::leak(Box::new(Node::default()));
            node.helping.init();
            // We don't want to read any data in addition to the head, Relaxed is fine
            // here.
            //
            // We do need to release the data to others, but for that, we acquire in the
            // compare_exchange below.
            let mut head = LIST_HEAD.load(Relaxed);
            loop {
                node.next = unsafe { head.as_ref() };
                if let Err(old) = LIST_HEAD.compare_exchange_weak(
                    head, node,
                    // We need to release *the whole chain* here. For that, we need to
                    // acquire it first.
                    AcqRel, Relaxed, // Nothing changed, go next round of the loop.
                ) {
                    head = old;
                } else {
                    return node;
                }
            }
        })
    }

    /// Iterate over the fast slots.
    pub(crate) fn fast_slots(&self) -> Iter<Debt> {
        self.fast.into_iter()
    }

    /// Access the helping slot.
    pub(crate) fn helping_slot(&self) -> &Debt {
        self.helping.slot()
    }
}

/// A wrapper around a node pointer, to un-claim the node on thread shutdown.
pub(crate) struct LocalNode {
    /// Node for this thread, if any.
    ///
    /// We don't necessarily have to own one, but if we don't, we'll get one before the first use.
    node: Cell<Option<&'static Node>>,

    /// Thread-local data for the fast slots.
    fast: FastLocal,

    /// Thread local data for the helping strategy.
    helping: HelpingLocal,
}

impl LocalNode {
    pub(crate) fn with<R, F: FnOnce(&LocalNode) -> R>(f: F) -> R {
        let f = Cell::new(Some(f));
        thread_head_try_with(|head| {
                if head.node.get().is_none() {
                    head.node.set(Some(Node::get()));
                }
                let f = f.take().unwrap();
                f(head)
            })
            // During the application shutdown, the thread local storage may be already
            // deallocated. In that case, the above fails but we still need something. So we just
            // find or allocate a node and use it just once.
            //
            // Note that the situation should be very very rare and not happen often, so the slower
            // performance doesn't matter that much.
            .unwrap_or_else(|_| {
                let tmp_node = LocalNode {
                    node: Cell::new(Some(Node::get())),
                    fast: FastLocal::default(),
                    helping: HelpingLocal::default(),
                };
                let f = f.take().unwrap();
                f(&tmp_node)
                // Drop of tmp_node -> sends the node we just used into cooldown.
            })
    }

    /// Creates a new debt.
    ///
    /// This stores the debt of the given pointer (untyped, casted into an usize) and returns a
    /// reference to that slot, or gives up with `None` if all the slots are currently full.
    #[inline]
    pub(crate) fn new_fast(&self, ptr: usize) -> Option<&'static Debt> {
        let node = &self.node.get().expect("LocalNode::with ensures it is set");
        debug_assert_eq!(node.in_use.load(Relaxed), NODE_USED);
        node.fast.get_debt(ptr, &self.fast)
    }

    /// Initializes a helping slot transaction.
    ///
    /// Returns the generation (with tag).
    pub(crate) fn new_helping(&self, ptr: usize) -> usize {
        let node = &self.node.get().expect("LocalNode::with ensures it is set");
        debug_assert_eq!(node.in_use.load(Relaxed), NODE_USED);
        let (gen, discard) = node.helping.get_debt(ptr, &self.helping);
        if discard {
            // Too many generations happened, make sure the writers give the poor node a break for
            // a while so they don't observe the generation wrapping around.
            node.start_cooldown();
            self.node.take();
        }
        gen
    }

    /// Confirm the helping transaction.
    ///
    /// The generation comes from previous new_helping.
    ///
    /// Will either return a debt with the pointer, or a debt to pay and a replacement (already
    /// protected) address.
    pub(crate) fn confirm_helping(
        &self,
        gen: usize,
        ptr: usize,
    ) -> Result<&'static Debt, (&'static Debt, usize)> {
        let node = &self.node.get().expect("LocalNode::with ensures it is set");
        debug_assert_eq!(node.in_use.load(Relaxed), NODE_USED);
        let slot = node.helping_slot();
        node.helping
            .confirm(gen, ptr)
            .map(|()| slot)
            .map_err(|repl| (slot, repl))
    }

    /// The writer side of a helping slot.
    ///
    /// This potentially helps the `who` node (uses self as the local node, which must be
    /// different) by loading the address that one is trying to load.
    pub(super) fn help<R, T>(&self, who: &Node, storage_addr: usize, replacement: &R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        let node = &self.node.get().expect("LocalNode::with ensures it is set");
        debug_assert_eq!(node.in_use.load(Relaxed), NODE_USED);
        node.helping.help(&who.helping, storage_addr, replacement)
    }
}

impl Drop for LocalNode {
    fn drop(&mut self) {
        if let Some(node) = self.node.get() {
            // Release - syncing writes/ownership of this Node
            node.start_cooldown();
        }
    }
}

#[cfg(not(feature = "no_std"))]
extern crate std;
#[cfg(not(feature = "no_std"))]
use std::thread_local;
#[cfg(not(feature = "no_std"))]
thread_local! {
    /// A debt node assigned to this thread.
    static THREAD_HEAD: LocalNode = LocalNode {
        node: Cell::new(None),
        fast: FastLocal::default(),
        helping: HelpingLocal::default(),
    };
}
#[cfg(not(feature = "no_std"))]
fn thread_head_try_with<F, R>(f: F) -> Result<R, std::thread::AccessError>
where
    F: FnOnce(&LocalNode) -> R,
{
    THREAD_HEAD.try_with(f)
}

#[cfg(feature = "no_std")]
use static_init::dynamic;
#[cfg(feature = "no_std")]
#[dynamic(drop)]
#[thread_local]
static THREAD_HEAD: LocalNode = LocalNode {
    node: Cell::new(None),
    fast: FastLocal::default(),
    helping: HelpingLocal::default(),
};
#[cfg(feature = "no_std")]
fn thread_head_try_with<F, R>(f: F) -> Result<R, ()>
    where
        F: FnOnce(&LocalNode) -> R,
{
    Result::Ok(f(&THREAD_HEAD))
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Node {
        fn is_empty(&self) -> bool {
            self.fast_slots()
                .chain(core::iter::once(self.helping_slot()))
                .all(|d| d.0.load(Relaxed) == Debt::NONE)
        }

        fn get_thread() -> &'static Self {
            LocalNode::with(|h| h.node.get().unwrap())
        }
    }

    /// A freshly acquired thread local node is empty.
    #[test]
    fn new_empty() {
        assert!(Node::get_thread().is_empty());
    }
}

//! A strategy where the writer is helping the reader in case of collision.

// TODO: Figure out how much we can relax the orderings in here. The ones from Debts are copied, so
// already proven to be OK, but the new ones need some thinking.

use std::borrow::Borrow;
use std::cell::Cell;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::Ordering::*;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize};

use super::sealed::{InnerStrategy, Protected};
use crate::RefCnt;

/// A wrapper around a node pointer, to un-claim the node on thread shutdown.
struct DebtHead {
    // Node for this thread.
    node: Cell<Option<&'static Node>>,
}

impl Drop for DebtHead {
    fn drop(&mut self) {
        if let Some(node) = self.node.get() {
            // TODO: Ordering
            assert!(node.in_use.swap(false, Release));
        }
    }
}

/// The value of pointer `3` should be pretty safe, for two reasons:
///
/// * It's an odd number, but the pointers we have are likely aligned at least to the word size,
///   because the data at the end of the `Arc` has the counters.
/// * It's in the very first page where NULL lives, so it's not mapped.
///
/// Note that we don't want to use 0 directly, because NULL is a valid pointer.
const NO_DEBT: usize = 0b11;
const REPLACEMENT_TAG: usize = 0b01;
const GEN_TAG: usize = 0b10;
const TAG_MASK: usize = 0b11;
const DEBT_SLOT_CNT: usize = 8;

thread_local! {
    /// A debt node assigned to this thread.
    static THREAD_HEAD: DebtHead = DebtHead {
        node: Cell::new(None),
    };
}

/// A mostly copy of the Dept from elsewhere
///
/// For now, we keep a separate copy, both for simplicity and because we handle them diffeerntly
/// based on the strategy used. We may either delete one copy eventually or unify them, but for the
/// experiment, let's just copy-paste..
struct Debt(AtomicUsize);

impl Debt {
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    fn new(ptr: usize) -> (&'static Self, bool, usize) {
        let node = Node::get_thread();
        // Check it is in use by *us*
        debug_assert!(node.in_use.load(Relaxed));

        // We are sole users of this node. We could actually use something like UnsafeCell
        // here directly.
        let gen = node.generation.load(Relaxed).wrapping_add(1);
        node.generation.store(gen, Relaxed);
        let gen = gen << 2 | GEN_TAG;
        // We will sync by the write to the debt.
        node.active_addr.store(ptr, Relaxed);

        let len = node.slots.0.len();
        for (i, slot) in node.slots.0.iter().enumerate() {
            let got_it = slot
                .0
                // Try to acquire the slot. Relaxed if it doesn't work is fine, as we don't
                // synchronize by it.
                .compare_exchange(NO_DEBT, gen, SeqCst, Relaxed)
                .is_ok();
            let last = i == len - 1;
            if got_it {
                return (slot, last, gen);
            }
        }
        unreachable!("Run out of slots in {:#?}", node);
    }

    /// Confirms the debt.
    ///
    /// Replaces the pre-debt mark (the address we are reading from) with the actual debt and
    /// confirm it that way.
    ///
    /// If it got changed in the meantime, returns the (already protected) replacement value.
    #[inline]
    fn confirm(&self, gen: usize, ptr_addr: usize) -> Result<(), usize> {
        // TODO: Lower ordering?
        match self.0.compare_exchange(gen, ptr_addr, SeqCst, Acquire) {
            Ok(_) => Ok(()),
            Err(replacement) => {
                self.0.store(NO_DEBT, SeqCst);
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
    /// * It is possible the slot got reused and currently contains the same pointer, but tagged as
    ///   already protected. That is fine, as the raw value will not match and we won't change
    ///   that.
    #[inline]
    fn pay<T: RefCnt>(&self, ptr: *const T::Base) -> bool {
        self.0
            // If we don't change anything because there's something else, Relaxed is fine.
            //
            // The Release works as kind of Mutex. We make sure nothing from the debt-protected
            // sections leaks below this point.
            .compare_exchange(ptr as usize, NO_DEBT, Release, Relaxed)
            .is_ok()
    }

    /// Pays all the debts on the given pointer.
    #[inline]
    fn pay_all<T, R>(ptr: *const T::Base, storage_addr: usize, replacement: R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        let val = unsafe { T::from_ptr(ptr) };
        // Pre-pay one ref count that can be safely put into a debt slot to pay it.
        T::inc(&val);

        Node::traverse::<(), _>(|node| {
            for slot in &node.slots.0 {
                loop {
                    match slot
                        .0
                        // TODO: Ordering
                        .compare_exchange(ptr as usize, NO_DEBT, AcqRel, Acquire)
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
                            let active_addr = node.active_addr.load(Relaxed);
                            if active_addr != storage_addr {
                                // TODO: Are we sure this is fine? Some proofs that this is up to
                                // date and nothing can slip (eg. that by now it could be
                                // containing the same address)
                                break;
                            }
                            // Get a replacement value and try to donate it.
                            let replacement = replacement();
                            let replace_addr = T::as_ptr(&replacement) as usize;
                            assert_eq!(replace_addr & TAG_MASK, 0, "Not enough space for tags");
                            let replace_addr = replace_addr | REPLACEMENT_TAG;
                            if slot
                                .0
                                .compare_exchange_weak(gen, replace_addr, SeqCst, Relaxed)
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

impl Default for Debt {
    fn default() -> Self {
        Debt(AtomicUsize::new(NO_DEBT))
    }
}

#[repr(align(64))]
#[derive(Default)]
struct Slots([Debt; DEBT_SLOT_CNT]);

impl Debug for Slots {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_list()
            .entries(self.0.iter().map(|s| s.0.load(Relaxed) as *const u8))
            .finish()
    }
}

/// One thread-local node for debts.
#[repr(C)]
struct Node {
    slots: Slots,
    next: Option<&'static Node>,
    in_use: AtomicBool,
    active_addr: AtomicUsize,
    // TODO: Could this be not atomic, but merely UnsafeCell, locked by in_use?
    generation: AtomicUsize,
}

/// The head of the debt chain.
static DEBT_HEAD: AtomicPtr<Node> = AtomicPtr::new(ptr::null_mut());

impl Default for Node {
    fn default() -> Self {
        Node {
            next: None,
            in_use: AtomicBool::new(true),
            slots: Default::default(),
            active_addr: AtomicUsize::new(0),
            generation: AtomicUsize::new(0),
        }
    }
}

impl Node {
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
        let mut current = unsafe { DEBT_HEAD.load(Acquire).as_ref() };
        while let Some(node) = current {
            let result = f(node);
            if result.is_some() {
                return result;
            }
            current = node.next;
        }
        None
    }

    #[inline(never)]
    fn get() -> &'static Self {
        // Try to find an unused one in the chain and reuse it.
        Self::traverse(|node| {
            // TODO: Ordering
            if node
                .in_use
                .compare_exchange(false, true, SeqCst, Relaxed)
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
            // Not shared between threads yet, so ordinary write would be fine too.
            node.in_use.store(true, Relaxed);
            // We don't want to read any data in addition to the head, Relaxed is fine
            // here.
            //
            // We do need to release the data to others, but for that, we acquire in the
            // compare_exchange below.
            let mut head = DEBT_HEAD.load(Relaxed);
            loop {
                node.next = unsafe { head.as_ref() };
                if let Err(old) = DEBT_HEAD.compare_exchange_weak(
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

    #[inline]
    fn get_thread() -> &'static Self {
        THREAD_HEAD
            // FIXME: Can this actually fail sometime? Can people use us during some kind of mutual
            // thread-local destruction thing?
            .with(|head| {
                match head.node.get() {
                    // Already have my own node (most likely)?
                    Some(node) => node,
                    // No node yet, called for the first time in this thread. Set one up.
                    None => {
                        let new_node = Node::get();
                        head.node.set(Some(new_node));
                        new_node
                    }
                }
            })
    }
}

impl Debug for Node {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Node")
            .field("in_use", &self.in_use.load(Relaxed))
            .field("next", &self.next)
            .field("generation", &self.generation.load(Relaxed))
            .field(
                "active_addr",
                &(self.active_addr.load(Relaxed) as *const u8),
            )
            .field("slots", &self.slots)
            .finish()
    }
}

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
/// copies of the ones used by the hybrid strategy, but modified a bit.
///
/// The strategy is:
///
/// * Wait-free on readers (merely lock-free on the first use on each new OS thread).
/// * Lock free on writers.
/// * Precise (data is dropped as soon as possible).
#[derive(Copy, Clone, Debug, Default)]
pub struct Helping;

impl<T: RefCnt> InnerStrategy<T> for Helping {
    type Protected = Protection<T>;

    #[inline]
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        // First, we claim a debt slot and store the address of the atomic pointer there, so the
        // writer can optionally help us out with loading and protecting something.
        let (debt, last, gen) = Debt::new(storage as *const _ as usize);
        // Then we load the candidate from the pointer.
        // TODO: The ordering?
        let candidate = storage.load(SeqCst);
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
                debug_assert_eq!(NO_DEBT, debt.0.load(Relaxed));
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
        Debt::pay_all::<T, _>(old, storage as *const _ as usize, replacement);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    impl Node {
        fn is_empty(&self) -> bool {
            self.slots.0.iter().all(|d| d.0.load(Relaxed) == NO_DEBT)
        }
    }

    /// A freshly acquired thread local node is empty.
    #[test]
    fn new_empty() {
        assert!(Node::get_thread().is_empty());
    }

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
}

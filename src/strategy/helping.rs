//! A strategy where the writer is helping the reader in case of collision.

// TODO: Figure out how much we can relax the orderings in here. The ones from Debts are copied, so
// already proven to be OK, but the new ones need some thinking.

use std::borrow::Borrow;
use std::cell::Cell;
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};

use super::sealed::{InnerStrategy, Protected};
use crate::RefCnt;

use Ordering::*;

/// A wrapper around a node pointer, to un-claim the node on thread shutdown.
struct DebtHead {
    // Node for this thread.
    node: Cell<Option<&'static Node>>,
}

impl Drop for DebtHead {
    fn drop(&mut self) {
        if let Some(node) = self.node.get() {
            // Nothing synchronized by this atomic.
            assert!(node.in_use.swap(false, Relaxed));
        }
    }
}

/// The value of pointer `1` should be pretty safe, for two reasons:
///
/// * It's an odd number, but the pointers we have are likely aligned at least to the word size,
///   because the data at the end of the `Arc` has the counters.
/// * It's in the very first page where NULL lives, so it's not mapped.
const NO_DEBT: usize = 1;
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
    /// Creates a new debt.
    ///
    /// This stores the debt of the given pointer (untyped, casted into an usize) and returns a
    /// reference to that slot. The second part of the result signifies if this was the last empty
    /// slot available, therefore it needs to be freed ASAP (eg. before leaving that function).
    ///
    /// This is technically lock-free on the first call in a given thread and wait-free on all the
    /// other accesses.
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    fn new(ptr: usize) -> (&'static Self, bool) {
        THREAD_HEAD
            // FIXME: Can this actually fail sometime? Can people use us during some kind of mutual
            // thread-local destruction thing?
            .with(|head| {
                let node = match head.node.get() {
                    // Already have my own node (most likely)?
                    Some(node) => node,
                    // No node yet, called for the first time in this thread. Set one up.
                    None => {
                        let new_node = Node::get();
                        head.node.set(Some(new_node));
                        new_node
                    }
                };
                // Check it is in use by *us*
                debug_assert!(node.in_use.load(Relaxed));
                let len = node.slots.0.len();
                for i in 0..len {
                    // Note: the indexing check is almost certainly optimised out because the len
                    // is used above. And using .get_unchecked was actually *slower*.
                    let got_it = node.slots.0[i]
                        .0
                        // Try to acquire the slot. Relaxed if it doesn't work is fine, as we don't
                        // synchronize by it.
                        .compare_exchange(NO_DEBT, ptr, SeqCst, Relaxed)
                        .is_ok();
                    let last = i == len - 1;
                    if got_it {
                        return (&node.slots.0[i], last);
                    }
                }
                unreachable!("Run out of slots");
            })
    }

    /// Confirms the debt.
    ///
    /// Replaces the pre-debt mark (the address we are reading from) with the actual debt and
    /// confirm it that way.
    ///
    /// If it got changed in the meantime, returns the (already protected) replacement value.
    #[inline]
    fn confirm(&self, storage_addr: usize, ptr_addr: usize) -> Result<(), usize> {
        // TODO: Lower ordering?
        match self
            .0
            .compare_exchange(storage_addr, ptr_addr, SeqCst, Relaxed)
        {
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
            .compare_exchange(ptr as usize, NO_DEBT, Ordering::Release, Ordering::Relaxed)
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
                        .compare_exchange(ptr as usize, NO_DEBT, AcqRel, Relaxed)
                    {
                        Ok(_) => {
                            // We just paid the debt by the pre-paid ref count from before.
                            // Pre-pay one more, for another future slot
                            T::inc(&val);
                            break;
                        }
                        Err(addr) if addr == storage_addr => {
                            // We are touching the same address someone is currently trying to load
                            // from. Because we don't want to wait for them, we are going to help
                            // them out by giving them an already fully-protected replacement
                            // pointer and move on. They'll pick it up from this slot.
                            let replace = replacement();
                            let replace_addr = T::as_ptr(&replace) as usize;
                            assert_eq!(
                                replace_addr & Helping::REPLACEMENT_TAG,
                                0,
                                "Not enough space for tags"
                            );
                            let replace_addr = replace_addr | Helping::REPLACEMENT_TAG;
                            if slot
                                .0
                                .compare_exchange_weak(addr, replace_addr, SeqCst, Relaxed)
                                .is_ok()
                            {
                                // OK, it went it
                                T::into_ptr(replace);
                                break;
                            }
                            // else -> replace is dropped.
                            // Also, loop once more because the current slot did *not* get
                            // resolved. Retry and see if the reader already got what it wanted or
                            // try creating a new replacement.
                        }
                        // Something uninteresting, just continue
                        Err(_) => break,
                    }
                }
            }
            None
        });
        // Implicit dec by dropping val in here, pair for the above
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

/// One thread-local node for debts.
#[repr(C)]
struct Node {
    slots: Slots,
    next: Option<&'static Node>,
    in_use: AtomicBool,
}

/// The head of the debt chain.
static DEBT_HEAD: AtomicPtr<Node> = AtomicPtr::new(ptr::null_mut());

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
    fn get() -> &'static Self {
        // Try to find an unused one in the chain and reuse it.
        Self::traverse(|node| {
            // Try to claim this node. Nothing is synchronized through this atomic, we only
            // track if someone claims ownership of it.
            if !node.in_use.compare_and_swap(false, true, Relaxed) {
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

impl Helping {
    const STORAGE_TAG: usize = 0b10;
    const REPLACEMENT_TAG: usize = 0b01;

    fn storage_addr<T>(storage: &AtomicPtr<T>) -> usize {
        let addr = storage as *const _ as usize;
        assert_eq!(addr & Self::STORAGE_TAG, 0, "Not enough space for tags");
        addr | Self::STORAGE_TAG
    }
}

impl<T: RefCnt> InnerStrategy<T> for Helping {
    type Protected = Protection<T>;

    #[inline]
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected {
        let storage_addr = Self::storage_addr(storage);
        // First, we claim a debt slot and store the address of the atomic pointer there, so the
        // writer can optionally help us out with loading and protecting something.
        let (debt, last) = Debt::new(storage_addr);
        // Then we load the candidate from the pointer.
        // TODO: The ordering?
        let candidate = storage.load(SeqCst);
        let ptr_addr = candidate as usize;
        // Try to replace the debt with our candidate.
        match debt.confirm(storage_addr, ptr_addr) {
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
                // the slot is paid back.
                debug_assert_eq!(
                    replacement & Self::REPLACEMENT_TAG,
                    Self::REPLACEMENT_TAG,
                    "Missing tag on replacement",
                );
                let replacement = replacement & !Self::REPLACEMENT_TAG; // Remove the tag
                Protection::new(replacement as *const _, None)
            }
        }
    }

    // FIXME: There's a time-travel problem. It can happen (and the multi_writer test sometimes
    // exhibit it) that we find an address in the slot, try to provide a replacement. But we are
    // slow doing so, in the meantime the reader reads its value. Then when we try to put it there,
    // another round of the reader happened and there's the address again, so we never notice it
    // already changed in between and store the, now stale, value. It's a bit like the ABA problem.
    //
    // Any idea how to solve?
    unsafe fn wait_for_readers(&self, old: *const T::Base, storage: &AtomicPtr<T::Base>) {
        let storage_addr = Self::storage_addr(storage);
        // The pay_all may need to provide fresh replacement values if someone else is loading from
        // this particular storage. We do so by the exact same way, by `load` ‒ it's OK, a writer
        // does not hold a slot and the reader doesn't recurse back into writer, so we won't run
        // out of slots.
        let replacement = || {
            Protection::into_inner(self.load(storage))
        };
        Debt::pay_all::<T, _>(old, storage_addr, replacement);
    }
}

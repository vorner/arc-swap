//! A strategy where the writer is helping the reader in case of collision.

// # How does this work.
//
// This is a bit modified version of hazard pointers. The readers put the currently leased pointers
// into debt slots. That means the reader owes one reference count. But as long as there's one
// reference inside the storage, everything is fine.
//
// When a writer takes the pointer out, it walks all the old debt slots that got created before the
// change and pays them all by incrementing the reference count as many times, and erases the debts
// so the readers knows they need to decrement it after the fact.
//
// ## Reader, the fast path
//
// * Publish an active address ‒ the address we'll be loading stuff from.
// * Picks a debt slot (note that we always keep at least one free; if we used the last one, we
//   immediately pay the debt by bumping the reference count and release it and we never
//   recursively call load). Mark it with a generation (this is to prevent ABA in writers).
// * Load from the address.
// * CaS-replace the generation with the address of the destination, reserving the debt. At this
//   point, we are good to do our stuff.
//
// * Later, we pay it back by CaS-replacing it with the NO_DEPT.
//
// ## Writer, the non-colliding path
//
// * Replaces the pointer in the storage.
// * The writer walks over all debts. It pays each debt that it is concerned with by bumping the
//   reference and replacing the dept with NO_DEPT. The relevant reader will fail in the CaS
//   (either because it finds NO_DEPT or other pointer in there) and knows the reference was
//   bumped, so it needs to decrement it. Note that it is possible that someone also reuses the
//   slot for the _same_ pointer. In that case that reader will set it to NO_DEPT and the newer
//   reader will have a pre-paid debt, which is fine.
//
// ## The collision path
//
// The reservation of a slot is not atomic, therefore a writer can observe the reservation in
// progress. But it doesn't want to wait for it to complete (it wants to be lock-free, which means
// it needs to be able to resolve the situation on its own).
//
// The way it knows it is in progress of the reservation is by seeing a generation in there (it has
// a distinct tag). In that case it'll try to:
//
// * First verify that the reservation is being done for the same address it modified, by reading
//   and re-confirming the active_addr slot corresponding to the currently handled node. If it is
//   for some other address, the writer doesn't have to be concerned and proceeds to the next slot.
// * It does a full load. That is fine, because the writer must be on a different thread than the
//   reader and therefore there is at least one free slot. Full load means paying the debt right
//   away by incrementing the reference count.
// * Then it tries to pass the already fully protected/paid pointer to the reader, by CaS-replacing
//   the previously observed generation with the pointer. If it fails, it decrements the count
//   again and retries on the same slot, as it might now contain a debt for the same pointer it is
//   being concerned.
// * The reader then finds the generation got replaced by a pointer (tagged with the right tag, so
//   some other writer doesn't try to "pay" a debt that is actually a negative debt). It can then
//   proceed to using that one instead of what it loaded (and not ever retry anything).
//
// ## ABA protection
//
// The generation as pre-reserving the slot allows the writer to make sure it is offering the
// loaded pointer to the same reader and that the read value is new enough (and of the same type).
//
// This solves the general case, but there's also much less frequent but theoretical ABA problem
// that could lead to UB, if left unsolved:
//
// * There is a collision on generation G.
// * The writer loads a pointer, bumps it.
// * In the meantime, all the 2^30 or 2^62 generations (depending on the usize width) generations
//   wrap around.
// * The writer stores the outdated and possibly different-typed pointer in there and the reader
//   uses it.
//
// To mitigate that, every time the counter overflows we take the current node and un-assign it
// from our current thread. We mark it as in "cooldown" and let it in there until there are no
// writers messing with that node any more (if they are not on the node, they can't experience the
// ABA problem on it). After that, we are allowed to use it again.
//
// This doesn't block the reader, it'll simply find *a* node next time ‒ this one, or possibly a
// different (or new) one.
//
// # Orderings
//
// The debt linked list is the easy part:
// * The head is used to synchronize the non-atomic content of it through trivial Acquire /
//   Release. The non-atomic parts are never written to after publishing the node (publish
//   pattern).
// * The in_use field is used as a form of lock, to claim exclusive use / write privilege of some
//   of the fields, also with Acquire / Release (mutex pattern).
//
// The actual debt juggling is a bit more interesting. We need to ensure that:
//
// * We don't start using anything before the debt is installed.
// * We don't release the swapped-out pointer to the caller before all debts acquired from
//   previously loading the old value are paid (and none are somewhere in process).
//
// We use SeqCst both on the change of the pointer and on the initial setting of the generation
// into the debt slot. That way we can be sure both threads agree on what happened first.
//
// Then all things (with the small exception about the current address, see the comments inline)
// happen on the same debt slot, therefore the time line is well established on that. Everything
// that synchronizes anything is Acquire/Release as needed there.
//
// There's a gotcha in the destructor ‒ that one doesn't have the write with SeqCst on it. But at
// that time other things must have assured that there's an exclusive access to the ArcSwap
// (because drop takes &mut self) and that also assures nobody is currently loading anything ‒ all
// the debts that are there must have existed before that exclusive ownership was acquired by
// whoever drops it.

use std::borrow::Borrow;
use std::cell::Cell;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::mem::{self, ManuallyDrop};
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::Ordering::*;
use std::sync::atomic::{AtomicPtr, AtomicUsize};

use super::sealed::{CaS, InnerStrategy, Protected};
use crate::{AsRaw, RefCnt};

const NODE_UNUSED: usize = 0;
const NODE_USED: usize = 1;
const NODE_COOLDOWN: usize = 2;

/// A wrapper around a node pointer, to un-claim the node on thread shutdown.
struct DebtHead {
    // Node for this thread.
    node: Cell<Option<&'static Node>>,

    // The generation counter.
    generation: Cell<usize>,

    // Rotate the slots that are tried first.
    slot_pos: Cell<usize>,
}

impl Drop for DebtHead {
    fn drop(&mut self) {
        if let Some(node) = self.node.get() {
            // Release - syncing writes/ownership of this Node
            // We put it to cooldown, because we implicitly reset the generation count by giving it
            // to another thread. So just to be on the safe side and make sure no writers sees it
            // across that reset.
            assert_eq!(node.in_use.swap(NODE_COOLDOWN, Release), NODE_USED);
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
        generation: Cell::new(0),
        slot_pos: Cell::new(0),
    };
}

/// A mostly copy of the Dept from elsewhere
///
/// For now, we keep a separate copy, both for simplicity and because we handle them differently
/// based on the strategy used. We may either delete one copy eventually or unify them, but for the
/// experiment, let's just copy-paste..
struct Debt(AtomicUsize);

impl Debt {
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    fn new(ptr: usize) -> (&'static Debt, bool, usize) {
        Node::with_thread_head(|head| {
            let node = head
                .node
                .get()
                .expect("with_thread_head ensures we have one");
            // Check it is in use by *us*
            debug_assert_eq!(node.in_use.load(Relaxed), NODE_USED);

            // Incrementing by 4 ensures we always have enough space for 2 bit of tags.
            let gen = head.generation.get().wrapping_add(4);
            debug_assert_eq!(gen & GEN_TAG, 0);
            head.generation.set(gen);
            let discard = gen == 0;
            let gen = gen | GEN_TAG;
            // We will sync by the write to the debt. But we also sync the value of the previous
            // generation/released slot. That way we may re-confirm in the writer that the reader is
            // not in between here and the compare_exchange below with a stale gen (eg. if we are in
            // here, the re-confirm there will load the NO_DEPT and we are fine).
            node.active_addr.store(ptr, Release);
            let offset = head.slot_pos.get();

            let len = node.slots.0.len();
            for raw_i in 0..len {
                let i = (raw_i + offset) % len;
                let slot = &node.slots.0[i];

                // We could do load and store separately as we are the only ones allowed to
                // overwrite a NO_DEPT, but we actually need the SeqCst to be a read-write
                // operation in here (we need both the release and acquire part of it).
                if slot
                    .0
                    .compare_exchange(NO_DEBT, gen, SeqCst, Relaxed)
                    .is_ok()
                {
                    let mut last = true;
                    for raw_j in raw_i + 1..len {
                        let j = (raw_j + offset) % len;
                        if node.slots.0[j].0.load(Relaxed) == NO_DEBT {
                            last = false;
                            // We already discovered this one will be empty, so store the info for
                            // next time.
                            head.slot_pos.set(j);
                            break;
                        }
                    }
                    if last {
                        // OK, store _this_ one for next time because we are going to free it up in
                        // a moment.
                        head.slot_pos.set(i);
                    }
                    if discard {
                        // Cool, we have modified the gen, claimed the slot, that's all we need to
                        // do while still holding ownership of the node. This is the right time to
                        // put the node into cooldown and get rid of it. We'll pick a new one (or
                        // this one, if the cooldown succeeds in between) next time.
                        node.start_cooldown();
                    }
                    return (slot, last, gen);
                }
            }
            unreachable!("Run out of slots in {:#?}", node);
        })
    }

    /// Confirms the debt.
    ///
    /// Replaces the pre-debt mark (the address we are reading from) with the actual debt and
    /// confirm it that way.
    ///
    /// If it got changed in the meantime, returns the (already protected) replacement value.
    #[inline]
    fn confirm(&self, gen: usize, ptr_addr: usize) -> Result<(), usize> {
        // AcqRel -> Release to publish everything (do we need to publish anything?)/make sure
        // sutff stays "inside" the lock.
        // Acquire -> we read a potentially new address, need to bring the pointee in.
        match self.0.compare_exchange(gen, ptr_addr, AcqRel, Acquire) {
            Ok(_) => Ok(()),
            Err(replacement) => {
                // Nothing synchronized by this, we just clear the slot for future reuse. Release
                // just to make sure, but should Relaxed be enough?
                self.0.store(NO_DEBT, Release);
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
            // If we don't change anything because there's something else, Relaxed is fine (it's
            // been already paid)
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
            let _reservation = node.reserve_writer();
            for slot in &node.slots.0 {
                loop {
                    match slot
                        .0
                        // We don't need to release anything in here. The writes to the reference
                        // counts take care of themselves before they drop to 0 (they are all to
                        // the same atomic, and we are doing the decrements later on). We need to
                        // acquire any increments done by the other threads though and we need to
                        // acquire the active_addr
                        .compare_exchange(ptr as usize, NO_DEBT, Acquire, Acquire)
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
                            let active_addr = node.active_addr.load(Acquire);
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
                            assert_eq!(replace_addr & TAG_MASK, 0, "Not enough space for tags");
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

struct NodeReservation<'a>(&'a Node);

impl Drop for NodeReservation<'_> {
    fn drop(&mut self) {
        self.0.active_writers.fetch_sub(1, Release);
    }
}

/// One thread-local node for debts.
#[repr(C)]
struct Node {
    slots: Slots,
    next: Option<&'static Node>,
    // This would be enough with AtomicU8, but that doesn't exist on 1.31 yet.
    in_use: AtomicUsize,
    active_addr: AtomicUsize,
    /// How many writers are currently interested in this one.
    ///
    /// See the ABA protection at the top.
    active_writers: AtomicUsize,
}

/// The head of the debt chain.
static DEBT_HEAD: AtomicPtr<Node> = AtomicPtr::new(ptr::null_mut());

impl Default for Node {
    fn default() -> Self {
        Node {
            next: None,
            in_use: AtomicUsize::new(NODE_USED),
            slots: Default::default(),
            active_addr: AtomicUsize::new(0),
            active_writers: AtomicUsize::new(0),
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

    /// Put the current thread node into cooldown
    fn start_cooldown(&self) {
        assert_eq!(NODE_USED, self.in_use.swap(NODE_COOLDOWN, Release));
        THREAD_HEAD.with(|head| assert!(ptr::eq(head.node.take().unwrap(), self)));
    }

    /// Perform a cooldown if the node is ready.
    ///
    /// See the ABA protection at the top.
    fn check_cooldown(&self) {
        // Cooldown succeeded if it is not being looked at by any writers.
        // This value may be a little bit outdated. But that is fine, as we get an update at least
        // every load operation (there's a SeqCst read-write operation on both paths, for readers
        // this can be only one generation stale which is fine, and for writers, if it doesn't
        // enter the write, because it is not interested, we don't worry about not seeing that
        // writer).
        if self.active_writers.load(Relaxed) == 0 {
            // Relaxed: we don't synchronize anything by this. It'll get used in real just in a
            // moment. If it doesn't succeed, then this one wasn't doing a cooldown and that's OK.
            let _ = self
                .in_use
                .compare_exchange(NODE_COOLDOWN, NODE_UNUSED, Relaxed, Relaxed);
        }
    }

    /// Mark this node that a writer is currently playing with it.
    #[inline]
    fn reserve_writer(&self) -> NodeReservation {
        self.active_writers.fetch_add(1, Acquire);
        NodeReservation(self)
    }

    #[inline(never)]
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
            // Not shared between threads yet, so ordinary write would be fine too.
            node.in_use.store(NODE_USED, Relaxed);
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
    fn with_thread_head<R, F: FnOnce(&DebtHead) -> R>(f: F) -> R {
        THREAD_HEAD
            // FIXME: Can this actually fail sometime? Can people use us during some kind of mutual
            // thread-local destruction thing?
            .with(|head| {
                if head.node.get().is_none() {
                    head.node.set(Some(Node::get()));
                }
                f(head)
            })
    }
}

impl Debug for Node {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Node")
            .field("in_use", &self.in_use.load(Relaxed))
            .field("next", &self.next)
            .field(
                "active_addr",
                &(self.active_addr.load(Relaxed) as *const u8),
            )
            .field("active_writers", &self.active_writers)
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
/// copies of the ones used by the hybrid strategy, but modified a bit. Just like in the hybrid
/// strategy, in case the slots run out or when the writer updates the value, the debts are paid by
/// incrementing the ref count (which is a little slower, but still wait-free/lock-free and still
/// in order of nanoseconds).
///
/// The strategy is:
///
/// * Wait-free on readers (except every usize::MAX/4 accesses, when it is only lock-free).
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
        // We already synchronized the start of the sequence by SeqCst in the Debt::new vs swap on
        // the pointer. We just need to make sure to bring the pointee in (this can be newer than
        // what we got in the Debt)
        let candidate = storage.load(Acquire);
        // TODO: We should check this much sooner, possibly when the pointer gets into the storage.
        assert_eq!(
            candidate as usize & TAG_MASK,
            0,
            "Not enough space for tags"
        );

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

impl<T: RefCnt> CaS<T> for Helping {
    unsafe fn compare_and_swap<C: AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        loop {
            let old = <Self as InnerStrategy<T>>::load(self, storage);
            // Observation of their inequality is enough to make a verdict
            if old.ptr.deref().as_raw() != current.as_raw() {
                return old;
            }
            // If they are still equal, put the new one in.
            let new_raw = T::as_ptr(&new);
            if storage
                .compare_exchange_weak(current.as_raw(), new_raw, SeqCst, Relaxed)
                .is_ok()
            {
                // We successfully put the new value in. The ref count went in there too.
                T::into_ptr(new);
                <Self as InnerStrategy<T>>::wait_for_readers(
                    self,
                    old.ptr.deref().as_raw(),
                    storage,
                );
                // We just got one ref count out of the storage and we have one in old. We don't
                // need two.
                T::dec(old.ptr.deref().as_raw());
                return old;
            }
        }
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

        fn get_thread() -> &'static Self {
            Self::with_thread_head(|h| h.node.get().unwrap())
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

    /// A smoke test exercising the generation overflow thing.
    #[test]
    fn gen_overflow() {
        type T = Arc<usize>;
        let a = T::new(42);
        let s = AtomicPtr::new(T::as_ptr(&a) as *mut usize);
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        // Force it to overflow during the next one.
        THREAD_HEAD.with(|h| h.generation.set(usize::MAX / 4 * 4));
        drop(p);
        // When it overflows, the node is released back to the pool (no good way to check it got
        // into cooldown, because some other thread might claim it in between).
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        THREAD_HEAD.with(|h| assert!(h.node.get().is_none()));
        drop(p);
        // But next round will pick some node again.
        let p = unsafe { <Helping as InnerStrategy<T>>::load(&Helping, &s) };
        THREAD_HEAD.with(|h| assert!(h.node.get().is_some()));
        drop(p);
    }

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

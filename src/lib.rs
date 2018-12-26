#![doc(
    html_root_url = "https://docs.rs/arc-swap/0.3.7/arc-swap/",
    test(attr(deny(warnings)))
)]
#![deny(missing_docs, warnings)]
#![allow(renamed_and_removed_lints)]

//! Making [`Arc`][Arc] itself atomic
//!
//! This library provides the [`ArcSwapAny`][ArcSwapAny] type (which you probably don't want to use
//! directly) and several type aliases that set it up for common use cases:
//!
//! * [`ArcSwap`][ArcSwap], which operates on [`Arc<T>`][Arc].
//! * [`ArcSwapOption`][ArcSwapOption], which operates on `Option<Arc<T>>`.
//! * [`IndependentArcSwap`][ArcSwapOption], which uses slightly different trade-of decisions ‒ see
//!   below.
//!
//! Note that as these are *type aliases*, the useful methods are defined on
//! [`ArcSwapAny`][ArcSwapAny] directly and you want to look at its documentation, not on the
//! aliases.
//!
//! This is similar to [`RwLock<Arc<T>>`][RwLock], but it is faster, the readers are never blocked
//! (not even by writes) and it is more configurable.
//!
//! Or, you can look at it this way. There's [`Arc<T>`][Arc] ‒ it knows when it stops being used and
//! therefore can clean up memory. But once there's a [`Arc<T>`][Arc] somewhere, shared between
//! threads, it has to keep pointing to the same thing. On the other hand, there's
//! [`AtomicPtr`][AtomicPtr] which can be changed even when shared between
//! threads, but it doesn't know when the data pointed to is no longer in use so it
//! doesn't clean up. This is a hybrid between the two.
//!
//! # Motivation
//!
//! First, the C++ [`shared_ptr`][shared_ptr] can act this way. The fact that it's only the surface
//! API and all the implementations I could find hide a mutex inside wasn't known to me when I
//! started working on this. So I decided Rust needs to keep up there.
//!
//! Second, I like hard problems and this seems like an apt supply of them.
//!
//! And third, I actually have few use cases for something like this.
//!
//! # Performance characteristics
//!
//! It is optimised for read-heavy situations with only occasional writes. Few examples might be:
//!
//! * Global configuration data structure, which is updated once in a blue moon when an operator
//!   manually does some changes, but looked into through the whole program all the time. Looking
//!   into it should be cheap and multiple threads should be able to look into it at the same time.
//! * Some in-memory database or maybe routing tables, where lookup latency matters. Updating the
//!   routing tables isn't an excuse to stop processing packets even for a short while.
//!
//! ## Lock-free readers
//!
//! All the read operations are always [lock-free]. Most of the time, they are actually
//! [wait-free]. The only one that is *only* lock-free is the first [`lease`][lease] access in each
//! thread (across all the pointers).
//!
//! So, when the documentation talks about *contention*, it talks about multiple cores having to
//! sort out who changes the bytes in a cache line first and who is next. This slows things down,
//! but it still rolls forward and stop for no one, not like with the mutex-style contention when
//! one holds the lock and others wait outside.
//!
//! Unfortunately, there are cases where readers block writers from completion. It's much more
//! limited in scope than with [`Mutex`][Mutex] or [`RwLock`][RwLock] and steady stream of readers
//! will *not* prevent an update from happening indefinitely (only a reader stuck in a critical
//! section could, and when used according to recommendations, the critical sections contain no
//! loops and are only several instructions short).
//!
//! ## Speeds
//!
//! The base line speed of read operations is similar to using an *uncontended* [`Mutex`][Mutex].
//! However, [`lease`][lease] and [`peek`][peek] suffer no contention from any other read
//! operations and only slight ones during updates. The [`load`][load] operation is additionally
//! contended only on the reference count of the [`Arc`][Arc] inside ‒ so, in general, while
//! [`Mutex`][Mutex] rapidly loses its performance when being in active use by multiple threads at
//! once and [`RwLock`][RwLock] is slow to start with, [`ArcSwapAny`][ArcSwapAny] mostly keeps its
//! performance even when read by many threads in parallel.
//!
//! Write operations are considered expensive. A write operation is more expensive than access to
//! an *uncontended* [`Mutex`][Mutex] and on some architectures even slower than uncontended
//! [`RwLock`][RwLock]. However, it is faster than either when contended.
//!
//! There are some (very unscientific) [benchmarks] within the source code of the library.
//!
//! The exact numbers are highly dependant on the machine used (both absolute numbers and relative
//! between different data structures). Not only architectures have a huge impact (eg. x86 vs ARM),
//! but even AMD vs. Intel or two different Intel processors. Therefore, if what matters is more
//! the speed than the wait-free guarantees, you're advised to do your own measurements.
//!
//! ## Choosing the right reading operation
//!
//! Performance is world of trade-offs. Therefore, the library offers several very similar methods
//! to read the pointer. The default choice should nevertheless probably be [`lease`][lease].
//!
//! Only one of them is functionally different ‒
//! [`peek_signal_safe`][peek_signal_safe]. See below for [signals](#unix-signal-handlers), but in
//! general, it is the only thing you want to use inside a signal handler and you don't want to use
//! it anywhere else.
//!
//! * [`load`][load] creates a full blown [`Arc`][Arc]. It's the most heavy-weight around and while
//!   [wait-free], it suffers from contention on the reference count in the [`Arc`][Arc], so when
//!   used from too many threads at once, it'll become slow. On the other hand, there's no
//!   restriction on how long you can hold onto the result or how many of them you keep around, so
//!   this is appropriate if creating handles for long-term storage. It also provides a bridge to
//!   other algorithms which only take the [`Arc`][Arc].
//! * [`lease`][lease] returns a proxy object that works as a pointer to the stored data and can be
//!   upgraded to full [`Arc`][Arc] later on if needed. There's no limit on how long it can live
//!   around. However, it internally comes at two flavors, one cheap and one containing a full
//!   [`Arc`][Arc] in it. Each thread is entitled to only limited total number of cheap ones at a
//!   given time (currently 6) and if more are constructed, the others fall back on the full
//!   version (which then uses [`load`][load] internally). Therefore, [`lease`][lease] can be fast
//!   (almost as fast as [`peek`][peek]) but only as long as the thread calling it doesn't have too
//!   many leases around at the time.
//! * [`peek`][peek] is the cheapest with the most predictable performance characteristics.
//!   However, as long as the returned guard object is alive, the internal generation lock is being
//!   held and that prevents write operations from completion and they'll spin-wait for the unlock.
//!   By default, all the pointer instances share the *same* generation lock (and it'll therefore
//!   prevent write operations even on *other* pointers from completion). However, the
//!   [`IndependentArcSwap`][IndependentArcSwap] uses a private generation lock for each instance.
//!   In general, this is suitable for very fast things ‒ like reading a single scalar value out of
//!   a configuration, but not keeping it around or doing expensive lookups in data.
//!
//! # RCU
//!
//! This also offers an [RCU implementation][rcu], for read-heavy
//! situations. Note that the RCU update is considered relatively slow operation (slower than
//! simple write). In case there's only one update thread, using
//! [`store`](struct.ArcSwapAny.html#method.store) is enough.
//!
//! # Atomic orderings
//!
//! It is guaranteed each operation performs at least one `SeqCst` atomic read-write operation,
//! therefore even operations on different instances have a defined global order of operations.
//!
//! # Unix signal handlers
//!
//! Unix signals are hard to use correctly, partly because there is a very restricted set of
//! functions one might use inside them. Specifically, it is *not* allowed to use mutexes inside
//! them (because that could cause a deadlock).
//!
//! On the other hand, it is possible to use [`peek_signal_safe`][peek_signal_safe] (but not the
//! others). Note that the signal handler is not allowed to allocate or deallocate
//! memory, therefore it is not recommended to [`upgrade`](struct.Guard.html#method.upgrade) the
//! returned guard (it is strictly speaking possible to use that safely, but it is hard and brings
//! no benefit).
//!
//! # Customization
//!
//! While the default [`ArcSwap`][ArcSwap] and [`lease`][lease] is probably good enough for most of
//! the needs, the library allows a wide range of customizations:
//!
//! * It allows storing nullable (`Option<Arc<_>>`) and non-nullable pointers.
//! * It is possible to store other reference counted pointers (eg. if you want to use it with a
//!   hypothetical `Arc` that doesn't have weak counts), by implementing the [`RefCnt`][RefCnt]
//!   trait.
//! * It allows choosing internal locking strategy by the [`LockStorage`][LockStorage] trait.
//!
//! # Examples
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//!
//! use std::sync::Arc;
//!
//! use arc_swap::ArcSwap;
//! use crossbeam_utils::thread;
//!
//! fn main() {
//!     let config = ArcSwap::from(Arc::new(String::default()));
//!     thread::scope(|scope| {
//!         scope.spawn(|| {
//!             let new_conf = Arc::new("New configuration".to_owned());
//!             config.store(new_conf);
//!         });
//!         for _ in 0..10 {
//!             scope.spawn(|| {
//!                 loop {
//!                     let cfg = config.lease();
//!                     if !cfg.is_empty() {
//!                         assert_eq!(*cfg, "New configuration");
//!                         return;
//!                     }
//!                 }
//!             });
//!         }
//!     });
//! }
//! ```
//!
//! # Alternatives
//!
//! There are other means to get similar functionality you might want to consider:
//!
//! ## `Mutex<Arc<_>>` and `RwLock<Arc<_>>`
//!
//! They have significantly worse performance in the contented scenario but are comparable in
//! uncontended cases. They are directly in the standard library, which means better testing and
//! less dependencies.
//!
//! ## The same, but with [parking_lot]
//!
//! Parking lot contains alternative implementations of `Mutex` and `RwLock` that are faster than
//! the standard library primitives. They still suffer from contention.
//!
//! ## [`crossbeam::atomic::ArcCell`]
//!
//! This internally contains a spin-lock equivalent and is very close to the characteristics of
//! `parking_lot::Mutex<Arc<_>>`. This is unofficially deprecated. See the
//! [relevant issue](https://github.com/crossbeam-rs/crossbeam/issues/160).
//!
//! ## [`crossbeam-arccell`]
//!
//! It is mentioned here because of the name. Despite of the name, this does something very
//! different (which *might* possibly solve similar problems). It's API is not centric to `Arc` or
//! any kind of pointer, but rather it has snapshots of its internal value that can be exchanged
//! very fast.
//!
//! ## [`AtomicArc`]
//!
//! This one is probably the closest thing to [`ArcSwap`][ArcSwap] on the API level. Both read and
//! write operations are [lock-free], but neither is [wait-free], and the performance of reads and
//! writes are more balanced ‒ while [`ArcSwap`][ArcSwap] is optimized for reading, [`AtomicArc`]
//! is „balanced“.
//!
//! The biggest current downside is, it is in a prototype stage and not released yet.
//!
//! [Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [RwLock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
//! [Mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
//! [AtomicPtr]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html
//! [ArcSwapAny]: struct.ArcSwapAny.html
//! [ArcSwap]: type.ArcSwap.html
//! [IndependentArcSwap]: type.IndependentArcSwap.html
//! [shared_ptr]: http://en.cppreference.com/w/cpp/memory/shared_ptr
//! [lock-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Lock-freedom
//! [wait-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Wait-freedom
//! [lease]: struct.ArcSwapAny.html#method.lease
//! [load]: struct.ArcSwapAny.html#method.load
//! [peek]: struct.ArcSwapAny.html#method.peek
//! [peek_signal_safe]: struct.ArcSwapAny.html#method.peek_signal_safe
//! [rcu]: struct.ArcSwapAny.html#method.rcu
//! [RefCnt]: trait.RefCnt.hmtl
//! [LockStorage]: gen_lock/trait.LockStorage.html
//! [benchmarks]: https://github.com/vorner/arc-swap/tree/master/benches
//! [parking_lot]: https://docs.rs/parking_lot
//! [`crossbeam::atomic::ArcCell`]: https://docs.rs/crossbeam/0.5.0/crossbeam/atomic/struct.ArcCell.html
//! [`crossbeam-arccell`]: https://docs.rs/crossbeam-arccell/
//! [`AtomicArc`]: https://github.com/stjepang/atomic/blob/master/src/atomic_arc.rs#L20

mod as_raw;
mod debt;
pub mod gen_lock;
mod ref_cnt;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::{self, AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;

pub use as_raw::AsRaw;
use debt::Debt;
use gen_lock::{Global, LockStorage, PrivateUnsharded, GEN_CNT};
pub use ref_cnt::{NonNull, RefCnt};

// # Implementation details
//
// The first idea would be to just use AtomicPtr with whatever the Arc::into_raw returns. Then
// replacing it would be fine (there's no need to update ref counts). The load needs to increment
// the reference count ‒ one still stays inside and another is returned to the caller. This is done
// by re-creating the Arc from the raw pointer and then cloning it, throwing one instance away
// (without destroying it).
//
// This approach has a problem. There's a short time between we read the raw pointer and increment
// the count. If some other thread replaces the stored Arc and throws it away, the ref count could
// drop to 0, get destroyed and we would be trying to bump ref counts in a ghost, which would be
// totally broken.
//
// To prevent this, we actually use two approaches in a hybrid manner.
//
// The first one is based on hazard pointers idea, but slightly modified. There's a global
// repository of pointers that owe a reference. When someone swaps a pointer, it walks this list
// and pays all the debts (and takes them).
//
// If storing into the repository fails (because the thread used up all its own slots, or because
// the pointer got replaced in just the wrong moment and it can't confirm the reservation), unlike
// the full hazard-pointers approach, we don't retry, but fall back onto secondary strategy.
//
// Each reader registers itself so it can be tracked, but only as a number. Each writer first
// switches the pointer. Then it takes a snapshot of all the current readers and waits until all of
// them confirm bumping their reference count. Only then the writer returns to the caller, handing
// it the ownership of the Arc and allowing possible bad things (like being destroyed) to happen to
// it. This has its own disadvantages, so it is only the second opportunity.
//
// Because bumping the reference count can cause contention and slows things down, there's another
// way to track missing reference counts. There's a global registry of debts and each lease of a
// missing reference count is written there. If someone wants to replace the pointer inside, it
// traverses the whole registry and pays all the debts (by incrementing the reference counts) ‒
// this avoids paying the debt in the most common read only cases.
//
// # Unsafety
//
// All the uses of the unsafe keyword is just to turn the raw pointer back to Arc. It originated
// from an Arc in the first place, so the only thing to ensure is it is still valid. That means its
// ref count never dropped to 0.
//
// At the beginning, there's ref count of 1 stored in the raw pointer (and maybe some others
// elsewhere, but we can't rely on these). This 1 stays there for the whole time the pointer is
// stored there. When the arc is replaced, this 1 is returned to the caller, so we just have to
// make sure no more readers access it by that time.
//
// # Tracking of readers
//
// The simple way would be to have a count of all readers that could be in the dangerous area
// between reading the pointer and bumping the reference count. We could „lock“ the ref count by
// incrementing this atomic counter and „unlock“ it when done. The writer would just have to
// busy-wait for this number to drop to 0 ‒ then there are no readers at all. This is safe, but a
// steady inflow of readers could make a writer wait forever.
//
// Therefore, we separate readers into two groups, odd and even ones (see below how). When we see
// both groups to drop to 0 (not necessarily at the same time, though), we are sure all the
// previous readers were flushed ‒ each of them had to be either odd or even.
//
// To do that, we define a generation. A generation is a number, incremented at certain times and a
// reader decides by this number if it is odd or even.
//
// One of the writers may increment the generation when it sees a zero in the next-generation's
// group (if the writer sees 0 in the odd group and the current generation is even, all the current
// writers are even ‒ so it remembers it saw odd-zero and increments the generation, so new readers
// start to appear in the odd group and the even has a chance to drop to zero later on). Only one
// writer does this switch, but all that witness the zero can remember it.
//
// We also split the reader threads into shards ‒ we have multiple copies of the counters, which
// prevents some contention and sharing of the cache lines. The writer reads them all and sums them
// up.
//
// # Leases and debts
//
// Instead of incrementing the reference count, the pointer reference can be owed. In such case, it
// is recorded into a global storage. As each thread has its own storage (the global storage is
// composed of multiple thread storages), the readers don't contend. When the pointer is no longer
// in use, the debt is erased.
//
// The writer pays all the existing debts, therefore the reader have the full Arc with ref count at
// that time. The reader is made aware the debt was paid and decrements the reference count.
//
// # Memory orders
//
// ## Synchronizing the data pointed to by the pointer.
//
// We have AcqRel (well, SeqCst, but that's included) on the swap and Acquire on the loads. In case
// of the double read around the debt allocation, we do that on the *second*, because of ABA.
// That's also why that SeqCst on the allocation itself is not enough.
//
// ## The generation lock
//
// Second, the dangerous area when we borrowed the pointer but haven't yet incremented its ref
// count needs to stay between incrementing and decrementing the reader count (in either group). To
// accomplish that, using Acquire on the increment and Release on the decrement would be enough.
// The loads in the writer use Acquire to complete the edge and make sure no part of the dangerous
// area leaks outside of it in the writers view. This Acquire, however, forms the edge only with
// the *latest* decrement. By making both the increment and decrement AcqRel, we effectively chain
// the edges together.
//
// Now the hard part :-). We need to ensure that whatever zero a writer sees is not stale in the
// sense that it happened before the switch of the pointer. In other words, we need to make sure
// that at the time we start to look for the zeroes, we already see all the current readers. To do
// that, we need to synchronize the time lines of the pointer itself and the corresponding group
// counters. As these are separate, unrelated, atomics, it calls for SeqCst ‒ on the swap and on
// the increment. This'll guarantee that they'll know which happened first (either increment or the
// swap), making a base line for the following operations (load of the pointer or looking for
// zeroes).
//
// # Memory orders around debts
//
// The linked list of debt nodes only grows. The shape of the list (existence of nodes) is
// synchronized through Release on creation and Acquire on load on the head pointer.
//
// The debts work similar to locks ‒ Acquire and Release make all the pointer manipulation at the
// interval where it is written down.
//
// In case the writer pays the debt, it sees the new enough data (for the same reasons the stale
// zeroes are not seen). The reference count on the Arc is AcqRel and makes sure it is not
// destroyed too soon. The writer traverses all the slots, therefore they don't need to synchronize
// with each other.
//
// # Orderings on the rest
//
// We don't really care much if we use a stale generation number ‒ it only works to route the
// readers into one or another bucket, but even if it was completely wrong, it would only slow the
// waiting for 0 down. So, the increments of it are just hints.
//
// All other operations can be Relaxed (they either only claim something, which doesn't need to
// synchronize with anything else, or they are failed attempts at something ‒ and another attempt
// will be made, the successful one will do the necessary synchronization).

/// Generation lock, to abstract locking and unlocking readers.
struct GenLock<'a, S: LockStorage + 'a> {
    shard: usize,
    gen: usize,
    lock_storage: &'a S,
}

impl<'a, S: LockStorage> GenLock<'a, S> {
    /// Creates a generation lock.
    fn new(signal_safe: SignalSafety, lock_storage: &'a S) -> Self {
        let shard = match signal_safe {
            SignalSafety::Safe => 0,
            SignalSafety::Unsafe => lock_storage.choose_shard(),
        };
        let gen = lock_storage.gen_idx().load(Ordering::Relaxed) % GEN_CNT;
        // Unlike the real Arc, we don't have to check for the ref count overflow. Nobody can drop
        // a reader.
        //
        // SeqCst: Acquire, so the dangerous section stays in. SeqCst to sync timelines with the
        // swap on the ptr in writer thread.
        lock_storage.shards().as_ref()[shard].0[gen].fetch_add(1, Ordering::SeqCst);
        GenLock {
            shard,
            gen,
            lock_storage,
        }
    }

    /// Removes a generation lock.
    fn unlock(self) {
        // Release, so the dangerous section stays in.
        self.lock_storage.shards().as_ref()[self.shard].0[self.gen].fetch_sub(1, Ordering::AcqRel);
        // Disarm the drop-bomb
        mem::forget(self);
    }
}

/// A bomb so one doesn't forget to unlock generations.
#[cfg(debug_assertions)] // The bomb actually makes it ~20% slower, so don't put it into production
impl<'a, S: LockStorage> Drop for GenLock<'a, S> {
    fn drop(&mut self) {
        unreachable!("Forgot to unlock generation");
    }
}

/// A short-term proxy object from [`peek`](struct.ArcSwapAny.html#method.peek).
///
/// This allows for upgrading to a full smart pointer and borrowing of the value inside. It also
/// dereferences to the actual pointed to type if the smart pointer guarantees not to contain NULL
/// values (eg. on `Arc`, but not on `Option<Arc>`).
///
/// # Warning
///
/// Do not store or keep around for a long time, as this prevents all the writer methods from
/// completing on all the swap objects in the whole program from completing.
pub struct Guard<'a, T: RefCnt + 'a, S: LockStorage + 'a = Global>
where
    T::Base: 'a,
{
    lock: Option<GenLock<'a, S>>,
    ptr: *const T::Base,
    _arc_swap: PhantomData<&'a ArcSwapAny<T>>,
}

impl<'a, T: RefCnt, S: LockStorage> Guard<'a, T, S> {
    /// Upgrades the guard to a real `Arc`.
    ///
    /// This shares the reference count with all the `Arc` inside the corresponding `ArcSwap`. Use
    /// this if you need to hold the object for longer periods of time.
    ///
    /// See [`peek`](struct.ArcSwapAny.html#method.peek) for details.
    ///
    /// Note that this is associated function (so it doesn't collide with the thing pointed to):
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::sync::Arc;
    /// # use arc_swap::{ArcSwap, Guard};
    /// let a = ArcSwap::from(Arc::new(42));
    /// let mut ptr = None;
    /// { // limit the scope where the guard lives
    ///     let guard = a.peek();
    ///     if *guard > 40 {
    ///         ptr = Some(Guard::upgrade(&guard));
    ///     }
    /// }
    /// # let _ = ptr;
    /// ```
    pub fn upgrade(guard: &Self) -> T {
        let res = unsafe { T::from_ptr(guard.ptr) };
        T::inc(&res);
        res
    }

    /// Upgrades the guard to a [`Lease`](struct.Lease.html).
    ///
    /// This is preserved here for backwards compatibility. Use [`upgrade`](#method.upgrade)
    /// instead (it has the same cost).
    #[deprecated(note = "Use upgrade instead")]
    pub fn lease(guard: &Self) -> Lease<T> {
        let res = unsafe { T::from_ptr(guard.ptr) };
        T::inc(&res);
        T::into_ptr(res);
        Lease {
            ptr: guard.ptr,
            debt: None,
            _data: PhantomData,
        }
    }

    /// Gets a reference to the value inside.
    ///
    /// This is returned as `Option` even for pointers that can't return Null, to have a common
    /// interface. The non-null ones also implement the `Deref` trait, so they can more easily be
    /// used as that.
    // Explicit lifetimes here:
    // * The ptr.as_ref() is more than willing to provide *any* lifetime we ask for. So being extra
    //   careful around it.
    // * While this is the lifetime that would get elided, one could also think the 'a lifetime
    //   would make sense. However, it is not so, because the arc-swap can replace the Arc inside
    //   and drop, the only thing preventing it from doing so is this guard. Therefore, at any time
    //   after the guard goes away, the pointed-to value can too.
    #[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
    pub fn get_ref<'g>(guard: &'g Self) -> Option<&'g T::Base> {
        unsafe { guard.ptr.as_ref() }
    }
}

impl<'a, T: NonNull, S: LockStorage> Deref for Guard<'a, T, S> {
    type Target = T::Base;
    fn deref(&self) -> &T::Base {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<'a, T: RefCnt, S: LockStorage> Drop for Guard<'a, T, S> {
    fn drop(&mut self) {
        self.lock.take().unwrap().unlock();
    }
}

/// A temporary storage of the pointer.
///
/// This, unlike [`Guard`](struct.Guard.html), does not block any write operations and is usually
/// faster than loading the full `Arc`. However, this holds only if each thread keeps only small
/// number of `Lease`s around and if too many are held, the following ones will just fall back to
/// creating the `Arc` internally.
pub struct Lease<T: RefCnt> {
    ptr: *const T::Base,
    debt: Option<&'static Debt>,
    _data: PhantomData<T>,
}

impl<T: RefCnt> Lease<T> {
    /// Loads a full `Arc` from the lease.
    pub fn upgrade(guard: &Self) -> T {
        let res = unsafe { T::from_ptr(guard.ptr) };
        T::inc(&res);
        res
    }

    /// A consuming version of [`upgrade`](#method.upgrade).
    ///
    /// This is a bit faster in certain situations, but consumes the lease.
    // Associated function on purpose, because of deref
    #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
    pub fn into_upgrade(lease: Self) -> T {
        let res = unsafe { T::from_ptr(lease.ptr) };
        if let Some(debt) = lease.debt {
            T::inc(&res);
            if !debt.pay::<T>(lease.ptr) {
                unsafe { T::dec(lease.ptr) };
            }
        }
        mem::forget(lease);
        res
    }

    /// Returns access to the data held.
    ///
    /// This returns `Option` even when it can't hold `NULL` internally, to keep the interface the
    /// same. But there's also the `Deref` trait for the non-`NULL` cases, which is usually more
    /// comfortable.
    pub fn get_ref(lease: &Self) -> Option<&T::Base> {
        unsafe { lease.ptr.as_ref() }
    }

    /// Checks if it contains a null pointer.
    ///
    /// Note that for non-`NULL` `T`, this always returns `false`.
    pub fn is_null(lease: &Self) -> bool {
        lease.ptr.is_null()
    }
}

/// Comparison of two pointer-like things.
// A and B are likely to *be* references, or thin wrappers around that. Calling that with extra
// reference is just annoying.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn ptr_eq<Base, A, B>(a: A, b: B) -> bool
where
    A: AsRaw<Base>,
    B: AsRaw<Base>,
{
    let a = a.as_raw();
    let b = b.as_raw();
    ptr::eq(a, b)
}

impl<T: NonNull> Deref for Lease<T> {
    type Target = T::Base;
    fn deref(&self) -> &T::Base {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl<T> Debug for Lease<T>
where
    T: RefCnt,
    T::Base: Debug,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        let l = Lease::get_ref(&self);
        if T::can_null() {
            l.fmt(formatter)
        } else {
            l.unwrap().fmt(formatter)
        }
    }
}

impl<T> Display for Lease<T>
where
    T: NonNull,
    T::Base: Display,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.deref().fmt(formatter)
    }
}

impl<T: RefCnt> Drop for Lease<T> {
    fn drop(&mut self) {
        if let Some(debt) = self.debt {
            if debt.pay::<T>(self.ptr) {
                return;
            }
        }
        unsafe { T::dec(self.ptr) };
    }
}

#[derive(Copy, Clone)]
enum SignalSafety {
    Safe,
    Unsafe,
}

/// When waiting to something, yield the thread every so many iterations so something else might
/// get a chance to run and release whatever is being held.
const YIELD_EVERY: usize = 16;

/// An atomic storage for a smart pointer like [`Arc`] or `Option<Arc>`.
///
/// This is a storage where a smart pointer may live. It can be read and written atomically from
/// several threads, but doesn't act like a pointer itself.
///
/// One can be created [`from`] an [`Arc`]. To get the pointer back, use the [`load`](#method.load)
/// method. But to general access to the data, [`lease`](#method.lease) may be more appropriate.
///
/// # Note
///
/// This is the generic low-level implementation. This allows sharing the same code for storing
/// both `Arc` and `Option<Arc>` (and possibly other similar types).
///
/// In your code, you most probably want to interact with it through the
/// [`ArcSwap`](type.ArcSwap.html) and [`ArcSwapOption`](type.ArcSwapOption.html) aliases. However,
/// the methods they share are described here and are applicable to both of them. That's why the
/// examples here use `ArcSwap` ‒ but they could as well be written with `ArcSwapOption` or
/// `ArcSwapAny`.
///
/// # Type parameters
///
/// * `T`: The smart pointer to be kept inside. This crate provides implementation for `Arc<_>` and
///   `Option<Arc<_>>`. But third party could provide implementations of the
///   [`RefCnt`] trait and plug in others.
/// * `S`: This describes where the generation lock is stored and how it works.
///
/// # Examples
///
/// ```rust
/// # use std::sync::Arc;
/// # use arc_swap::ArcSwap;
/// let arc = Arc::new(42);
/// let arc_swap = ArcSwap::from(arc);
/// assert_eq!(42, *arc_swap.load());
/// // It can be read multiple times
/// assert_eq!(42, *arc_swap.load());
///
/// // Put a new one in there
/// let new_arc = Arc::new(0);
/// assert_eq!(42, *arc_swap.swap(new_arc));
/// assert_eq!(0, *arc_swap.load());
/// ```
///
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// [`from`]: https://doc.rust-lang.org/nightly/std/convert/trait.From.html#tymethod.from
/// [`RefCnt`]: trait.RefCnt.html
pub struct ArcSwapAny<T: RefCnt, S: LockStorage = Global> {
    // Notes: AtomicPtr needs Sized
    /// The actual pointer, extracted from the Arc.
    ptr: AtomicPtr<T::Base>,

    /// We are basically an Arc in disguise. Inherit parameters from Arc by pretending to contain
    /// it.
    _phantom_arc: PhantomData<T>,

    lock_storage: S,
}

impl<T: RefCnt, S: LockStorage> From<T> for ArcSwapAny<T, S> {
    fn from(val: T) -> Self {
        // The AtomicPtr requires *mut in its interface. We are more like *const, so we cast it.
        // However, we always go back to *const right away when we get the pointer on the other
        // side, so it should be fine.
        let ptr = T::into_ptr(val);
        Self {
            ptr: AtomicPtr::new(ptr),
            _phantom_arc: PhantomData,
            lock_storage: S::default(),
        }
    }
}

impl<T: RefCnt, S: LockStorage> Drop for ArcSwapAny<T, S> {
    fn drop(&mut self) {
        let ptr = *self.ptr.get_mut();
        // To pay any possible debts
        self.wait_for_readers(ptr);
        // We are getting rid of the one stored ref count
        unsafe { T::dec(ptr) };
    }
}

impl<T: RefCnt, S: LockStorage> Clone for ArcSwapAny<T, S> {
    fn clone(&self) -> Self {
        Self::from(self.load())
    }
}

impl<T, S: LockStorage> Debug for ArcSwapAny<T, S>
where
    T: RefCnt,
    T::Base: Debug,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        let guard = self.peek();
        let r = Guard::get_ref(&guard);
        if T::can_null() {
            r.fmt(formatter)
        } else {
            r.unwrap().fmt(formatter)
        }
    }
}

impl<T, S: LockStorage> Display for ArcSwapAny<T, S>
where
    T: NonNull,
    T::Base: Display,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.peek().deref().fmt(formatter)
    }
}

impl<T: RefCnt, S: LockStorage> ArcSwapAny<T, S> {
    /// Constructs a new value.
    pub fn new(val: T) -> Self {
        Self::from(val)
    }

    /// Extracts the value inside.
    pub fn into_inner(mut self) -> T {
        let ptr = *self.ptr.get_mut();
        // To pay all the debts
        self.wait_for_readers(ptr);
        mem::forget(self);
        unsafe { T::from_ptr(ptr) }
    }

    /// Loads the value.
    ///
    /// This makes another copy (reference) and returns it, atomically (it is safe even when other
    /// thread stores into the same instance at the same time).
    ///
    /// The method is lock-free and wait-free, but usually more expensive than
    /// [`lease`](#method.lease).
    ///
    /// # Signal safety
    ///
    /// The method is *not* async-signal-safe. Use [`peek_signal_safe`](#method.peek_signal_safe)
    /// for that.
    pub fn load(&self) -> T {
        Guard::upgrade(&self.peek())
    }

    fn peek_inner(&self, signal_safe: SignalSafety) -> Guard<T, S> {
        let gen = GenLock::new(signal_safe, &self.lock_storage);
        let ptr = self.ptr.load(Ordering::Acquire);

        Guard {
            lock: Some(gen),
            _arc_swap: PhantomData,
            ptr,
        }
    }

    /// Provides a peek inside the held value.
    ///
    /// This returns a temporary borrow of the object currently held inside. This is slightly
    /// faster than [`load`](#method.load), but it is not suitable for holding onto for longer
    /// periods of time.
    ///
    /// If you discover later on that you need to hold onto it for longer, you can
    /// [`Guard::upgrade`](struct.Guard.html#method.upgrade) it.
    ///
    /// # Warning
    ///
    /// This currently prevents the pointer inside from being replaced. Any [`swap`](#method.swap),
    /// [`store`](#method.store) or [`rcu`](#method.rcu) will busy-loop while waiting for the proxy
    /// object to be destroyed, even on unrelated objects. Therefore, this is suitable only for
    /// things like reading a (reasonably small) configuration value, but not for eg. computations
    /// on the held values.
    ///
    /// If you want to do anything non-trivial, prefer [`lease`](#method.lease).
    ///
    /// If you are not sure what is better, benchmarking is recommended.
    ///
    /// # Signal safety
    ///
    /// For an async-signal-safe version, use [`peek_signal_safe`](#method.peek_signal_safe).
    pub fn peek(&self) -> Guard<T, S> {
        self.peek_inner(SignalSafety::Unsafe)
    }

    /// An async-signal-safe version of [`peek`](#method.peek)
    ///
    /// This method uses only restricted set of primitives to be async-signal-safe, at a slight
    /// performance hit in a contended scenario (signals should be rare, so it shouldn't be a
    /// problem in practice).
    ///
    /// As the returned guard prevents the value inside to be dropped, the value can be used during
    /// the signal handler. Unless it is upgraded (which is *not* recommended in a signal handler),
    /// there's also no way the signal handler would have to drop the pointed to value.
    ///
    /// The same performance warning about writer methods of [`peek`](#method.peek) applies, so it
    /// is recommended not to spend too much time holding the returned guard.
    pub fn peek_signal_safe(&self) -> Guard<T, S> {
        self.peek_inner(SignalSafety::Safe)
    }

    #[inline]
    fn lease_fallible(&self) -> Option<Lease<T>> {
        // Relaxed is good enough here, see the Acquire below
        let ptr = self.ptr.load(Ordering::Relaxed);
        // Try to get a debt slot. If not possible, fail.
        let debt = Debt::new(ptr as usize)?;

        let confirm = self.ptr.load(Ordering::Acquire);
        if ptr == confirm {
            Some(Lease {
                ptr,
                debt: Some(debt),
                _data: PhantomData,
            })
        } else if debt.pay::<T>(ptr) {
            None
        } else {
            Some(Lease {
                ptr,
                debt: None,
                _data: PhantomData,
            })
        }
    }

    /// Provides a temporary borrow of the object inside.
    ///
    /// This returns a proxy object allowing access to the thing held inside and it is *usually*
    /// as fast as [`peek`](#method.peek). However, there's only limited amount of possible cheap
    /// proxies in existence for each thread ‒ if more are created, it falls back to
    /// [`load`](#method.load) internally.
    ///
    /// This is therefore a good choice to use for eg. searching a data structure or juggling the
    /// pointers around a bit, but not as something to store in larger amounts. The rule of thumb
    /// is this is suited for local variables on stack, but not in structures.
    #[allow(deprecated)] // Allow Guard::lease internally
    #[inline]
    pub fn lease(&self) -> Lease<T> {
        self.lease_fallible()
            .unwrap_or_else(|| Guard::lease(&self.peek()))
    }

    /// Replaces the value inside this instance.
    ///
    /// Further loads will yield the new value. Uses [`swap`](#method.swap) internally.
    pub fn store(&self, val: T) {
        drop(self.swap(val));
    }

    /// Exchanges the value inside this instance.
    ///
    /// While multiple `swap`s can run concurrently and won't block each other, each one needs to
    /// wait for all the [`load`s](#method.load) and [`peek` Guards](#method.peek) that have seen
    /// the old value to finish before returning. This is in a way similar to locking ‒ a living
    /// [`Guard`](struct.Guard.html) can prevent this from finishing. However, unlike `RwLock`, a
    /// steady stream of readers will not block writers and if each guard is held only for a short
    /// period of time, writers will progress too.
    ///
    /// However, it is also possible to cause a deadlock (eg. this is an example of *broken* code):
    ///
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use arc_swap::ArcSwap;
    /// let shared = ArcSwap::from(Arc::new(42));
    /// let guard = shared.peek();
    /// // This will deadlock, because the guard is still active here and swap
    /// // can't pull the value from under its feet.
    /// shared.swap(Arc::new(0));
    /// # drop(guard);
    /// ```
    pub fn swap(&self, new: T) -> T {
        let new = T::into_ptr(new);
        // AcqRel needed to publish the target of the new pointer and get the target of the old
        // one.
        //
        // SeqCst to synchronize the time lines with the group counters.
        let old = self.ptr.swap(new, Ordering::SeqCst);
        self.wait_for_readers(old);
        unsafe { T::from_ptr(old) }
    }

    /// Swaps the stored Arc if it is equal to `current`.
    ///
    /// If the current value of the `ArcSwapAny` is equal to `current`, the `new` is stored inside.
    /// If not, nothing happens.
    ///
    /// The previous value (no matter if the swap happened or not) is returned. Therefore, if the
    /// returned value is equal to `current`, the swap happened. You want to do a pointer-based
    /// comparison to determine it (like `Arc::ptr_eq`).
    ///
    /// In other words, if the caller „guesses“ the value of current correctly, it acts like
    /// [`swap`](#method.swap), otherwise it acts like [`load`](#method.load) (including the
    /// limitations).
    ///
    /// The `current` can be specified as `&Arc`, [`Guard`](struct.Guard.html),
    /// [`&Lease`](struct.Lease.html) or as a raw pointer.
    pub fn compare_and_swap<C: AsRaw<T::Base>>(&self, current: C, new: T) -> Lease<T> {
        let cur_ptr = current.as_raw();
        let new = T::into_ptr(new);

        // As noted above, this method has either semantics of load or of store. We don't know
        // which ones upfront, so we need to implement safety measures for both.
        let gen = GenLock::new(SignalSafety::Unsafe, &self.lock_storage);

        let previous_ptr = self.ptr.compare_and_swap(cur_ptr, new, Ordering::SeqCst);
        let swapped = ptr::eq(cur_ptr, previous_ptr);

        // Drop it here, because:
        // * We can't drop it before the compare_and_swap ‒ in such case, it could get recycled,
        //   put into the pointer by another thread with a different value and create a fake
        //   success.
        // * We drop it before waiting for readers, because it could have been a Guard. In such
        //   case, the caller doesn't have it any more and can't check if it succeeded, but that's
        //   OK.
        drop(current);

        let debt = if swapped {
            // New went in, previous out, but their ref counts are correct. So nothing to do here.
            None
        } else {
            // Previous is a new copy of what is inside (and it stays there as well), so bump its
            // ref count. New is thrown away so dec its ref count (but do it outside of the
            // gen-lock).
            //
            // We try to do that by registering a debt and only if that fails by actually bumping
            // the ref.
            let debt = Debt::new(previous_ptr as usize);
            if debt.is_none() {
                let previous = unsafe { T::from_ptr(previous_ptr) };
                T::inc(&previous);
                T::into_ptr(previous);
            }
            debt
        };

        gen.unlock();

        if swapped {
            // We swapped. Before releasing the (possibly only) ref count of previous to user, wait
            // for all readers to make sure there are no more untracked copies of it.
            self.wait_for_readers(previous_ptr);
        } else {
            // We didn't swap, so new is black-holed.
            unsafe { T::dec(new) };
        }

        Lease {
            ptr: previous_ptr,
            debt,
            _data: PhantomData,
        }
    }

    /// Wait until all readers go away.
    fn wait_for_readers(&self, old: *const T::Base) {
        let mut seen_group = [false; GEN_CNT];
        let mut iter = 0usize;
        while !seen_group.iter().all(|seen| *seen) {
            // Note that we don't need the snapshot to be consistent. We just need to see both
            // halves being zero, not necessarily at the same time.
            let gen = self.lock_storage.gen_idx().load(Ordering::Relaxed);
            let groups = self
                .lock_storage
                .shards()
                .as_ref()
                .iter()
                .fold([0, 0], |[a1, a2], s| {
                    let [v1, v2] = s.snapshot();
                    [a1 + v1, a2 + v2]
                });
            // Should we increment the generation? Is the next one empty?
            let next_gen = gen.wrapping_add(1);
            if groups[next_gen % GEN_CNT] == 0 {
                // Replace it only if someone else didn't do it in the meantime
                self.lock_storage
                    .gen_idx()
                    .compare_and_swap(gen, next_gen, Ordering::Relaxed);
            }
            for i in 0..GEN_CNT {
                seen_group[i] = seen_group[i] || (groups[i] == 0);
            }
            iter = iter.wrapping_add(1);
            if iter % YIELD_EVERY == 0 {
                thread::yield_now();
            } else {
                atomic::spin_loop_hint();
            }
        }
        Debt::pay_all::<T>(old);
    }

    /// Read-Copy-Update of the pointer inside.
    ///
    /// This is useful in read-heavy situations with several threads that sometimes update the data
    /// pointed to. The readers can just repeatedly use [`load`](#method.load) without any locking.
    /// The writer uses this method to perform the update.
    ///
    /// In case there's only one thread that does updates or in case the next version is
    /// independent of the previous one, simple [`swap`](#method.swap) or [`store`](#method.store)
    /// is enough. Otherwise, it may be needed to retry the update operation if some other thread
    /// made an update in between. This is what this method does.
    ///
    /// # Examples
    ///
    /// This will *not* work as expected, because between loading and storing, some other thread
    /// might have updated the value.
    ///
    /// ```rust
    /// extern crate arc_swap;
    /// extern crate crossbeam_utils;
    ///
    /// use std::sync::Arc;
    ///
    /// use arc_swap::ArcSwap;
    /// use crossbeam_utils::thread;
    ///
    /// fn main() {
    ///     let cnt = ArcSwap::from(Arc::new(0));
    ///     thread::scope(|scope| {
    ///         for _ in 0..10 {
    ///             scope.spawn(|| {
    ///                 let inner = cnt.load();
    ///                 // Another thread might have stored some other number than what we have
    ///                 // between the load and store.
    ///                 cnt.store(Arc::new(*inner + 1));
    ///             });
    ///         }
    ///     });
    ///     // This will likely fail:
    ///     // assert_eq!(10, *cnt.load());
    /// }
    /// ```
    ///
    /// This will, but it can call the closure multiple times to do retries:
    ///
    /// ```rust
    /// extern crate arc_swap;
    /// extern crate crossbeam_utils;
    ///
    /// use std::sync::Arc;
    ///
    /// use arc_swap::ArcSwap;
    /// use crossbeam_utils::thread;
    ///
    /// fn main() {
    ///     let cnt = ArcSwap::from(Arc::new(0));
    ///     thread::scope(|scope| {
    ///         for _ in 0..10 {
    ///             scope.spawn(|| cnt.rcu(|inner| **inner + 1));
    ///         }
    ///     });
    ///     assert_eq!(10, *cnt.load());
    /// }
    /// ```
    ///
    /// Due to the retries, you might want to perform all the expensive operations *before* the
    /// rcu. As an example, if there's a cache of some computations as a map, and the map is cheap
    /// to clone but the computations are not, you could do something like this:
    ///
    /// ```rust
    /// extern crate arc_swap;
    /// extern crate crossbeam_utils;
    /// #[macro_use]
    /// extern crate lazy_static;
    ///
    /// use std::collections::HashMap;
    /// use std::sync::Arc;
    ///
    /// use arc_swap::ArcSwap;
    ///
    /// fn expensive_computation(x: usize) -> usize {
    ///     x * 2 // Let's pretend multiplication is really expensive
    /// }
    ///
    /// type Cache = HashMap<usize, usize>;
    ///
    /// lazy_static! {
    ///     static ref CACHE: ArcSwap<Cache> = ArcSwap::from(Arc::new(HashMap::new()));
    /// }
    ///
    /// fn cached_computation(x: usize) -> usize {
    ///     let cache = CACHE.load();
    ///     if let Some(result) = cache.get(&x) {
    ///         return *result;
    ///     }
    ///     // Not in cache. Compute and store.
    ///     // The expensive computation goes outside, so it is not retried.
    ///     let result = expensive_computation(x);
    ///     CACHE.rcu(|cache| {
    ///         // The cheaper clone of the cache can be retried if need be.
    ///         let mut cache = HashMap::clone(&cache);
    ///         cache.insert(x, result);
    ///         cache
    ///     });
    ///     result
    /// }
    ///
    /// fn main() {
    ///     assert_eq!(42, cached_computation(21));
    ///     assert_eq!(42, cached_computation(21));
    /// }
    /// ```
    ///
    /// # The cost of cloning
    ///
    /// Depending on the size of cache above, the cloning might not be as cheap. You can however
    /// use persistent data structures ‒ each modification creates a new data structure, but it
    /// shares most of the data with the old one (which is usually accomplished by using `Arc`s
    /// inside to share the unchanged values). Something like
    /// [`rpds`](https://crates.io/crates/rpds) or [`im`](https://crates.io/crates/im) might do
    /// what you need.
    pub fn rcu<R, F>(&self, mut f: F) -> T
    where
        F: FnMut(&Lease<T>) -> R,
        R: Into<T>,
    {
        let mut cur = self.lease();
        loop {
            let new = f(&cur).into();
            let prev = self.compare_and_swap(&cur, new);
            let swapped = ptr_eq(&cur, &prev);
            if swapped {
                return Lease::into_upgrade(prev);
            } else {
                cur = prev;
            }
        }
    }
}

/// An atomic storage for `Arc`.
///
/// This is a type alias only. Most of its methods are described on
/// [`ArcSwapAny`](struct.ArcSwapAny.html).
pub type ArcSwap<T> = ArcSwapAny<Arc<T>>;

impl<T, S: LockStorage> ArcSwapAny<Arc<T>, S> {
    /// A convenience constructor directly from the pointed-to value.
    ///
    /// Direct equivalent for `ArcSwap::new(Arc::new(val))`.
    pub fn from_pointee(val: T) -> Self {
        Self::from(Arc::new(val))
    }

    /// An [`rcu`](struct.ArcSwapAny.html#method.rcu) which waits to be the sole owner of the
    /// original value and unwraps it.
    ///
    /// This one works the same way as the [`rcu`](struct.ArcSwapAny.html#method.rcu) method, but
    /// works on the inner type instead of `Arc`. After replacing the original, it waits until
    /// there are no other owners of the arc and unwraps it.
    ///
    /// Possible use case might be an RCU with a structure that is rather slow to drop ‒ if it was
    /// left to random reader (the last one to hold the old value), it could cause a timeout or
    /// jitter in a query time. With this, the deallocation is done in the updater thread,
    /// therefore outside of the hot path.
    ///
    /// # Warning
    ///
    /// Note that if you store a copy of the `Arc` somewhere except the `ArcSwap` itself for
    /// extended period of time, this'll busy-wait the whole time. Unless you need the assurance
    /// the `Arc` is deconstructed here, prefer [`rcu`](#method.rcu).
    pub fn rcu_unwrap<R, F>(&self, mut f: F) -> T
    where
        F: FnMut(&T) -> R,
        R: Into<Arc<T>>,
    {
        let mut wrapped = self.rcu(|prev| f(&*prev));
        loop {
            match Arc::try_unwrap(wrapped) {
                Ok(val) => return val,
                Err(w) => {
                    wrapped = w;
                    thread::yield_now();
                }
            }
        }
    }
}

impl<T, S: LockStorage> ArcSwapAny<Option<Arc<T>>, S> {
    /// A convenience constructor directly from a pointed-to value.
    ///
    /// This just allocates the `Arc` under the hood.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arc_swap::ArcSwapOption;
    ///
    /// let empty: ArcSwapOption<usize> = ArcSwapOption::from_pointee(None);
    /// assert!(empty.load().is_none());
    /// let non_empty: ArcSwapOption<usize> = ArcSwapOption::from_pointee(42);
    /// assert_eq!(42, *non_empty.load().unwrap());
    /// ```
    pub fn from_pointee<V: Into<Option<T>>>(val: V) -> Self {
        Self::new(val.into().map(Arc::new))
    }

    /// A convenience constructor for an empty value.
    ///
    /// This is equivalent to `ArcSwapOption::new(None)`.
    pub fn empty() -> Self {
        Self::new(None)
    }
}

impl<T: RefCnt + Default, S: LockStorage> Default for ArcSwapAny<T, S> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// An atomic storage for `Option<Arc>`.
///
/// This is very similar to [`ArcSwap`](type.ArcSwap.html), but allows storing NULL values, which
/// is useful in some situations.
///
/// This is a type alias only. Most of the methods are described on
/// [`ArcSwapAny`](struct.ArcSwapAny.html). Even though the examples there often use `ArcSwap`,
/// they are applicable to `ArcSwapOption` with appropriate changes.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use arc_swap::ArcSwapOption;
///
/// let shared = ArcSwapOption::from(None);
/// assert!(shared.load().is_none());
/// assert!(shared.swap(Some(Arc::new(42))).is_none());
/// assert_eq!(42, *shared.load().unwrap());
/// ```
pub type ArcSwapOption<T> = ArcSwapAny<Option<Arc<T>>>;

/// An atomic storage that doesn't share the internal generation locks with others.
///
/// This makes it bigger and it also might suffer contention (on the HW level) if used from many
/// threads at once. But a peek [`Guard`](struct.Guard.html) produced by it won't block other
/// storages from updating.
///
/// ```rust
/// # use std::sync::Arc;
/// # use arc_swap::{ArcSwap, IndependentArcSwap};
/// // This one shares locks with others.
/// let shared = ArcSwap::from_pointee(42);
/// // But this one has an independent lock.
/// let independent = IndependentArcSwap::from_pointee(42);
///
/// // This'll hold a lock so any writers there wouldn't complete
/// let l = independent.peek();
/// // But the lock doesn't influence the shared one, so this goes through just fine
/// shared.store(Arc::new(43));
///
/// assert_eq!(42, *l);
/// ```
pub type IndependentArcSwap<T> = ArcSwapAny<Arc<T>, PrivateUnsharded>;

#[cfg(test)]
mod tests {
    extern crate crossbeam_utils;

    use std::panic;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Barrier;

    use self::crossbeam_utils::thread;

    use super::*;

    /// Similar to the one in doc tests of the lib, but more times and more intensive (we want to
    /// torture it a bit).
    ///
    /// Takes some time, presumably because this starts 21 000 threads during its lifetime and 20
    /// 000 of them just wait in a tight loop for the other thread to happen.
    #[test]
    fn publish() {
        for _ in 0..100 {
            let config = ArcSwap::<String>::default();
            let ended = AtomicUsize::new(0);
            thread::scope(|scope| {
                for _ in 0..20 {
                    scope.spawn(|| loop {
                        let cfg = config.load();
                        if !cfg.is_empty() {
                            assert_eq!(*cfg, "New configuration");
                            ended.fetch_add(1, Ordering::Relaxed);
                            return;
                        }
                        atomic::spin_loop_hint();
                    });
                }
                scope.spawn(|| {
                    let new_conf = Arc::new("New configuration".to_owned());
                    config.store(new_conf);
                });
            });
            assert_eq!(20, ended.load(Ordering::Relaxed));
            assert_eq!(2, Arc::strong_count(&config.load()));
            assert_eq!(0, Arc::weak_count(&config.load()));
        }
    }

    /// Similar to the doc tests of ArcSwap, but happens more times.
    #[test]
    fn swap_load() {
        for _ in 0..100 {
            let arc = Arc::new(42);
            let arc_swap = ArcSwap::from(Arc::clone(&arc));
            assert_eq!(42, *arc_swap.load());
            // It can be read multiple times
            assert_eq!(42, *arc_swap.load());

            // Put a new one in there
            let new_arc = Arc::new(0);
            assert_eq!(42, *arc_swap.swap(Arc::clone(&new_arc)));
            assert_eq!(0, *arc_swap.load());
            // One loaded here, one in the arc_swap, one in new_arc
            assert_eq!(3, Arc::strong_count(&arc_swap.load()));
            assert_eq!(0, Arc::weak_count(&arc_swap.load()));
            // The original got released from the arc_swap
            assert_eq!(1, Arc::strong_count(&arc));
            assert_eq!(0, Arc::weak_count(&arc));
        }
    }

    /// Two different writers publish two series of values. The readers check that it is always
    /// increasing in each serie.
    ///
    /// For performance, we try to reuse the threads here.
    #[test]
    fn multi_writers() {
        let first_value = Arc::new((0, 0));
        let shared = ArcSwap::from(Arc::clone(&first_value));
        const WRITER_CNT: usize = 2;
        const READER_CNT: usize = 3;
        const ITERATIONS: usize = 100;
        const SEQ: usize = 50;
        let barrier = Barrier::new(READER_CNT + WRITER_CNT);
        thread::scope(|scope| {
            for w in 0..WRITER_CNT {
                // We need to move w into the closure. But we want to just reference the other
                // things.
                let barrier = &barrier;
                let shared = &shared;
                let first_value = &first_value;
                scope.spawn(move || {
                    for _ in 0..ITERATIONS {
                        barrier.wait();
                        shared.store(Arc::clone(&first_value));
                        barrier.wait();
                        for i in 0..SEQ {
                            shared.store(Arc::new((w, i + 1)));
                        }
                    }
                });
            }
            for _ in 0..READER_CNT {
                scope.spawn(|| {
                    for _ in 0..ITERATIONS {
                        barrier.wait();
                        barrier.wait();
                        let mut previous = [0; 2];
                        let mut last = Arc::clone(&first_value);
                        loop {
                            let cur = shared.load();
                            if Arc::ptr_eq(&last, &cur) {
                                atomic::spin_loop_hint();
                                continue;
                            }
                            let (w, s) = *cur;
                            assert!(previous[w] < s);
                            previous[w] = s;
                            last = cur;
                            if s == SEQ {
                                break;
                            }
                        }
                    }
                });
            }
        });
    }

    #[test]
    /// Make sure the reference count and compare_and_swap works as expected.
    fn cas_ref_cnt() {
        const ITERATIONS: usize = 50;
        let shared = ArcSwap::from(Arc::new(0));
        for i in 0..ITERATIONS {
            let orig = shared.load();
            assert_eq!(i, *orig);
            if i % 2 == 1 {
                // One for orig, one for shared
                assert_eq!(2, Arc::strong_count(&orig));
            }
            let n1 = Arc::new(i + 1);
            // Fill up the slots sometimes
            let fillup = || {
                if i % 2 == 0 {
                    Some(
                        (0..50)
                            .into_iter()
                            .map(|_| shared.lease())
                            .collect::<Vec<_>>(),
                    )
                } else {
                    None
                }
            };
            let leases = fillup();
            // Success
            let prev = shared.compare_and_swap(&orig, Arc::clone(&n1));
            assert!(ptr_eq(&orig, &prev));
            drop(leases);
            // One for orig, one for prev
            assert_eq!(2, Arc::strong_count(&orig));
            // One for n1, one for shared
            assert_eq!(2, Arc::strong_count(&n1));
            assert_eq!(i + 1, *shared.peek());
            let n2 = Arc::new(i);
            drop(prev);
            let leases = fillup();
            // Failure
            let prev = Lease::into_upgrade(shared.compare_and_swap(&orig, Arc::clone(&n2)));
            drop(leases);
            assert!(ptr_eq(&n1, &prev));
            // One for orig
            assert_eq!(1, Arc::strong_count(&orig));
            // One for n1, one for shared, one for prev
            assert_eq!(3, Arc::strong_count(&n1));
            // n2 didn't get increased
            assert_eq!(1, Arc::strong_count(&n2));
            assert_eq!(i + 1, *shared.peek());
        }

        let a = shared.load();
        // One inside shared, one for a
        assert_eq!(2, Arc::strong_count(&a));
        drop(shared);
        // Only a now
        assert_eq!(1, Arc::strong_count(&a));
    }

    #[test]
    /// Multiple RCUs interacting.
    fn rcu() {
        const ITERATIONS: usize = 50;
        const THREADS: usize = 10;
        let shared = ArcSwap::from(Arc::new(0));
        thread::scope(|scope| {
            for _ in 0..THREADS {
                scope.spawn(|| {
                    for _ in 0..ITERATIONS {
                        shared.rcu(|old| **old + 1);
                    }
                });
            }
        });
        assert_eq!(THREADS * ITERATIONS, *shared.load());
    }

    #[test]
    /// Multiple RCUs interacting, with unwrapping.
    fn rcu_unwrap() {
        const ITERATIONS: usize = 50;
        const THREADS: usize = 10;
        let shared = ArcSwap::from(Arc::new(0));
        thread::scope(|scope| {
            for _ in 0..THREADS {
                scope.spawn(|| {
                    for _ in 0..ITERATIONS {
                        shared.rcu_unwrap(|old| *old + 1);
                    }
                });
            }
        });
        assert_eq!(THREADS * ITERATIONS, *shared.load());
    }

    /// Handling null/none values
    #[test]
    fn nulls() {
        let shared = ArcSwapOption::from(Some(Arc::new(0)));
        let orig = shared.swap(None);
        assert_eq!(1, Arc::strong_count(&orig.unwrap()));
        let null = shared.load();
        assert!(null.is_none());
        let a = Arc::new(42);
        let orig = shared.compare_and_swap(ptr::null(), Some(Arc::clone(&a)));
        assert!(Lease::is_null(&orig));
        assert_eq!(2, Arc::strong_count(&a));
        let orig = Lease::into_upgrade(shared.compare_and_swap(&None::<Arc<_>>, None));
        assert_eq!(3, Arc::strong_count(&a));
        assert!(ptr_eq(&a, &orig));
    }

    /// We have a callback in RCU. Check what happens if we access the value from within.
    #[test]
    fn recursive() {
        let shared = ArcSwap::from(Arc::new(0));

        shared.rcu(|i| {
            if **i < 10 {
                shared.rcu(|i| **i + 1);
            }
            **i
        });
        assert_eq!(10, *shared.peek());
        assert_eq!(2, Arc::strong_count(&shared.load()));
    }

    /// A panic from within the rcu callback should not change anything.
    #[test]
    fn rcu_panic() {
        let shared = ArcSwap::from(Arc::new(0));
        assert!(panic::catch_unwind(|| shared.rcu(|_| -> usize { panic!() })).is_err());
        assert_eq!(1, Arc::strong_count(&shared.swap(Arc::new(42))));
    }

    /// Accessing the value inside ArcSwap with Lease (and checks for the reference counts).
    #[test]
    fn lease_cnt() {
        let a = Arc::new(0);
        let shared = ArcSwap::from(Arc::clone(&a));
        // One in shared, one in a
        assert_eq!(2, Arc::strong_count(&a));
        let lease = shared.lease();
        assert_eq!(0, *lease);
        // The lease doesn't have its own ref count now
        assert_eq!(2, Arc::strong_count(&a));
        let lease_2 = shared.lease();
        // Unlike with guard, this does not deadlock
        shared.store(Arc::new(1));
        // But now, each lease got a full Arc inside it
        assert_eq!(3, Arc::strong_count(&a));
        // And when we get rid of them, they disappear
        drop(lease_2);
        assert_eq!(2, Arc::strong_count(&a));
        let _b = Lease::upgrade(&lease);
        assert_eq!(3, Arc::strong_count(&a));
        // We can drop the lease it came from
        drop(lease);
        assert_eq!(2, Arc::strong_count(&a));
        let lease = shared.lease();
        assert_eq!(1, *lease);
        drop(shared);
        // We can still use the lease after the shared disappears
        assert_eq!(1, *lease);
        let ptr = Lease::upgrade(&lease);
        // One in shared, one in lease
        assert_eq!(2, Arc::strong_count(&ptr));
        drop(lease);
        assert_eq!(1, Arc::strong_count(&ptr));
    }

    /// There can be only limited amount of leases on one thread. Following ones are created, but
    /// contain full Arcs.
    #[test]
    fn lease_overflow() {
        let a = Arc::new(0);
        let shared = ArcSwap::from(Arc::clone(&a));
        assert_eq!(2, Arc::strong_count(&a));
        let mut leases = (0..1000)
            .into_iter()
            .map(|_| shared.lease())
            .collect::<Vec<_>>();
        let count = Arc::strong_count(&a);
        assert!(count > 2);
        let lease = shared.lease();
        assert_eq!(count + 1, Arc::strong_count(&a));
        drop(lease);
        assert_eq!(count, Arc::strong_count(&a));
        // When we delete the first one, it didn't have an Arc in it, so the ref count doesn't drop
        leases.swap_remove(0);
        // But new one reuses now vacant the slot and doesn't create a new Arc
        let _lease = shared.lease();
        assert_eq!(count, Arc::strong_count(&a));
    }

    #[test]
    fn lease_null() {
        let shared = ArcSwapOption::<usize>::default();
        let lease = shared.lease();
        assert!(Lease::get_ref(&lease).is_none());
        shared.store(Some(Arc::new(42)));
        assert_eq!(42, *Lease::get_ref(&shared.lease()).unwrap());
    }

    #[test]
    fn from_into() {
        let a = Arc::new(42);
        let shared = ArcSwap::new(a);
        let lease = shared.lease();
        let a = shared.into_inner();
        assert_eq!(42, *a);
        assert_eq!(2, Arc::strong_count(&a));
        drop(lease);
        assert_eq!(1, Arc::strong_count(&a));
    }
}

#![doc(html_root_url = "https://docs.rs/arc-swap/0.1.0/arc-swap/")]
#![deny(missing_docs)]

//! Making [`Arc`] itself atomic
//!
//! The [`Arc`] uses atomic reference counters, so the object behind it can be safely pointed to by
//! several threads at once. However, the [`Arc`] itself is quite ordinary ‒ to change its value
//! (make it point somewhere else), one has to be the sole owner of it (or store it behind a
//! [`Mutex`]).
//!
//! On the other hand, there's [`AtomicPtr`]. It can be modified and read from multiple threads,
//! allowing to pass the value from one thread to another without the use of a [`Mutex`]. The
//! downside is, tracking when the data can be safely deleted is hard.
//!
//! This library provides [`ArcSwap`] that allows both at once. It can be constructed from ordinary
//! [`Arc`], but its value can be loaded and stored atomically, my multiple concurrent threads.
//!
//! # Motivation
//!
//! For one, the C++ [`shared_ptr`] has this
//! [ability](http://en.cppreference.com/w/cpp/memory/shared_ptr/atomic), so it is only fair to
//! have it too.
//!
//! For another, it seemed like a really good exercise.
//!
//! And finally, there are some real use cases for this functionality. For example, when one thread
//! publishes something (for example configuration) and other threads want to have a peek to the
//! current one from time to time. There's a global [`ArcSwap`], holding the current snapshot and
//! everyone is free to make a copy and hold onto it for a while. The publisher thread simply
//! stores a new snapshot every time and the old configuration gets dropped once all the other
//! threads give up their copies of the pointer.
//!
//! # Performance characteristics
//!
//! Some benchmarks need to be done. Due to the complexity, it may be possible that using
//! `Mutex<Arc<T>>` might be faster in some cases.
//!
//! However, this implementation doesn't suffer from contention. Specifically, arbitrary number of
//! readers can access the shared value and won't be blocked. Even when there are many readers and
//! writers at once, they don't block each other. The writers will be somewhat slower when there
//! are active readers at the same time, but won't be stopped indefinitely. Readers always perform
//! the same number of instructions.
//!
//! # Unix signal handlers
//!
//! Unix signals are hard to use correctly, partly because there is a very restricted set of
//! functions one might use inside them. Specifically, it is *not* allowed to use mutexes inside
//! them (because that could cause a deadlock).
//!
//! On the other hand, it is possible to use [`ArcSwap::load`](struct.ArcSwap.html#method.load)
//! (and theoretically some of the others, but a correct use would be very hard). Note that the
//! signal handler is not allowed to allocate or deallocate memory. This also means the signal
//! handler must not drop the last reference to the `Arc` received by loading ‒ therefore, the part
//! changing it from the outside needs to make sure it does not drop the only other instance once
//! it swapped the content while the signal handler still runs. See
//! [`ArcSwap::rcu_unwrap`](struct.ArcSwap.html#method.rcu_unwrap).
//!
//! # Example
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//!
//! use std::sync::Arc;
//!
//! use arc_swap::ArcSwap;
//! use crossbeam_utils::scoped as thread;
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
//!                     let cfg = config.load();
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
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`AtomicPtr`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html
//! [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
//! [`shared_ptr`]: http://en.cppreference.com/w/cpp/memory/shared_ptr

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::mem;
use std::sync::atomic::{self, AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

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
// To prevent this, the readers work as usual, but register themselves so they can be tracked. Each
// writer first switches the pointer. Then it takes a snapshot of all the current readers and waits
// until all of them confirm bumping their reference count. Only then the writer returns to the
// caller, handing it the ownership of the Arc and allowing possible bad things (like being
// destroyed) to happen to it.
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
// # Memory orders
//
// We need to make sure several things happen or don't.
//
// First, we have to guarantee the target of the pointer is visible in whatever thread receives a
// copy of the Arc. Having AcqRel on the swap (because it can both publish and read the pointer)
// and Acquire on the load is enough for this purpose.
//
// Second, the dangerous area when we borrowed the pointer but haven't yet incremented its ref
// count needs to stay between incrementing and decrementing the reader count (in either group). To
// accomplish that, using Acquire on the increment and Release on the decrement would be enough.
// The loads in the writer use Acquire to complete the edge and make sure no part of the dangerous
// area leaks outside of it in the writers view.
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
// All other operations can be Relaxed.

/// Generation lock, to abstract locking and unlocking readers.
struct GenLock(usize);

/// A bomb so one doesn't forget to unlock generations.
impl Drop for GenLock {
    fn drop(&mut self) {
        unreachable!("Forgot to unlock generation");
    }
}

/// Store count for 2 newest generations (others must always be 0)
const GEN_CNT: usize = 2;

/// Turn the arc into a raw pointer.
fn strip<T>(arc: Arc<T>) -> *mut T {
    Arc::into_raw(arc) as *mut T
}

/// An atomic storage for [`Arc`].
///
/// This is a storage where an [`Arc`] may live. It can be read and written atomically from several
/// threads, but doesn't act like a pointer itself.
///
/// One can be created [`from`] an [`Arc`]. To get an [`Arc`] back, use the [`load`](#method.load)
/// method.
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
pub struct ArcSwap<T> {
    // Notes: AtomicPtr needs Sized
    /// The actual pointer, extracted from the Arc.
    ptr: AtomicPtr<T>,

    /// The current generation. Module by their count to discover the groups.
    gen_idx: AtomicUsize,

    /// Count of readers in either generation.
    reader_group_cnts: [AtomicUsize; GEN_CNT],

    /// We are basically an Arc in disguise. Inherit parameters from Arc by pretending to contain
    /// it.
    _phantom_arc: PhantomData<Arc<T>>,
}

impl<T> From<Arc<T>> for ArcSwap<T> {
    fn from(arc: Arc<T>) -> Self {
        // The AtomicPtr requires *mut in its interface. We are more like *const, so we cast it.
        // However, we always go back to *const right away when we get the pointer on the other
        // side, so it should be fine.
        let ptr = strip(arc);
        Self {
            ptr: AtomicPtr::new(ptr),
            gen_idx: AtomicUsize::new(0),
            reader_group_cnts: [AtomicUsize::new(0), AtomicUsize::new(0)],
            _phantom_arc: PhantomData,
        }
    }
}

impl<T> Drop for ArcSwap<T> {
    fn drop(&mut self) {
        // Note that by now we are visible only by one thread (otherwise we couldn't get `&mut`),
        // so we can abandon all these atomic-ordering madnesses.

        // We hold one reference in the Arc, but it's hidden. Convert us back to Arc and drop that
        // Arc instead of us, which will clear the ref.
        let ptr = *self.ptr.get_mut();
        // Turn it back into the Arc and then drop it.
        drop(unsafe { Arc::from_raw(ptr) });
    }
}

impl<T> Clone for ArcSwap<T> {
    fn clone(&self) -> Self {
        Self::from(self.load())
    }
}

impl<T: Debug> Debug for ArcSwap<T> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.load().fmt(formatter)
    }
}

impl<T: Display> Display for ArcSwap<T> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.load().fmt(formatter)
    }
}

impl<T> ArcSwap<T> {
    /// Loads the value.
    ///
    /// This makes another copy (reference) and returns it, atomically (it is safe even when other
    /// thread stores into the same instance at the same time).
    #[inline]
    pub fn load(&self) -> Arc<T> {
        let gen = self.gen_lock();
        // Acquire, to get the target of the pointer.
        let ptr = self.ptr.load(Ordering::Acquire);
        let arc = unsafe { Arc::from_raw(ptr) };
        // Bump the reference count by one, so we can return one into the arc and another to
        // the caller.
        Arc::into_raw(Arc::clone(&arc));
        // Release, so the dangerous section stays in.
        self.gen_unlock(gen);
        arc
    }

    /// Replaces the value inside this instance.
    ///
    /// Further loads will yield the new value.
    #[inline]
    pub fn store(&self, arc: Arc<T>) {
        drop(self.swap(arc));
    }

    /// Exchanges the value inside this instance.
    #[inline]
    pub fn swap(&self, arc: Arc<T>) -> Arc<T> {
        let new = strip(arc);
        // AcqRel needed to publish the target of the new pointer and get the target of the old
        // one.
        //
        // SeqCst to synchronize the time lines with the group counters.
        let old = self.ptr.swap(new, Ordering::SeqCst);
        self.wait_for_readers();
        unsafe { Arc::from_raw(old) }
    }

    /// Swaps the stored Arc if it is equal to `current`.
    ///
    /// If the current value of the `ArcSwap` is equal to `current`, the `new` is stored inside. If
    /// not, nothing happens.
    ///
    /// True is returned as the first part of result if it did swap content, false otherwise.
    /// Either way, the previous content is returned as the second part. The property of standard
    /// library atomics that if previous is the same as current, the swap happened, is still true,
    /// but unlike the values in the atomics, `Arc` is not copy ‒ therefore, you may want to pass
    /// the only instance of current into it and not clone it just to compare.
    ///
    /// In other words, if the caller „guesses“ the value of current correctly, it acts like
    /// [`swap`](#method.swap), otherwise it acts like [`load`](#method.load).
    #[inline]
    pub fn compare_and_swap(&self, current: Arc<T>, new: Arc<T>) -> (bool, Arc<T>) {
        // As noted above, this method has either semantics of load or of store. We don't know
        // which ones upfront, so we need to implement safety measures for both.
        let current = strip(current);
        let new = strip(new);

        let gen = self.gen_lock();

        let previous = self.ptr.compare_and_swap(current, new, Ordering::SeqCst);
        let swapped = current == previous;
        let previous = unsafe { Arc::from_raw(previous) };

        if swapped {
            // New went in, previous out, but their ref counts are correct. We handle current later
            // on. So nothing to do here.
        } else {
            // Previous is a new copy of what is inside (and it stays there as well), so bump its
            // ref count. New is thrown away so dec its ref count (but do it outside of the
            // gen-lock).
            Arc::into_raw(Arc::clone(&previous));
        }

        self.gen_unlock(gen);

        if swapped {
            // We swapped. Before releasing the (possibly only) ref count of previous to user, wait
            // for all readers to make sure there are no more untracked copies of it.
            self.wait_for_readers();
        } else {
            // We didn't swap, so new is black-holed.
            drop(unsafe { Arc::from_raw(new) });
        }
        // The current is black-holed every time.
        drop(unsafe { Arc::from_raw(current) });

        (swapped, previous)
    }

    /// Wait until all readers go away.
    #[inline]
    fn wait_for_readers(&self) {
        let mut seen_group = [false; GEN_CNT];
        while !seen_group.iter().all(|seen| *seen) {
            // Note that we don't need the snapshot to be consistent. We just need to see both
            // halves being zero, not necessarily at the same time.
            let gen = self.gen_idx.load(Ordering::Relaxed);
            let groups = [
                self.reader_group_cnts[0].load(Ordering::Acquire),
                self.reader_group_cnts[1].load(Ordering::Acquire),
            ];
            // Should we increment the generation? Is the next one empty?
            let next_gen = gen.wrapping_add(1);
            if groups[next_gen % GEN_CNT] == 0 {
                // Replace it only if someone else didn't do it in the meantime
                self.gen_idx
                    .compare_and_swap(gen, next_gen, Ordering::Relaxed);
            }
            for i in 0..GEN_CNT {
                seen_group[i] = seen_group[i] || (groups[i] == 0);
            }
            atomic::spin_loop_hint();
        }
    }

    #[inline]
    fn gen_lock(&self) -> GenLock {
        let gen = self.gen_idx.load(Ordering::Relaxed) % GEN_CNT;
        // Unlike the real Arc, we don't have to check for the ref count overflow. Nobody can drop
        // a reader.
        //
        // SeqCst: Acquire, so the dangerous section stays in. SeqCst to sync timelines with the
        // swap on the ptr in writer thread.
        self.reader_group_cnts[gen].fetch_add(1, Ordering::SeqCst);
        GenLock(gen)
    }

    #[inline]
    fn gen_unlock(&self, lock: GenLock) {
        let gen = lock.0;
        // Disarm the drop-bomb
        mem::forget(lock);
        // Release, so the dangerous section stays in.
        self.reader_group_cnts[gen].fetch_sub(1, Ordering::Release);
    }

    /// Read-Copy-Update of the pointer inside.
    ///
    /// This is useful in read-heavy situations with several threads that sometimes update the data
    /// pointed to. The readers can just repeatedly use [`load`](#method.load) without any locking.
    /// The writer uses this method to perform the update.
    ///
    /// In case there's only one thread that does updates or in case the next version is
    /// independent of the previous onse, simple [`swap`](#method.swap) or [`store`](#method.store)
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
    /// use crossbeam_utils::scoped as thread;
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
    /// use crossbeam_utils::scoped as thread;
    ///
    /// fn main() {
    ///     let cnt = ArcSwap::from(Arc::new(0));
    ///     thread::scope(|scope| {
    ///         for _ in 0..10 {
    ///             scope.spawn(|| cnt.rcu(|inner| Arc::new(**inner + 1)));
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
    /// use crossbeam_utils::scoped as thread;
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
    ///         let mut cache = HashMap::clone(cache);
    ///         cache.insert(x, result);
    ///         Arc::new(cache)
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
    /// shares most of the data with the old one. Something like
    /// [`rpds`](https://crates.io/crates/rpds) or [`im`](https://crates.io/crates/im) might do
    /// what you need.
    pub fn rcu<F: FnMut(&Arc<T>) -> Arc<T>>(&self, mut f: F) -> Arc<T> {
        let mut cur = self.load();
        loop {
            let new = f(&cur);
            let (swapped, prev) = self.compare_and_swap(cur, new);
            if swapped {
                return prev;
            } else {
                cur = prev;
            }
        }
    }

    /// An [`rcu`](#method.rcu) which waits to be the sole owner of the original value and unwraps
    /// it.
    ///
    /// This one works the same way as the [`rcu`](#method.rcu) method, but works on the inner type
    /// instead of `Arc`. After replacing the original, it waits until there are no other owners of
    /// the arc and unwraps it.
    ///
    /// One use case is around signal handlers. The signal handler loads the pointer, but it must
    /// not be the last one to drop it. Therefore, this methods make sure there are no signal
    /// handlers owning it at the time of deconstruction of the `Arc`.
    ///
    /// # Warning
    ///
    /// Note that if you store a copy of the `Arc` somewhere except the `ArcSwap` itself for
    /// extended period of time, this'll busy-wait the whole time. Unless you need the assurance
    /// the `Arc` is deconstructed here, prefer [`rcu`](#method.rcu).
    pub fn rcu_unwrap<F: FnMut(&T) -> T>(&self, mut f: F) -> T {
        let mut wrapped = self.rcu(|prev| Arc::new(f(prev)));
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

#[cfg(test)]
mod tests {
    extern crate crossbeam_utils;

    use std::sync::atomic::AtomicUsize;
    use std::sync::Barrier;

    use self::crossbeam_utils::scoped as thread;
    use super::*;

    /// Similar to the one in doc tests of the lib, but more times and more intensive (we want to
    /// torture it a bit).
    ///
    /// Takes some time, presumably because this starts 21 000 threads during its lifetime and 20
    /// 000 of them just wait in a tight loop for the other thread to happen.
    #[test]
    fn publish() {
        for _ in 0..100 {
            let config = ArcSwap::from(Arc::new(String::default()));
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
            // One for orig, one for shared
            assert_eq!(2, Arc::strong_count(&orig));
            let n1 = Arc::new(i + 1);
            // Success
            let (swapped, prev) = shared.compare_and_swap(Arc::clone(&orig), Arc::clone(&n1));
            assert!(swapped);
            assert!(Arc::ptr_eq(&orig, &prev));
            // One for orig, one for prev
            assert_eq!(2, Arc::strong_count(&orig));
            // One for n1, one for shared
            assert_eq!(2, Arc::strong_count(&n1));
            assert_eq!(i + 1, *shared.load());
            let n2 = Arc::new(i);
            drop(prev);
            // Failure
            let (swapped, prev) = shared.compare_and_swap(Arc::clone(&orig), Arc::clone(&n2));
            assert!(!swapped);
            assert!(Arc::ptr_eq(&n1, &prev));
            // One for orig
            assert_eq!(1, Arc::strong_count(&orig));
            // One for n1, one for shared, one for prev
            assert_eq!(3, Arc::strong_count(&n1));
            // n2 didn't get increased
            assert_eq!(1, Arc::strong_count(&n2));
            assert_eq!(i + 1, *shared.load());
        }
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
                        shared.rcu(|old| Arc::new(**old + 1));
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
}

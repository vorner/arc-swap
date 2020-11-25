#![doc(test(attr(deny(warnings))))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Making [`Arc`][Arc] itself atomic
//!
//! The [`ArcSwap`] type is a container for an `Arc` that can be changed atomically. Semantically,
//! it is similar to something like `Atomic<Arc<T>>` (if there was such a thing) or
//! `RwLock<Arc<T>>` (but without the need for the locking). It is optimized for read-mostly
//! scenarios, with consistent performance characteristics.
//!
//! # Motivation
//!
//! There are many situations in which one might want to have some data structure that is often
//! read and seldom updated. Some examples might be a configuration of a service, routing tables,
//! snapshot of some data that is renewed every few minutes, etc.
//!
//! In all these cases one needs:
//! * Being able to read the current value of the data structure, fast, often and concurrently from
//!   many threads.
//! * Using the same version of the data structure over longer period of time ‒ a query should be
//!   answered by a consistent version of data, a packet should be routed either by an old or by a
//!   new version of the routing table but not by a combination, etc.
//! * Perform an update without disrupting the processing.
//!
//! The first idea would be to use [`RwLock<T>`][RwLock] and keep a read-lock for the whole time of
//! processing. Update would, however, pause all processing until done.
//!
//! Better option would be to have [`RwLock<Arc<T>>`][RwLock]. Then one would lock, clone the [Arc]
//! and unlock. This suffers from CPU-level contention (on the lock and on the reference count of
//! the [Arc]) which makes it relatively slow. Depending on the implementation, an update may be
//! blocked for arbitrary long time by a steady inflow of readers.
//!
//! ```rust
//! # use std::sync::{Arc, RwLock};
//! # use once_cell::sync::Lazy;
//! # struct RoutingTable; struct Packet; impl RoutingTable { fn route(&self, _: Packet) {} }
//! static ROUTING_TABLE: Lazy<RwLock<Arc<RoutingTable>>> = Lazy::new(|| {
//!     RwLock::new(Arc::new(RoutingTable))
//! });
//!
//! fn process_packet(packet: Packet) {
//!     let table = Arc::clone(&ROUTING_TABLE.read().unwrap());
//!     table.route(packet);
//! }
//! # fn main() { process_packet(Packet); }
//! ```
//!
//! The [ArcSwap] can be used instead, which solves the above problems and has better performance
//! characteristics than the [RwLock], both in contended and non-contended scenarios.
//!
//! ```rust
//! # use arc_swap::ArcSwap;
//! # use once_cell::sync::Lazy;
//! # struct RoutingTable; struct Packet; impl RoutingTable { fn route(&self, _: Packet) {} }
//! static ROUTING_TABLE: Lazy<ArcSwap<RoutingTable>> = Lazy::new(|| {
//!     ArcSwap::from_pointee(RoutingTable)
//! });
//!
//! fn process_packet(packet: Packet) {
//!     let table = ROUTING_TABLE.load();
//!     table.route(packet);
//! }
//! # fn main() { process_packet(Packet); }
//! ```
//!
//! # Crate contents
//!
//! At the heart of the crate there are [`ArcSwap`] and [`ArcSwapOption`] types, containers for an
//! [`Arc`] and [`Option<Arc>`][Option].
//!
//! Technically, these are type aliases for partial instantiations of the [`ArcSwapAny`] type. The
//! [`ArcSwapAny`] is more flexible and allows tweaking of many things (can store other things than
//! [`Arc`]s, can configure the locking [`Strategy`]). For details about the tweaking, see the
//! documentation of the [`strategy`] module and the [`RefCnt`] trait.
//!
//! The [`cache`] module provides means for speeding up read access of the contained data at the
//! cost of delayed reclamation.
//!
//! The [`access`] module can be used to do projections into the contained data to separate parts
//! of application from each other (eg. giving a component access to only its own part of
//! configuration while still having it reloaded as a whole).
//!
//! # Before using
//!
//! The data structure is a bit niche. Before using, please check the
//! [limitations and common pitfalls][docs::limitations] and the [performance
//! characteristics][docs::performance], including choosing the right [read
//! operation][docs::performance#read-operations].
//!
//! You can also get an inspiration about what's possible in the [common patterns][docs::patterns]
//! section.
//!
//! # Examples
//!
//! ```rust
//! use std::sync::Arc;
//!
//! use arc_swap::ArcSwap;
//! use crossbeam_utils::thread;
//!
//! fn main() {
//!     let config = ArcSwap::from(Arc::new(String::default()));
//!     thread::scope(|scope| {
//!         scope.spawn(|_| {
//!             let new_conf = Arc::new("New configuration".to_owned());
//!             config.store(new_conf);
//!         });
//!         for _ in 0..10 {
//!             scope.spawn(|_| {
//!                 loop {
//!                     let cfg = config.load();
//!                     if !cfg.is_empty() {
//!                         assert_eq!(**cfg, "New configuration");
//!                         return;
//!                     }
//!                 }
//!             });
//!         }
//!     }).unwrap();
//! }
//! ```
//!
//! [RwLock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html

pub mod access;
mod as_raw;
pub mod cache;
mod compile_fail_tests;
mod debt;
pub mod docs;
mod gen_lock;
mod ref_cnt;
pub mod strategy;
#[cfg(feature = "weak")]
mod weak;

use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

use crate::access::{Access, Map};
pub use crate::as_raw::AsRaw;
pub use crate::cache::Cache;
pub use crate::ref_cnt::RefCnt;
use crate::strategy::sealed::Protected;
use crate::strategy::{CaS, Strategy};
pub use crate::strategy::{DefaultStrategy, IndependentStrategy};

/// A temporary storage of the pointer.
///
/// This guard object is returned from most loading methods (with the notable exception of
/// [`load_full`](struct.ArcSwapAny.html#method.load_full)). It dereferences to the smart pointer
/// loaded, so most operations are to be done using that.
pub struct Guard<T: RefCnt, S: Strategy<T> = DefaultStrategy> {
    inner: S::Protected,
}

impl<'a, T: RefCnt, S: Strategy<T>> Guard<T, S> {
    /// Converts it into the held value.
    ///
    /// This, on occasion, may be a tiny bit faster than cloning the Arc or whatever is being held
    /// inside.
    // Associated function on purpose, because of deref
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn into_inner(lease: Self) -> T {
        lease.inner.into_inner()
    }

    /// Create a guard for a given value `inner`.
    ///
    /// This can be useful on occasion to pass a specific object to code that expects or
    /// wants to store a Guard.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use arc_swap::{ArcSwap, DefaultStrategy, Guard};
    /// # use std::sync::Arc;
    /// # let p = ArcSwap::from_pointee(42);
    /// // Create two guards pointing to the same object
    /// let g1 = p.load();
    /// let g2 = Guard::<_, DefaultStrategy>::from_inner(Arc::clone(&*g1));
    /// # drop(g2);
    /// ```
    pub fn from_inner(inner: T) -> Self {
        Guard {
            inner: S::Protected::from_inner(inner),
        }
    }
}

impl<'a, T: RefCnt, S: Strategy<T>> Deref for Guard<T, S> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.inner.borrow()
    }
}

impl<T: RefCnt, S: Strategy<T>> From<T> for Guard<T, S> {
    fn from(inner: T) -> Self {
        Self::from_inner(inner)
    }
}

impl<T: Default + RefCnt, S: Strategy<T>> Default for Guard<T, S> {
    fn default() -> Self {
        Self::from(T::default())
    }
}

impl<T: Debug + RefCnt, S: Strategy<T>> Debug for Guard<T, S> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.deref().fmt(formatter)
    }
}

impl<T: Display + RefCnt, S: Strategy<T>> Display for Guard<T, S> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.deref().fmt(formatter)
    }
}

/// Comparison of two pointer-like things.
// A and B are likely to *be* references, or thin wrappers around that. Calling that with extra
// reference is just annoying.
#[allow(clippy::needless_pass_by_value)]
fn ptr_eq<Base, A, B>(a: A, b: B) -> bool
where
    A: AsRaw<Base>,
    B: AsRaw<Base>,
{
    let a = a.as_raw();
    let b = b.as_raw();
    ptr::eq(a, b)
}

/// An atomic storage for a reference counted smart pointer like [`Arc`] or `Option<Arc>`.
///
/// This is a storage where a smart pointer may live. It can be read and written atomically from
/// several threads, but doesn't act like a pointer itself.
///
/// One can be created [`from`] an [`Arc`]. To get the pointer back, use the
/// [`load`](#method.load).
///
/// # Note
///
/// This is the common generic implementation. This allows sharing the same code for storing
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
///   `Option<Arc<_>>` (`Rc` too, but that one is not practically useful). But third party could
///   provide implementations of the [`RefCnt`] trait and plug in others.
/// * `S`: Chooses the [strategy] used to protect the data inside. They come with various
///   performance trade offs, the default [`DefaultStrategy`] is good rule of thumb for most use
///   cases.
///
/// # Examples
///
/// ```rust
/// # use std::sync::Arc;
/// # use arc_swap::ArcSwap;
/// let arc = Arc::new(42);
/// let arc_swap = ArcSwap::from(arc);
/// assert_eq!(42, **arc_swap.load());
/// // It can be read multiple times
/// assert_eq!(42, **arc_swap.load());
///
/// // Put a new one in there
/// let new_arc = Arc::new(0);
/// assert_eq!(42, *arc_swap.swap(new_arc));
/// assert_eq!(0, **arc_swap.load());
/// ```
///
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// [`from`]: https://doc.rust-lang.org/nightly/std/convert/trait.From.html#tymethod.from
/// [`RefCnt`]: trait.RefCnt.html
pub struct ArcSwapAny<T: RefCnt, S: Strategy<T> = DefaultStrategy> {
    // Notes: AtomicPtr needs Sized
    /// The actual pointer, extracted from the Arc.
    ptr: AtomicPtr<T::Base>,

    /// We are basically an Arc in disguise. Inherit parameters from Arc by pretending to contain
    /// it.
    _phantom_arc: PhantomData<T>,

    /// Strategy to protect the data.
    strategy: S,
}

impl<T: RefCnt, S: Default + Strategy<T>> From<T> for ArcSwapAny<T, S> {
    fn from(val: T) -> Self {
        Self::with_strategy(val, S::default())
    }
}

impl<T: RefCnt, S: Strategy<T>> Drop for ArcSwapAny<T, S> {
    fn drop(&mut self) {
        let ptr = *self.ptr.get_mut();
        unsafe {
            // To pay any possible debts
            self.strategy.wait_for_readers(ptr);
            // We are getting rid of the one stored ref count
            T::dec(ptr);
        }
    }
}

impl<T, S: Strategy<T>> Debug for ArcSwapAny<T, S>
where
    T: Debug + RefCnt,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter
            .debug_tuple("ArcSwapAny")
            .field(&self.load())
            .finish()
    }
}

impl<T, S: Strategy<T>> Display for ArcSwapAny<T, S>
where
    T: Display + RefCnt,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.load().fmt(formatter)
    }
}

impl<T: RefCnt + Default, S: Default + Strategy<T>> Default for ArcSwapAny<T, S> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: RefCnt, S: Strategy<T>> ArcSwapAny<T, S> {
    /// Constructs a new storage.
    pub fn new(val: T) -> Self
    where
        S: Default,
    {
        Self::from(val)
    }

    /// Constructs a new storage while customizing the protection strategy.
    pub fn with_strategy(val: T, strategy: S) -> Self {
        // The AtomicPtr requires *mut in its interface. We are more like *const, so we cast it.
        // However, we always go back to *const right away when we get the pointer on the other
        // side, so it should be fine.
        let ptr = T::into_ptr(val);
        Self {
            ptr: AtomicPtr::new(ptr),
            _phantom_arc: PhantomData,
            strategy,
        }
    }

    /// Extracts the value inside.
    pub fn into_inner(mut self) -> T {
        let ptr = *self.ptr.get_mut();
        // To pay all the debts
        unsafe { self.strategy.wait_for_readers(ptr) };
        mem::forget(self);
        unsafe { T::from_ptr(ptr) }
    }

    /// Loads the value.
    ///
    /// This makes another copy of the held pointer and returns it, atomically (it is
    /// safe even when other thread stores into the same instance at the same time).
    ///
    /// The method is lock-free and wait-free, but usually more expensive than
    /// [`load`](#method.load).
    pub fn load_full(&self) -> T {
        Guard::into_inner(self.load())
    }

    /// Provides a temporary borrow of the object inside.
    ///
    /// This returns a proxy object allowing access to the thing held inside. However, there's
    /// only limited amount of possible cheap proxies in existence for each thread ‒ if more are
    /// created, it falls back to equivalent of [`load_full`](#method.load_full) internally.
    ///
    /// This is therefore a good choice to use for eg. searching a data structure or juggling the
    /// pointers around a bit, but not as something to store in larger amounts. The rule of thumb
    /// is this is suited for local variables on stack, but not in long-living data structures.
    ///
    /// # Consistency
    ///
    /// In case multiple related operations are to be done on the loaded value, it is generally
    /// recommended to call `load` just once and keep the result over calling it multiple times.
    /// First, keeping it is usually faster. But more importantly, the value can change between the
    /// calls to load, returning different objects, which could lead to logical inconsistency.
    /// Keeping the result makes sure the same object is used.
    ///
    /// ```rust
    /// # use arc_swap::ArcSwap;
    /// struct Point {
    ///     x: usize,
    ///     y: usize,
    /// }
    ///
    /// fn print_broken(p: &ArcSwap<Point>) {
    ///     // This is broken, because the x and y may come from different points,
    ///     // combining into an invalid point that never existed.
    ///     println!("X: {}", p.load().x);
    ///     // If someone changes the content now, between these two loads, we
    ///     // have a problem
    ///     println!("Y: {}", p.load().y);
    /// }
    ///
    /// fn print_correct(p: &ArcSwap<Point>) {
    ///     // Here we take a snapshot of one specific point so both x and y come
    ///     // from the same one.
    ///     let point = p.load();
    ///     println!("X: {}", point.x);
    ///     println!("Y: {}", point.y);
    /// }
    /// # let p = ArcSwap::from_pointee(Point { x: 10, y: 20 });
    /// # print_correct(&p);
    /// # print_broken(&p);
    /// ```
    #[inline]
    pub fn load(&self) -> Guard<T, S> {
        let protected = unsafe { self.strategy.load(&self.ptr) };
        Guard { inner: protected }
    }

    /// Replaces the value inside this instance.
    ///
    /// Further loads will yield the new value. Uses [`swap`](#method.swap) internally.
    pub fn store(&self, val: T) {
        drop(self.swap(val));
    }

    /// Exchanges the value inside this instance.
    ///
    /// Note that this method is *not* lock-free with the [`DefaultStrategy`].
    pub fn swap(&self, new: T) -> T {
        let new = T::into_ptr(new);
        // AcqRel needed to publish the target of the new pointer and get the target of the old
        // one.
        //
        // SeqCst to synchronize the time lines with the group counters.
        let old = self.ptr.swap(new, Ordering::SeqCst);
        unsafe {
            self.strategy.wait_for_readers(old);
            T::from_ptr(old)
        }
    }

    /// Swaps the stored Arc if it equals to `current`.
    ///
    /// If the current value of the `ArcSwapAny` equals to `current`, the `new` is stored inside.
    /// If not, nothing happens.
    ///
    /// The previous value (no matter if the swap happened or not) is returned. Therefore, if the
    /// returned value is equal to `current`, the swap happened. You want to do a pointer-based
    /// comparison to determine it.
    ///
    /// In other words, if the caller „guesses“ the value of current correctly, it acts like
    /// [`swap`](#method.swap), otherwise it acts like [`load_full`](#method.load_full) (including
    /// the limitations).
    ///
    /// The `current` can be specified as `&Arc`, [`Guard`](struct.Guard.html),
    /// [`&Guards`](struct.Guards.html) or as a raw pointer.
    pub fn compare_and_swap<C>(&self, current: C, new: T) -> Guard<T, S>
    where
        C: AsRaw<T::Base>,
        S: CaS<T>,
    {
        let protected = unsafe { self.strategy.compare_and_swap(&self.ptr, current, new) };
        Guard { inner: protected }
        /*
         */
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
    /// # use std::sync::Arc;
    /// #
    /// # use arc_swap::ArcSwap;
    /// # use crossbeam_utils::thread;
    /// #
    /// let cnt = ArcSwap::from_pointee(0);
    /// thread::scope(|scope| {
    ///     for _ in 0..10 {
    ///         scope.spawn(|_| {
    ///            let inner = cnt.load_full();
    ///             // Another thread might have stored some other number than what we have
    ///             // between the load and store.
    ///             cnt.store(Arc::new(*inner + 1));
    ///         });
    ///     }
    /// }).unwrap();
    /// // This will likely fail:
    /// // assert_eq!(10, *cnt.load_full());
    /// ```
    ///
    /// This will, but it can call the closure multiple times to retry:
    ///
    /// ```rust
    /// # use arc_swap::ArcSwap;
    /// # use crossbeam_utils::thread;
    /// #
    /// let cnt = ArcSwap::from_pointee(0);
    /// thread::scope(|scope| {
    ///     for _ in 0..10 {
    ///         scope.spawn(|_| cnt.rcu(|inner| **inner + 1));
    ///     }
    /// }).unwrap();
    /// assert_eq!(10, *cnt.load_full());
    /// ```
    ///
    /// Due to the retries, you might want to perform all the expensive operations *before* the
    /// rcu. As an example, if there's a cache of some computations as a map, and the map is cheap
    /// to clone but the computations are not, you could do something like this:
    ///
    /// ```rust
    /// # use std::collections::HashMap;
    /// #
    /// # use arc_swap::ArcSwap;
    /// # use once_cell::sync::Lazy;
    /// #
    /// fn expensive_computation(x: usize) -> usize {
    ///     x * 2 // Let's pretend multiplication is *really expensive expensive*
    /// }
    ///
    /// type Cache = HashMap<usize, usize>;
    ///
    /// static CACHE: Lazy<ArcSwap<Cache>> = Lazy::new(|| ArcSwap::default());
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
    /// assert_eq!(42, cached_computation(21));
    /// assert_eq!(42, cached_computation(21));
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
        F: FnMut(&T) -> R,
        R: Into<T>,
        S: CaS<T>,
    {
        let mut cur = self.load();
        loop {
            let new = f(&cur).into();
            let prev = self.compare_and_swap(&*cur, new);
            let swapped = ptr_eq(&*cur, &*prev);
            if swapped {
                return Guard::into_inner(prev);
            } else {
                cur = prev;
            }
        }
    }

    /// Provides an access to an up to date projection of the carried data.
    ///
    /// # Motivation
    ///
    /// Sometimes, an application consists of components. Each component has its own configuration
    /// structure. The whole configuration contains all the smaller config parts.
    ///
    /// For the sake of separation and abstraction, it is not desirable to pass the whole
    /// configuration to each of the components. This allows the component to take only access to
    /// its own part.
    ///
    /// # Lifetimes & flexibility
    ///
    /// This method is not the most flexible way, as the returned type borrows into the `ArcSwap`.
    /// To provide access into eg. `Arc<ArcSwap<T>>`, you can create the [`Map`] type directly. See
    /// the [`access`] module.
    ///
    /// # Performance
    ///
    /// As the provided function is called on each load from the shared storage, it should
    /// generally be cheap. It is expected this will usually be just referencing of a field inside
    /// the structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    ///
    /// use arc_swap::ArcSwap;
    /// use arc_swap::access::Access;
    ///
    /// struct Cfg {
    ///     value: usize,
    /// }
    ///
    /// fn print_many_times<V: Access<usize>>(value: V) {
    ///     for _ in 0..25 {
    ///         let value = value.load();
    ///         println!("{}", *value);
    ///     }
    /// }
    ///
    /// let shared = ArcSwap::from_pointee(Cfg { value: 0 });
    /// let mapped = shared.map(|c: &Cfg| &c.value);
    /// crossbeam_utils::thread::scope(|s| {
    ///     // Will print some zeroes and some twos
    ///     s.spawn(|_| print_many_times(mapped));
    ///     s.spawn(|_| shared.store(Arc::new(Cfg { value: 2 })));
    /// }).expect("Something panicked in a thread");
    /// ```
    pub fn map<I, R, F>(&self, f: F) -> Map<&Self, I, F>
    where
        F: Fn(&I) -> &R,
        Self: Access<I>,
    {
        Map::new(self, f)
    }
}

/// An atomic storage for `Arc`.
///
/// This is a type alias only. Most of its methods are described on
/// [`ArcSwapAny`](struct.ArcSwapAny.html).
pub type ArcSwap<T> = ArcSwapAny<Arc<T>>;

impl<T, S: Strategy<Arc<T>>> ArcSwapAny<Arc<T>, S> {
    /// A convenience constructor directly from the pointed-to value.
    ///
    /// Direct equivalent for `ArcSwap::new(Arc::new(val))`.
    pub fn from_pointee(val: T) -> Self
    where
        S: Default,
    {
        Self::from(Arc::new(val))
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
/// assert!(shared.load_full().is_none());
/// assert!(shared.swap(Some(Arc::new(42))).is_none());
/// assert_eq!(42, **shared.load_full().as_ref().unwrap());
/// ```
pub type ArcSwapOption<T> = ArcSwapAny<Option<Arc<T>>>;

impl<T, S: Strategy<Option<Arc<T>>>> ArcSwapAny<Option<Arc<T>>, S> {
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
    /// assert_eq!(42, **non_empty.load().as_ref().unwrap());
    /// ```
    pub fn from_pointee<V: Into<Option<T>>>(val: V) -> Self
    where
        S: Default,
    {
        Self::new(val.into().map(Arc::new))
    }

    /// A convenience constructor for an empty value.
    ///
    /// This is equivalent to `ArcSwapOption::new(None)`.
    pub fn empty() -> Self
    where
        S: Default,
    {
        Self::new(None)
    }
}

/// An atomic storage that doesn't share the internal generation locks with others.
///
/// This makes it bigger and it also might suffer contention (on the HW level) if used from many
/// threads at once. On the other hand, it can't block writes in other instances.
///
/// See the [`IndependentStrategy`] for further details.
pub type IndependentArcSwap<T> = ArcSwapAny<Arc<T>, IndependentStrategy>;

/// Arc swap for the [Weak] pointer.
///
/// This is similar to [ArcSwap], but it doesn't store [Arc], it stores [Weak]. It doesn't keep the
/// data alive when pointed to.
///
/// This is a type alias only. Most of the methods are described on the
/// [`ArcSwapAny`](struct.ArcSwapAny.html).
///
/// [Weak]: std::sync::Weak
#[cfg(feature = "weak")]
pub type ArcSwapWeak<T> = ArcSwapAny<std::sync::Weak<T>>;

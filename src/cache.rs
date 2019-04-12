#![deny(unsafe_code)]
//! Caching handle into the [ArcSwapAny].
//!
//! The [Cache] keeps a copy of the internal [Arc] for faster access.
//!
//! [Cache]: cache::Cache
//! [Arc]: std::sync::Arc

use std::ops::Deref;
use std::sync::atomic::Ordering;

use super::gen_lock::LockStorage;
use super::ref_cnt::RefCnt;
use super::ArcSwapAny;

/// Caching handle for [ArcSwapAny].
///
/// Instead of loading (or leasing or something) the [Arc] on every request from the shared
/// storage, this keeps another copy inside. Upon request it only cheaply revalidates it is up to
/// date. If it is, access is significantly faster. If it is stale, the full [load] is done and the
/// cache value is replaced. Under a read-heavy loads, the measured speedup are 10-25 times,
/// depending on the architecture.
///
/// There are, however, downsides:
///
/// * The handle needs to be kept around by the caller (usually, one per thread). This is fine if
///   there's one global instance, but starts being tricky with eg. data structures build from
///   them.
/// * As it keeps a copy of the [Arc] inside the cache, the old value may be kept alive for longer
///   period of time ‒ it is replaced by the new value on [load][Cache::load]. You may not want to
///   use this if dropping the old value in timely manner is important (possibly because of
///   releasing large amount of RAM or because of closing file handles).
///
/// # Examples
///
/// ```rust
/// # fn do_something<V>(_v: V) { }
/// use std::sync::Arc;
///
/// use arc_swap::ArcSwap;
/// use arc_swap::cache::Cache;
///
/// let shared = Arc::new(ArcSwap::from_pointee(42));
/// // Start 10 worker threads...
/// for _ in 0..10 {
///     let mut cache = Cache::new(Arc::clone(&shared));
///     std::thread::spawn(move || {
///         // Keep loading it like mad..
///         loop {
///             let value = cache.load();
///             do_something(value);
///         }
///     });
/// }
/// shared.store(Arc::new(12));
/// ```
///
/// [Arc]: std::sync::Arc
/// [load]: ArcSwapAny::load
#[derive(Clone, Debug)]
pub struct Cache<A, T> {
    arc_swap: A,
    cached: T,
}

impl<A, T, S> Cache<A, T>
where
    A: Deref<Target = ArcSwapAny<T, S>>,
    T: RefCnt,
    S: LockStorage,
{
    /// Creates a new caching handle.
    ///
    /// The parameter is something dereferencing into an [`ArcSwapAny`] (eg. either to [`ArcSwap`]
    /// or [`ArcSwapOption`]). That can be [`ArcSwapAny`] itself, but that's not very useful. But
    /// it also can be a reference to it or `Arc`, which makes it possible to share the
    /// [`ArcSwapAny`] with multiple caches or access it in non-cached way too.
    ///
    /// [`ArcSwapOption`]: ::ArcSwapOption
    /// [`ArcSwap`]: ::ArcSwap
    pub fn new(arc_swap: A) -> Self {
        let cached = arc_swap.load();
        Self { arc_swap, cached }
    }

    /// Gives access to the (possibly shared) cached [`ArcSwapAny`].
    pub fn arc_swap(&self) -> &A::Target {
        &self.arc_swap
    }

    /// Loads the currently held value.
    ///
    /// This first checks if the cached value is up to date. This check is very cheap.
    ///
    /// If it is up to date, the cached value is simply returned without additional costs. If it is
    /// outdated, a load is done on the underlying shared storage. The newly loaded value is then
    /// stored in the cache and returned.
    #[inline]
    pub fn load(&mut self) -> &T {
        self.revalidate();
        self.load_no_revalidate()
    }

    #[inline]
    fn load_no_revalidate(&self) -> &T {
        &self.cached
    }

    #[inline]
    fn revalidate(&mut self) {
        let cached_ptr = RefCnt::as_ptr(&self.cached);
        let shared_ptr = self.arc_swap.ptr.load(Ordering::Relaxed);
        if cached_ptr != shared_ptr {
            self.cached = self.arc_swap.load();
        }
    }
}

impl<A, T, S> From<A> for Cache<A, T>
where
    A: Deref<Target = ArcSwapAny<T, S>>,
    T: RefCnt,
    S: LockStorage,
{
    fn from(arc_swap: A) -> Self {
        Self::new(arc_swap)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use {ArcSwap, ArcSwapOption};

    #[test]
    fn cached_value() {
        let a = ArcSwap::from_pointee(42);
        let mut c1 = Cache::new(&a);
        let mut c2 = Cache::new(&a);

        assert_eq!(42, **c1.load());
        assert_eq!(42, **c2.load());

        a.store(Arc::new(43));
        assert_eq!(42, **c1.load_no_revalidate());
        assert_eq!(43, **c1.load());
    }

    #[test]
    fn cached_through_arc() {
        let a = Arc::new(ArcSwap::from_pointee(42));
        let mut c = Cache::new(Arc::clone(&a));
        assert_eq!(42, **c.load());
        a.store(Arc::new(0));
        drop(a); // A is just one handle, the ArcSwap is kept alive by the cache.
    }

    #[test]
    fn cache_option() {
        let a = ArcSwapOption::from_pointee(42);
        let mut c = Cache::new(&a);

        assert_eq!(42, **c.load().as_ref().unwrap());
        a.store(None);
        assert!(c.load().is_none());
    }
}

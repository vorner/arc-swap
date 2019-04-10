#![deny(unsafe_code)]
//! XXX

use std::ops::Deref;
use std::sync::atomic::Ordering;

use super::gen_lock::LockStorage;
use super::ref_cnt::RefCnt;
use super::ArcSwapAny;

/// XXX
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
    /// XXX
    pub fn new(arc_swap: A) -> Self {
        let cached = arc_swap.load();
        Self { arc_swap, cached }
    }

    /// XXX
    pub fn arc_swap(&self) -> &A::Target {
        &self.arc_swap
    }

    /// XXX
    #[inline]
    pub fn load(&mut self) -> &T {
        self.revalidate();
        self.load_no_revalidate()
    }

    /// XXX
    #[inline]
    pub fn load_no_revalidate(&self) -> &T {
        &self.cached
    }

    /// XXX
    #[inline]
    pub fn revalidate(&mut self) {
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

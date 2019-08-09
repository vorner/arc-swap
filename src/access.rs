//! TODO

use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use crate::gen_lock::LockStorage;
use crate::ref_cnt::RefCnt;
use crate::{ArcSwapAny, Guard};

/// TODO
pub trait Access<T> {
    /// TODO
    type Guard: Deref<Target = T>;

    /// TODO
    fn load(&self) -> Self::Guard;
}

impl<T, A: Access<T>, P: Deref<Target = A>> Access<T> for P {
    type Guard = A::Guard;
    fn load(&self) -> Self::Guard {
        self.deref().load()
    }
}

impl<T: RefCnt, S: LockStorage> Access<T> for ArcSwapAny<T, S> {
    type Guard = Guard<'static, T>;

    fn load(&self) -> Self::Guard {
        self.load()
    }
}

/// TODO
pub struct DirectDeref<T: RefCnt>(Guard<'static, T>);

impl<T> Deref for DirectDeref<Arc<T>> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref().deref()
    }
}

impl<T, S: LockStorage> Access<T> for ArcSwapAny<Arc<T>, S> {
    type Guard = DirectDeref<Arc<T>>;
    fn load(&self) -> Self::Guard {
        DirectDeref(self.load())
    }
}

impl<T> Deref for DirectDeref<Rc<T>> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref().deref()
    }
}

impl<T, S: LockStorage> Access<T> for ArcSwapAny<Rc<T>, S> {
    type Guard = DirectDeref<Rc<T>>;
    fn load(&self) -> Self::Guard {
        DirectDeref(self.load())
    }
}

/// TODO
pub struct DynGuard<T: ?Sized>(Box<dyn Deref<Target = T>>);

impl<T: ?Sized> Deref for DynGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// TODO
pub trait DynAccess<T> {
    /// TODO
    fn load(&self) -> DynGuard<T>;
}

impl<T, A> DynAccess<T> for A
where
    A: Access<T>,
    A::Guard: 'static,
{
    fn load(&self) -> DynGuard<T> {
        DynGuard(Box::new(Access::load(self)))
    }
}

/// TODO
pub struct MapGuard<G, T> {
    _guard: G,
    // TODO: Send/Sync
    value: *const T,
}

unsafe impl<G, T> Send for MapGuard<G, T>
where
    G: Send,
    for<'a> &'a T: Send,
{
}

unsafe impl<G, T> Sync for MapGuard<G, T>
where
    G: Sync,
    for<'a> &'a T: Sync,
{
}

impl<G, T> Deref for MapGuard<G, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}

/// TODO
#[derive(Copy, Clone, Debug)]
pub struct Map<A, T, F> {
    access: A,
    projection: F,
    _t: PhantomData<fn() -> T>,
}

impl<A, T, F> Map<A, T, F> {
    pub(crate) fn new<R>(access: A, projection: F) -> Self
    where
        F: Fn(&T) -> &R,
    {
        Map {
            access,
            projection,
            _t: PhantomData,
        }
    }
}

impl<A, T, F, R> Access<R> for Map<A, T, F>
where
    A: Access<T>,
    F: Fn(&T) -> &R,
{
    type Guard = MapGuard<A::Guard, R>;
    fn load(&self) -> Self::Guard {
        let guard = self.access.load();
        let value: *const _ = (self.projection)(&guard);
        MapGuard {
            _guard: guard,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArcSwap, ArcSwapOption};

    use super::*;

    fn check_static_dispatch_direct<A: Access<usize>>(a: A) {
        assert!(42 == *a.load());
    }

    fn check_static_dispatch<A: Access<Arc<usize>>>(a: A) {
        assert!(42 == **a.load());
    }

    /// Tests dispatching statically from arc-swap works
    #[test]
    fn static_dispatch() {
        let a = ArcSwap::from_pointee(42);
        check_static_dispatch_direct(&a);
        check_static_dispatch(&a);
        check_static_dispatch(a);
    }

    fn check_dyn_dispatch_direct(a: &dyn DynAccess<usize>) {
        assert!(42 == *a.load());
    }

    fn check_dyn_dispatch(a: &dyn DynAccess<Arc<usize>>) {
        assert!(42 == **a.load());
    }

    /// Tests we can also do a dynamic dispatch of the companion trait
    #[test]
    fn dyn_dispatch() {
        let a = ArcSwap::from_pointee(42);
        check_dyn_dispatch_direct(&a);
        check_dyn_dispatch(&a);
    }

    fn check_transition<A>(a: A)
    where
        A: Access<usize>,
        A::Guard: 'static,
    {
        check_dyn_dispatch_direct(&a)
    }

    /// Tests we can easily transition from the static dispatch trait to the dynamic one
    #[test]
    fn transition() {
        let a = ArcSwap::from_pointee(42);
        check_transition(&a);
        check_transition(a);
    }

    /// Test we can dispatch from Arc<ArcSwap<_>> or similar.
    #[test]
    fn indirect() {
        let a = Arc::new(ArcSwap::from_pointee(42));
        check_static_dispatch(&a);
        check_dyn_dispatch(&a);
    }

    struct Cfg {
        value: usize,
    }

    #[test]
    fn map() {
        let a = ArcSwap::from_pointee(Cfg { value: 42 });
        let map = a.map(|a: &Cfg| &a.value);
        check_static_dispatch_direct(&map);
        check_dyn_dispatch_direct(&map);
    }

    #[test]
    fn map_option_some() {
        let a = ArcSwapOption::from_pointee(Cfg { value: 42 });
        let map = a.map(|a: &Option<Arc<Cfg>>| a.as_ref().map(|c| &c.value).unwrap());
        check_static_dispatch_direct(&map);
        check_dyn_dispatch_direct(&map);
    }

    #[test]
    fn map_option_none() {
        let a = ArcSwapOption::empty();
        let map = a.map(|a: &Option<Arc<Cfg>>| a.as_ref().map(|c| &c.value).unwrap_or(&42));
        check_static_dispatch_direct(&map);
        check_dyn_dispatch_direct(&map);
    }
}

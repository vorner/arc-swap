//! TODO

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

#[cfg(test)]
mod tests {
    use crate::ArcSwap;

    use super::*;

    fn check_static_dispatch_direct<A: Access<usize>>(a: A) {
        assert!(42 == *a.load());
    }

    fn check_static_dispatch<A: Access<Arc<usize>>>(a: A) {
        assert!(42 == **a.load());
    }

    /// Tests dispatching statically from arc-swap works
    #[test]
    fn test_static() {
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
    fn test_dyn() {
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
    fn test_transition() {
        let a = ArcSwap::from_pointee(42);
        check_transition(&a);
        check_transition(a);
    }

    /// Test we can dispatch from Arc<ArcSwap<_>> or similar.
    #[test]
    fn test_indirect() {
        let a = Arc::new(ArcSwap::from_pointee(42));
        check_static_dispatch(&a);
        check_dyn_dispatch(&a);
    }
}

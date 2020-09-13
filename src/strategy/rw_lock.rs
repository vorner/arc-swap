use std::sync::RwLock;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::ref_cnt::RefCnt;
use super::{Protected, Sealed, Strategy};

impl<T: RefCnt> Sealed for T {}
impl<T: RefCnt> Protected<T> for T {
    #[inline]
    fn from_inner(ptr: T) -> Self {
        ptr
    }

    #[inline]
    fn into_inner(self) -> T {
        self
    }
}

impl Sealed for RwLock<()> {}
impl<T: RefCnt> Strategy<T> for RwLock<()> {
    type Protected = T;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> T {
        let _guard = self.read().expect("We don't panic in here");
        let ptr = storage.load(Ordering::Acquire);
        let ptr = T::from_ptr(ptr as *const T::Base);
        T::inc(&ptr);

        ptr
    }

    unsafe fn wait_for_readers(&self, _: *const T::Base) {
        // By acquiring the write lock, we make sure there are no read locks present across it.
        drop(self.write().expect("We don't panic in here"));
    }
}

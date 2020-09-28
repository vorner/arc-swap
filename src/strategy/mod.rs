use std::borrow::Borrow;
use std::sync::atomic::AtomicPtr;

use crate::ref_cnt::RefCnt;
use inner::Sealed;

mod hybrid;
mod rw_lock;

pub use self::hybrid::HybridStrategy;

mod inner {
    // TODO: When we are ready to un-seal, should these traits become unsafe?
    pub trait Sealed {}
}

pub trait Protected<T>: inner::Sealed + Borrow<T> {
    fn into_inner(self) -> T;
    fn from_inner(ptr: T) -> Self;
}

pub trait Strategy<T: RefCnt>: Sealed {
    // Drop „unlocks“
    type Protected: Protected<T>;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected;
    unsafe fn wait_for_readers(&self, old: *const T::Base);
}

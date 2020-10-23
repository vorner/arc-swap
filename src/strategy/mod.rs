use std::borrow::Borrow;
use std::sync::atomic::AtomicPtr;

use crate::ref_cnt::RefCnt;

mod hybrid;
mod rw_lock;

pub use self::hybrid::HybridStrategy;

// TODO: When we are ready to un-seal, should these traits become unsafe?

pub(crate) mod sealed {
    use crate::as_raw::AsRaw;
    use super::*;

    pub trait Protected<T>: Borrow<T> {
        fn into_inner(self) -> T;
        fn from_inner(ptr: T) -> Self;
    }

    pub trait InnerStrategy<T: RefCnt> {
        // Drop „unlocks“
        type Protected: Protected<T>;
        unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected;
        unsafe fn wait_for_readers(&self, old: *const T::Base);
    }

    pub trait CaS<T: RefCnt>: InnerStrategy<T> {
        unsafe fn compare_and_swap<C: AsRaw<T::Base>>(&self, storage: &AtomicPtr<T::Base>, current: C, new: T) -> Self::Protected;
    }
}

pub trait Strategy<T: RefCnt>: sealed::InnerStrategy<T> {}
impl<T: RefCnt, S: sealed::InnerStrategy<T>> Strategy<T> for S {}

pub trait CaS<T: RefCnt>: sealed::CaS<T> {}
impl<T: RefCnt, S: sealed::CaS<T>> CaS<T> for S {}
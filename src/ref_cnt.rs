use std::mem;
use std::ptr;
use std::sync::Arc;

pub unsafe trait RefCnt: Clone {
    type Base;
    fn into_ptr(me: Self) -> *mut Self::Base;
    fn as_ptr(me: &Self) -> *mut Self::Base;
    unsafe fn from_ptr(ptr: *const Self::Base) -> Self;
    fn inc(me: &Self) {
        mem::forget(Self::clone(me));
    }
    unsafe fn dec(ptr: *const Self::Base) {
        drop(Self::from_ptr(ptr));
    }
    fn can_null() -> bool {
        true
    }
}

pub unsafe trait NonNull: RefCnt {}

unsafe impl<T> RefCnt for Arc<T> {
    type Base = T;
    fn into_ptr(me: Arc<T>) -> *mut T {
        Arc::into_raw(me) as *mut T
    }
    fn as_ptr(me: &Arc<T>) -> *mut T {
        me as &T as *const T as *mut T
    }
    unsafe fn from_ptr(ptr: *const T) -> Arc<T> {
        Arc::from_raw(ptr)
    }
    fn can_null() -> bool {
        false
    }
}

unsafe impl<T> RefCnt for Option<Arc<T>> {
    type Base = T;
    fn into_ptr(me: Option<Arc<T>>) -> *mut T {
        me.map(Arc::into_ptr).unwrap_or_else(ptr::null_mut)
    }
    fn as_ptr(me: &Option<Arc<T>>) -> *mut T {
        me.as_ref().map(Arc::as_ptr).unwrap_or_else(ptr::null_mut)
    }
    unsafe fn from_ptr(ptr: *const T) -> Option<Arc<T>> {
        if ptr.is_null() {
            None
        } else {
            Some(Arc::from_ptr(ptr))
        }
    }
}

unsafe impl<T> NonNull for Arc<T> {}

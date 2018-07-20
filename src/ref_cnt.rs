use std::ptr;
use std::sync::Arc;

/// A trait describing smart reference counted pointers.
///
/// Note that in a form `Option<Arc<T>>` is also a smart reference counted pointer, just one that
/// can hold NULL.
///
/// The trait is unsafe, because a wrong implementation will break the
/// [`ArcSwapAny`](struct.ArcSwapAny.html) implementation and lead to UB.
///
/// This is not actually expected for downstream crate to implement, this is just means to reuse
/// code for `Arc` and `Option<Arc>` variants. However, it is theoretically possible (if you have
/// your own `Arc` implementation).
///
/// Implementing it for `Rc` is possible, but not useful (because the `ArcSwap` then would not be
/// `Send` nor `Sync`, so there's very little advantage of using it if it can't be shared between
/// threads).
///
/// Aside from the obvious properties (like that incrementing and decrementing a reference count
/// cancel each out and that having less references tracked than how many things actually point to
/// the value is fine as long as the count doesn't drop to 0), it also must satisfy that if two
/// pointers have the same value, they point to the same object. This is specifically not true for
/// ZSTs, but it is true for `Arc`s of ZSTs, because they have the reference counts just after the
/// value. It would be fine to point to a type-erased version of the same object, though (if one
/// could use this trait with unsized types in the first place).
pub unsafe trait RefCnt: Clone {
    /// The base type the pointer points to.
    type Base;

    /// Converts the smart pointer into a raw pointer, without affecting the reference count.
    ///
    /// This can be seen as kind of freezing the pointer ‒ it'll be later converted back using
    /// [`from_ptr`](#method.from_ptr).
    ///
    /// The pointer must point to the value stored (and the value must be the same as one returned
    /// by [`as_ptr`](#method.as_ptr).
    fn into_ptr(me: Self) -> *mut Self::Base;

    /// Provides a view into the smart pointer as a raw pointer.
    ///
    /// This must not affect the reference count ‒ the pointer is only borrowed.
    fn as_ptr(me: &Self) -> *mut Self::Base;

    /// Converts a raw pointer back into the smart pointer, without affecting the reference count.
    ///
    /// This is only called on values previously returned by [`into_ptr`](#method.into_ptr).
    /// However, it is not guaranteed to be 1:1 relation ‒ `from_ptr` may be called more times than
    /// `into_ptr` temporarily provided the reference count never drops under 1 during that time
    /// (the implementation sometimes owes a reference).
    unsafe fn from_ptr(ptr: *const Self::Base) -> Self;

    /// Increments the reference count by one.
    fn inc(me: &Self) {
        Self::into_ptr(Self::clone(me));
    }

    /// Decrements the reference count by one.
    ///
    /// Note this is called on a raw pointer (one previously returned by
    /// [`into_ptr`](#method.into_ptr). This may lead to dropping of the reference count to 0 and
    /// destruction of the internal pointer.
    unsafe fn dec(ptr: *const Self::Base) {
        drop(Self::from_ptr(ptr));
    }

    /// Describes if the raw pointer can ever be null.
    ///
    /// Things like `Arc` are never null and can safely return false here. This is used only for
    /// better formatting ‒ lying here won't cause an UB, but can cause uglier debug output or
    /// panic inside debug formatting.
    fn can_null() -> bool;
}

/// A trait describing smart pointers that can't hold NULL.
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

unsafe impl<T: NonNull> RefCnt for Option<T> {
    type Base = T::Base;
    fn into_ptr(me: Option<T>) -> *mut T::Base {
        me.map(T::into_ptr).unwrap_or_else(ptr::null_mut)
    }
    fn as_ptr(me: &Option<T>) -> *mut T::Base {
        me.as_ref().map(T::as_ptr).unwrap_or_else(ptr::null_mut)
    }
    unsafe fn from_ptr(ptr: *const T::Base) -> Option<T> {
        if ptr.is_null() {
            None
        } else {
            Some(T::from_ptr(ptr))
        }
    }
    fn can_null() -> bool {
        true
    }
}

unsafe impl<T> NonNull for Arc<T> {}

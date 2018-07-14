use super::{Guard, RefCnt};

pub trait AsRaw<T> {
    fn as_raw(self) -> *mut T;
}

impl<'a, T: RefCnt> AsRaw<T::Base> for &'a T {
    fn as_raw(self) -> *mut T::Base {
        T::as_ptr(self)
    }
}

impl<'a, 'r, T: RefCnt> AsRaw<T::Base> for &'r Guard<'a, T> {
    fn as_raw(self) -> *mut T::Base {
        self.ptr as *mut _
    }
}

impl<'a, T: RefCnt> AsRaw<T::Base> for Guard<'a, T> {
    fn as_raw(self) -> *mut T::Base {
        self.ptr as *mut _
    }
}

impl<T> AsRaw<T> for *mut T {
    fn as_raw(self) -> *mut T {
        self
    }
}

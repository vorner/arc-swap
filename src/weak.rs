use std::sync::Weak;

use crate::RefCnt;

unsafe impl<T> RefCnt for Weak<T> {
    type Base = T;
    fn as_ptr(me: &Self) -> *mut T {
        Weak::as_raw(me) as *mut T
    }
    fn into_ptr(me: Self) -> *mut T {
        Weak::into_raw(me) as *mut T
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        Weak::from_raw(ptr)
    }
    fn can_null() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Weak};

    use crate::{ArcSwapWeak, Guard, Lease};

    // Convert to weak, push it through the shared and pull it out again.
    #[test]
    fn there_and_back() {
        let data = Arc::new("Hello");
        let shared = ArcSwapWeak::new(Arc::downgrade(&data));
        assert_eq!(1, Arc::strong_count(&data));
        assert_eq!(1, Arc::weak_count(&data));
        let weak = shared.load();
        assert_eq!("Hello", *weak.upgrade().unwrap());
        assert!(Arc::ptr_eq(&data, &weak.upgrade().unwrap()));

        assert_eq!("Hello", *Lease::get_ref(&shared.lease()).unwrap());
        assert_eq!("Hello", *Guard::get_ref(&shared.peek()).unwrap());
    }

    // Replace a weak pointer with a NULL one
    #[test]
    fn reset() {
        let data = Arc::new("Hello");
        let shared = ArcSwapWeak::new(Arc::downgrade(&data));
        assert_eq!(1, Arc::strong_count(&data));
        assert_eq!(1, Arc::weak_count(&data));

        // An empty weak (eg. NULL)
        shared.store(Weak::new());
        assert_eq!(1, Arc::strong_count(&data));
        assert_eq!(0, Arc::weak_count(&data));

        let weak = shared.load();
        assert!(weak.upgrade().is_none());

        assert!(Lease::get_ref(&shared.lease()).is_none());
        assert!(Lease::is_null(&shared.lease()));

        assert!(Guard::get_ref(&shared.peek()).is_none());
    }

    // Destroy the underlying data while the weak is still stored inside. Should make it go
    // NULL-ish
    #[test]
    fn destroy() {
        let data = Arc::new("Hello");
        let shared = ArcSwapWeak::new(Arc::downgrade(&data));

        drop(data);
        let weak = shared.load();
        assert!(weak.upgrade().is_none());

        // FIXME: These don't work. That's because while the target of Weak got already destroyed,
        // the Weak::as_raw and similar still have non-null *dangling* pointer.
        assert!(Lease::get_ref(&shared.lease()).is_none());
        assert!(Lease::is_null(&shared.lease()));

        assert!(Guard::get_ref(&shared.peek()).is_none());
    }
}

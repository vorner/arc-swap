use std::marker::PhantomData;
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

fn dispose<T>(ptr: *const T) {
    assert!(!ptr.is_null());
    unsafe {
        Arc::from_raw(ptr);
    }
}

fn strip<T>(arc: Arc<T>) -> *mut T {
    Arc::into_raw(arc) as *mut T
}

pub struct ArcSwap<T> {
    // Notes: AtomicPtr needs Sized
    ptr: AtomicPtr<T>,
    // We are basically an Arc in disguise. Inherit parameters from Arc by pretending to contain
    // it.
    _phantom_arc: PhantomData<Arc<T>>,
}

impl<T> From<Arc<T>> for ArcSwap<T> {
    fn from(arc: Arc<T>) -> Self {
        // The AtomicPtr requires *mut in its interface. We are more like *const, so we cast it.
        // However, we always go back to *const right away when we get the pointer on the other
        // side, so it should be fine.
        let ptr = strip(arc);
        Self {
            ptr: AtomicPtr::new(ptr),
            _phantom_arc: PhantomData,
        }
    }
}

impl<T> Drop for ArcSwap<T> {
    fn drop(&mut self) {
        // Note that by now we are visible only by one thread, so we can abandon all these
        // atomic-ordering madnesses.

        // We hold one reference in the Arc, but it's hidden. Convert us back to Arc and drop that
        // Arc instead of us, which will clear the ref.
        let ptr = *self.ptr.get_mut();
        dispose(ptr);
    }
}

impl<T> ArcSwap<T> {
    fn get(&self) -> *const T {
        loop {
            // Get the content of the pointer.
            //
            // The ordering: We don't publish anything (that null points nowhere), so it's fine not
            // to release.
            //
            // We do acquire, because incrementing an Arc's ref count doesn't and relies on
            // something that provided the Arc in the first place to do so. In this case it is us
            // that smuggles the pointer from one thread to another through this atomic.
            let ptr = self.ptr.swap(ptr::null_mut(), Ordering::Acquire);
            // If, by accident, the ptr is null, it means it is just borrowed to play with. It'll
            // get back soon, so try again.
            if !ptr.is_null() {
                return ptr;
            }
        }
    }
    pub fn load(&self) -> Arc<T> {
        // Borrow the reference count by taking it out of the atomic ptr for a short while
        let ptr = self.get();
        assert!(ptr.is_null());
        let arc = unsafe {
            Arc::from_raw(ptr)
        };
        // Bump the reference count by one, so we can return one into the shared ptr and another to
        // the caller.
        assert!(Arc::into_raw(Arc::clone(&arc)) == ptr);
        // Return the pointer and reference count back
        let likely_null = self.ptr.compare_and_swap(
            ptr::null_mut(), // Store it only if there's still NULL
            ptr as *mut T,
            // The relaxed ordering: we don't publish anything new, the data at the end of the
            // pointer is still published by whatever put it into there in the first place. And we
            // don't read the target of the returned pointer.
            Ordering::Relaxed
        );
        if !likely_null.is_null() {
            dispose(ptr);
        }

        arc
    }
    pub fn store(&self, arc: Arc<T>) {
        let ptr = strip(arc);
        let orig = self.ptr.swap(ptr, Ordering::Release);
        if !orig.is_null() {
            // If there was something before (there might have been a temporary hole, but the one
            // trying to plug it back will discover we replaced it in between), drop it properly.
            dispose(orig);
        }
    }
    pub fn swap(&self, arc: Arc<T>) -> Arc<T> {
        let ptr = strip(arc);
        loop {
            let orig = self.ptr.load(Ordering::Relaxed);
            if orig.is_null() {
                continue;
            }
            let exchanged = self.ptr.compare_exchange_weak(
                orig,
                ptr,
                Ordering::AcqRel,
                Ordering::Relaxed
            );
            if let Ok(extracted) = exchanged {
                assert!(extracted == orig);
                unsafe {
                    return Arc::from_raw(extracted);
                }
            }
        }
    }
}

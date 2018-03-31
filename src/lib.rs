//! Making [`Arc`] itself atomic
//!
//! The [`Arc`] uses atomic reference counters, so the object behind it can be safely pointed to by
//! several threads at once. However, the [`Arc`] itself is quite ordinary ‒ to change its value
//! (make it point somewhere else), one has to be the sole owner of it.
//!
//! On the other hand, there's [`AtomicPtr`]. It can be modified and read from multiple threads,
//! allowing to pass the value from one thread to another without the use of a [`Mutex`]. The
//! downside is, tracking when the data can be safely deleted is hard.
//!
//! This library provides [`ArcSwap`] that allows both at once. It can be constructed from ordinary
//! [`Arc`], but its value can be loaded and stored atomically, my multiple concurrent threads.
//!
//! # Motivation
//!
//! For one, the C++ [`shared_ptr`] has this
//! [ability](http://en.cppreference.com/w/cpp/memory/shared_ptr/atomic), so it is only fair to
//! have it too.
//!
//! For another, it seemed like a really good exercise.
//!
//! And finally, there are some real use cases for this functionality. For example, when one thread
//! publishes something (for example configuration) and other threads want to have a peek to the
//! current one from time to time. There's a global [`ArcSwap`], holding the current snapshot and
//! everyone is free to make a copy and hold onto it for a while. The publisher thread simply
//! stores a new snapshot every time and the old configuration gets dropped once all the other
//! threads give up their copies of the pointer.
//!
//! # Example
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//!
//! use std::sync::Arc;
//!
//! use arc_swap::ArcSwap;
//! use crossbeam_utils::scoped as thread;
//!
//! fn main() {
//!     let config = ArcSwap::from(Arc::new(String::default()));
//!     thread::scope(|scope| {
//!         scope.spawn(|| {
//!             let new_conf = Arc::new("New configuration".to_owned());
//!             config.store(new_conf);
//!         });
//!         for _ in 0..10 {
//!             scope.spawn(|| {
//!                 loop {
//!                     let cfg = config.load();
//!                     if !cfg.is_empty() {
//!                         assert_eq!(*cfg, "New configuration");
//!                         return;
//!                     }
//!                 }
//!             });
//!         }
//!     });
//! }
//! ```
//!
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`AtomicPtr`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html
//! [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
//! [`shared_ptr`]: http://en.cppreference.com/w/cpp/memory/shared_ptr

use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{self, AtomicPtr, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

// TODO: Implementation notes/rationale/proofs

/// Dispose of one ref count for this pointer.
fn dispose<T>(ptr: *const T) {
    assert!(!ptr.is_null());
    unsafe {
        Arc::from_raw(ptr);
    }
}

/// Turn the arc into a raw pointer.
fn strip<T>(arc: Arc<T>) -> *mut T {
    Arc::into_raw(arc) as *mut T
}

/// An atomic storage for [`Arc`].
///
/// This is a storage where an [`Arc`] may live. It can be read and written atomically from several
/// threads, but doesn't act like a pointer itself.
///
/// One can be created [`from`] an [`Arc`]. To get an [`Arc`] back, use the [`load`] method.
///
/// # Examples
///
/// ```rust
/// # use std::sync::Arc;
/// # use arc_swap::ArcSwap;
/// let arc = Arc::new(42);
/// let arc_swap = ArcSwap::from(arc);
/// assert_eq!(42, *arc_swap.load());
/// // It can be read multiple times
/// assert_eq!(42, *arc_swap.load());
///
/// // Put a new one in there
/// let new_arc = Arc::new(0);
/// assert_eq!(42, *arc_swap.swap(new_arc));
/// assert_eq!(0, *arc_swap.load());
/// ```
///
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// [`from`]: https://doc.rust-lang.org/nightly/std/convert/trait.From.html#tymethod.from
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
        assert!(!ptr.is_null());
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

static GEN_IDX: AtomicUsize = ATOMIC_USIZE_INIT;
static GEN_READERS: AtomicUsize = ATOMIC_USIZE_INIT;

impl<T> ArcSwap<T> {
    /// Loads the value.
    ///
    /// This makes another copy (reference) and returns it, atomically (it is safe even when other
    /// thread stores into the same instance at the same time).
    pub fn load(&self) -> Arc<T> {
        let gen = GEN_IDX.load(Ordering::Acquire);
        let gen_val = 1 << (16 * gen);
        GEN_READERS.fetch_add(gen_val, Ordering::Acquire);
        let ptr = self.ptr.load(Ordering::Acquire);
        assert!(!ptr.is_null());
        let arc = unsafe {
            Arc::from_raw(ptr)
        };
        // Bump the reference count by one, so we can return one into the arc and another to
        // the caller.
        assert!(Arc::into_raw(Arc::clone(&arc)) == ptr);
        GEN_READERS.fetch_sub(gen_val, Ordering::Release);
        arc
    }

    /// Replaces the value inside this instance.
    ///
    /// Further loads will yield the new value.
    pub fn store(&self, arc: Arc<T>) {
        self.swap(arc);
    }

    /// Exchanges the value inside this instance.
    pub fn swap(&self, arc: Arc<T>) -> Arc<T> {
        let new = strip(arc);
        let old = self.ptr.swap(new, Ordering::AcqRel);
        assert!(!old.is_null());
        // TODO: Handle the generations
        while GEN_READERS.load(Ordering::Acquire) > 0 {
            atomic::spin_loop_hint();
        }
        unsafe {
            Arc::from_raw(old)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate crossbeam_utils;

    use super::*;
    use self::crossbeam_utils::scoped as thread;

    use std::sync::atomic::AtomicUsize;

    #[test]
    fn publish() {
        for _ in 0..1000 {
            let config = ArcSwap::from(Arc::new(String::default()));
            let ended = AtomicUsize::new(0);
            thread::scope(|scope| {
                scope.spawn(|| {
                    let new_conf = Arc::new("New configuration".to_owned());
                    config.store(new_conf);
                });
                for _ in 0..10 {
                    scope.spawn(|| {
                        loop {
                            let cfg = config.load();
                            if !cfg.is_empty() {
                                assert_eq!(*cfg, "New configuration");
                                ended.fetch_add(1, Ordering::Relaxed);
                                return;
                            }
                        }
                    });
                }
            });
            assert_eq!(10, ended.load(Ordering::Relaxed));
        }
    }
}

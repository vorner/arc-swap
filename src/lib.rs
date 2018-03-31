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
//! # Performance characteristics
//!
//! Some benchmarks need to be done. Due to the complexity, it may be possible that using
//! `Mutex<Arc<T>>` might be faster in some cases.
//!
//! However, this implementation doesn't suffer from contention. Specifically, arbitrary number of
//! readers can access the shared value and won't be blocked. Even when there are many readers and
//! writers at once, the writers, they don't block each other. The writers will be somewhat slower
//! when there are active readers at the same time, but won't be stopped indefinitely.
//!
//! # Limitations
//!
//! Current implementation doesn't support more than 2^16 on 32-bit systems or 2^32 on 64-bit
//! systems concurrent readers (that shouldn't be a problem, as so many threads would probably kill
//! the OS anyway).
//!
//! Other pointer widths are currently not supported, but if specific size is needed, it can be
//! added.
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
use std::mem;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{self, AtomicPtr, AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

// TODO: Implementation notes/rationale/proofs

/// Store count for 2 newest generations (others must always be 0)
const GEN_CNT: usize = 2;

/// Either 0 or 1, describing current generation where readers should register.
static GEN_IDX: AtomicUsize = ATOMIC_USIZE_INIT;

/// Count of readers in either generation (at bits 0-15 & 16-31).
static GEN_READERS: AtomicUsize = ATOMIC_USIZE_INIT;

/// Dispose of one ref count for this pointer.
fn dispose<T>(ptr: *const T) {
    unsafe {
        Arc::from_raw(ptr);
    }
}

/// Turn the arc into a raw pointer.
fn strip<T>(arc: Arc<T>) -> *mut T {
    Arc::into_raw(arc) as *mut T
}

fn mask(gen: usize) -> usize {
    if mem::size_of::<usize>() == 4 {
        match gen {
            0 => 0x0000_ffff,
            1 => 0xffff_0000,
            _ => unreachable!(),
        }
    } else if mem::size_of::<usize>() == 8 {
        match gen {
            0 => 0x0000_0000_ffff_ffff,
            1 => 0xffff_ffff_0000_0000,
            _ => unreachable!(),
        }
    } else {
        unimplemented!("Unsupported pointer width");
    }
}

/// Wait until all readers go away.
fn wait_for_readers() {
    for _ in 0..2 {
        let gen = GEN_IDX.load(Ordering::Acquire);
        let mask = mask(1 - gen % GEN_CNT);
        loop {
            let state = GEN_READERS.load(Ordering::Acquire);
            // If there are no readers at all, it's safe ‒ nobody to wait for, shortcircuit the
            // whole thing.
            if state == 0 {
                return;
            }
            // If the other generation gets empty, we can proceed and perform the switch
            if state & mask == 0 {
                break;
            }
            let cur_gen = GEN_IDX.load(Ordering::Relaxed);
            // Someone has advanced the generation already
            if cur_gen != gen {
                break;
            }
            atomic::spin_loop_hint();
        }
        let new_gen = gen.wrapping_add(1);
        // Try advancing the generation. Don't worry if it doesn't work, because it can happen only
        // if someone else already did that.
        GEN_IDX.compare_and_swap(gen, new_gen, Ordering::Release);
    }
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
    /// Loads the value.
    ///
    /// This makes another copy (reference) and returns it, atomically (it is safe even when other
    /// thread stores into the same instance at the same time).
    pub fn load(&self) -> Arc<T> {
        let gen = GEN_IDX.load(Ordering::Acquire) % GEN_CNT;
        let gen_val = 1 << (mem::size_of::<usize>() * 4 * gen);
        let previous = GEN_READERS.fetch_add(gen_val, Ordering::Acquire);
        let mask = mask(gen);
        // Too many readers at once. We detect it after the fact and have no way to fix it, so give
        // up. Note that panic wouldn't be enough.
        if previous & mask == mask {
            process::abort();
        }
        let ptr = self.ptr.load(Ordering::Acquire);
        let arc = unsafe {
            Arc::from_raw(ptr)
        };
        // Bump the reference count by one, so we can return one into the arc and another to
        // the caller.
        Arc::into_raw(Arc::clone(&arc));
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
        wait_for_readers();
        unsafe {
            Arc::from_raw(old)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate crossbeam_utils;

    use std::sync::Barrier;
    use std::sync::atomic::AtomicUsize;

    use super::*;
    use self::crossbeam_utils::scoped as thread;

    /// Similar to the one in doc tests of the lib, but more times and more intensive (we want to
    /// torture it a bit).
    ///
    /// Takes some time, presumably because this starts 21 000 threads during its lifetime and 20
    /// 000 of them just wait in a tight loop for the other thread to happen.
    #[test]
    fn publish() {
        for _ in 0..1000 {
            let config = ArcSwap::from(Arc::new(String::default()));
            let ended = AtomicUsize::new(0);
            thread::scope(|scope| {
                for _ in 0..20 {
                    scope.spawn(|| {
                        loop {
                            let cfg = config.load();
                            if !cfg.is_empty() {
                                assert_eq!(*cfg, "New configuration");
                                ended.fetch_add(1, Ordering::Relaxed);
                                return;
                            }
                            atomic::spin_loop_hint();
                        }
                    });
                }
                scope.spawn(|| {
                    let new_conf = Arc::new("New configuration".to_owned());
                    config.store(new_conf);
                });
            });
            assert_eq!(20, ended.load(Ordering::Relaxed));
            assert_eq!(2, Arc::strong_count(&config.load()));
            assert_eq!(0, Arc::weak_count(&config.load()));
        }
    }

    /// Similar to the doc tests of ArcSwap, but happens more times.
    #[test]
    fn swap_load() {
        for _ in 0..1000 {
            let arc = Arc::new(42);
            let arc_swap = ArcSwap::from(Arc::clone(&arc));
            assert_eq!(42, *arc_swap.load());
            // It can be read multiple times
            assert_eq!(42, *arc_swap.load());

            // Put a new one in there
            let new_arc = Arc::new(0);
            assert_eq!(42, *arc_swap.swap(Arc::clone(&new_arc)));
            assert_eq!(0, *arc_swap.load());
            // One loaded here, one in the arc_swap, one in new_arc
            assert_eq!(3, Arc::strong_count(&arc_swap.load()));
            assert_eq!(0, Arc::weak_count(&arc_swap.load()));
            // The original got released from the arc_swap
            assert_eq!(1, Arc::strong_count(&arc));
            assert_eq!(0, Arc::weak_count(&arc));
        }
    }

    /// Two different writers publish two series of values. The readers check that it is always
    /// increasing in each serie.
    ///
    /// For performance, we try to reuse the threads here.
    #[test]
    fn multi_writers() {
        let first_value = Arc::new((0, 0));
        let shared = ArcSwap::from(Arc::clone(&first_value));
        const WRITER_CNT: usize = 2;
        const READER_CNT: usize = 3;
        const ITERATIONS: usize = 100;
        const SEQ: usize = 50;
        let barrier = Barrier::new(READER_CNT + WRITER_CNT);
        thread::scope(|scope| {
            for w in 0..WRITER_CNT {
                // We need to move w into the closure. But we want to just reference the other
                // things.
                let barrier = &barrier;
                let shared = &shared;
                let first_value = &first_value;
                scope.spawn(move || {
                    for _ in 0..ITERATIONS {
                        barrier.wait();
                        shared.store(Arc::clone(&first_value));
                        barrier.wait();
                        for i in 0..SEQ {
                            shared.store(Arc::new((w, i + 1)));
                        }
                    }
                });
            }
            for _ in 0..READER_CNT {
                scope.spawn(|| {
                    for _ in 0..ITERATIONS {
                        barrier.wait();
                        barrier.wait();
                        let mut previous = [0; 2];
                        let mut last = Arc::clone(&first_value);
                        loop {
                            let cur = shared.load();
                            if Arc::ptr_eq(&last, &cur) {
                                atomic::spin_loop_hint();
                                continue;
                            }
                            let (w, s) = *cur;
                            assert!(previous[w] < s);
                            previous[w] = s;
                            last = cur;
                            if s == SEQ {
                                break;
                            }
                        }
                    }
                });
            }
        });
    }
}

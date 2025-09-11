use core::sync::atomic::{AtomicPtr, Ordering};
use shuttle::sync::RwLock;

use super::sealed::{CaS, InnerStrategy};
use crate::as_raw::AsRaw;
use crate::ref_cnt::RefCnt;

/// Strategy for concurrency testing using the [`shuttle`] crate.
/// 
/// This strategy uses shuttle concurrency primitives to implement ArcSwap. This allows shuttle to
/// perform reordering around ArcLock operations such as [`rcu`], and explore races associated with
/// the ordering of ArcSwap stores and loads.
/// 
/// This strategy will panic if used outside of the context of a shuttle execution.
/// 
/// Requires the `shuttle` feature to be enabled.
/// 
/// [`shuttle`]: https://github.com/awslabs/shuttle
/// [`rcu`]: crate::ArcSwapAny::rcu
#[derive(Default)]
pub struct ShuttleStrategy {
    inner: RwLock<()>,
}
impl std::fmt::Debug for ShuttleStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShuttleStrategy").finish()
    }
}
impl Clone for ShuttleStrategy {
    fn clone(&self) -> Self {
        Self::default()
    }
}

// impl Protected<T> provided in rw_lock.rs

// The below strategy is essentially the same as rw_lock.rs, but uses shuttle's implementation of
// RwLock instead.
impl<T: RefCnt> InnerStrategy<T> for ShuttleStrategy {
    type Protected = T;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> T {
        let _guard = self.inner.read().expect("We don't panic in here");
        let ptr = storage.load(Ordering::Acquire);
        let ptr = T::from_ptr(ptr as *const T::Base);
        T::inc(&ptr);

        ptr
    }

    unsafe fn wait_for_readers(&self, _: *const T::Base, _: &AtomicPtr<T::Base>) {
        // By acquiring the write lock, we make sure there are no read locks present across it.
        drop(self.inner.write().expect("We don't panic in here"));
    }
}

impl<T: RefCnt> CaS<T> for ShuttleStrategy {
    unsafe fn compare_and_swap<C: AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        let _lock = self.inner.write();
        let cur = current.as_raw();
        let new = T::into_ptr(new);
        let swapped = storage.compare_exchange(cur, new, Ordering::AcqRel, Ordering::Relaxed);
        let old = match swapped {
            Ok(old) => old,
            Err(old) => old,
        };
        let old = T::from_ptr(old as *const T::Base);
        if swapped.is_err() {
            // If the new didn't go in, we need to destroy it and increment count in the old that
            // we just duplicated
            T::inc(&old);
            drop(T::from_ptr(new));
        }
        drop(current);
        old
    }
}

#[cfg(test)]
mod test {
    use shuttle::thread;
    use shuttle::sync::{Mutex, Arc};

    use crate::ArcSwapAny;
    use super::ShuttleStrategy;

    type ArcSwap<T> = ArcSwapAny<Arc<T>, ShuttleStrategy>;

    #[test]
    fn arc_swap_works() {
        shuttle::check_random(|| {
            let arcswap: Arc<ArcSwapAny<Arc<u64>, ShuttleStrategy>> = Arc::new(ArcSwap::from_pointee(42));
            let result = Arc::new(Mutex::new(0));

            let arcswap_ = arcswap.clone();
            let result_ = result.clone();
            let t1 = thread::spawn(move || {
                *result_.lock().unwrap() = **arcswap_.load();
            });
            let t2 = thread::spawn(move || {
                arcswap.store(Arc::new(12345));
            });

            t1.join().unwrap();
            t2.join().unwrap();

            assert!(matches!(*result.lock().unwrap(), 42 | 12345));
        }, 100);
    }

    #[test]
    #[should_panic]
    fn rcu_catches_races() {
        shuttle::check_dfs(|| {
            let arcswap: Arc<ArcSwapAny<Arc<u64>, ShuttleStrategy>> = Arc::new(ArcSwap::from_pointee(42));

            let arcswap_ = arcswap.clone();
            let t1 = thread::spawn(move || {
                arcswap_.store(Arc::new(43));
            });

            let mut did_run_once = false;
            let t2 = thread::spawn(move || {
                arcswap.rcu(|n| {
                    assert!(!did_run_once); // should panic
                    did_run_once = true;

                    **n + 1
                });
            });

            t1.join().unwrap();
            t2.join().unwrap(); // should panic
        }, Some(10000));
    }
}
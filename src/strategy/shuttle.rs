use shuttle::sync::RwLock;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::LockResult;

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
impl ShuttleStrategy {
    /// Creates a new instance of the ShuttleStrategy.
    pub fn new() -> Self {
        Self::default()
    }

    // RwLock API shims used by rw_lock.rs. These are `pub(crate)` as the use of RwLock internally
    // is not an exposed part of our API contract.

    pub(crate) fn read(&self) -> LockResult<shuttle::sync::RwLockReadGuard<'_, ()>> {
        self.inner.read()
    }

    pub(crate) fn write(&self) -> LockResult<shuttle::sync::RwLockWriteGuard<'_, ()>> {
        self.inner.write()
    }
}

// The actual implementation is provided in rw_lock.rs

#[cfg(test)]
mod test {
    use shuttle::sync::{Arc, Mutex};
    use shuttle::thread;

    use super::ShuttleStrategy;
    use crate::ArcSwapAny;

    type ArcSwap<T> = ArcSwapAny<Arc<T>, ShuttleStrategy>;

    #[test]
    fn arc_swap_works() {
        shuttle::check_random(
            || {
                let arcswap: Arc<ArcSwapAny<Arc<u64>, ShuttleStrategy>> =
                    Arc::new(ArcSwap::from_pointee(42));
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
            },
            100,
        );
    }

    #[test]
    #[should_panic]
    fn rcu_catches_races() {
        shuttle::check_dfs(
            || {
                let arcswap: Arc<ArcSwapAny<Arc<u64>, ShuttleStrategy>> =
                    Arc::new(ArcSwap::from_pointee(42));

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
            },
            Some(10000),
        );
    }
}

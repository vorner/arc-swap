#![feature(test)]

// FIXME: This still uses old terminology in the bench names and internal functions. It should be
// eventually renamed, eg. lease → load, etc.

extern crate test;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use arc_swap::{ArcSwap, ArcSwapOption, Cache};
use crossbeam_utils::thread;
use once_cell::sync::Lazy;
use test::Bencher;

const ITERS: usize = 100_000;

macro_rules! method {
    ($name:ident) => {
        mod $name {
            use super::{lock, noise, Bencher};

            #[bench]
            fn uncontended(b: &mut Bencher) {
                let _lock = lock();
                b.iter(super::$name);
            }

            #[bench]
            fn r1(b: &mut Bencher) {
                noise(b, 1, 0, 0, super::$name);
            }

            #[bench]
            fn r3(b: &mut Bencher) {
                noise(b, 3, 0, 0, super::$name);
            }

            #[bench]
            fn l1(b: &mut Bencher) {
                noise(b, 0, 1, 0, super::$name);
            }

            #[bench]
            fn l3(b: &mut Bencher) {
                noise(b, 0, 3, 0, super::$name);
            }

            #[bench]
            fn rw(b: &mut Bencher) {
                noise(b, 1, 0, 1, super::$name);
            }

            #[bench]
            fn lw(b: &mut Bencher) {
                noise(b, 0, 1, 1, super::$name);
            }

            #[bench]
            fn w2(b: &mut Bencher) {
                noise(b, 0, 0, 2, super::$name);
            }
        }
    };
}

macro_rules! noise {
    () => {
        use super::{
            test, thread, Arc, AtomicBool, Bencher, Lazy, Mutex, MutexGuard, Ordering, PoisonError,
            ITERS,
        };

        static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

        /// We want to prevent these tests from running concurrently, because they run
        /// multi-threaded.
        ///
        /// If it is run as benchmark, it is OK. But if it is run as a test, they run in multiple
        /// threads and some of them fight (especially the rwlock ones run for a really long time).
        fn lock() -> MutexGuard<'static, ()> {
            LOCK.lock().unwrap_or_else(PoisonError::into_inner)
        }

        fn noise<F: Fn()>(b: &mut Bencher, readers: usize, leasers: usize, writers: usize, f: F) {
            let _lock = lock();
            let flag = Arc::new(AtomicBool::new(true));
            thread::scope(|s| {
                for _ in 0..readers {
                    s.spawn(|_| {
                        while flag.load(Ordering::Relaxed) {
                            read();
                        }
                    });
                }
                for _ in 0..leasers {
                    s.spawn(|_| {
                        while flag.load(Ordering::Relaxed) {
                            lease();
                        }
                    });
                }
                for _ in 0..writers {
                    s.spawn(|_| {
                        while flag.load(Ordering::Relaxed) {
                            write();
                        }
                    });
                }
                b.iter(f);
                flag.store(false, Ordering::Relaxed);
            })
            .unwrap();
        }
    };
}

mod arc_swap_b {
    use super::ArcSwap;

    static A: Lazy<ArcSwap<usize>> = Lazy::new(|| ArcSwap::from_pointee(0));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(**A.load());
        }
    }

    // Leases kind of degrade in performance if there are multiple on the same thread.
    fn four_leases() {
        for _ in 0..ITERS {
            let l1 = A.load();
            let l2 = A.load();
            let l3 = A.load();
            let l4 = A.load();
            test::black_box((**l1, **l2, **l3, **l4));
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(A.load_full());
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(A.store(Arc::new(0)));
        }
    }

    noise!();

    method!(read);
    method!(write);
    method!(lease);
    method!(four_leases);
}

mod arc_swap_option {
    use super::ArcSwapOption;

    static A: Lazy<ArcSwapOption<usize>> = Lazy::new(|| ArcSwapOption::from(None));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(A.load().as_ref().map(|l| **l).unwrap_or(0));
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(A.load_full().map(|a| -> usize { *a }).unwrap_or(0));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(A.store(Some(Arc::new(0))));
        }
    }

    noise!();

    method!(read);
    method!(write);
    method!(lease);
}

mod arc_swap_cached {
    use super::{ArcSwap, Cache};

    static A: Lazy<ArcSwap<usize>> = Lazy::new(|| ArcSwap::from_pointee(0));

    fn read() {
        let mut cache = Cache::from(&A as &ArcSwap<usize>);
        for _ in 0..ITERS {
            test::black_box(Arc::clone(cache.load()));
        }
    }

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(**A.load());
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(A.store(Arc::new(0)));
        }
    }

    noise!();

    method!(read);
    method!(write);
}

mod mutex {
    static M: Lazy<Mutex<Arc<usize>>> = Lazy::new(|| Mutex::new(Arc::new(0)));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(**M.lock().unwrap());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(Arc::clone(&*M.lock().unwrap()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(*M.lock().unwrap() = Arc::new(42));
        }
    }

    noise!();

    method!(read);
    method!(write);
}

mod parking_mutex {
    use parking_lot::Mutex as ParkingMutex;

    static M: Lazy<ParkingMutex<Arc<usize>>> = Lazy::new(|| ParkingMutex::new(Arc::new(0)));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(**M.lock());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(Arc::clone(&*M.lock()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(*M.lock() = Arc::new(42));
        }
    }

    noise!();

    method!(read);
    method!(write);
}

mod rwlock {
    use std::sync::RwLock;

    static L: Lazy<RwLock<Arc<usize>>> = Lazy::new(|| RwLock::new(Arc::new(0)));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(**L.read().unwrap());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(Arc::clone(&*L.read().unwrap()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(*L.write().unwrap() = Arc::new(42));
        }
    }

    noise!();

    method!(read);
    method!(write);
}

mod parking_rwlock {
    use parking_lot::RwLock;

    static L: Lazy<RwLock<Arc<usize>>> = Lazy::new(|| RwLock::new(Arc::new(0)));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(**L.read());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(Arc::clone(&*L.read()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(*L.write() = Arc::new(42));
        }
    }

    noise!();

    method!(read);
    method!(write);
}

mod arc_cell {
    use crossbeam::atomic::ArcCell;

    static A: Lazy<ArcCell<usize>> = Lazy::new(|| ArcCell::new(Arc::new(0)));

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(A.get());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(A.get());
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(A.set(Arc::new(42)));
        }
    }

    noise!();

    method!(read);
    method!(write);
}

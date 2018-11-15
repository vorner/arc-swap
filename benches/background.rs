#![feature(test)]

extern crate arc_swap;
extern crate crossbeam;
extern crate crossbeam_utils;
#[macro_use]
extern crate lazy_static;
extern crate parking_lot;
extern crate test;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use arc_swap::{ArcSwap, ArcSwapOption, Guard, Lease};
use crossbeam_utils::thread;
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
                noise(b, 1, 0, 0, 0, super::$name);
            }

            #[bench]
            fn r3(b: &mut Bencher) {
                noise(b, 3, 0, 0, 0, super::$name);
            }

            #[bench]
            fn p1(b: &mut Bencher) {
                noise(b, 0, 1, 0, 0, super::$name);
            }

            #[bench]
            fn p3(b: &mut Bencher) {
                noise(b, 0, 3, 0, 0, super::$name);
            }

            #[bench]
            fn l1(b: &mut Bencher) {
                noise(b, 0, 0, 1, 0, super::$name);
            }

            #[bench]
            fn l3(b: &mut Bencher) {
                noise(b, 0, 0, 3, 0, super::$name);
            }

            #[bench]
            fn rw(b: &mut Bencher) {
                noise(b, 1, 0, 0, 1, super::$name);
            }

            #[bench]
            fn pw(b: &mut Bencher) {
                noise(b, 0, 1, 0, 1, super::$name);
            }

            #[bench]
            fn lw(b: &mut Bencher) {
                noise(b, 0, 0, 1, 1, super::$name);
            }
        }
    };
}

macro_rules! noise {
    () => {
        use super::{
            test, thread, Arc, AtomicBool, Bencher, Mutex, MutexGuard, Ordering, PoisonError, ITERS,
        };

        lazy_static! {
            static ref LOCK: Mutex<()> = Mutex::new(());
        }

        /// We want to prevent these tests from running concurrently, because they run multi-threaded.
        ///
        /// If it is run as benchmark, it is OK. But if it is run as a test, they run in multiple threads
        /// and some of them fight (especially the rwlock ones run for a really long time).
        fn lock() -> MutexGuard<'static, ()> {
            LOCK.lock().unwrap_or_else(PoisonError::into_inner)
        }

        fn noise<F: Fn()>(
            b: &mut Bencher,
            readers: usize,
            peekers: usize,
            leasers: usize,
            writers: usize,
            f: F,
        ) {
            let _lock = lock();
            let flag = Arc::new(AtomicBool::new(true));
            thread::scope(|s| {
                for _ in 0..readers {
                    s.spawn(|| {
                        while flag.load(Ordering::Relaxed) {
                            read();
                        }
                    });
                }
                for _ in 0..peekers {
                    s.spawn(|| {
                        while flag.load(Ordering::Relaxed) {
                            peek();
                        }
                    });
                }
                for _ in 0..leasers {
                    s.spawn(|| {
                        while flag.load(Ordering::Relaxed) {
                            lease();
                        }
                    });
                }
                for _ in 0..writers {
                    s.spawn(|| {
                        while flag.load(Ordering::Relaxed) {
                            write();
                        }
                    });
                }
                b.iter(f);
                flag.store(false, Ordering::Relaxed);
            });
        }
    };
}

mod arc_swap_b {
    use super::ArcSwap;

    lazy_static! {
        static ref A: ArcSwap<usize> = ArcSwap::from(Arc::new(0));
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(*A.peek());
        }
    }

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(*A.lease());
        }
    }

    // Leases kind of degrade in performance if there are multiple on the same thread.
    fn four_leases() {
        for _ in 0..ITERS {
            let l1 = A.lease();
            let l2 = A.lease();
            let l3 = A.lease();
            let l4 = A.lease();
            test::black_box((*l1, *l2, *l3, *l4));
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(A.load());
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(A.store(Arc::new(0)));
        }
    }

    noise!();

    method!(peek);
    method!(read);
    method!(write);
    method!(lease);
    method!(four_leases);
}

mod arc_swap_option {
    use super::{ArcSwapOption, Guard, Lease};

    lazy_static! {
        static ref A: ArcSwapOption<usize> = ArcSwapOption::from(None);
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(*Guard::get_ref(&A.peek()).unwrap_or(&0));
        }
    }

    fn lease() {
        for _ in 0..ITERS {
            test::black_box(*Lease::get_ref(&A.lease()).unwrap_or(&0));
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(A.load().map(|a| -> usize { *a }).unwrap_or(0));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            test::black_box(A.store(Some(Arc::new(0))));
        }
    }

    noise!();

    method!(peek);
    method!(read);
    method!(write);
    method!(lease);
}

mod mutex {
    lazy_static! {
        static ref M: Mutex<Arc<usize>> = Mutex::new(Arc::new(0));
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(**M.lock().unwrap());
        }
    }

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

    method!(peek);
    method!(read);
    method!(write);
}

mod parking_mutex {
    use parking_lot::Mutex as ParkingMutex;

    lazy_static! {
        static ref M: ParkingMutex<Arc<usize>> = ParkingMutex::new(Arc::new(0));
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(**M.lock());
        }
    }

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

    method!(peek);
    method!(read);
    method!(write);
}

mod rwlock {
    use std::sync::RwLock;

    lazy_static! {
        static ref L: RwLock<Arc<usize>> = RwLock::new(Arc::new(0));
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(**L.read().unwrap());
        }
    }

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

    method!(peek);
    method!(read);
    method!(write);
}

mod parking_rwlock {
    use parking_lot::RwLock;

    lazy_static! {
        static ref L: RwLock<Arc<usize>> = RwLock::new(Arc::new(0));
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(**L.read());
        }
    }

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

    method!(peek);
    method!(read);
    method!(write);
}

mod arc_cell {
    use crossbeam::atomic::ArcCell;

    lazy_static! {
        static ref A: ArcCell<usize> = ArcCell::new(Arc::new(0));
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(*A.get());
        }
    }

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

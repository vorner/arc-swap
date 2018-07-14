#![feature(test)]

extern crate arc_swap;
extern crate crossbeam_utils;
#[macro_use]
extern crate lazy_static;
extern crate test;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use arc_swap::{ArcSwap, ArcSwapOption, Guard};
use crossbeam_utils::scoped;
use test::Bencher;

const ITERS: usize = 100_000;

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
            fn p1(b: &mut Bencher) {
                noise(b, 0, 1, 0, super::$name);
            }

            #[bench]
            fn p3(b: &mut Bencher) {
                noise(b, 0, 3, 0, super::$name);
            }

            #[bench]
            fn rw(b: &mut Bencher) {
                noise(b, 1, 0, 1, super::$name);
            }

            #[bench]
            fn pw(b: &mut Bencher) {
                noise(b, 0, 1, 1, super::$name);
            }
        }
    };
}

macro_rules! noise {
    () => {
        use super::{lock, scoped, test, Arc, AtomicBool, Bencher, Ordering, ITERS};

        fn noise<F: Fn()>(b: &mut Bencher, readers: usize, peekers: usize, writers: usize, f: F) {
            let _lock = lock();
            let flag = Arc::new(AtomicBool::new(true));
            scoped::scope(|s| {
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
}

mod arc_swap_option {
    use super::{ArcSwapOption, Guard};

    lazy_static! {
        static ref A: ArcSwapOption<usize> = ArcSwapOption::from(None);
    }

    fn peek() {
        for _ in 0..ITERS {
            test::black_box(Guard::get_ref(&A.peek()));
        }
    }

    fn read() {
        for _ in 0..ITERS {
            test::black_box(A.load());
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
}

mod mutex {
    use std::sync::Mutex;

    lazy_static! {
        static ref M: Mutex<Arc<usize>> = Mutex::new(Arc::new(0));
    }

    fn peek() {
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

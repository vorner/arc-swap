#![feature(test)]

extern crate arc_swap;
extern crate crossbeam_utils;
#[macro_use]
extern crate lazy_static;
extern crate test;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use arc_swap::{ArcSwap, ArcSwapOption, Guard};
use crossbeam_utils::scoped;
use test::Bencher;

const ITERS: usize = 100_000;

macro_rules! method {
    ($name:ident) => {
        mod $name {
            use super::{noise, Bencher};

            #[bench]
            fn uncontended(b: &mut Bencher) {
                b.iter(super::$name);
            }

            #[bench]
            fn r1(b: &mut Bencher) {
                noise(b, 1, 0, super::$name);
            }

            #[bench]
            fn r3(b: &mut Bencher) {
                noise(b, 3, 0, super::$name);
            }

            #[bench]
            fn rw(b: &mut Bencher) {
                noise(b, 1, 1, super::$name);
            }
        }
    };
}

macro_rules! noise {
    () => {
        use super::{scoped, test, Arc, AtomicBool, Bencher, Ordering, ITERS};

        fn noise<F: Fn()>(b: &mut Bencher, readers: usize, writers: usize, f: F) {
            let flag = Arc::new(AtomicBool::new(true));
            scoped::scope(|s| {
                for _ in 0..readers {
                    s.spawn(|| {
                        while flag.load(Ordering::Relaxed) {
                            read();
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

#![feature(test)]

extern crate crossbeam_utils;
extern crate test;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use test::Bencher;

const BATCH: usize = 1_000_000;
const PARAL: usize = 4;

#[bench]
fn non_atomic(b: &mut Bencher) {
    b.iter(|| {
        let mut x = 0;
        for i in 0..BATCH {
            let x = test::black_box(&mut x);
            *x += i;
        }
        test::black_box(x);
    });
}

#[bench]
fn non_atomic_ub_mt(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(0usize);
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = unsafe {
                    // This is unsafe & UB, because we are creating multiple mutable aliases to the
                    // same thing. But we just want to measure how fast that UB is.
                    let x: &usize = &*x;
                    let x = x as *const usize as *mut usize;
                    x.as_mut().unwrap()
                };
                let x = test::black_box(x);
                *x += i;
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

#[bench]
fn load_store_relaxed(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.store(x.load(Ordering::Relaxed) + i, Ordering::Relaxed);
        }
        test::black_box(x.load(Ordering::Relaxed));
    });
}

#[bench]
fn load_swap_relaxed(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            test::black_box(x.swap(x.load(Ordering::Relaxed) + i, Ordering::Relaxed));
        }
        test::black_box(x.load(Ordering::Relaxed));
    });
}

#[bench]
fn load_store_acq_rel(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.store(x.load(Ordering::Acquire) + i, Ordering::Release);
        }
        test::black_box(x.load(Ordering::Acquire));
    });
}

#[bench]
fn load_store_mt_relaxed_broken(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(AtomicUsize::new(0));
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = test::black_box(&x);
                x.store(x.load(Ordering::Relaxed) + i, Ordering::Relaxed);
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

#[bench]
fn load_store_mt_acq_rel_broken(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(AtomicUsize::new(0));
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = test::black_box(&x);
                x.store(x.load(Ordering::Acquire) + i, Ordering::Release);
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

#[bench]
fn load_store_mt_cst_broken(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(AtomicUsize::new(0));
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = test::black_box(&x);
                x.store(x.load(Ordering::SeqCst) + i, Ordering::SeqCst);
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

#[bench]
fn load_swap_cst(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            test::black_box(x.swap(x.load(Ordering::SeqCst) + i, Ordering::SeqCst));
        }
        test::black_box(x.load(Ordering::SeqCst));
    });
}

#[bench]
fn load_store_cst(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.store(x.load(Ordering::SeqCst) + i, Ordering::SeqCst);
        }
        test::black_box(x.load(Ordering::SeqCst));
    });
}

#[bench]
fn fetch_add_relaxed(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.fetch_add(i, Ordering::Relaxed);
        }
        test::black_box(x.load(Ordering::Relaxed));
    });
}

#[bench]
fn fetch_add_mt_relaxed(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(AtomicUsize::new(0));
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = test::black_box(&x);
                x.fetch_add(i, Ordering::Relaxed);
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

#[bench]
fn fetch_add_cst(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.fetch_add(i, Ordering::SeqCst);
        }
        test::black_box(x.load(Ordering::SeqCst));
    });
}

#[bench]
fn fetch_add_mt_cst(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(AtomicUsize::new(0));
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = test::black_box(&x);
                x.fetch_add(i, Ordering::SeqCst);
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

#[bench]
fn cas_relaxed(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            let orig = x.load(Ordering::Relaxed);
            x.compare_and_swap(orig, orig + i, Ordering::Relaxed);
        }
        test::black_box(x.load(Ordering::Relaxed));
    });
}

#[bench]
fn cas_seq(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            let orig = x.load(Ordering::SeqCst);
            x.compare_and_swap(orig, orig + i, Ordering::SeqCst);
        }
        test::black_box(x.load(Ordering::SeqCst));
    });
}

#[bench]
fn cas_mt_cst(b: &mut Bencher) {
    let bar = Arc::new(Barrier::new(PARAL + 1));
    let x = Arc::new(AtomicUsize::new(0));
    for _ in 0..PARAL {
        let bar = Arc::clone(&bar);
        let x = Arc::clone(&x);
        thread::spawn(move || loop {
            bar.wait();
            for i in 0..BATCH {
                let x = test::black_box(&x);
                let orig = x.load(Ordering::SeqCst);
                x.compare_and_swap(orig, orig + i, Ordering::SeqCst);
            }
        });
    }
    bar.wait();
    bar.wait();
    b.iter(|| {
        bar.wait();
    });
}

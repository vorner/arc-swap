#![feature(test)]

extern crate test;

use std::sync::atomic::{AtomicUsize, Ordering};

use test::Bencher;

const BATCH: usize = 1000;

#[bench]
fn non_atomic(b: &mut Bencher) {
    b.iter(|| {
        let mut x = 0;
        for i in 0..BATCH {
            let x = test::black_box(&mut x);
            *x += test::black_box(i);
        }
        test::black_box(x);
    });
}

// TODO: These two give somehow suspect results, about as fast as non-atomic. Any chance something
// optimises them away? :-(
#[bench]
fn load_store_relaxed(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.store(x.load(Ordering::Relaxed) + test::black_box(i), Ordering::Relaxed);
        }
        test::black_box(x.load(Ordering::Relaxed));
    });
}

#[bench]
fn load_store_cst(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.store(x.load(Ordering::SeqCst) + test::black_box(i), Ordering::SeqCst);
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
            x.fetch_add(test::black_box(i), Ordering::Relaxed);
        }
        test::black_box(x.load(Ordering::Relaxed));
    });
}

#[bench]
fn fetch_add_cst(b: &mut Bencher) {
    b.iter(|| {
        let x = AtomicUsize::new(0);
        for i in 0..BATCH {
            let x = test::black_box(&x);
            x.fetch_add(test::black_box(i), Ordering::SeqCst);
        }
        test::black_box(x.load(Ordering::SeqCst));
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

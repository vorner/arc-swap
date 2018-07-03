#![feature(test)]

extern crate arc_swap;
extern crate crossbeam_utils;
#[macro_use]
extern crate lazy_static;
extern crate test;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_utils::scoped;
use test::Bencher;

const ITERS: usize = 1_000_000;

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

#[bench]
fn peek_uncontended(b: &mut Bencher) {
    b.iter(peek);
}

#[bench]
fn peek_r1(b: &mut Bencher) {
    noise(b, 1, 0, peek);
}

#[bench]
fn peek_r4(b: &mut Bencher) {
    noise(b, 4, 0, peek);
}

#[bench]
fn peek_rw(b: &mut Bencher) {
    noise(b, 2, 1, peek);
}

#[bench]
fn load_uncontended(b: &mut Bencher) {
    b.iter(read);
}

#[bench]
fn load_r1(b: &mut Bencher) {
    noise(b, 1, 0, read);
}

#[bench]
fn load_r4(b: &mut Bencher) {
    noise(b, 4, 0, read);
}

#[bench]
fn load_rw(b: &mut Bencher) {
    noise(b, 2, 1, read);
}

#[bench]
fn store_uncontended(b: &mut Bencher) {
    b.iter(write);
}

#[bench]
fn store_r1(b: &mut Bencher) {
    noise(b, 1, 0, write);
}

#[bench]
fn store_r4(b: &mut Bencher) {
    noise(b, 4, 0, write);
}

#[bench]
fn store_rw(b: &mut Bencher) {
    noise(b, 2, 1, write);
}

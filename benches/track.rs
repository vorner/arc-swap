//! Benchmarks to track basic performance across changes.
//!
//! Slightly based on the <background.rs> benchmarks, but simplified and stripped down to run
//! reasonably fast.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use arc_swap::ArcSwap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam_utils::thread;

/// Execute a group of measurements
///
/// It expects any kind of „environment“ is already in place for it.
fn batch(c: &mut Criterion, name: &str, shared_number: &ArcSwap<usize>) {
    let mut g = c.benchmark_group(name);

    g.bench_function("load", |b| {
        b.iter(|| {
            black_box(shared_number.load());
        })
    });
    g.bench_function("load_full", |b| {
        b.iter(|| {
            black_box(shared_number.load_full());
        })
    });
    // Here we simulate running out of the debt slots scenario
    const MANY: usize = 32;
    let mut guards = Vec::with_capacity(MANY);
    g.bench_function("load_many", |b| {
        b.iter(|| {
            for _ in 0..MANY {
                guards.push(black_box(shared_number.load()));
            }
            guards.clear();
        })
    });
    g.bench_function("store", |b| {
        b.iter(|| {
            black_box(shared_number.store(Arc::new(42)));
        })
    });

    g.finish();
}

fn with_background<F: Fn(&ArcSwap<usize>) + Sync>(
    c: &mut Criterion,
    name: &str,
    cnt: usize,
    noise: F,
) {
    let stop = AtomicBool::new(false);
    let shared_number = ArcSwap::from_pointee(42);
    thread::scope(|s| {
        // Start some background noise threads, to contend the arc swap.
        for _ in 0..cnt {
            s.spawn(|_| {
                while !stop.load(Ordering::Relaxed) {
                    noise(&shared_number);
                }
            });
        }

        // Perform the benchmarks
        batch(c, name, &shared_number);

        // Ask the threads to terminate, so they don't disturb any other banchmarks
        stop.store(true, Ordering::Relaxed);
    })
    .unwrap();
}

fn benchmark(c: &mut Criterion) {
    batch(c, "uncontended", &ArcSwap::from_pointee(42));
    with_background(c, "concurrent_loads", 2, |s| {
        black_box(s.load());
    });
    with_background(c, "concurrent_store", 1, |s| {
        black_box(s.store(Arc::new(42)));
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

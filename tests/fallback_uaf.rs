use std::sync::Arc;
use std::thread;

#[allow(deprecated)]
use arc_swap::strategy::test_strategies::FillFastSlots;
use arc_swap::ArcSwapAny;

type FallbackSwap<T> = ArcSwapAny<Arc<T>, FillFastSlots>;

/// Triggers UAF under miri on aarch64 (e.g. seed=39) when storage.load uses
/// Acquire instead of SeqCst in the fallback path.
#[test]
fn fallback_rcu() {
    const ITERS: usize = 20;
    const THREADS: usize = 4;
    let shared = FallbackSwap::<usize>::new(Arc::new(0));
    thread::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|| {
                for _ in 0..ITERS {
                    shared.rcu(|old| **old + 1);
                }
            });
        }
    });
    assert_eq!(THREADS * ITERS, **shared.load());
}

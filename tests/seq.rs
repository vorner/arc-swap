use arc_swap::{ArcSwapAny, ArcSwap, ArcSwapOption, AsRaw, Guard, DefaultStrategy};
use std::sync::atomic::Ordering;
use std::ptr;

#[cfg(feature = "internal-test-traps")]
use arc_swap::TrapGuard;
#[cfg(not(feature = "internal-test-traps"))]
struct TrapGuard;
#[cfg(not(feature = "internal-test-traps"))]
impl TrapGuard {
    fn new() -> Self { TrapGuard }
}

/// Comparison of two pointer-like things.
// A and B are likely to *be* references, or thin wrappers around that. Calling that with extra
// reference is just annoying.
#[allow(clippy::needless_pass_by_value)]
fn ptr_eq<Base, A, B>(a: A, b: B) -> bool
where
    A: AsRaw<Base>,
    B: AsRaw<Base>,
{
    let a = a.as_raw();
    let b = b.as_raw();
    ptr::eq(a, b)
}

macro_rules! t {
    ($name: ident, $strategy: ty) => {
        #[cfg(test)]
        mod $name {
            use std::sync::Arc;
            use std::borrow::ToOwned;
            use std::string::String;
            use std::vec::Vec;
            use std::sync::atomic::{self, AtomicUsize};


            use adaptive_barrier::{Barrier, PanicMode};
            use crossbeam_utils::thread;

            use super::*;

            const ITERATIONS: usize = 10;

            #[allow(deprecated)] // We use "deprecated" testing strategies in here.
            type As<T> = ArcSwapAny<Arc<T>, $strategy>;
            #[allow(deprecated)] // We use "deprecated" testing strategies in here.
            type Aso<T> = ArcSwapAny<Option<Arc<T>>, $strategy>;

            /// Similar to the one in doc tests of the lib, but more times and more intensive (we
            /// want to torture it a bit).
            #[test]
            #[cfg_attr(miri, ignore)] // Takes like 1 or 2 infinities to run under miri
            fn publish() {
                let _t = TrapGuard::new();
                const READERS: usize = 2;
                for _ in 0..ITERATIONS {
                    let config = As::<String>::default();
                    let ended = AtomicUsize::new(0);
                    thread::scope(|scope| {
                        for _ in 0..READERS {
                            scope.spawn(|_| loop {
                                let cfg = config.load_full();
                                if !cfg.is_empty() {
                                    assert_eq!(*cfg, "New configuration");
                                    ended.fetch_add(1, Ordering::Relaxed);
                                    return;
                                }
                                atomic::spin_loop_hint();
                            });
                        }
                        scope.spawn(|_| {
                            let new_conf = Arc::new("New configuration".to_owned());
                            config.store(new_conf);
                        });
                    })
                    .unwrap();
                    assert_eq!(READERS, ended.load(Ordering::Relaxed));
                    let arc = config.load_full();
                    assert_eq!(2, Arc::strong_count(&arc));
                    assert_eq!(0, Arc::weak_count(&arc));
                }
            }

            /// Similar to the doc tests of ArcSwap, but happens more times.
            #[test]
            fn swap_load() {
                let _t = TrapGuard::new();
                for _ in 0..100 {
                    let arc = Arc::new(42);
                    let arc_swap = As::from(Arc::clone(&arc));
                    assert_eq!(42, **arc_swap.load());
                    // It can be read multiple times
                    assert_eq!(42, **arc_swap.load());

                    // Put a new one in there
                    let new_arc = Arc::new(0);
                    assert_eq!(42, *arc_swap.swap(Arc::clone(&new_arc)));
                    assert_eq!(0, **arc_swap.load());
                    // One loaded here, one in the arc_swap, one in new_arc
                    let loaded = arc_swap.load_full();
                    assert_eq!(3, Arc::strong_count(&loaded));
                    assert_eq!(0, Arc::weak_count(&loaded));
                    // The original got released from the arc_swap
                    assert_eq!(1, Arc::strong_count(&arc));
                    assert_eq!(0, Arc::weak_count(&arc));
                }
            }

            /// Two different writers publish two series of values. The readers check that it is
            /// always increasing in each serie.
            ///
            /// For performance, we try to reuse the threads here.
            #[test]
            fn multi_writers() {
                let _t = TrapGuard::new();
                let first_value = Arc::new((0, 0));
                let shared = As::from(Arc::clone(&first_value));
                const WRITER_CNT: usize = 5;
                const READER_CNT: usize = 8;
                #[cfg(miri)]
                const ITERATIONS: usize = 5;
                #[cfg(not(miri))]
                const ITERATIONS: usize = 100;
                const SEQ: usize = 50;
                let barrier = Barrier::new(PanicMode::Poison);
                thread::scope(|scope| {
                    for w in 0..WRITER_CNT {
                        // We need to move w into the closure. But we want to just reference the
                        // other things.
                        let mut barrier = barrier.clone();
                        let shared = &shared;
                        let first_value = &first_value;
                        scope.spawn(move |_| {
                            for _ in 0..ITERATIONS {
                                barrier.wait();
                                shared.store(Arc::clone(&first_value));
                                barrier.wait();
                                for i in 0..SEQ {
                                    shared.store(Arc::new((w, i + 1)));
                                }
                            }
                        });
                    }
                    for _ in 0..READER_CNT {
                        let mut barrier = barrier.clone();
                        let shared = &shared;
                        let first_value = &first_value;
                        scope.spawn(move |_| {
                            for _ in 0..ITERATIONS {
                                barrier.wait();
                                barrier.wait();
                                let mut previous = [0; WRITER_CNT];
                                let mut last = Arc::clone(&first_value);
                                loop {
                                    let cur = shared.load();
                                    if Arc::ptr_eq(&last, &cur) {
                                        atomic::spin_loop_hint();
                                        continue;
                                    }
                                    let (w, s) = **cur;
                                    assert!(previous[w] < s, "{:?} vs {:?}", previous, cur);
                                    previous[w] = s;
                                    last = Guard::into_inner(cur);
                                    if s == SEQ {
                                        break;
                                    }
                                }
                            }
                        });
                    }

                    drop(barrier);
                })
                .unwrap();
            }

            #[test]
            fn load_null() {
                let _t = TrapGuard::new();
                let shared = Aso::<usize>::default();
                let guard = shared.load();
                assert!(guard.is_none());
                shared.store(Some(Arc::new(42)));
                assert_eq!(42, **shared.load().as_ref().unwrap());
            }

            #[test]
            fn from_into() {
                let _t = TrapGuard::new();
                let a = Arc::new(42);
                let shared = As::new(a);
                let guard = shared.load();
                let a = shared.into_inner();
                assert_eq!(42, *a);
                assert_eq!(2, Arc::strong_count(&a));
                drop(guard);
                assert_eq!(1, Arc::strong_count(&a));
            }

            // Note on the Relaxed order here. This should be enough, because there's that
            // barrier.wait in between that should do the synchronization of happens-before for us.
            // And using SeqCst would probably not help either, as there's nothing else with SeqCst
            // here in this test to relate it to.
            #[derive(Default)]
            struct ReportDrop(Arc<AtomicUsize>);
            impl Drop for ReportDrop {
                fn drop(&mut self) {
                    self.0.fetch_add(1, Ordering::Relaxed);
                }
            }

            /// Interaction of two threads about a guard and dropping it.
            ///
            /// We make sure everything works in timely manner (eg. dropping of stuff) even if multiple
            /// threads interact.
            ///
            /// The idea is:
            /// * Thread 1 loads a value.
            /// * Thread 2 replaces the shared value. The original value is not destroyed.
            /// * Thread 1 drops the guard. The value is destroyed and this is observable in both threads.
            #[test]
            fn guard_drop_in_thread() {
                let _t = TrapGuard::new();
                for _ in 0..ITERATIONS {
                    let cnt = Arc::new(AtomicUsize::new(0));

                    let shared = As::from_pointee(ReportDrop(cnt.clone()));
                    assert_eq!(cnt.load(Ordering::Relaxed), 0, "Dropped prematurely");
                    // We need the threads to wait for each other at places.
                    let sync = Barrier::new(PanicMode::Poison);

                    thread::scope(|scope| {
                        scope.spawn({
                            let sync = sync.clone();
                            |_| {
                                let mut sync = sync; // Move into the closure
                                let guard = shared.load();
                                sync.wait();
                                // Thread 2 replaces the shared value. We wait for it to confirm.
                                sync.wait();
                                drop(guard);
                                assert_eq!(cnt.load(Ordering::Relaxed), 1, "Value not dropped");
                                // Let thread 2 know we already dropped it.
                                sync.wait();
                            }
                        });

                        scope.spawn(|_| {
                            let mut sync = sync;
                            // Thread 1 loads, we wait for that
                            sync.wait();
                            shared.store(Default::default());
                            assert_eq!(
                                cnt.load(Ordering::Relaxed),
                                0,
                                "Dropped while still in use"
                            );
                            // Let thread 2 know we replaced it
                            sync.wait();
                            // Thread 1 drops its guard. We wait for it to confirm.
                            sync.wait();
                            assert_eq!(cnt.load(Ordering::Relaxed), 1, "Value not dropped");
                        });
                    })
                    .unwrap();
                }
            }

            /// Check dropping a lease in a different thread than it was created doesn't cause any
            /// problems.
            #[test]
            fn guard_drop_in_another_thread() {
                let _t = TrapGuard::new();
                for _ in 0..ITERATIONS {
                    let cnt = Arc::new(AtomicUsize::new(0));
                    let shared = As::from_pointee(ReportDrop(cnt.clone()));
                    assert_eq!(cnt.load(Ordering::Relaxed), 0, "Dropped prematurely");
                    let guard = shared.load();

                    drop(shared);
                    assert_eq!(cnt.load(Ordering::Relaxed), 0, "Dropped prematurely");

                    thread::scope(|scope| {
                        scope.spawn(|_| {
                            drop(guard);
                        });
                    })
                    .unwrap();

                    assert_eq!(cnt.load(Ordering::Relaxed), 1, "Not dropped");
                }
            }

            #[test]
            fn load_option() {
                let _t = TrapGuard::new();
                let shared = Aso::from_pointee(42);
                // The type here is not needed in real code, it's just addition test the type matches.
                let opt: Option<_> = Guard::into_inner(shared.load());
                assert_eq!(42, *opt.unwrap());

                shared.store(None);
                assert!(shared.load().is_none());
            }

            // Check stuff can get formatted
            #[test]
            fn debug_impl() {
                let _t = TrapGuard::new();
                let shared = As::from_pointee(42);
                assert_eq!("ArcSwapAny(42)", &format!("{:?}", shared));
                assert_eq!("42", &format!("{:?}", shared.load()));
            }

            #[test]
            fn display_impl() {
                let _t = TrapGuard::new();
                let shared = As::from_pointee(42);
                assert_eq!("42", &format!("{}", shared));
                assert_eq!("42", &format!("{}", shared.load()));
            }

            // The following "tests" are not run, only compiled. They check that things that should be
            // Send/Sync actually are.
            fn _check_stuff_is_send_sync() {
                let shared = As::from_pointee(42);
                let moved = As::from_pointee(42);
                let shared_ref = &shared;
                let lease = shared.load();
                let lease_ref = &lease;
                let lease = shared.load();
                thread::scope(|s| {
                    s.spawn(move |_| {
                        let _ = lease;
                        let _ = lease_ref;
                        let _ = shared_ref;
                        let _ = moved;
                    });
                })
                .unwrap();
            }

            /// We have a callback in RCU. Check what happens if we access the value from within.
            #[test]
            fn recursive() {
                let _t = TrapGuard::new();
                let shared = ArcSwap::from(Arc::new(0));

                shared.rcu(|i| {
                    if **i < 10 {
                        shared.rcu(|i| **i + 1);
                    }
                    **i
                });
                assert_eq!(10, **shared.load());
                assert_eq!(2, Arc::strong_count(&shared.load_full()));
            }

            /// A panic from within the rcu callback should not change anything.
            #[test]
            #[cfg(not(feature = "experimental-thread-local"))]
            fn rcu_panic() {
                let _t = TrapGuard::new();
                use std::panic;
                let shared = ArcSwap::from(Arc::new(0));
                assert!(panic::catch_unwind(|| shared.rcu(|_| -> usize { panic!() })).is_err());
                assert_eq!(1, Arc::strong_count(&shared.swap(Arc::new(42))));
            }

            /// Handling null/none values
            #[test]
            fn nulls() {
                let _t = TrapGuard::new();
                let shared = ArcSwapOption::from(Some(Arc::new(0)));
                let orig = shared.swap(None);
                assert_eq!(1, Arc::strong_count(&orig.unwrap()));
                let null = shared.load();
                assert!(null.is_none());
                let a = Arc::new(42);
                let orig = shared.compare_and_swap(ptr::null(), Some(Arc::clone(&a)));
                assert!(orig.is_none());
                assert_eq!(2, Arc::strong_count(&a));
                let orig = Guard::into_inner(shared.compare_and_swap(&None::<Arc<_>>, None));
                assert_eq!(3, Arc::strong_count(&a));
                assert!(ptr_eq(&a, &orig));
            }

            #[test]
            /// Multiple RCUs interacting.
            fn rcu() {
                let _t = TrapGuard::new();
                const ITERATIONS: usize = 500;
                const THREADS: usize = 10;
                let shared = ArcSwap::from(Arc::new(0));
                thread::scope(|scope| {
                    for _ in 0..THREADS {
                        scope.spawn(|_| {
                            for _ in 0..ITERATIONS {
                                shared.rcu(|old| **old + 1);
                            }
                        });
                    }
                })
                .unwrap();
                assert_eq!(THREADS * ITERATIONS, **shared.load());
            }

            #[test]
            /// Make sure the reference count and compare_and_swap works as expected.
            fn cas_ref_cnt() {
                let _t = TrapGuard::new();
                #[cfg(miri)]
                const ITERATIONS: usize = 10;
                #[cfg(not(miri))]
                const ITERATIONS: usize = 50;
                let shared = ArcSwap::from(Arc::new(0));
                for i in 0..ITERATIONS {
                    let orig = shared.load_full();
                    assert_eq!(i, *orig);
                    if i % 2 == 1 {
                        // One for orig, one for shared
                        assert_eq!(2, Arc::strong_count(&orig));
                    }
                    let n1 = Arc::new(i + 1);
                    // Fill up the slots sometimes
                    let fillup = || {
                        if i % 2 == 0 {
                            Some((0..ITERATIONS).map(|_| shared.load()).collect::<Vec<_>>())
                        } else {
                            None
                        }
                    };
                    let guards = fillup();
                    // Success
                    let prev = shared.compare_and_swap(&orig, Arc::clone(&n1));
                    assert!(ptr_eq(&orig, &prev));
                    drop(guards);
                    // One for orig, one for prev
                    assert_eq!(2, Arc::strong_count(&orig));
                    // One for n1, one for shared
                    assert_eq!(2, Arc::strong_count(&n1));
                    assert_eq!(i + 1, **shared.load());
                    let n2 = Arc::new(i);
                    drop(prev);
                    let guards = fillup();
                    // Failure
                    let prev = Guard::into_inner(shared.compare_and_swap(&orig, Arc::clone(&n2)));
                    drop(guards);
                    assert!(ptr_eq(&n1, &prev));
                    // One for orig
                    assert_eq!(1, Arc::strong_count(&orig));
                    // One for n1, one for shared, one for prev
                    assert_eq!(3, Arc::strong_count(&n1));
                    // n2 didn't get increased
                    assert_eq!(1, Arc::strong_count(&n2));
                    assert_eq!(i + 1, **shared.load());
                }

                let a = shared.load_full();
                // One inside shared, one for a
                assert_eq!(2, Arc::strong_count(&a));
                drop(shared);
                // Only a now
                assert_eq!(1, Arc::strong_count(&a));
            }
        }
    };
}

t!(tests_default, DefaultStrategy);
#[cfg(all(feature = "internal-test-strategies", test))]
#[allow(deprecated)]
mod internal_strategies {
    use super::*;
    t!(
        tests_full_slots,
        arc_swap::strategy::test_strategies::FillFastSlots
    );
}

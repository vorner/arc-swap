//! Stress-tests
//!
//! The tests in here try to torture the implementation with multiple threads, in an attempt to
//! discover any possible race condition.

extern crate arc_swap;
extern crate crossbeam_utils;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate num_cpus;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex, MutexGuard, PoisonError};

use arc_swap::gen_lock::{Global, LockStorage, PrivateSharded, PrivateUnsharded, Shard};
use arc_swap::{ArcSwapAny, ArcSwapOption, Lease};
use crossbeam_utils::thread;
use itertools::Itertools;

lazy_static! {
    static ref LOCK: Mutex<()> = Mutex::new(());
}

/// We want to prevent these tests from running concurrently, because they run multi-threaded.
fn lock() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A test that repeatedly builds a linked list concurrently with multiple threads.
///
/// The idea here is to stress-test the RCU implementation and see that no items get lost and that
/// the ref counts are correct afterwards.
fn storm_link_list<S: LockStorage + Send + Sync>(node_cnt: usize, iters: usize) {
    struct LLNode {
        next: ArcSwapOption<LLNode>,
        num: usize,
        owner: usize,
    }

    let _lock = lock();
    let head = ArcSwapAny::<_, S>::from(None::<Arc<LLNode>>);
    let cpus = num_cpus::get();
    // FIXME: If one thread fails, but others don't, it'll deadlock.
    let bar = Barrier::new(cpus);
    thread::scope(|scope| {
        for thread in 0..cpus {
            // We want to borrow these, but that kind-of conflicts with the move closure mode
            let bar = &bar;
            let head = &head;
            scope.spawn(move || {
                let nodes = (0..node_cnt)
                    .map(|i| LLNode {
                        next: ArcSwapAny::from(None),
                        num: i,
                        owner: thread,
                    })
                    .map(Arc::new)
                    .collect::<Vec<_>>();
                for iter in 0..iters {
                    bar.wait(); // Start synchronously
                    for n in nodes.iter().rev() {
                        head.rcu(|head| {
                            n.next.store(Lease::upgrade(head)); // Cloning the optional Arc
                            Some(Arc::clone(n))
                        });
                    }
                    // And do the checks once everyone finishes
                    bar.wait();
                    // First, check that all our numbers are increasing by one and all are present
                    let mut node = head.lease();
                    let mut expecting = 0;
                    while Lease::get_ref(&node).is_some() {
                        // A bit of gymnastics, we don't have NLL yet and we need to persuade the
                        // borrow checker this is safe.
                        let next = {
                            let inner = Lease::get_ref(&node).unwrap();
                            if inner.owner == thread {
                                assert_eq!(expecting, inner.num);
                                expecting += 1;
                            }
                            inner.next.lease()
                        };
                        node = next;
                    }
                    assert_eq!(node_cnt, expecting);
                    // We don't want to count the ref-counts while someone still plays around with
                    // them and loading.
                    bar.wait();
                    // Now that we've checked we have everything, check that all the nodes have ref
                    // count 2 ‒ once in the vector, once in the linked list.
                    for n in &nodes {
                        assert_eq!(
                            2,
                            Arc::strong_count(n),
                            "Wrong number of counts in item {} in iteration {}",
                            n.num,
                            iter,
                        );
                    }
                    // Reset the head so we don't mix the runs together, which would create a mess.
                    // Also, the tails might disturb the ref counts.
                    bar.wait();
                    head.store(None);
                    nodes.last().unwrap().next.store(None);
                }
                bar.wait();
                // We went through all the iterations. Dismantle the list and see that everything
                // has ref count 1.
                head.store(None);
                for n in &nodes {
                    n.next.store(None);
                }
                bar.wait(); // Wait until everyone resets their own nexts
                for n in &nodes {
                    assert_eq!(1, Arc::strong_count(n));
                }
            });
        }
    });
}

#[test]
fn storm_link_list_small() {
    storm_link_list::<Global>(100, 5);
}

#[test]
fn storm_link_list_small_private() {
    storm_link_list::<PrivateUnsharded>(100, 5);
}

#[test]
fn storm_link_list_small_private_sharded() {
    storm_link_list::<PrivateSharded<[Shard; 3]>>(100, 5);
}

#[test]
#[ignore]
fn storm_list_link_large() {
    storm_link_list::<Global>(10_000, 50);
}

#[test]
#[ignore]
fn storm_list_link_large_private() {
    storm_link_list::<PrivateUnsharded>(10_000, 50);
}

#[test]
#[ignore]
fn storm_link_list_large_private_sharded() {
    storm_link_list::<PrivateSharded<[Shard; 3]>>(10_000, 50);
}

/// Test where we build and then deconstruct a linked list using multiple threads.
fn storm_unroll<S: LockStorage + Send + Sync>(node_cnt: usize, iters: usize) {
    struct LLNode<'a> {
        next: Option<Arc<LLNode<'a>>>,
        num: usize,
        owner: usize,
        live_cnt: &'a AtomicUsize,
    }

    impl<'a> Drop for LLNode<'a> {
        fn drop(&mut self) {
            self.live_cnt.fetch_sub(1, Ordering::Relaxed);
        }
    }

    let _lock = lock();

    let cpus = num_cpus::get();
    let bar = Barrier::new(cpus);
    let global_cnt = AtomicUsize::new(0);
    // We plan to create this many nodes during the whole test.
    let live_cnt = AtomicUsize::new(cpus * node_cnt * iters);
    let head = ArcSwapAny::<_, S>::from(None);
    thread::scope(|scope| {
        for thread in 0..cpus {
            // Borrow these instead of moving.
            let head = &head;
            let bar = &bar;
            let global_cnt = &global_cnt;
            let live_cnt = &live_cnt;
            scope.spawn(move || {
                for _ in 0..iters {
                    bar.wait();
                    // Create bunch of nodes and put them into the list.
                    for i in 0..node_cnt {
                        let mut node = Arc::new(LLNode {
                            next: None,
                            num: i,
                            owner: thread,
                            live_cnt,
                        });
                        head.rcu(|head| {
                            Arc::get_mut(&mut node).unwrap().next = Lease::upgrade(head);
                            Arc::clone(&node)
                        });
                    }
                    bar.wait();
                    // Keep removing items, count how many there are and that they increase in each
                    // thread's list.
                    let mut last_seen = vec![node_cnt; cpus];
                    let mut cnt = 0;
                    while let Some(node) =
                        head.rcu(|head| Lease::get_ref(&head).and_then(|h| h.next.clone()))
                    {
                        assert!(last_seen[node.owner] > node.num);
                        last_seen[node.owner] = node.num;
                        cnt += 1;
                    }
                    global_cnt.fetch_add(cnt, Ordering::Relaxed);
                    if bar.wait().is_leader() {
                        assert_eq!(node_cnt * cpus, global_cnt.swap(0, Ordering::Relaxed));
                    }
                }
            });
        }
    });
    // Everything got destroyed properly.
    assert_eq!(0, live_cnt.load(Ordering::Relaxed));
}

#[test]
fn storm_unroll_small() {
    storm_unroll::<Global>(100, 5);
}

#[test]
fn storm_unroll_small_private() {
    storm_unroll::<PrivateUnsharded>(100, 5);
}

#[test]
fn storm_unroll_small_private_sharded() {
    storm_unroll::<PrivateSharded<[Shard; 3]>>(100, 5);
}

#[test]
#[ignore]
fn storm_unroll_large() {
    storm_unroll::<Global>(10_000, 50);
}

#[test]
#[ignore]
fn storm_unroll_large_private() {
    storm_unroll::<PrivateUnsharded>(10_000, 50);
}

#[test]
#[ignore]
fn storm_unroll_large_private_sharded() {
    storm_unroll::<PrivateSharded<[Shard; 3]>>(10_000, 50);
}

fn lease_parallel<S: LockStorage + Send + Sync>(iters: usize) {
    let _lock = lock();
    let cpus = num_cpus::get();
    let shared = ArcSwapAny::<_, S>::from(Arc::new(0));
    thread::scope(|scope| {
        scope.spawn(|| {
            for i in 0..iters {
                shared.store(Arc::new(i));
            }
        });
        for _ in 0..cpus {
            scope.spawn(|| {
                for _ in 0..iters {
                    let leases = (0..256)
                        .into_iter()
                        .map(|_| shared.lease())
                        .collect::<Vec<_>>();
                    for (l, h) in leases.iter().tuple_windows() {
                        assert!(**l <= **h, "{} > {}", l, h);
                    }
                }
            });
        }
    });
    let v = shared.load();
    assert_eq!(2, Arc::strong_count(&v));
}

#[test]
fn lease_parallel_small() {
    lease_parallel::<Global>(1000);
}

#[test]
fn lease_parallel_small_private() {
    lease_parallel::<PrivateUnsharded>(1000);
}

#[test]
fn lease_parallel_small_private_sharded() {
    lease_parallel::<PrivateSharded<[Shard; 3]>>(1000);
}

#[test]
#[ignore]
fn lease_parallel_large() {
    lease_parallel::<Global>(100_000);
}

#[test]
#[ignore]
fn lease_parallel_large_private() {
    lease_parallel::<PrivateUnsharded>(100_000);
}

#[test]
#[ignore]
fn lease_parallel_large_private_sharded() {
    lease_parallel::<PrivateSharded<[Shard; 3]>>(100_000);
}

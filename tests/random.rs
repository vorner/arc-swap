//! Let it torture the implementation with some randomized operations.

extern crate arc_swap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate model;
#[macro_use]
extern crate proptest;

use std::mem;
use std::sync::Arc;

use arc_swap::{ArcSwap, Lease};

#[test]
fn ops() {
    model! {
        Model => let mut u = 0usize,
        Implementation => let a: ArcSwap<usize> = ArcSwap::from(Arc::new(0usize)),
        Store(usize)(v in any::<usize>()) => {
            u = v;
            a.store(Arc::new(v));
        },
        Load(())(() in any::<()>()) => {
            assert_eq!(u, *a.load());
        },
        Peek(())(() in any::<()>()) => {
            assert_eq!(u, *a.peek());
        },
        PeekSignalSafe(())(() in any::<()>()) => {
            assert_eq!(u, *a.peek_signal_safe());
        },
        Lease(())(() in any::<()>()) => {
            assert_eq!(u, *a.lease());
        },
        Swap(usize)(v in any::<usize>()) => {
            let expected = u;
            u = v;
            let actual = a.swap(Arc::new(v));
            assert_eq!(expected, *actual);
        }
    }
}

const LIMIT: usize = 5;

lazy_static! {
    static ref ARCS: Vec<Arc<usize>> = (0..LIMIT).map(|v| Arc::new(v)).collect();
}

#[test]
fn selection() {
    model! {
        Model => let mut bare = Arc::clone(&ARCS[0]),
        Implementation => let a: ArcSwap<usize> = ArcSwap::from(Arc::clone(&ARCS[0])),
        Swap(usize)(idx in 0..LIMIT) => {
            let mut expected = Arc::clone(&ARCS[idx]);
            mem::swap(&mut expected, &mut bare);
            let actual = a.swap(Arc::clone(&ARCS[idx]));
            assert!(Arc::ptr_eq(&expected, &actual));
        },
        Cas((usize, usize))((current, new) in (0..LIMIT, 0..LIMIT)) => {
            let expected = Arc::clone(&bare);
            if bare == ARCS[current] {
                bare = Arc::clone(&ARCS[new]);
            }
            let actual = a.compare_and_swap(&ARCS[current], Arc::clone(&ARCS[new]));
            assert!(Arc::ptr_eq(&expected, &Lease::upgrade(&actual)));
        }
    }
}

#[test]
fn linearize() {
    use model::Shared;

    linearizable! {
        Implementation => let a = Shared::new(ArcSwap::from(Arc::clone(&ARCS[0]))),
        Store(usize)(idx in 0..LIMIT) -> () {
            a.store(Arc::clone(&ARCS[idx]));
        },
        Peek(())(() in any::<()>()) -> usize {
            *a.peek()
        },
        Cas((usize, usize))((current, new) in (0..LIMIT, 0..LIMIT)) -> usize {
            let new = Arc::clone(&ARCS[new]);
            *a.compare_and_swap(&ARCS[current], new)
        }
    }
}

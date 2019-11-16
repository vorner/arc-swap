//! Let it torture the implementation with some randomized operations.

extern crate arc_swap;
#[macro_use]
extern crate model;
extern crate once_cell;
#[macro_use]
extern crate proptest;

use std::mem;
use std::sync::Arc;

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[test]
fn ops() {
    model! {
        Model => let mut u = 0usize,
        Implementation => let a: ArcSwap<usize> = ArcSwap::from(Arc::new(0usize)),
        Store(usize)(v in any::<usize>()) => {
            u = v;
            a.store(Arc::new(v));
        },
        LoadFull(())(() in any::<()>()) => {
            assert_eq!(u, *a.load_full());
        },
        LoadSignalSafe(())(() in any::<()>()) => {
            assert_eq!(u, **a.load_signal_safe());
        },
        Load(())(() in any::<()>()) => {
            assert_eq!(u, **a.load());
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

static ARCS: Lazy<Vec<Arc<usize>>> = Lazy::new(|| (0..LIMIT).map(Arc::new).collect());

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
            assert!(Arc::ptr_eq(&expected, &actual));
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
        Load(())(() in any::<()>()) -> usize {
            **a.load()
        },
        Cas((usize, usize))((current, new) in (0..LIMIT, 0..LIMIT)) -> usize {
            let new = Arc::clone(&ARCS[new]);
            **a.compare_and_swap(&ARCS[current], new)
        }
    }
}

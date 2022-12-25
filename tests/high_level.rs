use std::sync::Arc;

use arc_swap::ArcSwapAny;

/// Bug #81.
#[test]
fn layered_options() {
    let x = ArcSwapAny::<Option<Option<Arc<i32>>>>::new(None);
    assert_eq!(*x.load(), None);

    x.store(Some(Some(Arc::new(1))));
    assert_eq!(x.load().as_ref().unwrap().as_deref(), Some(&1));

    x.store(Some(None));
    assert_eq!(*x.load(), Some(None));
}

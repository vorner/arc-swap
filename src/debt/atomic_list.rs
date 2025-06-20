use core::marker::PhantomData;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    next: *mut Node<T>,
    data: T,
}

/// An iterator.
pub(crate) struct Iter<'a, T> {
    current: *mut Node<T>,
    // To tie the lifetime (both of the iterator and the references returned) to the list properly.
    _owner: PhantomData<&'a List<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            let result = unsafe { &(*self.current).data };
            self.current = unsafe { (*self.current).next };
            Some(result)
        }
    }
}

/// A lock-free, atomic linked list.
///
/// Singly-linked and add only (there's no remove or pop or anything). Iterates only with shared
/// references.
///
/// The intended usage is for global data structures that live for the whole lifetime of the
/// application.
///
/// Destructor exists only for unit tests, globals don't need it (nor use it).
///
/// Uses SeqCst for all operations, for two reasons:
/// * There are cases where we need to make sure a new item is "published" before we do other
///   operations.
/// * This will be used rarely (start of new thread), not worth the effort of proving anything
/// else.
pub(crate) struct List<T> {
    head: AtomicPtr<Node<T>>,
    _marker: PhantomData<Node<T>>,
}

impl<T: Send + Sync + 'static> List<T> {
    pub(crate) const fn new() -> Self {
        List {
            head: AtomicPtr::new(ptr::null_mut()),
            _marker: PhantomData,
        }
    }

    pub(crate) fn insert(&self, data: T) -> &T {
        let mut head = self.head.load(Ordering::SeqCst);
        let new = Box::new(Node { next: head, data });
        let new = Box::into_raw(new);

        while let Err(updated) =
            self.head
                .compare_exchange_weak(head, new, Ordering::SeqCst, Ordering::SeqCst)
        {
            head = updated;
            unsafe {
                (*new).next = updated;
            }
        }

        // As this is a box, the target won't ever move anywhere (until we are destroyed).
        // We only ever hand out shared references to it from now on.
        let result = unsafe { &(*new).data };

        result
    }

    pub(crate) fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.load(Ordering::SeqCst),
            _owner: PhantomData,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.load(Ordering::SeqCst);
        while !current.is_null() {
            let boxed = unsafe { Box::from_raw(current) };
            current = boxed.next;
            // Drops here
        }
    }
}

unsafe impl<T: Send + Sync + 'static> Send for List<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for List<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial() {
        let list = List::new();
        let mut refs: Vec<&u32> = Vec::new();
        refs.push(list.insert(1));
        refs.push(list.insert(2));
        refs.push(list.insert(3));
        // Value-based comparison
        assert_eq!(refs, [&1, &2, &3]);
        // Note: the items are prepended to the list, therefore iteration runs in newest->oldest
        // order.
        refs.reverse();
        let refs_iter = list.iter().collect::<Vec<_>>();
        assert_eq!(refs, refs_iter);
        // Pointer-based comparison
        for (r1, r2) in refs.into_iter().zip(refs_iter.into_iter()) {
            assert!(ptr::eq(r1, r2));
        }
    }

    // TODO: Some parallel test.
}

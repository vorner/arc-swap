use core::sync::atomic::{AtomicBool, Ordering};

use super::atomic_list::List;
use super::node::Node;

type Local = (); // Placeholder

struct Owned<T> {
    owned: AtomicBool,
    data: T,   
}

struct Cache<T> {
    list: List<Owned<T>>,
}

impl<T: Send + Sync + 'static> Cache<T> {
    const fn new() -> Self {
        Cache {
            list: List::new(),
        }
    }

    fn get(&self) -> ThreadNode<T> where T: Default {
        let local = Local::default();
        for existing in self.list.iter() {
            if existing.owned.compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed).is_ok() {
                return ThreadNode {
                    global: existing,
                    local,
                };
            }
        }

        let new = Owned {
            owned: AtomicBool::new(true),
            data: T::default(),
        };

        let inserted = self.list.insert(new);

        ThreadNode {
            global: inserted,
            local,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.list.iter().map(|o| &o.data)
    }
}

struct ThreadNode<'a, T> {
    global: &'a Owned<T>, 
    local: Local,
}

impl<T> Drop for ThreadNode<'_, T> {
    fn drop(&mut self) {
        self.global.owned.store(false, Ordering::SeqCst);
    }
}

static CACHE: Cache<Node> = Cache::new();

thread_local! {
    static THREAD_LOCAL: ThreadNode<'static, Node> = CACHE.get();
}

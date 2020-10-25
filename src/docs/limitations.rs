//! Limitations and common pitfalls.
//!
//! # Sized types
//!
//! This currently works only for `Sized` types. Unsized types have „fat pointers“, which are twice
//! as large as the normal ones. The [`AtomicPtr`] doesn't support them. One could use something
//! like `AtomicU128` for them. The catch is this doesn't exist and the difference would make it
//! really hard to implement the debt storage/stripped down hazard pointers.
//!
//! A workaround is to use double indirection:
//!
//! ```rust
//! # use arc_swap::ArcSwap;
//! // This doesn't work:
//! // let data: ArcSwap<[u8]> = ArcSwap::new(Arc::from([1, 2, 3]));
//!
//! // But this does:
//! let data: ArcSwap<Box<[u8]>> = ArcSwap::from_pointee(Box::new([1, 2, 3]));
//! # drop(data);
//! ```
//!
//! # Cloning behaviour
//!
//! When the [`ArcSwap`] is cloned, a new *independent* storage for `Arc` is created, originally
//! containing the same one. This is similar to cloning `RwLock<Arc<T>>` ‒ the instances also can
//! be manipulated independently.
//!
//! ```rust
//! # use std::sync::Arc;
//! # use arc_swap::ArcSwap;
//! let first: ArcSwap<String> = ArcSwap::from_pointee("Hello".to_owned());
//! let second: ArcSwap<String> = first.clone(); // Now they both point to the same thing
//! assert_eq!("Hello", **first.load());
//! assert_eq!("Hello", **second.load());
//! // Second points to a new thing
//! second.swap(Arc::new("World".to_owned()));
//! assert_eq!("World", **second.load());
//! // But first is independent and still points to the old value
//! assert_eq!("Hello", **first.load());
//! ```
//!
//! It is often more useful to share the same instance, either as a global variable, as a reference
//! or wrapping it into another Arc.
//!
//! ```rust
//! # use std::sync::Arc;
//! # use arc_swap::ArcSwap;
//! let first = Arc::new(ArcSwap::from_pointee("Hello".to_owned()));
//! let second = Arc::clone(&first);
//! assert_eq!("Hello", **first.load());
//! assert_eq!("Hello", **second.load());
//!
//! second.swap(Arc::new("World".to_owned()));
//! assert_eq!("World", **second.load());
//! assert_eq!("World", **first.load());
//! ```
//!
//! # Too many [`Guard`]s
//!
//! There's only limited number of "fast" slots for borrowing from [`ArcSwap`] for each single
//! thread (currently 8, but this might change in future versions). If these run out, the algorithm
//! falls back to slower path.
//!
//! If too many [`Guard`]s are kept around, the performance might be poor. These are not intended
//! to be stored in data structures or used across async yield points.
//!
//! [`ArcSwap`]: crate::ArcSwap
//! [`Guard`]: crate::Guard
//! [`AtomicPtr`]: std::sync::atomic::AtomicPtr

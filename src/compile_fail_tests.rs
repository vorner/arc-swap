// The doc tests allow us to do a compile_fail test, which is cool and what we want, but we don't
// want to expose this in the docs, so we use a private struct for that reason.
//
// Note we also bundle one that *does* compile with each, just to make sure they don't silently
// not-compile by some different reason.
//! ```rust,compile_fail
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! std::thread::spawn(|| {
//!     drop(shared);
//! });
//! ```
//!
//! ```rust
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! std::thread::spawn(|| {
//!     drop(shared);
//! });
//! ```
//!
//! ```rust,compile_fail
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! let guard = shared.peek();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         drop(guard);
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! let guard = shared.peek();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         drop(guard);
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust,compile_fail
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! let lease = shared.lease();
//! std::thread::spawn(|| {
//!     drop(lease);
//! });
//! ```
//!
//! ```rust
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! let lease = shared.lease();
//! std::thread::spawn(|| {
//!     drop(lease);
//! });
//! ```
//!
//! ```rust,compile_fail
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &shared;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &shared;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust,compile_fail
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! let guard = shared.peek();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &guard;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! let guard = shared.peek();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &guard;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust,compile_fail
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(std::cell::Cell::new(42));
//! let lease = shared.lease();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &lease;
//!     });
//! }).unwrap();
//! ```
//!
//! ```rust
//! extern crate arc_swap;
//! extern crate crossbeam_utils;
//! let shared = arc_swap::ArcSwap::from_pointee(42);
//! let lease = shared.lease();
//! crossbeam_utils::thread::scope(|scope| {
//!     scope.spawn(|_| {
//!         let _ = &lease;
//!     });
//! }).unwrap();
//! ```

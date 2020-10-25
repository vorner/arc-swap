//! Strategies for protecting the reference counts.
//!
//! There are multiple algorithms how to protect the reference counts while they're being updated
//! by multiple threads, each with its own set of pros and cons. The [`DefaultStrategy`] is used by
//! default and should generally be the least surprising option. It is possible to pick a different
//! strategy.
//!
//! For now, the traits in here are sealed and don't expose any methods to the users of the crate.
//! This is because we are not confident about the details just yet. In the future it may be
//! possible for downstream users to implement their own, but for now it is only so users can
//! choose one of the provided.
//!
//! It is expected that future strategies would come with different capabilities and limitations.
//! In particular, some that are not "tight" in the cleanup (delay the cleanup) or not support the
//! compare and swap operations.
//!
//! Currently, we have these strategies:
//!
//! * [`DefaultStrategy`] (this one is used implicitly)
//! * [`IndependentStrategy`]
//! * [`RwLock<()>`][std::sync::RwLock]
//!
//! # Testing
//!
//! Formally, the [`RwLock<()>`][std::sync::RwLock] may be used as a strategy too. It doesn't have
//! the performance characteristics or lock-free guarantees of the others, but it is much simpler
//! and contains less `unsafe` code (actually, less code altogether). Therefore, it can be used for
//! testing purposes and cross-checking.
//!
//! Note that generally, using [`RwLock<Arc<T>>`][std::sync::RwLock] is likely to be better
//! performance wise. So if the goal is to not use third-party unsafe code, only the one in
//! [`std`], that is the better option. This is provided mostly for investigation and testing of
//! [`ArcSwap`] itself or algorithms written to use [`ArcSwap`].
//!
//! *This is not meant to be used in production code*.
//!
//! # Experimental strategies
//!
//! There are some more strategies inside the [`experimental`] module. Note that these **are not**
//! subject to the API stability guarantees and can be changed, renamed or removed at any time.
//!
//! They are available only with the `experimental-strategies` feature.
//!
//! [`ArcSwap`]: crate::ArcSwap
//! [`load`]: crate::ArcSwapAny::load

use std::borrow::Borrow;
use std::sync::atomic::AtomicPtr;

use crate::gen_lock::{Global, PrivateUnsharded};
use crate::ref_cnt::RefCnt;

#[cfg(feature = "experimental-strategies")]
pub mod experimental;
mod gen_lock;
mod hybrid;
mod rw_lock;

use self::gen_lock::GenLockStrategy;
use self::hybrid::HybridStrategy;

/// The default strategy.
///
/// It is used by the type aliases [`ArcSwap`][crate::ArcSwap] and
/// [`ArcSwapOption`][crate::ArcSwapOption]. Only the other strategies need to be used explicitly.
///
/// It is optimized for read heavy situations. The readers are wait-free (with the exception of the
/// first [`load`] in each thread, which is merely lock-free), writers are not lock-free. The
/// reclamation is tight ‒ the resource is released as soon as possible.
///
/// Each thread has a limited number of "fast" slots. If a thread holds less than this number,
/// loading is fast and does not suffer from contention (unlike using [`RwLock`][std::sync::RwLock]
/// or even updating reference counts on the [`Arc`][std::sync::Arc]). In other words, no matter
/// how many threads concurrently read, they should not be affecting performance of each other in
/// any way.
///
/// If the slots run out (the thread holds too many [`Guard`][crate::Guard], the loading becomes
/// slower (because the reference counts need to be updated).
///
/// Currently, the implementation is a hybrid between stripped-down hazard pointers and one-sided
/// spin lock. The hazard pointers are the primary fast path. The spin locks are shared between all
/// instances, therefore the fallbacks may influence other instances.
///
/// However, the actual implementation can change in future versions for something else with
/// similar or better properties.
///
/// [`load`]: crate::ArcSwapAny::load
pub type DefaultStrategy = HybridStrategy<GenLockStrategy<Global>>;

/// Strategy for isolating instances.
///
/// It is similar to [`DefaultStrategy`], however the spin lock is not sharded (therefore multiple
/// concurrent threads might get bigger hit when multiple threads have to fall back). Nevertheless,
/// each instance has a private spin lock, not influencing the other instances. That also makes
/// them bigger in memory.
///
/// The hazard pointers are still shared between all instances.
///
/// The purpose of this strategy is meant for cases where a single instance is going to be
/// "tortured" a lot, so it should not overflow to other instances.
///
/// This too may be changed for something else (but with at least as good guarantees, primarily
/// that other instances won't get influenced by the "torture").
pub type IndependentStrategy = HybridStrategy<GenLockStrategy<PrivateUnsharded>>;

// TODO: When we are ready to un-seal, should these traits become unsafe?

pub(crate) mod sealed {
    use super::*;
    use crate::as_raw::AsRaw;

    pub trait Protected<T>: Borrow<T> {
        fn into_inner(self) -> T;
        fn from_inner(ptr: T) -> Self;
    }

    pub trait InnerStrategy<T: RefCnt> {
        // Drop „unlocks“
        type Protected: Protected<T>;
        unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected;
        unsafe fn wait_for_readers(&self, old: *const T::Base);
    }

    pub trait CaS<T: RefCnt>: InnerStrategy<T> {
        unsafe fn compare_and_swap<C: AsRaw<T::Base>>(
            &self,
            storage: &AtomicPtr<T::Base>,
            current: C,
            new: T,
        ) -> Self::Protected;
    }
}

/// A strategy for protecting the reference counted pointer `T`.
///
/// This chooses the algorithm for how the reference counts are protected. Note that the user of
/// the crate can't implement the trait and can't access any method; this is hopefully temporary
/// measure to make sure the interface is not part of the stability guarantees of the crate. Once
/// enough experience is gained with implementing various strategies, it will be un-sealed and
/// users will be able to provide their own implementation.
///
/// For now, the trait works only as a bound to talk about the types that represent strategies.
pub trait Strategy<T: RefCnt>: sealed::InnerStrategy<T> {}
impl<T: RefCnt, S: sealed::InnerStrategy<T>> Strategy<T> for S {}

/// An extension of the [`Strategy`], allowing for compare and swap operation.
///
/// The compare and swap operation is "advanced" and not all strategies need to support them.
/// Therefore, it is a separate trait.
///
/// Similarly, it is not yet made publicly usable or implementable and works only as a bound.
pub trait CaS<T: RefCnt>: sealed::CaS<T> {}
impl<T: RefCnt, S: sealed::CaS<T>> CaS<T> for S {}

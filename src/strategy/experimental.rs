//! Additional, experimental, strategies.
//!
//! Some more strategies are available here, if the `experimental-strategies` feature flag is
//! enabled.
//!
//! Note that these strategies **are not part of the API stability guarantees** and may be changed,
//! renamed or removed at any point in time. They are also not necessarily as mature. They are here
//! to allow experimentation.
//!
//! If they are deemed good enough, they might mature and be moved from here to some better place.
//! You're welcome to try them out, see if they fit better and send feedback about them. It is also
//! possible to add some more strategies in here. But might not be as good idea to use in
//! production just yet.

use crate::gen_lock::PrivateUnsharded;
pub use crate::gen_lock::Shard;
use crate::strategy::gen_lock::GenLockStrategy;

/// A generation lock.
///
/// The generation lock is the fallback strategy of the default hybrid one. This variant has single
/// shard inside each separate [`ArcSwap`], therefore they don't contend each other. As it doesn't
/// support the borrowing mode and creates a fully-featured [`Arc`] behind the scenes, it is
/// potentially slower than the default.
///
/// However, it is simpler and can turn out to be faster in certain corner case situations. It is
/// suitable when:
///
/// * A lot of guards would be held or loading the full [`Arc`]s is desirable most of the times,
///   therefore the advantage of the default strategy is not in play. In that case skipping the
///   fast path that is nevertheless not used is beneficial.
/// * The accesses are mostly across different [`ArcSwap`] instances.
///
/// Note that each instance has its own generation lock. This means that they are fully independent
/// of each other, but is larger (each [`ArcSwap`] then is 4 words large instead of 1).
///
/// Readers are [wait-free], writers are not even [lock-free].
///
/// [`ArcSwap`]: crate::ArcSwap
/// [`Arc`]: std::sync::Arc
/// [lock-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Lock-freedom
/// [wait-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Wait-freedom
pub type SimpleGenLock = GenLockStrategy<PrivateUnsharded>;

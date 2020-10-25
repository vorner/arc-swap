//! Customization of where and how the generation lock works.
//!
//! By default, all the [`ArcSwapAny`](../struct.ArcSwapAny.html) instances share the same
//! generation lock. This is to save space in them (they have the same size as a single pointer),
//! because the default lock is quite a large data structure (it's sharded, to prevent too much
//! contention between different threads). This has the disadvantage that a lock on one instance
//! influences another instance.
//!
//! The things in this module allow customizing how the lock behaves. The default one is
//! [`Global`](struct.Global.html). If you want to use independent but unsharded lock, use the
//! [`PrivateUnsharded`](struct.PrivateUnsharded.html) (or the
//! [`IndependentArcSwap`](../type.IndependentArcSwap.html) type alias).
//!
//! Or you can implement your own lock, but you probably should study the internals of the library
//! first.
//!
//! # Not Implemented Yet
//!
//! These variants would probably make sense, but haven't been written yet:
//!
//! * A lock storage that is shared, but only between a certain group of pointers. It could be
//!   either as a reference (but then each `ArcSwap` would get a bit bigger), or a macro that could
//!   generate an independent but global storage.

use std::borrow::Borrow;
use std::cell::Cell;
use std::process;
use std::sync::atomic::{self, AtomicUsize, Ordering};
use std::thread;

/// Number of shards (see [`Shard`]).
const SHARD_CNT: usize = 9;

/// If waiting in a spin loop, do a thread yield to the OS scheduler this many iterations
const YIELD_EVERY: usize = 16;

/// Maximum number of guards in the critical section
const MAX_GUARDS: usize = (std::isize::MAX) as usize;

/// How many generations we have in the lock.
pub(crate) const GEN_CNT: usize = 2;

/// A single shard.
///
/// This is one copy of place where the library keeps tracks of generation locks. It consists of a
/// pair of counters and allows double-buffering readers (therefore, even if there's a never-ending
/// stream of readers coming in, writer will get through eventually).
///
/// To avoid contention and sharing of the counters between readers, we don't have one pair of
/// generation counters, but several. The reader picks one shard and uses that, while the writer
/// looks through all of them. This is still not perfect (two threads may choose the same ID), but
/// it helps.
///
/// Each [`LockStorage`](trait.LockStorage.html) must provide a (non-empty) array of these.
#[repr(align(64))]
#[derive(Default)]
pub struct Shard(pub(crate) [AtomicUsize; GEN_CNT]);

fn snapshot(shard: &[AtomicUsize; GEN_CNT]) -> [usize; GEN_CNT] {
    [
        shard[0].load(Ordering::Acquire),
        shard[1].load(Ordering::Acquire),
    ]
}

impl Borrow<[AtomicUsize; GEN_CNT]> for Shard {
    #[inline]
    fn borrow(&self) -> &[AtomicUsize; GEN_CNT] {
        &self.0
    }
}

/// Abstraction of the place where generation locks are stored.
///
/// The trait is unsafe because if the trait messes up with the values stored in there in any way
/// (or makes the values available to something else that messes them up), this can cause UB and
/// daemons and discomfort to users and such. The library expects it is the only one storing values
/// there. In other words, it is expected the trait is only a dumb storage and doesn't actively do
/// anything.
pub unsafe trait LockStorage: Default {
    /// Type of one shard.
    type Shard: Borrow<[AtomicUsize; GEN_CNT]>;

    /// The type for keeping several shards.
    ///
    /// In general, it is expected to be a fixed-size array, but different implementations can have
    /// different sizes.
    type Shards: AsRef<[Self::Shard]>;

    /// Access to the generation index.
    ///
    /// Must return the same instance of the `AtomicUsize` for the lifetime of the storage, must
    /// start at `0` and the trait itself must not modify it.
    fn gen_idx(&self) -> &AtomicUsize;

    /// Access to the shards storage.
    ///
    /// Must return the same instance of the shards for the lifetime of the storage. Must start
    /// zeroed-out and the trait itself must not modify it.
    fn shards(&self) -> &Self::Shards;

    /// Pick one shard of the all selected.
    ///
    /// Returns the index of one of the shards. The choice can be arbitrary, but it should be fast
    /// and avoid collisions.
    fn choose_shard(&self) -> usize;
}

static GEN_IDX: AtomicUsize = AtomicUsize::new(0);

macro_rules! sh {
    () => {
        Shard([AtomicUsize::new(0), AtomicUsize::new(0)])
    };
}

type Shards = [Shard; SHARD_CNT];

/// The global shards.
static SHARDS: [Shard; SHARD_CNT] = [
    sh!(),
    sh!(),
    sh!(),
    sh!(),
    sh!(),
    sh!(),
    sh!(),
    sh!(),
    sh!(),
];

/// Global counter of threads.
///
/// We specifically don't use ThreadId here, because it is opaque and doesn't give us a number :-(.
static THREAD_ID_GEN: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    /// A shard a thread has chosen.
    ///
    /// The default value is just a marker it hasn't been set.
    static THREAD_SHARD: Cell<usize> = Cell::new(SHARD_CNT);
}

/// The default, global lock.
///
/// The lock is stored out-of-band, globally. This means that one `ArcSwap` with this lock storage
/// is only one machine word large, but a lock on one instance blocks the other, independent ones.
///
/// It has several shards so threads are less likely to collide (HW-contend) on them.
#[derive(Copy, Clone, Default)]
pub struct Global;

unsafe impl LockStorage for Global {
    type Shard = Shard;
    type Shards = Shards;

    #[inline]
    fn gen_idx(&self) -> &AtomicUsize {
        &GEN_IDX
    }

    #[inline]
    fn shards(&self) -> &Shards {
        &SHARDS
    }

    #[inline]
    fn choose_shard(&self) -> usize {
        THREAD_SHARD
            .try_with(|ts| {
                let mut val = ts.get();
                if val >= SHARD_CNT {
                    val = THREAD_ID_GEN.fetch_add(1, Ordering::Relaxed) % SHARD_CNT;
                    ts.set(val);
                }
                val
            })
            .unwrap_or(0)
    }
}

/// A single „shard“ that is stored inline, inside the corresponding `ArcSwap`. Therefore, locks on
/// each instance won't influence any other instances. On the other hand, the `ArcSwap` itself gets
/// bigger and doesn't have multiple shards, so concurrent uses might contend each other a bit.
///
/// Note that there`s a type alias [`IndependentArcSwap`](../type.IndependentArcSwap.html) that can
/// be used instead.
#[derive(Default)]
pub struct PrivateUnsharded {
    gen_idx: AtomicUsize,
    shard: [[AtomicUsize; GEN_CNT]; 1],
}

unsafe impl LockStorage for PrivateUnsharded {
    type Shard = [AtomicUsize; GEN_CNT];
    type Shards = [Self::Shard; 1];

    #[inline]
    fn gen_idx(&self) -> &AtomicUsize {
        &self.gen_idx
    }

    #[inline]
    fn shards(&self) -> &[Self::Shard; 1] {
        &self.shard
    }

    #[inline]
    fn choose_shard(&self) -> usize {
        0
    }
}

// TODO: Some nice docs about this thing
pub(crate) fn wait_for_readers<S: LockStorage>(storage: &S) {
    let mut seen_group = [false; GEN_CNT];
    let mut iter = 0usize;
    let gen_idk = storage.gen_idx();
    let shards = storage.shards().as_ref();

    loop {
        // Note that we don't need the snapshot to be consistent. We just need to see both
        // halves being zero, not necessarily at the same time.
        let gen = gen_idk.load(Ordering::Relaxed);
        let groups = shards.iter().fold([0, 0], |[a1, a2], s| {
            let [v1, v2] = snapshot(s.borrow());
            [a1 + v1, a2 + v2]
        });
        // Should we increment the generation? Is the next one empty?
        let next_gen = gen.wrapping_add(1);
        if groups[next_gen % GEN_CNT] == 0 {
            // Replace it only if someone else didn't do it in the meantime
            gen_idk.compare_and_swap(gen, next_gen, Ordering::Relaxed);
        }
        for i in 0..GEN_CNT {
            seen_group[i] = seen_group[i] || (groups[i] == 0);
        }

        if seen_group.iter().all(|seen| *seen) {
            break;
        }

        iter = iter.wrapping_add(1);
        if cfg!(not(miri)) {
            if iter % YIELD_EVERY == 0 {
                thread::yield_now();
            } else {
                atomic::spin_loop_hint();
            }
        }
    }
}

pub(crate) struct GenLock<'a> {
    slot: &'a AtomicUsize,
}

impl<'a> GenLock<'a> {
    pub(crate) fn new<S: LockStorage + 'a>(storage: &'a S) -> Self {
        let shard = storage.choose_shard();
        let gen = storage.gen_idx().load(Ordering::Relaxed) % GEN_CNT;
        // TODO: Is this still needed? Is the other SeqCst needed, in the writer? Is *there* any?
        // Or should it be Release in there and SeqCst barrier as part of wait_for_readers?
        // SeqCst: Acquire, so the dangerous section stays in. SeqCst to sync timelines with the
        // swap on the ptr in writer thread.
        let slot = &storage.shards().as_ref()[shard].borrow()[gen];
        let old = slot.fetch_add(1, Ordering::SeqCst);
        // The trick is taken from Arc.
        if old > MAX_GUARDS {
            process::abort();
        }

        Self { slot }
    }
}

impl Drop for GenLock<'_> {
    fn drop(&mut self) {
        // Release, so the dangerous section stays in. Acquire to chain the operations.
        // Do not drop the inner (maybe we should do into_raw for proper measures?)
        self.slot.fetch_sub(1, Ordering::AcqRel);
    }
}

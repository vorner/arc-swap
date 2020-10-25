//! Internal details.
//!
//! While the other parts of documentation are useful to users of the crate, this part is probably
//! helpful only if you want to look into the code or are curious about how it works internally.
//!
//! Also note that any of these details may change in future versions and are not part of the
//! stability guarantees. Don't rely on anything here.
//!
//! # Storing the [`Arc`].
//!
//! The [`Arc`] can be turned into a raw pointer and back. This is abstracted by the [`RefCnt`]
//! trait and it is technically possible to implement it for custom types (this crate also
//! implements it for [`Rc`] and [`Weak`], though the actual usefulness of these is a bit
//! questionable).
//!
//! The raw pointer is stored inside an [`AtomicPtr`].
//!
//! # Protection of reference counts
//!
//! The first idea would be to just use [`AtomicPtr`] with whatever the [`Arc::into_raw`] returns.
//! Then replacing it would be fine (there's no need to update ref counts). The load needs to
//! increment the reference count ‒ one still stays inside and another is returned to the caller.
//! This is done by re-creating the Arc from the raw pointer and then cloning it, throwing one
//! instance away (without destroying it).
//!
//! This approach has a problem. There's a short time between we read the raw pointer and increment
//! the count. If some other thread replaces the stored Arc and throws it away, the ref count could
//! drop to 0, get destroyed and we would be trying to bump ref counts in a ghost, which would be
//! totally broken.
//!
//! To prevent this, we actually use two approaches in a hybrid manner.
//!
//! The first one is based on hazard pointers idea, but slightly modified. There's a global
//! repository of pointers that owe a reference. When someone swaps a pointer, it walks this list
//! and pays all the debts (and takes them out of the repository).
//!
//! For simplicity and performance, storing into the repository is fallible. If storing into the
//! repository fails (because the thread used up all its own slots, or because the pointer got
//! replaced in just the wrong moment and it can't confirm the reservation), unlike the full
//! hazard-pointers approach, we don't retry, but fall back onto secondary strategy.
//!
//! Each reader registers itself so it can be tracked, but only as a number. Each writer first
//! switches the pointer. Then it takes a snapshot of all the current readers and waits until all of
//! them confirm bumping their reference count. Only then the writer returns to the caller, handing
//! it the ownership of the Arc and allowing possible bad things (like being destroyed) to happen to
//! it. This has its own disadvantages, so it is only the second approach.
//!
//! # Unsafety
//!
//! All the uses of the unsafe keyword is just to turn the raw pointer back to Arc. It originated
//! from an Arc in the first place, so the only thing to ensure is it is still valid. That means its
//! ref count never dropped to 0.
//!
//! At the beginning, there's ref count of 1 stored in the raw pointer (and maybe some others
//! elsewhere, but we can't rely on these). This 1 stays there for the whole time the pointer is
//! stored there. When the arc is replaced, this 1 is returned to the caller, so we just have to
//! make sure no more readers access it by that time.
//!
//! # Tracking of readers
//!
//! The simple way would be to have a count of all readers that could be in the dangerous area
//! between reading the pointer and bumping the reference count. We could „lock“ the ref count by
//! incrementing this atomic counter and „unlock“ it when done. The writer would just have to
//! busy-wait for this number to drop to 0 ‒ then there are no readers at all. This is safe, but a
//! steady inflow of readers could make a writer wait forever.
//!
//! Therefore, we separate readers into two groups, odd and even ones (see below how). When we see
//! both groups to drop to 0 (not necessarily at the same time, though), we are sure all the
//! previous readers were flushed ‒ each of them had to be either odd or even.
//!
//! To do that, we define a generation. A generation is a number, incremented at certain times and a
//! reader decides by this number if it is odd or even.
//!
//! One of the writers may increment the generation when it sees a zero in the next-generation's
//! group (if the writer sees 0 in the odd group and the current generation is even, all the current
//! writers are even ‒ so it remembers it saw odd-zero and increments the generation, so new readers
//! start to appear in the odd group and the even has a chance to drop to zero later on). Only one
//! writer does this switch, but all that witness the zero can remember it.
//!
//! We also split the reader threads into shards ‒ we have multiple copies of the counters, which
//! prevents some contention and sharing of the cache lines. The writer reads them all and sums them
//! up.
//!
//! # Leases and debts
//!
//! Instead of incrementing the reference count, the pointer reference can be owed. In such case, it
//! is recorded into a global storage. As each thread has its own storage (the global storage is
//! composed of multiple thread storages), the readers don't contend. When the pointer is no longer
//! in use, the debt is erased.
//!
//! The writer pays all the existing debts, therefore the reader have the full Arc with ref count at
//! that time. The reader is made aware the debt was paid and decrements the reference count.
//!
//! # Memory orders
//!
//! ## Synchronizing the data pointed to by the pointer.
//!
//! We have AcqRel (well, SeqCst, but that's included) on the swap and Acquire on the loads. In case
//! of the double read around the debt allocation, we do that on the *second*, because of ABA.
//! That's also why that SeqCst on the allocation of debt itself is not enough.
//!
//! ## The generation lock
//!
//! Second, the dangerous area when we borrowed the pointer but haven't yet incremented its ref
//! count needs to stay between incrementing and decrementing the reader count (in either group). To
//! accomplish that, using Acquire on the increment and Release on the decrement would be enough.
//! The loads in the writer use Acquire to complete the edge and make sure no part of the dangerous
//! area leaks outside of it in the writers view. This Acquire, however, forms the edge only with
//! the *latest* decrement. By making both the increment and decrement AcqRel, we effectively chain
//! the edges together.
//!
//! Now the hard part :-). We need to ensure that whatever zero a writer sees is not stale in the
//! sense that it happened before the switch of the pointer. In other words, we need to make sure
//! that at the time we start to look for the zeroes, we already see all the current readers. To do
//! that, we need to synchronize the time lines of the pointer itself and the corresponding group
//! counters. As these are separate, unrelated, atomics, it calls for SeqCst ‒ on the swap and on
//! the increment. This'll guarantee that they'll know which happened first (either increment or the
//! swap), making a base line for the following operations (load of the pointer or looking for
//! zeroes).
//!
//! # Memory orders around debts
//!
//! The linked list of debt nodes only grows. The shape of the list (existence of nodes) is
//! synchronized through Release on creation and Acquire on load on the head pointer.
//!
//! The debts work similar to locks ‒ Acquire and Release make all the pointer manipulation at the
//! interval where it is written down. However, we use the SeqCst on the allocation of the debt for
//! the same reason we do so with the generation lock.
//!
//! In case the writer pays the debt, it sees the new enough data (for the same reasons the stale
//! zeroes are not seen). The reference count on the Arc is AcqRel and makes sure it is not
//! destroyed too soon. The writer traverses all the slots, therefore they don't need to synchronize
//! with each other.
//!
//! # Orderings on the rest
//!
//! We don't really care much if we use a stale generation number ‒ it only works to route the
//! readers into one or another bucket, but even if it was completely wrong, it would only slow the
//! waiting for 0 down. So, the increments of it are just hints.
//!
//! All other operations can be Relaxed (they either only claim something, which doesn't need to
//! synchronize with anything else, or they are failed attempts at something ‒ and another attempt
//! will be made, the successful one will do the necessary synchronization).
//!
//! [`RefCnt`]: crate::RefCnt
//! [`Arc`]: std::sync::Arc
//! [`Arc::into_raw`]: std::sync::Arc::into_raw
//! [`Rc`]: std::rc::Rc
//! [`Weak`]: std::sync::Weak
//! [`AtomicPtr`]: std::sync::atomic::AtomicPtr

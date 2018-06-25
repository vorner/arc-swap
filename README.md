# ArcSwap

[![Travis Build Status](https://api.travis-ci.org/vorner/arc-swap.png?branch=master)](https://travis-ci.org/vorner/arc-swap)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/d9p4equeuhymfny6/branch/master?svg=true)](https://ci.appveyor.com/project/vorner/arc-swap/branch/master)

The Rust's [`Arc`] can be used from multiple threads and the count is safely
updated as needed. However, the [`Arc`] itself can't be atomically replaced. To
do that, one needs to place it under a lock.

On the other hand, [`AtomicPtr`] can be replaced atomically, but it's hard to
know when the target can be safely freed.

This is a cross-breed between the two ‒ an [`ArcSwap`] can be seeded with an
[`Arc`] and the [`Arc`] can be simultaneously replaced and read by multiple
threads.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`AtomicPtr`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html
[`ArcSwap`]: https://docs.rs/arc-swap/*/arc_swap/struct.ArcSwap.html

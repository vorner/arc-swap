#!/bin/sh

set -ex

export PATH="$PATH":~/.cargo/bin
export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=1

rm -f Cargo.lock
cargo build

if [ "$TRAVIS_RUST_VERSION" = 1.31.0 ] ; then
	exit
fi

# Allow some warnings on the very old compiler.
export RUSTFLAGS="-D warnings"

cargo test --release
cargo test --release -- --ignored
cargo doc --no-deps

# Sometimes nightly doesn't have clippy or rustfmt, so don't try that there.
if [ "$TRAVIS_RUST_VERSION" = nightly ] ; then
	cargo test --all --release --benches --all-features
	exit
fi

cargo clippy --all --tests -- --deny clippy::all
cargo fmt --all -- --check

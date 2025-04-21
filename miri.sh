#!/bin/sh

set -ex
export PROPTEST_CASES=10
# Seeds: 250 ‒ data race
#export MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-permissive-provenance -Zmiri-many-seeds=250..251"
export MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-strict-provenance -Zmiri-backtrace=full -Zmiri-many-seeds=0..2000"
cargo +nightly-2025-03-15 miri test --features weak,internal-test-strategies,experimental-strategies #,internal-test-traps
#cargo +nightly miri test --features weak,internal-test-strategies,experimental-strategies

* `compare_and_swap` accepts borrowed `Arc` as `current` and doesn't consume one
  ref count.
* Sharding internal counters, to improve performance on read-mostly contented
  scenarios.
* Providing `peek_signal_safe` as the only async signal safe method to use
  inside signal handlers. This removes the footgun with dropping the `Arc`
  returned from `load` inside a signal handler.

# 0.1.4

* The `peek` method to use the `Arc` inside without incrementing the reference
  count.
* Some more (and hopefully better) benchmarks.

# 0.1.3

* Documentation fix (swap is *not* lock-free in current implementation).

# 0.1.2

* More freedom in the `rcu` and `rcu_unwrap` return types.

# 0.1.1

* `rcu` support.
* `compare_and_swap` support.
* Added some primitive benchmarks.

# 0.1.0

* Initial implementation.

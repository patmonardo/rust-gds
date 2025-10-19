use crate::concurrency::Concurrency;

/// Batch calculation utilities for parallel execution.
///
/// This module provides pure functions for calculating optimal batch sizes
/// and thread counts for parallel graph algorithm execution.
///
/// These utilities mirror Java GDS ParallelUtil batch sizing methods.
pub struct BatchUtil;

impl BatchUtil {
    /// Calculates the number of threads required to process elements with given batch size.
    ///
    /// Returns the ceiling division of `element_count / batch_size`.
    ///
    /// # Panics
    ///
    /// Panics if `batch_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::parallel_util::BatchUtil;
    ///
    /// assert_eq!(BatchUtil::thread_count(1000, 10_000), 1);
    /// assert_eq!(BatchUtil::thread_count(1000, 5_000), 5);
    /// assert_eq!(BatchUtil::thread_count(1000, 5_500), 6);
    /// ```
    pub fn thread_count(batch_size: usize, element_count: usize) -> usize {
        if batch_size == 0 {
            panic!("Invalid batch size: 0");
        }
        if batch_size >= element_count {
            return 1;
        }
        Self::ceil_div(element_count, batch_size)
    }

    /// Calculates an adjusted batch size so that `node_count` is equally divided
    /// by `concurrency`, but no smaller than `min_batch_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::{Concurrency, parallel_util::BatchUtil};
    ///
    /// let concurrency = Concurrency::of(4);
    ///
    /// // 1000 nodes / 4 threads = 250, but min is 100
    /// assert_eq!(BatchUtil::adjusted_batch_size(1000, concurrency, 100), 250);
    ///
    /// // 100 nodes / 4 threads = 25, but min is 100
    /// assert_eq!(BatchUtil::adjusted_batch_size(100, concurrency, 100), 100);
    /// ```
    pub fn adjusted_batch_size(
        node_count: usize,
        concurrency: Concurrency,
        min_batch_size: usize,
    ) -> usize {
        let target_batch_size = Self::ceil_div(node_count, concurrency.value());
        target_batch_size.max(min_batch_size)
    }

    /// Calculates an adjusted batch size with both minimum and maximum bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::{Concurrency, parallel_util::BatchUtil};
    ///
    /// let concurrency = Concurrency::of(4);
    ///
    /// // 100000 nodes / 4 threads = 25000, capped at max 10000
    /// assert_eq!(
    ///     BatchUtil::adjusted_batch_size_with_max(100_000, concurrency, 100, 10_000),
    ///     10_000
    /// );
    /// ```
    pub fn adjusted_batch_size_with_max(
        node_count: usize,
        concurrency: Concurrency,
        min_batch_size: usize,
        max_batch_size: usize,
    ) -> usize {
        Self::adjusted_batch_size(node_count, concurrency, min_batch_size).min(max_batch_size)
    }

    /// Calculates a batch size that is:
    /// 1. At least `batch_size`
    /// 2. A power of two
    /// 3. Divides `node_count` into manageable chunks
    ///
    /// This is useful for cache-aligned processing and bit-manipulation optimizations.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::parallel_util::BatchUtil;
    ///
    /// assert_eq!(BatchUtil::power_of_two_batch_size(1000, 10), 16);
    /// assert_eq!(BatchUtil::power_of_two_batch_size(1000, 100), 128);
    /// assert_eq!(BatchUtil::power_of_two_batch_size(1000, 1024), 1024);
    /// ```
    pub fn power_of_two_batch_size(node_count: usize, mut batch_size: usize) -> usize {
        if batch_size == 0 {
            batch_size = 1;
        }

        batch_size = Self::next_highest_power_of_two(batch_size);

        // Keep doubling batch size until we can fit node_count into reasonable chunks
        while ((node_count + batch_size + 1) / batch_size) > (i32::MAX as usize) {
            batch_size <<= 1;
        }

        batch_size
    }

    /// Returns the next highest power of two, or the current value if already a power of two.
    ///
    /// Returns 0 if input is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::parallel_util::BatchUtil;
    ///
    /// assert_eq!(BatchUtil::next_highest_power_of_two(0), 0);
    /// assert_eq!(BatchUtil::next_highest_power_of_two(1), 1);
    /// assert_eq!(BatchUtil::next_highest_power_of_two(5), 8);
    /// assert_eq!(BatchUtil::next_highest_power_of_two(64), 64);
    /// assert_eq!(BatchUtil::next_highest_power_of_two(100), 128);
    /// ```
    pub fn next_highest_power_of_two(mut v: usize) -> usize {
        if v == 0 {
            return 0;
        }
        v -= 1;
        v |= v >> 1;
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;
        #[cfg(target_pointer_width = "64")]
        {
            v |= v >> 32;
        }
        v += 1;
        v
    }

    /// Ceiling division: `ceil(dividend / divisor)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::parallel_util::BatchUtil;
    ///
    /// assert_eq!(BatchUtil::ceil_div(10, 3), 4);
    /// assert_eq!(BatchUtil::ceil_div(9, 3), 3);
    /// assert_eq!(BatchUtil::ceil_div(10, 1), 10);
    /// ```
    pub fn ceil_div(dividend: usize, divisor: usize) -> usize {
        1 + (dividend.saturating_sub(1)) / divisor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_count_single_batch() {
        assert_eq!(BatchUtil::thread_count(1000, 500), 1);
        assert_eq!(BatchUtil::thread_count(1000, 1000), 1);
    }

    #[test]
    fn test_thread_count_multiple_batches() {
        assert_eq!(BatchUtil::thread_count(1000, 5000), 5);
        assert_eq!(BatchUtil::thread_count(1000, 5500), 6);
        assert_eq!(BatchUtil::thread_count(100, 1000), 10);
    }

    #[test]
    #[should_panic(expected = "Invalid batch size: 0")]
    fn test_thread_count_zero_batch_size() {
        BatchUtil::thread_count(0, 1000);
    }

    #[test]
    fn test_adjusted_batch_size_respects_min() {
        let concurrency = Concurrency::of(4);
        // 100 nodes / 4 = 25, but min is 100
        assert_eq!(BatchUtil::adjusted_batch_size(100, concurrency, 100), 100);
    }

    #[test]
    fn test_adjusted_batch_size_above_min() {
        let concurrency = Concurrency::of(4);
        // 1000 nodes / 4 = 250, min is 100
        assert_eq!(BatchUtil::adjusted_batch_size(1000, concurrency, 100), 250);
    }

    #[test]
    fn test_adjusted_batch_size_exact_division() {
        let concurrency = Concurrency::of(8);
        // 800 nodes / 8 = 100
        assert_eq!(BatchUtil::adjusted_batch_size(800, concurrency, 10), 100);
    }

    #[test]
    fn test_adjusted_batch_size_with_max_below_max() {
        let concurrency = Concurrency::of(4);
        // 1000 / 4 = 250, between min 100 and max 10000
        assert_eq!(
            BatchUtil::adjusted_batch_size_with_max(1000, concurrency, 100, 10_000),
            250
        );
    }

    #[test]
    fn test_adjusted_batch_size_with_max_hits_max() {
        let concurrency = Concurrency::of(4);
        // 100000 / 4 = 25000, capped at max 10000
        assert_eq!(
            BatchUtil::adjusted_batch_size_with_max(100_000, concurrency, 100, 10_000),
            10_000
        );
    }

    #[test]
    fn test_adjusted_batch_size_with_max_hits_min() {
        let concurrency = Concurrency::of(4);
        // 100 / 4 = 25, min 100, max 10000
        assert_eq!(
            BatchUtil::adjusted_batch_size_with_max(100, concurrency, 100, 10_000),
            100
        );
    }

    #[test]
    fn test_power_of_two_batch_size_zero() {
        assert_eq!(BatchUtil::power_of_two_batch_size(1000, 0), 1);
    }

    #[test]
    fn test_power_of_two_batch_size_already_power() {
        assert_eq!(BatchUtil::power_of_two_batch_size(1000, 64), 64);
        assert_eq!(BatchUtil::power_of_two_batch_size(1000, 1024), 1024);
    }

    #[test]
    fn test_power_of_two_batch_size_rounds_up() {
        assert_eq!(BatchUtil::power_of_two_batch_size(1000, 10), 16);
        assert_eq!(BatchUtil::power_of_two_batch_size(1000, 100), 128);
        assert_eq!(BatchUtil::power_of_two_batch_size(1000, 500), 512);
    }

    #[test]
    fn test_next_highest_power_of_two_zero() {
        assert_eq!(BatchUtil::next_highest_power_of_two(0), 0);
    }

    #[test]
    fn test_next_highest_power_of_two_one() {
        assert_eq!(BatchUtil::next_highest_power_of_two(1), 1);
    }

    #[test]
    fn test_next_highest_power_of_two_already_power() {
        assert_eq!(BatchUtil::next_highest_power_of_two(2), 2);
        assert_eq!(BatchUtil::next_highest_power_of_two(4), 4);
        assert_eq!(BatchUtil::next_highest_power_of_two(64), 64);
        assert_eq!(BatchUtil::next_highest_power_of_two(1024), 1024);
    }

    #[test]
    fn test_next_highest_power_of_two_rounds_up() {
        assert_eq!(BatchUtil::next_highest_power_of_two(3), 4);
        assert_eq!(BatchUtil::next_highest_power_of_two(5), 8);
        assert_eq!(BatchUtil::next_highest_power_of_two(100), 128);
        assert_eq!(BatchUtil::next_highest_power_of_two(1000), 1024);
    }

    #[test]
    fn test_ceil_div_exact() {
        assert_eq!(BatchUtil::ceil_div(10, 5), 2);
        assert_eq!(BatchUtil::ceil_div(100, 10), 10);
    }

    #[test]
    fn test_ceil_div_with_remainder() {
        assert_eq!(BatchUtil::ceil_div(10, 3), 4);
        assert_eq!(BatchUtil::ceil_div(100, 7), 15);
        assert_eq!(BatchUtil::ceil_div(1000, 333), 4);
    }

    #[test]
    fn test_ceil_div_one() {
        assert_eq!(BatchUtil::ceil_div(42, 1), 42);
    }

    #[test]
    fn test_ceil_div_larger_divisor() {
        assert_eq!(BatchUtil::ceil_div(5, 10), 1);
    }
}

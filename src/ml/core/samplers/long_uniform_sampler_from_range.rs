//! Long uniform sampler from range for ML in GDS.
//!
//! Translated from Java GDS ml-core LongUniformSamplerFromRange.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::long_uniform_sampler_by_exclusion::LongUniformSamplerByExclusion;
use super::long_uniform_sampler_with_retries::LongUniformSamplerWithRetries;

/// Threshold for choosing retry vs exclusion strategy
const RETRY_SAMPLING_RATIO: f64 = 0.6;

/// Adaptive uniform sampler that chooses between retry and exclusion strategies.
///
/// **Dialectical Strategy Selection**:
/// - **Thesis**: Retry-based sampling (optimistic, for sparse selection)
/// - **Antithesis**: Exclusion-based sampling (pessimistic, for dense selection)
/// - **Synthesis**: Adaptive choice based on sampling ratio
///
/// Uses retry-based sampling when ratio < 60%, exclusion-based when ratio ≥ 60%.
pub struct LongUniformSamplerFromRange {
    retry_based_sampler: LongUniformSamplerWithRetries,
    exclusion_based_sampler: LongUniformSamplerByExclusion,
}

impl LongUniformSamplerFromRange {
    /// Create a new adaptive sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            retry_based_sampler: LongUniformSamplerWithRetries::new(seed),
            exclusion_based_sampler: LongUniformSamplerByExclusion::new(seed),
        }
    }

    /// Sample unique values from the range using the optimal strategy.
    ///
    /// Automatically selects retry-based or exclusion-based sampling based on
    /// the ratio of requested samples to available samples.
    ///
    /// # Arguments
    /// * `inclusive_min` - Minimum value (inclusive)
    /// * `exclusive_max` - Maximum value (exclusive)
    /// * `lower_bound_on_valid_samples` - Lower bound on number of valid samples in range
    /// * `number_of_samples` - Desired number of samples
    /// * `is_invalid_sample` - Predicate that returns true if a sample should be rejected
    ///
    /// # Returns
    /// Array of unique samples
    pub fn sample<F>(
        &mut self,
        inclusive_min: u64,
        exclusive_max: u64,
        lower_bound_on_valid_samples: u64,
        number_of_samples: usize,
        is_invalid_sample: F,
    ) -> Vec<u64>
    where
        F: Fn(u64) -> bool,
    {
        // Calculate sampling ratio to choose strategy
        let sampling_ratio = number_of_samples as f64 / lower_bound_on_valid_samples as f64;

        if sampling_ratio < RETRY_SAMPLING_RATIO {
            // Sparse sampling - use retry-based (thesis: optimistic)
            self.retry_based_sampler.sample(
                inclusive_min,
                exclusive_max,
                lower_bound_on_valid_samples,
                number_of_samples,
                is_invalid_sample,
            )
        } else {
            // Dense sampling - use exclusion-based (antithesis: pessimistic)
            self.exclusion_based_sampler.sample(
                inclusive_min,
                exclusive_max,
                lower_bound_on_valid_samples,
                number_of_samples,
                is_invalid_sample,
            )
        }
    }

    /// Estimate memory usage for this sampler.
    ///
    /// Since only one sampler is used at a time, we can deduct the cost of one result array.
    pub fn memory_estimation(number_of_samples: usize) -> usize {
        let retry_estimation = LongUniformSamplerWithRetries::memory_estimation(number_of_samples);

        let exclusion_estimation = LongUniformSamplerByExclusion::memory_estimation(
            number_of_samples,
            ((number_of_samples as f64) / RETRY_SAMPLING_RATIO).ceil() as usize,
        );

        let base_size = std::mem::size_of::<Self>();
        let result_array = number_of_samples * std::mem::size_of::<u64>();

        // Add both samplers, but deduct one result array since only one is active
        retry_estimation + exclusion_estimation + base_size - result_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_sample_low_ratio_uses_retry() {
        let mut sampler = LongUniformSamplerFromRange::new(42);
        // 10/100 = 0.1 < 0.6, should use retry strategy
        let samples = sampler.sample(0, 100, 100, 10, |_| false);

        assert_eq!(samples.len(), 10);
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 10);
    }

    #[test]
    fn test_sample_high_ratio_uses_exclusion() {
        let mut sampler = LongUniformSamplerFromRange::new(42);
        // 80/100 = 0.8 >= 0.6, should use exclusion strategy
        let samples = sampler.sample(0, 100, 100, 80, |_| false);

        assert_eq!(samples.len(), 80);
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 80);
    }

    #[test]
    fn test_sample_at_threshold() {
        let mut sampler = LongUniformSamplerFromRange::new(42);
        // 60/100 = 0.6, exactly at threshold
        let samples = sampler.sample(0, 100, 100, 60, |_| false);

        assert_eq!(samples.len(), 60);
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 60);
    }

    #[test]
    fn test_sample_with_invalid() {
        let mut sampler = LongUniformSamplerFromRange::new(42);
        // Reject even numbers
        let samples = sampler.sample(0, 20, 10, 5, |x| x % 2 == 0);

        assert_eq!(samples.len(), 5);
        // All should be odd
        assert!(samples.iter().all(|&x| x % 2 == 1));
    }

    #[test]
    fn test_adaptive_strategy_correctness() {
        let mut sampler1 = LongUniformSamplerFromRange::new(42);
        let mut sampler2 = LongUniformSamplerFromRange::new(42);

        // Both should produce valid results regardless of strategy chosen
        let samples_low = sampler1.sample(0, 1000, 1000, 100, |_| false);
        let samples_high = sampler2.sample(0, 1000, 1000, 900, |_| false);

        assert_eq!(samples_low.len(), 100);
        assert_eq!(samples_high.len(), 900);

        let unique_low: HashSet<_> = samples_low.iter().copied().collect();
        let unique_high: HashSet<_> = samples_high.iter().copied().collect();

        assert_eq!(unique_low.len(), 100);
        assert_eq!(unique_high.len(), 900);
    }
}

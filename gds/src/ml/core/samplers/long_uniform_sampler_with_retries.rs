//! Long uniform sampler with retries for ML in GDS.
//!
//! Translated from Java GDS ml-core LongUniformSamplerWithRetries.java.
//! This is a literal 1:1 translation following repository translation policy.

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashSet;

/// Samples with retries until the desired number of unique samples are obtained.
///
/// WARNING: There is no maximum number of retries, so can take a long while if the number
/// of possible samples are close to the number of desired samples.
///
/// This sampler uses a retry-based strategy:
/// - **Being**: The sparse sample space (most candidates are valid)
/// - **Strategy**: Optimistic - assumes we'll find valid samples quickly
/// - **Best for**: Low sampling ratios (<60% of space)
pub struct LongUniformSamplerWithRetries {
    rng: StdRng,
    sampled_values_cache: HashSet<u64>,
}

impl LongUniformSamplerWithRetries {
    /// Create a new sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            sampled_values_cache: HashSet::new(),
        }
    }

    /// Sample unique values from the range [inclusive_min, exclusive_max).
    ///
    /// # Arguments
    /// * `inclusive_min` - Minimum value (inclusive)
    /// * `exclusive_max` - Maximum value (exclusive)
    /// * `lower_bound_on_valid_samples` - Lower bound on number of valid samples in range
    /// * `number_of_samples` - Desired number of samples
    /// * `is_invalid_sample` - Predicate that returns true if a sample should be rejected
    ///
    /// # Returns
    /// Array of >= max(k, lower_bound_on_valid_samples) unique samples
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
        // If we need all valid samples, just return them all
        if number_of_samples >= lower_bound_on_valid_samples as usize {
            return (inclusive_min..exclusive_max)
                .filter(|&x| !is_invalid_sample(x))
                .collect();
        }

        let mut samples = Vec::with_capacity(number_of_samples);
        self.sampled_values_cache.clear();

        while samples.len() < number_of_samples {
            let sample = self.rng.gen_range(inclusive_min..exclusive_max);

            // Skip invalid samples
            if is_invalid_sample(sample) {
                continue;
            }

            // Skip duplicates
            if !self.sampled_values_cache.insert(sample) {
                continue;
            }

            samples.push(sample);
        }

        samples
    }

    /// Estimate memory usage for this sampler.
    ///
    /// Returns approximate bytes needed for the sampler and result array.
    pub fn memory_estimation(number_of_samples: usize) -> usize {
        std::mem::size_of::<Self>()
            + number_of_samples * std::mem::size_of::<u64>() // HashSet entries
            + number_of_samples * std::mem::size_of::<u64>() // Result array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_basic() {
        let mut sampler = LongUniformSamplerWithRetries::new(42);
        let samples = sampler.sample(0, 100, 100, 10, |_| false);

        assert_eq!(samples.len(), 10);
        // Check all samples are unique
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 10);
        // Check all in range
        assert!(samples.iter().all(|&x| x < 100));
    }

    #[test]
    fn test_sample_with_invalid() {
        let mut sampler = LongUniformSamplerWithRetries::new(42);
        // Reject even numbers
        let samples = sampler.sample(0, 20, 10, 5, |x| x % 2 == 0);

        assert_eq!(samples.len(), 5);
        // All should be odd
        assert!(samples.iter().all(|&x| x % 2 == 1));
    }

    #[test]
    fn test_sample_all_valid() {
        let mut sampler = LongUniformSamplerWithRetries::new(42);
        // Request more samples than available
        let samples = sampler.sample(0, 10, 10, 15, |_| false);

        assert_eq!(samples.len(), 10);
        // Should return all values 0..10
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 10);
    }

    #[test]
    fn test_deterministic_with_seed() {
        let mut sampler1 = LongUniformSamplerWithRetries::new(42);
        let samples1 = sampler1.sample(0, 100, 100, 10, |_| false);

        let mut sampler2 = LongUniformSamplerWithRetries::new(42);
        let samples2 = sampler2.sample(0, 100, 100, 10, |_| false);

        assert_eq!(samples1, samples2);
    }
}

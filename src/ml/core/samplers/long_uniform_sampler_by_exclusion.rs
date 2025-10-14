//! Long uniform sampler by exclusion for ML in GDS.
//!
//! Translated from Java GDS ml-core LongUniformSamplerByExclusion.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::long_uniform_sampler_with_retries::LongUniformSamplerWithRetries;

/// Sample numbers by excluding from the given range.
///
/// This method is appropriate to call if the amount of samples one wants is not much
/// smaller than the amount of valid numbers we sample from.
///
/// This sampler uses an exclusion-based strategy:
/// - **Being**: The dense sample space (most candidates will be selected)
/// - **Strategy**: Build valid space, then sample indices to *remove*
/// - **Best for**: High sampling ratios (≥60% of space)
pub struct LongUniformSamplerByExclusion {
    sampler_with_retries: LongUniformSamplerWithRetries,
}

impl LongUniformSamplerByExclusion {
    /// Create a new exclusion-based sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            sampler_with_retries: LongUniformSamplerWithRetries::new(seed),
        }
    }

    /// Sample unique values from the range by building the valid space and excluding samples.
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

        // Build the valid sample space
        let valid_sample_space: Vec<u64> = (inclusive_min..exclusive_max)
            .filter(|&x| !is_invalid_sample(x))
            .collect();

        assert!(
            valid_sample_space.len() >= number_of_samples,
            "Valid sample space {} must be >= number_of_samples {}",
            valid_sample_space.len(),
            number_of_samples
        );

        // Sample indices to *remove* from the valid space
        let num_to_remove = valid_sample_space.len() - number_of_samples;
        let mut samples_to_remove = self.sampler_with_retries.sample(
            0,
            valid_sample_space.len() as u64,
            valid_sample_space.len() as u64,
            num_to_remove,
            |_| false,
        );

        // Sort indices to remove for efficient copying
        samples_to_remove.sort_unstable();

        // Build result by keeping elements NOT in samples_to_remove
        let mut samples = Vec::with_capacity(number_of_samples);
        let mut next_idx_to_keep = 0;

        for &next_idx_to_remove in &samples_to_remove {
            let next_idx_to_remove = next_idx_to_remove as usize;
            // Copy all elements between last remove and this remove
            samples.extend_from_slice(&valid_sample_space[next_idx_to_keep..next_idx_to_remove]);
            next_idx_to_keep = next_idx_to_remove + 1;
        }

        // Copy remaining elements after last remove
        samples.extend_from_slice(&valid_sample_space[next_idx_to_keep..]);

        samples
    }

    /// Estimate memory usage for this sampler.
    ///
    /// Returns approximate bytes needed for the sampler and intermediate structures.
    pub fn memory_estimation(
        number_of_samples: usize,
        max_lower_bound_on_valid_samples: usize,
    ) -> usize {
        let sampler_with_retries_min = LongUniformSamplerWithRetries::memory_estimation(0);
        let sampler_with_retries_max = LongUniformSamplerWithRetries::memory_estimation(
            number_of_samples.min(max_lower_bound_on_valid_samples - number_of_samples),
        );

        let base_size = std::mem::size_of::<Self>();
        let result_array = number_of_samples * std::mem::size_of::<u64>();

        // Min: empty valid sample space
        let min = base_size + result_array + sampler_with_retries_min;
        // Max: full valid sample space
        let max = base_size
            + result_array
            + (max_lower_bound_on_valid_samples * std::mem::size_of::<u64>())
            + sampler_with_retries_max;

        (min + max) / 2 // Return average
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_sample_basic() {
        let mut sampler = LongUniformSamplerByExclusion::new(42);
        let samples = sampler.sample(0, 100, 100, 80, |_| false);

        assert_eq!(samples.len(), 80);
        // Check all samples are unique
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 80);
        // Check all in range
        assert!(samples.iter().all(|&x| x < 100));
    }

    #[test]
    fn test_sample_with_invalid() {
        let mut sampler = LongUniformSamplerByExclusion::new(42);
        // Reject even numbers (10 valid odd numbers in range)
        let samples = sampler.sample(0, 20, 10, 8, |x| x % 2 == 0);

        assert_eq!(samples.len(), 8);
        // All should be odd
        assert!(samples.iter().all(|&x| x % 2 == 1));
    }

    #[test]
    fn test_sample_all_valid() {
        let mut sampler = LongUniformSamplerByExclusion::new(42);
        // Request all valid samples
        let samples = sampler.sample(0, 10, 10, 10, |_| false);

        assert_eq!(samples.len(), 10);
        // Should return all values 0..10
        let mut sorted = samples.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..10).collect::<Vec<_>>());
    }

    #[test]
    fn test_high_sampling_ratio() {
        let mut sampler = LongUniformSamplerByExclusion::new(42);
        // Sample 95% of space (ideal for exclusion method)
        let samples = sampler.sample(0, 100, 100, 95, |_| false);

        assert_eq!(samples.len(), 95);
        let unique: HashSet<_> = samples.iter().copied().collect();
        assert_eq!(unique.len(), 95);
    }
}

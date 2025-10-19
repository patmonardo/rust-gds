//! Uniform random sampling without replacement.
//!
//! Implements Algorithm L for efficient reservoir sampling from streams.
//! Automatically chooses between index-based and reservoir-based strategies
//! depending on the sample size relative to input size.
//!
//! # Algorithm
//!
//! - **Index sampling**: When sampling < 50% of input (sparse sampling)
//! - **Reservoir sampling**: When sampling >= 50% of input (dense sampling)
//!
//! # References
//!
//! Algorithm L: https://richardstartin.github.io/posts/reservoir-sampling#algorithm-l
//!
//! # Examples
//!
//! ```rust,ignore
//! use gds::ml::core::samplers::UniformSampler;
//!
//! let sampler = UniformSampler::new(42);
//!
//! // Sample 10 neighbors from stream of 100
//! let neighbors = sampler.sample_from_stream(neighbor_stream, 100, 10);
//! ```

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

/// Uniform random sampler using Algorithm L.
pub struct UniformSampler {
    rng: ChaCha8Rng,
}

impl UniformSampler {
    /// Create a new uniform sampler with the given random seed.
    ///
    /// # Arguments
    ///
    /// * `random_seed` - Seed for deterministic sampling
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::ml::core::samplers::UniformSampler;
    ///
    /// let sampler = UniformSampler::new(42);
    /// ```
    pub fn new(random_seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(random_seed),
        }
    }

    /// Sample from an iterator, automatically choosing the optimal strategy.
    ///
    /// # Arguments
    ///
    /// * `input` - Iterator of node IDs
    /// * `lower_bound_input_length` - Known or estimated input size
    /// * `number_of_samples` - Number of samples to draw
    ///
    /// # Returns
    ///
    /// Vector of sampled node IDs
    pub fn sample<I>(
        &mut self,
        input: I,
        lower_bound_input_length: u64,
        number_of_samples: usize,
    ) -> Vec<u64>
    where
        I: Iterator<Item = u64>,
    {
        if (number_of_samples as f64) / (lower_bound_input_length as f64) < 0.5 {
            self.sample_with_indexes(input, lower_bound_input_length, number_of_samples)
        } else {
            self.sample_with_reservoir(input, lower_bound_input_length, number_of_samples)
        }
    }

    /// Sample using reservoir sampling (Algorithm L).
    ///
    /// Efficient when sampling a large fraction of the input (>= 50%).
    ///
    /// # Algorithm
    ///
    /// 1. Fill reservoir with first k elements
    /// 2. For remaining elements, randomly replace with decreasing probability
    /// 3. Use skip optimization to avoid processing every element
    ///
    /// # Arguments
    ///
    /// * `input` - Iterator of node IDs
    /// * `lower_bound_input_length` - Known or estimated input size
    /// * `number_of_samples` - Number of samples to draw
    pub fn sample_with_reservoir<I>(
        &mut self,
        input: I,
        lower_bound_input_length: u64,
        number_of_samples: usize,
    ) -> Vec<u64>
    where
        I: Iterator<Item = u64>,
    {
        if number_of_samples == 0 {
            return Vec::new();
        }

        if number_of_samples >= lower_bound_input_length as usize {
            return input.collect();
        }

        let mut reservoir = Vec::with_capacity(number_of_samples);
        let mut input_iter = input;

        // Fill reservoir with first k elements
        for _ in 0..number_of_samples {
            if let Some(value) = input_iter.next() {
                reservoir.push(value);
            } else {
                // Input was shorter than expected
                return reservoir;
            }
        }

        let mut next_idx_to_sample = number_of_samples - 1;
        let mut skip_factor = self.compute_skip_factor(number_of_samples);

        // Compute first skip
        next_idx_to_sample += self.compute_number_of_skips(skip_factor);
        skip_factor *= self.compute_skip_factor(number_of_samples);

        let mut idx = number_of_samples;
        for input_value in input_iter {
            if idx == next_idx_to_sample {
                // Replace random element in reservoir
                let replace_idx = self.rng.gen_range(0..number_of_samples);
                reservoir[replace_idx] = input_value;

                // Compute next skip
                next_idx_to_sample += self.compute_number_of_skips(skip_factor);
                skip_factor *= self.compute_skip_factor(number_of_samples);
            }
            idx += 1;
        }

        reservoir
    }

    /// Sample using index-based strategy.
    ///
    /// Efficient when sampling a small fraction of the input (< 50%).
    /// Pre-generates random indices, then filters the input stream.
    ///
    /// # Arguments
    ///
    /// * `input` - Iterator of node IDs
    /// * `lower_bound_input_length` - Known or estimated input size
    /// * `number_of_samples` - Number of samples to draw
    pub fn sample_with_indexes<I>(
        &mut self,
        input: I,
        lower_bound_input_length: u64,
        number_of_samples: usize,
    ) -> Vec<u64>
    where
        I: Iterator<Item = u64>,
    {
        if number_of_samples == 0 {
            return Vec::new();
        }

        if number_of_samples >= lower_bound_input_length as usize {
            return input.collect();
        }

        // Generate unique random indices
        let sampled_indexes =
            self.sample_unique_numbers_hashset(number_of_samples, lower_bound_input_length);

        // Filter input by sampled indices
        input
            .enumerate()
            .filter_map(|(idx, value)| {
                if sampled_indexes.contains(&(idx as u64)) {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Generate m unique random numbers in range [0, n).
    ///
    /// # Arguments
    ///
    /// * `m` - Number of unique values to generate
    /// * `n` - Upper bound (exclusive)
    ///
    /// # Returns
    ///
    /// HashSet of unique random numbers
    ///
    /// # Panics
    ///
    /// Panics if m > n (cannot sample more elements than available)
    pub fn sample_unique_numbers_hashset(&mut self, m: usize, n: u64) -> HashSet<u64> {
        assert!(
            m <= n as usize,
            "Cannot sample more unique numbers than the range allows: {} > {}",
            m,
            n
        );

        let mut unique_numbers = HashSet::with_capacity(m);

        if n == m as u64 {
            // Special case: sample all elements
            for i in 0..n {
                unique_numbers.insert(i);
            }
            return unique_numbers;
        }

        // Generate unique random numbers until we have m
        while unique_numbers.len() < m {
            let random_number = self.rng.gen_range(0..n);
            unique_numbers.insert(random_number);
        }

        unique_numbers
    }

    /// Compute skip factor for Algorithm L.
    ///
    /// w = exp(log(U) / k), where U is uniform random in [0,1]
    fn compute_skip_factor(&mut self, number_of_samples: usize) -> f64 {
        let u: f64 = self.rng.gen();
        (u.ln() / number_of_samples as f64).exp()
    }

    /// Compute number of elements to skip.
    ///
    /// S = floor(log(U) / log(1-w)) + 1
    fn compute_number_of_skips(&mut self, skip_factor: f64) -> usize {
        let u: f64 = self.rng.gen();
        ((u.ln() / (1.0 - skip_factor).ln()).floor() as usize) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_empty() {
        let mut sampler = UniformSampler::new(42);
        let result = sampler.sample(std::iter::empty(), 0, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_sample_all() {
        let mut sampler = UniformSampler::new(42);
        let input = vec![1, 2, 3, 4, 5];
        let result = sampler.sample(input.into_iter(), 5, 5);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_sample_more_than_available() {
        let mut sampler = UniformSampler::new(42);
        let input = vec![1, 2, 3];
        let result = sampler.sample(input.into_iter(), 3, 10);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_sample_with_reservoir_small() {
        let mut sampler = UniformSampler::new(42);
        let input = (0..100).collect::<Vec<_>>();
        let result = sampler.sample_with_reservoir(input.into_iter(), 100, 10);

        assert_eq!(result.len(), 10);

        // All samples should be unique
        let unique: HashSet<_> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 10);

        // All samples should be in valid range
        for &val in &result {
            assert!(val < 100);
        }
    }

    #[test]
    fn test_sample_with_indexes_small() {
        let mut sampler = UniformSampler::new(42);
        let input = (0..100).collect::<Vec<_>>();
        let result = sampler.sample_with_indexes(input.into_iter(), 100, 10);

        assert_eq!(result.len(), 10);

        // All samples should be unique
        let unique: HashSet<_> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 10);
    }

    #[test]
    fn test_sample_unique_numbers_hashset() {
        let mut sampler = UniformSampler::new(42);
        let result = sampler.sample_unique_numbers_hashset(5, 10);

        assert_eq!(result.len(), 5);

        for &num in &result {
            assert!(num < 10);
        }
    }

    #[test]
    fn test_sample_unique_numbers_all() {
        let mut sampler = UniformSampler::new(42);
        let result = sampler.sample_unique_numbers_hashset(5, 5);

        assert_eq!(result.len(), 5);
        assert_eq!(result, (0..5).collect());
    }

    #[test]
    #[should_panic(expected = "Cannot sample more unique numbers than the range allows")]
    fn test_sample_unique_numbers_too_many() {
        let mut sampler = UniformSampler::new(42);
        sampler.sample_unique_numbers_hashset(10, 5);
    }

    #[test]
    fn test_deterministic_sampling() {
        let mut sampler1 = UniformSampler::new(42);
        let mut sampler2 = UniformSampler::new(42);

        let input1 = (0..100).collect::<Vec<_>>();
        let input2 = (0..100).collect::<Vec<_>>();

        let result1 = sampler1.sample(input1.into_iter(), 100, 10);
        let result2 = sampler2.sample(input2.into_iter(), 100, 10);

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_different_seeds_different_results() {
        let mut sampler1 = UniformSampler::new(42);
        let mut sampler2 = UniformSampler::new(43);

        let input1 = (0..100).collect::<Vec<_>>();
        let input2 = (0..100).collect::<Vec<_>>();

        let result1 = sampler1.sample(input1.into_iter(), 100, 10);
        let result2 = sampler2.sample(input2.into_iter(), 100, 10);

        assert_ne!(result1, result2);
    }

    #[test]
    fn test_sample_threshold_behavior() {
        // Test that threshold at 50% switches strategies correctly
        let mut sampler = UniformSampler::new(42);

        // Just under 50% should use index strategy
        let input1 = (0..100).collect::<Vec<_>>();
        let result1 = sampler.sample(input1.into_iter(), 100, 49);
        assert_eq!(result1.len(), 49);

        // At 50% should use reservoir strategy
        let input2 = (0..100).collect::<Vec<_>>();
        let result2 = sampler.sample(input2.into_iter(), 100, 50);
        assert_eq!(result2.len(), 50);
    }
}

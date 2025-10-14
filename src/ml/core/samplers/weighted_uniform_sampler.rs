//! Weighted uniform sampler for ML in GDS.
//!
//! Translated from Java GDS ml-core WeightedUniformSampler.java.
//! This is a literal 1:1 translation following repository translation policy.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Used in denominators to avoid division by zero.
const EPSILON: f64 = 1e-10;

/// Entry in the weighted reservoir (node ID + priority).
#[derive(Debug, Clone)]
struct WeightedEntry {
    node_id: u64,
    priority: f64,
}

impl PartialEq for WeightedEntry {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for WeightedEntry {}

impl PartialOrd for WeightedEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering for max-heap (BinaryHeap is max-heap by default)
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for WeightedEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Weighted Reservoir Sampling based on Algorithm A-Res.
///
/// Reference: https://en.wikipedia.org/wiki/Reservoir_sampling#Algorithm_A-Res
///
/// Higher weights increase the probability of being sampled.
/// This is appropriate for sampling neighbors in weighted graphs where
/// edge weights represent importance or strength of relationship.
pub struct WeightedUniformSampler {
    random: StdRng,
}

impl WeightedUniformSampler {
    /// Create a new weighted sampler with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            random: StdRng::seed_from_u64(seed),
        }
    }

    /// Sample from weighted (node_id, weight) pairs.
    ///
    /// # Arguments
    /// * `input` - Iterator of (node_id, weight) pairs
    /// * `input_size` - Size hint for the input (used for optimization)
    /// * `number_of_samples` - Desired number of samples
    ///
    /// # Returns
    /// Vector of sampled node IDs (higher weights more likely to be selected)
    pub fn sample<I>(&mut self, input: I, input_size: usize, number_of_samples: usize) -> Vec<u64>
    where
        I: IntoIterator<Item = (u64, f64)>,
    {
        self.sample_filtered(input, input_size, number_of_samples, |_| true)
    }

    /// Sample from weighted (node_id, weight) pairs with a filter predicate.
    ///
    /// # Arguments
    /// * `input` - Iterator of (node_id, weight) pairs
    /// * `input_size` - Size hint for the input
    /// * `number_of_samples` - Desired number of samples
    /// * `include_node` - Predicate that returns true if node should be included
    ///
    /// # Returns
    /// Vector of sampled node IDs that pass the filter
    pub fn sample_filtered<I, F>(
        &mut self,
        input: I,
        input_size: usize,
        number_of_samples: usize,
        include_node: F,
    ) -> Vec<u64>
    where
        I: IntoIterator<Item = (u64, f64)>,
        F: Fn(u64) -> bool,
    {
        if number_of_samples == 0 {
            return Vec::new();
        }

        if number_of_samples >= input_size {
            return input
                .into_iter()
                .filter(|(node_id, _)| include_node(*node_id))
                .map(|(node_id, _)| node_id)
                .collect();
        }

        // Use a bounded priority queue (max-heap) as reservoir
        let mut reservoir = BinaryHeap::with_capacity(number_of_samples);

        for (node_id, weight) in input {
            if !include_node(node_id) {
                continue;
            }

            self.process_weighted_entry(&mut reservoir, node_id, weight, number_of_samples);
        }

        // Extract node IDs from reservoir
        reservoir.into_iter().map(|entry| entry.node_id).collect()
    }

    /// Process a single weighted entry and update the reservoir.
    ///
    /// Uses Algorithm A-Res: priority = random^(1/weight)
    /// Higher weights → higher priority → more likely to be kept in reservoir.
    fn process_weighted_entry(
        &mut self,
        reservoir: &mut BinaryHeap<WeightedEntry>,
        node_id: u64,
        weight: f64,
        capacity: usize,
    ) {
        // Calculate priority: higher weights should be more likely to be sampled
        // priority = random^(1/weight) where random in [0,1)
        let random_value: f64 = self.random.gen();
        let priority = random_value.powf(1.0 / (weight + EPSILON));

        let entry = WeightedEntry { node_id, priority };

        if reservoir.len() < capacity {
            // Reservoir not full, just add
            reservoir.push(entry);
        } else if let Some(min_entry) = reservoir.peek() {
            // Reservoir full, replace minimum if new priority is higher
            if priority > min_entry.priority {
                reservoir.pop();
                reservoir.push(entry);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_sample_basic() {
        let mut sampler = WeightedUniformSampler::new(42);
        let input = vec![(0, 1.0), (1, 1.0), (2, 1.0), (3, 1.0), (4, 1.0)];

        let samples = sampler.sample(input.into_iter(), 5, 3);

        assert_eq!(samples.len(), 3);
        // All samples should be in range [0, 5)
        assert!(samples.iter().all(|&x| x < 5));
    }

    #[test]
    fn test_sample_weighted_bias() {
        let mut sampler = WeightedUniformSampler::new(42);
        // Node 1 has 100x higher weight than node 0
        let input = vec![(0, 1.0), (1, 100.0)];

        // Sample many times to check probability
        let mut counts = HashMap::new();
        for _ in 0..1000 {
            let samples = sampler.sample(input.clone().into_iter(), 2, 1);
            *counts.entry(samples[0]).or_insert(0) += 1;
        }

        // Node 1 should be sampled much more often than node 0
        let count_0 = *counts.get(&0).unwrap_or(&0);
        let count_1 = *counts.get(&1).unwrap_or(&0);

        assert!(
            count_1 > count_0 * 5,
            "Higher weight should be sampled more often: node1={}, node0={}",
            count_1,
            count_0
        );
    }

    #[test]
    fn test_sample_with_filter() {
        let mut sampler = WeightedUniformSampler::new(42);
        let input = vec![(0, 1.0), (1, 1.0), (2, 1.0), (3, 1.0), (4, 1.0)];

        // Only include even numbers
        let samples = sampler.sample_filtered(input.into_iter(), 5, 3, |node_id| node_id % 2 == 0);

        assert_eq!(samples.len(), 3);
        // All samples should be even
        assert!(samples.iter().all(|&x| x % 2 == 0));
    }

    #[test]
    fn test_sample_all() {
        let mut sampler = WeightedUniformSampler::new(42);
        let input = vec![(0, 1.0), (1, 2.0), (2, 3.0)];

        // Request more samples than available
        let samples = sampler.sample(input.into_iter(), 3, 10);

        assert_eq!(samples.len(), 3);
        assert!(samples.contains(&0));
        assert!(samples.contains(&1));
        assert!(samples.contains(&2));
    }

    #[test]
    fn test_sample_zero() {
        let mut sampler = WeightedUniformSampler::new(42);
        let input = vec![(0, 1.0), (1, 1.0)];

        let samples = sampler.sample(input.into_iter(), 2, 0);

        assert_eq!(samples.len(), 0);
    }

    #[test]
    fn test_epsilon_prevents_division_by_zero() {
        let mut sampler = WeightedUniformSampler::new(42);
        // Zero weight should not cause panic
        let input = vec![(0, 0.0), (1, 1.0)];

        let samples = sampler.sample(input.into_iter(), 2, 1);

        assert_eq!(samples.len(), 1);
    }
}

//! Feature bagger for random feature selection in decision trees.
//!
//! Translated from Java GDS ml-algo FeatureBagger.java.
//! This is a literal 1:1 translation following repository translation policy.

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// Samples a subset of features for each split (feature bagging).
///
/// NOTE: This struct is not thread-safe.
#[derive(Clone)]
pub struct FeatureBagger {
    rng: StdRng,
    total_number_of_features: usize,
    number_of_samples: usize,
}

impl FeatureBagger {
    pub fn memory_estimation(number_of_samples: usize) -> usize {
        std::mem::size_of::<Self>() + std::mem::size_of::<usize>() * number_of_samples
    }

    pub fn new(seed: u64, total_number_of_features: usize, max_features_ratio: f64) -> Self {
        assert!(
            max_features_ratio != 0.0,
            "Invalid maxFeaturesRatio: {}",
            max_features_ratio
        );

        let number_of_samples =
            (max_features_ratio * total_number_of_features as f64).ceil() as usize;
        let rng = StdRng::seed_from_u64(seed);

        Self {
            rng,
            total_number_of_features,
            number_of_samples,
        }
    }

    pub fn sample(&mut self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.total_number_of_features).collect();
        indices.partial_shuffle(&mut self.rng, self.number_of_samples);
        indices.truncate(self.number_of_samples);
        indices
    }
}

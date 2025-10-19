//! Node splitting utilities for train/test splits
//! 1:1 translation of NodeSplitter.java

use crate::{
    collections::HugeLongArray,
    ml::splitting::{FractionSplitter, TrainingExamplesSplit},
};
use std::sync::Arc;

/// Result of splitting nodes into training and test sets
/// 1:1 with NodeSplits interface in Java
#[derive(Debug, Clone)]
pub struct NodeSplits {
    pub all_training_examples: Arc<Vec<i64>>,
    pub outer_split: TrainingExamplesSplit,
}

/// Splits nodes into training and test sets
/// 1:1 with NodeSplitter.java
pub struct NodeSplitter {
    number_of_examples: usize,
    to_original_id: Arc<dyn Fn(usize) -> i64 + Send + Sync>,
    to_mapped_id: Arc<dyn Fn(i64) -> usize + Send + Sync>,
}

impl NodeSplitter {
    /// Creates a new node splitter
    pub fn new(
        number_of_examples: usize,
        to_original_id: Arc<dyn Fn(usize) -> i64 + Send + Sync>,
        to_mapped_id: Arc<dyn Fn(i64) -> usize + Send + Sync>,
    ) -> Self {
        Self {
            number_of_examples,
            to_original_id,
            to_mapped_id,
        }
    }

    /// Splits nodes into train/test sets
    /// 1:1 with split() in Java
    pub fn split(
        &self,
        test_fraction: f64,
        validation_folds: usize,
        random_seed: Option<u64>,
    ) -> NodeSplits {
        let mut all_training_examples = HugeLongArray::new(self.number_of_examples);

        // Sort by original IDs for deterministic projections (matches Java)
        all_training_examples.set_all(|i| (self.to_original_id)(i));

        // Simple sequential sort (parallel sort with infrastructure comes later)
        let mut vec: Vec<i64> = (0..self.number_of_examples)
            .map(|i| all_training_examples.get(i))
            .collect();
        vec.sort_unstable();
        for (i, &val) in vec.iter().enumerate() {
            all_training_examples.set(i, val);
        }

        for i in 0..self.number_of_examples {
            let original_id = all_training_examples.get(i);
            let mapped = (self.to_mapped_id)(original_id) as i64;
            all_training_examples.set(i, mapped);
        }

        // Shuffle with seed if provided (matches Java ShuffleUtil)
        if let Some(seed) = random_seed {
            use rand::rngs::StdRng;
            use rand::{Rng, SeedableRng};
            let mut rng = StdRng::seed_from_u64(seed);

            for i in (1..self.number_of_examples).rev() {
                let j = rng.gen_range(0..=i);
                let temp = all_training_examples.get(i);
                all_training_examples.set(i, all_training_examples.get(j));
                all_training_examples.set(j, temp);
            }
        }

        // Convert to Vec<u64> for ReadOnlyHugeLongArray equivalent
        let all_vec: Vec<i64> = (0..self.number_of_examples)
            .map(|i| all_training_examples.get(i))
            .collect();
        let all_examples = Arc::new(all_vec);

        let outer_split = FractionSplitter::split(all_examples.clone(), 1.0 - test_fraction);

        // Warn for small node sets (matches Java)
        self.warn_for_small_node_sets(
            outer_split.train_set().len(),
            outer_split.test_set().len(),
            validation_folds,
        );

        NodeSplits {
            all_training_examples: all_examples,
            outer_split,
        }
    }

    /// Warns if training or test sets are too small
    /// 1:1 with warnForSmallNodeSets in Java
    fn warn_for_small_node_sets(
        &self,
        train_size: usize,
        test_size: usize,
        validation_folds: usize,
    ) {
        // Note: In Java this uses ProgressTracker.logWarning
        // For now we use eprintln until infrastructure is available
        if train_size < 500 || test_size < 100 {
            eprintln!(
                "Warning: Small node sets detected: training={}, test={}. Consider adjusting split fractions.",
                train_size, test_size
            );
        }

        if validation_folds > 0 && train_size / validation_folds < 100 {
            eprintln!(
                "Warning: Small validation fold size: {} nodes per fold. Consider reducing validation_folds.",
                train_size / validation_folds
            );
        }
    }
}

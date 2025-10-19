use crate::core::{
    concurrency::Concurrency,
    parallel::merge_sort::parallel_merge_sort,
    progress::ProgressTracker,
    utils::huge_array::{HugeLongArray, ReadOnlyHugeLongArray},
};
use crate::ml::splitting::{FractionSplitter, TrainingExamplesSplit};
use std::sync::Arc;

/// Result of node splitting containing training examples and outer split
#[derive(Debug)]
pub struct NodeSplits {
    all_training_examples: Arc<ReadOnlyHugeLongArray>,
    outer_split: TrainingExamplesSplit,
}

impl NodeSplits {
    /// Creates new NodeSplits
    pub fn new(
        all_training_examples: Arc<ReadOnlyHugeLongArray>,
        outer_split: TrainingExamplesSplit,
    ) -> Self {
        Self {
            all_training_examples,
            outer_split,
        }
    }

    /// Returns all training examples
    pub fn all_training_examples(&self) -> &Arc<ReadOnlyHugeLongArray> {
        &self.all_training_examples
    }

    /// Returns the outer split
    pub fn outer_split(&self) -> &TrainingExamplesSplit {
        &self.outer_split
    }
}

/// Node splitter for creating training/test splits
pub struct NodeSplitter {
    concurrency: Concurrency,
    number_of_examples: usize,
    progress_tracker: Arc<ProgressTracker>,
    to_original_id: Arc<dyn Fn(usize) -> i64 + Send + Sync>,
    to_mapped_id: Arc<dyn Fn(i64) -> usize + Send + Sync>,
}

impl NodeSplitter {
    /// Creates a new NodeSplitter
    pub fn new(
        concurrency: Concurrency,
        number_of_examples: usize,
        progress_tracker: Arc<ProgressTracker>,
        to_original_id: Arc<dyn Fn(usize) -> i64 + Send + Sync>,
        to_mapped_id: Arc<dyn Fn(i64) -> usize + Send + Sync>,
    ) -> Self {
        Self {
            concurrency,
            number_of_examples,
            progress_tracker,
            to_original_id,
            to_mapped_id,
        }
    }

    /// Splits nodes into training and test sets
    pub fn split(
        &self,
        test_fraction: f64,
        validation_folds: usize,
        random_seed: Option<u64>,
    ) -> NodeSplits {
        let mut all_training_examples = HugeLongArray::new(self.number_of_examples);

        // Sort by original IDs for deterministic behavior across projections
        all_training_examples.set_all(|i| (self.to_original_id)(i));
        parallel_merge_sort(&mut all_training_examples, self.concurrency.clone());
        all_training_examples.set_all(|i| (self.to_mapped_id)(all_training_examples.get(i)));

        // Shuffle with provided seed
        if let Some(seed) = random_seed {
            all_training_examples.shuffle_with_seed(seed);
        } else {
            all_training_examples.shuffle();
        }

        let all_examples = Arc::new(ReadOnlyHugeLongArray::new(all_training_examples));
        let outer_split = FractionSplitter::new().split(all_examples.clone(), 1.0 - test_fraction);

        self.warn_for_small_node_sets(
            outer_split.train_set().len(),
            outer_split.test_set().len(),
            validation_folds,
        );

        NodeSplits::new(all_examples, outer_split)
    }

    fn warn_for_small_node_sets(
        &self,
        train_size: usize,
        test_size: usize,
        validation_folds: usize,
    ) {
        // Warning thresholds based on Java implementation
        if train_size < 500 || test_size < 100 {
            self.progress_tracker.log_warning(format!(
                "Small node sets detected: training={}, test={}. Consider adjusting split fractions.",
                train_size, test_size
            ));
        }

        if validation_folds > 0 && train_size / validation_folds < 100 {
            self.progress_tracker.log_warning(format!(
                "Small validation fold size: {} nodes per fold. Consider reducing validation_folds.",
                train_size / validation_folds
            ));
        }
    }
}

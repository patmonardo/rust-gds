use std::sync::Arc;

/// ReadOnlyHugeLongArray - simple wrapper for array of node IDs
/// Matches Java's ReadOnlyHugeLongArray
pub type ReadOnlyHugeLongArray = Arc<Vec<i64>>;

/// Represents a split of training examples into train and test sets.
/// 1:1 translation of TrainingExamplesSplit from Java GDS.
#[derive(Debug, Clone)]
pub struct TrainingExamplesSplit {
    train_set: ReadOnlyHugeLongArray,
    test_set: ReadOnlyHugeLongArray,
}

impl TrainingExamplesSplit {
    /// Creates a new TrainingExamplesSplit
    pub fn of(train_set: ReadOnlyHugeLongArray, test_set: ReadOnlyHugeLongArray) -> Self {
        Self {
            train_set,
            test_set,
        }
    }

    /// Returns the training set
    pub fn train_set(&self) -> ReadOnlyHugeLongArray {
        self.train_set.clone()
    }

    /// Returns the test set
    pub fn test_set(&self) -> ReadOnlyHugeLongArray {
        self.test_set.clone()
    }
}

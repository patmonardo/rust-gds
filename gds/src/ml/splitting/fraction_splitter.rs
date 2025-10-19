use super::training_examples_split::{ReadOnlyHugeLongArray, TrainingExamplesSplit};

/// Splits data into training and test sets based on a fraction.
/// 1:1 translation of FractionSplitter from Java GDS.
pub struct FractionSplitter;

impl FractionSplitter {
    /// Splits an array into training and test sets
    pub fn split(ids: ReadOnlyHugeLongArray, train_fraction: f64) -> TrainingExamplesSplit {
        let train_size = Self::train_size(ids.len(), train_fraction);
        let test_size = ids.len() - train_size;

        let train = Self::init_array(train_size, |i| ids[i]);
        let test = Self::init_array(test_size, |i| ids[i + train_size]);

        TrainingExamplesSplit::of(train, test)
    }

    fn train_size(node_count: usize, train_fraction: f64) -> usize {
        (node_count as f64 * train_fraction) as usize
    }

    fn init_array<F>(size: usize, transform: F) -> ReadOnlyHugeLongArray
    where
        F: Fn(usize) -> i64,
    {
        let mut array = Vec::with_capacity(size);
        for i in 0..size {
            array.push(transform(i));
        }
        ReadOnlyHugeLongArray::new(array)
    }
}

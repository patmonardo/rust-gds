use super::training_examples_split::{ReadOnlyHugeLongArray, TrainingExamplesSplit};
use rand::{prelude::SliceRandom, rngs::StdRng, SeedableRng};
use std::collections::BTreeSet;
use std::sync::Arc;

/// Splits data into k folds maintaining class distribution.
/// 1:1 translation of StratifiedKFoldSplitter from Java GDS.
///
/// For each distinct class, nodes with that class are divided into k buckets,
/// and for each fold, one bucket is used as test set and the remaining as train set.
pub struct StratifiedKFoldSplitter {
    k: usize,
    ids: ReadOnlyHugeLongArray,
    targets: Box<dyn Fn(i64) -> i64 + Send + Sync>,
    random: StdRng,
    distinct_internal_targets: BTreeSet<i64>,
}

impl StratifiedKFoldSplitter {
    /// Creates a new StratifiedKFoldSplitter
    pub fn new(
        k: usize,
        ids: ReadOnlyHugeLongArray,
        targets: impl Fn(i64) -> i64 + Send + Sync + 'static,
        random_seed: Option<u64>,
        distinct_internal_targets: BTreeSet<i64>,
    ) -> Self {
        let random = match random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        Self {
            k,
            ids,
            targets: Box::new(targets),
            random,
            distinct_internal_targets,
        }
    }

    /// Creates k stratified splits
    pub fn splits(mut self) -> Vec<TrainingExamplesSplit> {
        let node_count = self.ids.len();
        let mut train_sets = Vec::new();
        let mut test_sets = Vec::new();
        let mut train_nodes_added = vec![0; self.k];
        let mut test_nodes_added = vec![0; self.k];

        // Allocate arrays
        self.allocate_arrays(node_count, &mut train_sets, &mut test_sets);

        let mut round_robin_pointer = 0;

        // For each distinct target class
        for current_class in self.distinct_internal_targets {
            // Assign nodes with this class to folds in round-robin fashion
            for offset in 0..self.ids.len() {
                let node_id = self.ids[offset];
                if (self.targets)(node_id) == current_class {
                    let fold = round_robin_pointer % self.k;

                    // Add to test set of this fold
                    test_sets[fold][test_nodes_added[fold]] = node_id;
                    test_nodes_added[fold] += 1;

                    // Add to train sets of all other folds
                    for other_fold in 0..self.k {
                        if other_fold != fold {
                            train_sets[other_fold][train_nodes_added[other_fold]] = node_id;
                            train_nodes_added[other_fold] += 1;
                        }
                    }

                    round_robin_pointer += 1;
                }
            }
        }

        // Shuffle each set and create splits
        (0..self.k)
            .map(|fold| {
                train_sets[fold].shuffle(&mut self.random);
                test_sets[fold].shuffle(&mut self.random);
                TrainingExamplesSplit::of(
                    Arc::new(train_sets[fold].clone()),
                    Arc::new(test_sets[fold].clone()),
                )
            })
            .collect()
    }

    fn allocate_arrays(
        &self,
        node_count: usize,
        train_sets: &mut Vec<Vec<i64>>,
        test_sets: &mut Vec<Vec<i64>>,
    ) {
        let base_bucket_size = node_count / self.k;
        for fold in 0..self.k {
            // Make the first buckets larger when node_count is not divisible by k
            let test_size = if fold < node_count % self.k {
                base_bucket_size + 1
            } else {
                base_bucket_size
            };
            test_sets.push(vec![0; test_size]);
            train_sets.push(vec![0; node_count - test_size]);
        }
    }
}

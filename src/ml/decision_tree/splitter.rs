//! Splitter for finding optimal decision tree splits.
//!
//! Translated from Java GDS ml-algo Splitter.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeLongArray;
use crate::ml::decision_tree::{
    FeatureBagger, Group, Groups, ImpurityCriterion, ImpurityData, Split,
};

/// Placeholder for Features until ml-models is translated.
#[derive(Clone)]
pub struct Features {
    // TODO: Replace with actual Features from ml-models
}

impl Features {
    pub fn get(&self, _idx: usize) -> &[f64] {
        todo!("Implement when Features is available")
    }

    pub fn size(&self) -> usize {
        todo!("Implement when Features is available")
    }
}

pub struct Splitter {
    impurity_criterion: Box<dyn ImpurityCriterion>,
    features: Features,
    feature_bagger: FeatureBagger,
    min_leaf_size: usize,
    sort_cache: HugeLongArray,
    right_impurity_data: Box<dyn ImpurityData>,
}

impl Splitter {
    pub fn new(
        train_set_size: usize,
        impurity_criterion: Box<dyn ImpurityCriterion>,
        feature_bagger: FeatureBagger,
        features: Features,
        min_leaf_size: usize,
    ) -> Self {
        let sort_cache = HugeLongArray::new(train_set_size);
        let right_impurity_data = impurity_criterion.group_impurity(&HugeLongArray::new(0), 0, 0);

        Self {
            impurity_criterion,
            features,
            feature_bagger,
            min_leaf_size,
            sort_cache,
            right_impurity_data,
        }
    }

    pub fn memory_estimation(
        number_of_training_samples: usize,
        size_of_impurity_data: usize,
    ) -> usize {
        std::mem::size_of::<Self>()
            + std::mem::size_of::<i64>() * number_of_training_samples // sort_cache
            + 4 * size_of_impurity_data
            + 4 * std::mem::size_of::<i64>() * number_of_training_samples // child arrays
    }

    pub fn find_best_split(&mut self, group: &Group) -> Split {
        let mut best_idx = -1i32;
        let mut best_value = f64::MAX;
        let mut best_impurity = f64::MAX;
        let mut best_left_group_size = 0usize;

        let mut left_child_array = HugeLongArray::new(group.size());
        let mut right_child_array = HugeLongArray::new(group.size());
        let mut best_left_child_array = HugeLongArray::new(group.size());
        let mut best_right_child_array = HugeLongArray::new(group.size());

        let mut best_left_impurity_data =
            self.impurity_criterion
                .group_impurity(&HugeLongArray::new(0), 0, 0);
        let mut best_right_impurity_data =
            self.impurity_criterion
                .group_impurity(&HugeLongArray::new(0), 0, 0);

        // Initialize right_child_array with all group indices
        for idx in 0..group.size() {
            right_child_array.set(idx, group.array().get(group.start_idx() + idx));
        }
        right_child_array.copy_to(&mut best_right_child_array, group.size());

        let feature_bag = self.feature_bagger.sample();

        for &feature_idx in &feature_bag {
            // Sort by current feature
            self.sort_by_feature(&mut right_child_array, group.size(), feature_idx);

            group
                .impurity_data()
                .copy_to(self.right_impurity_data.as_mut());

            // Move vectors to left until reaching min_leaf_size
            for left_group_size in 1..self.min_leaf_size {
                let splitting_feature_vector_idx =
                    right_child_array.get(left_group_size - 1) as usize;
                left_child_array.set(left_group_size - 1, splitting_feature_vector_idx as i64);
                self.impurity_criterion.decremental_impurity(
                    splitting_feature_vector_idx,
                    self.right_impurity_data.as_mut(),
                );
            }

            let mut left_impurity_data = self.impurity_criterion.group_impurity(
                &left_child_array,
                0,
                self.min_leaf_size - 1,
            );
            let mut found_improvement_with_idx = false;

            // Continue moving and compute combined impurity
            for left_group_size in self.min_leaf_size..=(group.size() - self.min_leaf_size) {
                let splitting_feature_vector_idx =
                    right_child_array.get(left_group_size - 1) as usize;
                left_child_array.set(left_group_size - 1, splitting_feature_vector_idx as i64);

                self.impurity_criterion.incremental_impurity(
                    splitting_feature_vector_idx,
                    left_impurity_data.as_mut(),
                );
                self.impurity_criterion.decremental_impurity(
                    splitting_feature_vector_idx,
                    self.right_impurity_data.as_mut(),
                );

                let combined_impurity = self.impurity_criterion.combined_impurity(
                    left_impurity_data.as_ref(),
                    self.right_impurity_data.as_ref(),
                );

                if combined_impurity < best_impurity {
                    found_improvement_with_idx = true;
                    best_idx = feature_idx as i32;
                    best_value = self.features.get(splitting_feature_vector_idx)[feature_idx];
                    best_impurity = combined_impurity;
                    best_left_group_size = left_group_size;
                    left_impurity_data.copy_to(best_left_impurity_data.as_mut());
                    self.right_impurity_data
                        .copy_to(best_right_impurity_data.as_mut());
                }
            }

            if found_improvement_with_idx {
                std::mem::swap(&mut best_right_child_array, &mut right_child_array);
                std::mem::swap(&mut best_left_child_array, &mut left_child_array);
            }
        }

        Split::new(
            best_idx as usize,
            best_value,
            Groups::new(
                Group::new(
                    best_left_child_array,
                    0,
                    best_left_group_size,
                    best_left_impurity_data,
                ),
                Group::new(
                    best_right_child_array,
                    best_left_group_size,
                    group.size() - best_left_group_size,
                    best_right_impurity_data,
                ),
            ),
        )
    }

    fn sort_by_feature(&mut self, array: &mut HugeLongArray, size: usize, feature_idx: usize) {
        // TODO: Implement HugeSerialIndirectMergeSort when available
        // For now, stub to indicate where sorting happens
        let _ = (array, size, feature_idx, &mut self.sort_cache);
        todo!("Implement indirect merge sort when collections module is complete")
    }
}

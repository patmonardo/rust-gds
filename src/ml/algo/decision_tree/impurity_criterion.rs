//! Impurity criterion trait for decision trees.
//!
//! Translated from Java GDS ml-algo ImpurityCriterion.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeLongArray;
use std::any::Any;

/// Trait for computing impurity of decision tree node splits.
pub trait ImpurityCriterion: Send + Sync {
    /// Compute the impurity of a group of samples.
    fn group_impurity(
        &self,
        group: &HugeLongArray,
        start_idx: usize,
        size: usize,
    ) -> Box<dyn ImpurityData>;

    /// Update impurity data incrementally when adding a feature vector.
    fn incremental_impurity(&self, feature_vector_idx: usize, impurity_data: &mut dyn ImpurityData);

    /// Update impurity data incrementally when removing a feature vector.
    fn decremental_impurity(&self, feature_vector_idx: usize, impurity_data: &mut dyn ImpurityData);

    /// Compute combined weighted impurity of left and right groups.
    fn combined_impurity(
        &self,
        left_impurity_data: &dyn ImpurityData,
        right_impurity_data: &dyn ImpurityData,
    ) -> f64 {
        let total_size = left_impurity_data.group_size() + right_impurity_data.group_size();
        let left_weight = left_impurity_data.group_size() as f64 / total_size as f64;
        let right_weight = right_impurity_data.group_size() as f64 / total_size as f64;
        left_weight * left_impurity_data.impurity() + right_weight * right_impurity_data.impurity()
    }
}

/// Lightweight representation of a decision tree node's impurity.
pub trait ImpurityData: ImpurityDataAny + Send + Sync {
    fn impurity(&self) -> f64;
    fn group_size(&self) -> usize;

    /// Copies all significant data to another ImpurityData instance.
    fn copy_to(&self, target: &mut dyn ImpurityData);
}

/// Helper trait for downcasting ImpurityData implementations.
pub trait ImpurityDataAny {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

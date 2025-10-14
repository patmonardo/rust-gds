//! Gini index impurity criterion for classification.
//!
//! Translated from Java GDS ml-algo GiniIndex.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! NOTE: Implementation awaiting HugeIntArray from meta-macro processor.

use crate::collections::HugeLongArray;
use crate::ml::algo::decision_tree::{ImpurityCriterion, ImpurityData, ImpurityDataAny};
use std::any::Any;

// TODO: Uncomment when HugeIntArray is available
// use crate::collections::HugeIntArray;

pub struct GiniIndex {
    // expected_mapped_labels: HugeIntArray,  // Awaiting HugeIntArray
    #[allow(dead_code)]
    number_of_classes: usize,
}

// Stub implementation until HugeIntArray is available
impl GiniIndex {
    #[allow(dead_code)]
    pub fn memory_estimation(_number_of_training_samples: usize) -> usize {
        todo!("GiniIndex requires HugeIntArray from meta-macro processor")
    }
}

impl ImpurityCriterion for GiniIndex {
    fn group_impurity(
        &self,
        _group: &HugeLongArray,
        _start_index: usize,
        _size: usize,
    ) -> Box<dyn ImpurityData> {
        todo!("GiniIndex requires HugeIntArray from meta-macro processor")
    }

    fn incremental_impurity(
        &self,
        _feature_vector_idx: usize,
        _impurity_data: &mut dyn ImpurityData,
    ) {
        todo!("GiniIndex requires HugeIntArray from meta-macro processor")
    }

    fn decremental_impurity(
        &self,
        _feature_vector_idx: usize,
        _impurity_data: &mut dyn ImpurityData,
    ) {
        todo!("GiniIndex requires HugeIntArray from meta-macro processor")
    }
}

pub struct GiniImpurityData {
    impurity: f64,
    class_counts: Vec<i64>,
    group_size: usize,
}

impl GiniImpurityData {
    pub fn new(impurity: f64, class_counts: Vec<i64>, group_size: usize) -> Self {
        Self {
            impurity,
            class_counts,
            group_size,
        }
    }

    pub fn memory_estimation(number_of_classes: usize) -> usize {
        std::mem::size_of::<Self>() + std::mem::size_of::<i64>() * number_of_classes
    }
}

impl ImpurityData for GiniImpurityData {
    fn impurity(&self) -> f64 {
        self.impurity
    }

    fn group_size(&self) -> usize {
        self.group_size
    }

    fn copy_to(&self, target: &mut dyn ImpurityData) {
        let target_gini = target
            .as_any_mut()
            .downcast_mut::<GiniImpurityData>()
            .expect("Expected GiniImpurityData");
        target_gini.impurity = self.impurity;
        target_gini.group_size = self.group_size;
        target_gini.class_counts.copy_from_slice(&self.class_counts);
    }
}

impl ImpurityDataAny for GiniImpurityData {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

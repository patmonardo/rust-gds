//! Split mean squared error impurity criterion for regression.
//!
//! Translated from Java GDS ml-algo SplitMeanSquaredError.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::{HugeDoubleArray, HugeLongArray};
use crate::ml::algo::decision_tree::{ImpurityCriterion, ImpurityData, ImpurityDataAny};
use std::any::Any;
use std::sync::Arc;

pub struct SplitMeanSquaredError {
    targets: Arc<HugeDoubleArray>,
}

impl SplitMeanSquaredError {
    pub fn new(targets: Arc<HugeDoubleArray>) -> Self {
        Self { targets }
    }

    pub fn memory_estimation() -> usize {
        std::mem::size_of::<Self>()
    }

    fn update_impurity_data(
        sum: f64,
        sum_of_squares: f64,
        group_size: usize,
        mse_impurity_data: &mut MSEImpurityData,
    ) {
        let mean = sum / group_size as f64;
        let mse = sum_of_squares / group_size as f64 - mean * mean;

        mse_impurity_data.set_impurity(mse);
        mse_impurity_data.set_sum(sum);
        mse_impurity_data.set_sum_of_squares(sum_of_squares);
        mse_impurity_data.set_group_size(group_size);
    }
}

impl ImpurityCriterion for SplitMeanSquaredError {
    fn group_impurity(
        &self,
        group: &HugeLongArray,
        start_idx: usize,
        size: usize,
    ) -> Box<dyn ImpurityData> {
        if size == 0 {
            return Box::new(MSEImpurityData::new(0.0, 0.0, 0.0, 0));
        }

        let mut sum = 0.0;
        let mut sum_of_squares = 0.0;
        for i in start_idx..(start_idx + size) {
            let value = self.targets.get(group.get(i) as usize);
            sum += value;
            sum_of_squares += value * value;
        }

        let mean = sum / size as f64;
        let mse = sum_of_squares / size as f64 - mean * mean;

        Box::new(MSEImpurityData::new(mse, sum_of_squares, sum, size))
    }

    fn incremental_impurity(
        &self,
        feature_vector_idx: usize,
        impurity_data: &mut dyn ImpurityData,
    ) {
        let mse_impurity_data = impurity_data
            .as_any_mut()
            .downcast_mut::<MSEImpurityData>()
            .expect("Expected MSEImpurityData");

        let value = self.targets.get(feature_vector_idx);

        let sum = mse_impurity_data.sum() + value;
        let sum_of_squares = mse_impurity_data.sum_of_squares() + value * value;
        let group_size = mse_impurity_data.group_size() + 1;

        Self::update_impurity_data(sum, sum_of_squares, group_size, mse_impurity_data);
    }

    fn decremental_impurity(
        &self,
        feature_vector_idx: usize,
        impurity_data: &mut dyn ImpurityData,
    ) {
        let mse_impurity_data = impurity_data
            .as_any_mut()
            .downcast_mut::<MSEImpurityData>()
            .expect("Expected MSEImpurityData");

        let value = self.targets.get(feature_vector_idx);

        let sum = mse_impurity_data.sum() - value;
        let sum_of_squares = mse_impurity_data.sum_of_squares() - value * value;
        let group_size = mse_impurity_data.group_size() - 1;

        Self::update_impurity_data(sum, sum_of_squares, group_size, mse_impurity_data);
    }
}

pub struct MSEImpurityData {
    impurity: f64,
    sum_of_squares: f64,
    sum: f64,
    group_size: usize,
}

impl MSEImpurityData {
    pub fn new(impurity: f64, sum_of_squares: f64, sum: f64, group_size: usize) -> Self {
        Self {
            impurity,
            sum_of_squares,
            sum,
            group_size,
        }
    }

    pub fn memory_estimation() -> usize {
        std::mem::size_of::<Self>()
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn set_sum(&mut self, sum: f64) {
        self.sum = sum;
    }

    pub fn sum_of_squares(&self) -> f64 {
        self.sum_of_squares
    }

    pub fn set_sum_of_squares(&mut self, sum_of_squares: f64) {
        self.sum_of_squares = sum_of_squares;
    }

    pub fn set_impurity(&mut self, impurity: f64) {
        self.impurity = impurity;
    }

    pub fn set_group_size(&mut self, group_size: usize) {
        self.group_size = group_size;
    }
}

impl ImpurityData for MSEImpurityData {
    fn impurity(&self) -> f64 {
        self.impurity
    }

    fn group_size(&self) -> usize {
        self.group_size
    }

    fn copy_to(&self, target: &mut dyn ImpurityData) {
        let target_mse = target
            .as_any_mut()
            .downcast_mut::<MSEImpurityData>()
            .expect("Expected MSEImpurityData");
        target_mse.impurity = self.impurity;
        target_mse.sum_of_squares = self.sum_of_squares;
        target_mse.sum = self.sum;
        target_mse.group_size = self.group_size;
    }
}

impl ImpurityDataAny for MSEImpurityData {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

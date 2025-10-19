//! Linear Regression objective function.
//!
//! Literal translation of `LinearRegressionObjective.java` from Java GDS.

use crate::collections::HugeDoubleArray;
use crate::ml::core::functions::{
    constant::Constant, constant_scale::ConstantScale, element_sum::ElementSum,
    l2_norm_squared::L2NormSquared, mean_square_error::MeanSquareError,
};
use crate::ml::core::tensor::Tensor;
use crate::ml::gradient_descent::{batch_feature_matrix, Objective};
use crate::ml::models::linear::{data::LinearRegressionData, regressor::LinearRegressor};
use crate::ml::models::Features;
use parking_lot::RwLock;
use std::sync::Arc;

/// Objective used by gradient descent training of linear regression.
pub struct LinearRegressionObjective<'a> {
    features: &'a dyn Features,
    targets: &'a HugeDoubleArray,
    model_data: LinearRegressionData,
    penalty: f64,
}

impl<'a> LinearRegressionObjective<'a> {
    /// Create a new objective wrapping feature and target stores.
    pub fn new(features: &'a dyn Features, targets: &'a HugeDoubleArray, penalty: f64) -> Self {
        let model_data = LinearRegressionData::of(features.feature_dimension());
        Self {
            features,
            targets,
            model_data,
            penalty,
        }
    }

    fn penalty_for_batch<B: crate::ml::core::batch::Batch>(
        &self,
        batch: &B,
        train_size: usize,
    ) -> ConstantScale {
        let penalty_variable = L2NormSquared::new(Box::new(self.model_data.weights().clone()));
        let scale = (batch.size() as f64) * self.penalty / (train_size as f64);
        ConstantScale::new(Box::new(penalty_variable), scale)
    }

    fn batch_targets<B: crate::ml::core::batch::Batch>(&self, batch: &B) -> Constant {
        let mut batched_targets = Vec::with_capacity(batch.size());
        for element_id in batch.element_ids() {
            batched_targets.push(self.targets.get(element_id as usize));
        }
        Constant::vector(batched_targets)
    }

    pub fn penalty(&self) -> f64 {
        self.penalty
    }
}

impl<'a> Objective for LinearRegressionObjective<'a> {
    type ModelData = LinearRegressionData;

    fn weight_handles(&self) -> Vec<Arc<RwLock<Box<dyn Tensor>>>> {
        vec![
            self.model_data.weights().handle(),
            self.model_data.bias().handle(),
        ]
    }

    fn loss<B: crate::ml::core::batch::Batch>(
        &self,
        batch: &B,
        train_size: usize,
    ) -> Box<dyn crate::ml::core::variable::Variable> {
        let batch_features = batch_feature_matrix(batch, self.features);
        let regressor = LinearRegressor::new(self.model_data.clone());
        let predictions = regressor.predictions_variable(Box::new(batch_features));
        let targets = self.batch_targets(batch);

        let mse = MeanSquareError::new(predictions, Box::new(targets));
        let penalty = self.penalty_for_batch(batch, train_size);

        Box::new(ElementSum::new(vec![Box::new(mse), Box::new(penalty)]))
    }

    fn model_data(&self) -> &Self::ModelData {
        &self.model_data
    }
}

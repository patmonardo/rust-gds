// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Linear Regression Trainer.
//!
//! Stub for 1:1 translation of LinearRegressionTrainer.java from Java GDS.
//! Full implementation requires gradient descent infrastructure (Training, Objective, etc.)

use super::linear_regression::{LinearRegressionConfig, LinearRegressionData, LinearRegressor};
use crate::collections::HugeDoubleArray;
use crate::ml::core::tensor::Vector;
use crate::ml::models::Features;
use std::convert::TryFrom;
use std::sync::Arc;

/// ReadOnlyHugeLongArray type alias
type ReadOnlyHugeLongArray = Arc<Vec<u64>>;

/// Linear Regression Trainer (STUB).
/// 1:1 API from LinearRegressionTrainer.java from Java GDS.
///
/// TODO: Implement using gradient descent infrastructure:
/// - LinearRegressionObjective (loss function)
/// - Training (gradient descent runner)
/// - BatchQueue (mini-batch processing)
pub struct LinearRegressionTrainer {
    config: LinearRegressionConfig,
}

impl LinearRegressionTrainer {
    pub fn new(config: LinearRegressionConfig) -> Self {
        Self { config }
    }

    /// Train a linear regression model.
    /// 1:1 signature from LinearRegressionTrainer.train() in Java GDS.
    ///
    /// # Arguments
    /// * `features` - Feature vectors
    /// * `targets` - Target values
    /// * `train_set` - Training set node indices
    pub fn train(
        &self,
        features: &dyn Features,
        targets: &HugeDoubleArray,
        train_set: &ReadOnlyHugeLongArray,
    ) -> LinearRegressor {
        let feature_dimension = features.feature_dimension();

        if feature_dimension == 0 || train_set.is_empty() {
            return LinearRegressor::new(LinearRegressionData::of(feature_dimension));
        }

        let learning_rate = self.config.learning_rate;
        let penalty = self.config.penalty;
        let batch_size = self.config.batch_size.max(1);

        let mut weights = vec![0.0; feature_dimension];
        let mut bias = 0.0;

        for _ in 0..self.config.max_iterations {
            let mut weight_gradient = vec![0.0; feature_dimension];
            let mut bias_gradient = 0.0;
            let mut processed_in_batch = 0;

            for &id in train_set.iter() {
                let idx = usize::try_from(id).expect("training id does not fit usize");

                let sample_features = features.get(idx);
                let target_value = targets.get(idx);

                let mut prediction = bias;
                for (w, &x) in weights.iter().zip(sample_features.iter()) {
                    prediction += w * x;
                }

                let error = prediction - target_value;

                for (grad, &x) in weight_gradient.iter_mut().zip(sample_features.iter()) {
                    *grad += error * x;
                }
                bias_gradient += error;
                processed_in_batch += 1;

                if processed_in_batch == batch_size {
                    Self::apply_update(
                        &mut weights,
                        &mut weight_gradient,
                        &mut bias,
                        &mut bias_gradient,
                        processed_in_batch,
                        learning_rate,
                        penalty,
                    );
                    processed_in_batch = 0;
                }
            }

            if processed_in_batch > 0 {
                Self::apply_update(
                    &mut weights,
                    &mut weight_gradient,
                    &mut bias,
                    &mut bias_gradient,
                    processed_in_batch,
                    learning_rate,
                    penalty,
                );
            }
        }

        let data = LinearRegressionData {
            weights: Vector::new(weights),
            bias,
            num_features: feature_dimension,
        };

        LinearRegressor::new(data)
    }
}

impl LinearRegressionTrainer {
    fn apply_update(
        weights: &mut [f64],
        gradient: &mut [f64],
        bias: &mut f64,
        bias_gradient: &mut f64,
        batch_count: usize,
        learning_rate: f64,
        penalty: f64,
    ) {
        let inv_batch = 1.0 / batch_count as f64;

        for (weight, grad_sum) in weights.iter_mut().zip(gradient.iter_mut()) {
            let regularized_grad = (*grad_sum * inv_batch) + (penalty * *weight);
            *weight -= learning_rate * regularized_grad;
            *grad_sum = 0.0;
        }

        let bias_grad = *bias_gradient * inv_batch;
        *bias -= learning_rate * bias_grad;
        *bias_gradient = 0.0;
    }
}

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

//! Linear Regression implementation.
//!
//! 1:1 translation of LinearRegressor.java and LinearRegressionData.java from Java GDS.

use crate::ml::core::tensor::Vector;
use crate::ml::models::{BaseModelData, ModelData, Regressor, RegressorData, TrainingMethod};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Linear regression model.
/// 1:1 translation of LinearRegressor.java from Java GDS.
#[derive(Debug)]
pub struct LinearRegressor {
    data: LinearRegressionData,
}

impl LinearRegressor {
    pub fn new(data: LinearRegressionData) -> Self {
        Self { data }
    }

    pub fn weights(&self) -> &Vector {
        &self.data.weights
    }

    pub fn bias(&self) -> f64 {
        self.data.bias
    }
}

impl Regressor for LinearRegressor {
    fn data(&self) -> &dyn RegressorData {
        &self.data
    }

    /// Predict a single value for given features
    /// 1:1 with Regressor.predict() in Java
    fn predict(&self, features: &[f64]) -> f64 {
        if features.len() != self.data.weights.len() {
            panic!(
                "Feature dimension mismatch: got {}, expected {}",
                features.len(),
                self.data.weights.len()
            );
        }

        // prediction = weights · features + bias
        let mut prediction = 0.0;
        for i in 0..self.data.num_features {
            prediction += self.data.weights[i] * features[i];
        }
        prediction += self.data.bias;

        prediction
    }
}

/// Linear Regression Model Data.
/// 1:1 translation of LinearRegressionData.java from Java GDS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearRegressionData {
    pub weights: Vector,
    pub bias: f64,
    pub num_features: usize,
}

impl BaseModelData for LinearRegressionData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::LinearRegression
    }

    fn feature_dimension(&self) -> usize {
        self.num_features
    }
}

impl RegressorData for LinearRegressionData {}

impl LinearRegressionData {
    /// Create new LinearRegressionData with given feature dimension.
    /// 1:1 translation of LinearRegressionData.of() from Java GDS.
    pub fn of(feature_dimension: usize) -> Self {
        Self {
            weights: Vector::zeros(feature_dimension),
            bias: 0.0,
            num_features: feature_dimension,
        }
    }
}

impl ModelData for LinearRegressionData {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization failed: {}", e))
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization failed: {}", e))
    }

    fn num_features(&self) -> usize {
        self.num_features
    }
}

/// Linear Regression Training Configuration.
/// 1:1 translation of LinearRegressionTrainConfig.java from Java GDS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearRegressionConfig {
    /// L2 regularization penalty
    #[serde(default)]
    pub penalty: f64,

    /// Maximum number of iterations
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Learning rate
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f64,

    /// Batch size for gradient descent
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

impl Default for LinearRegressionConfig {
    fn default() -> Self {
        Self {
            penalty: 0.0,
            max_iterations: default_max_iterations(),
            learning_rate: default_learning_rate(),
            batch_size: default_batch_size(),
        }
    }
}

fn default_max_iterations() -> usize {
    100
}

fn default_learning_rate() -> f64 {
    0.001
}

fn default_batch_size() -> usize {
    100
}

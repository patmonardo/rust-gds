//! Linear Regression model data structure.
//!
//! 1:1 translation of `LinearRegressionData.java` from Java GDS.

use crate::ml::{
    core::{
        functions::weights::Weights,
        tensor::{Matrix, Scalar, Tensor},
    },
    models::{BaseModelData, ModelData, RegressorData},
    TrainingMethod,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Stored parameters for a trained linear regression model.
#[derive(Clone)]
pub struct LinearRegressionData {
    weights: Weights,
    bias: Weights,
    feature_dimension: usize,
}

impl std::fmt::Debug for LinearRegressionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinearRegressionData")
            .field("feature_dimension", &self.feature_dimension)
            .finish()
    }
}

impl LinearRegressionData {
    /// Factory method mirroring `LinearRegressionData.of(int featureDimension)` in Java.
    pub fn of(feature_dimension: usize) -> Self {
        Self {
            weights: Weights::of_matrix(1, feature_dimension),
            bias: Weights::of_scalar(0.0),
            feature_dimension,
        }
    }

    /// Trainable weight matrix (shape 1 × feature_dimension).
    pub fn weights(&self) -> &Weights {
        &self.weights
    }

    /// Trainable bias scalar.
    pub fn bias(&self) -> &Weights {
        &self.bias
    }
}

impl BaseModelData for LinearRegressionData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::LinearRegression
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }
}

impl RegressorData for LinearRegressionData {}

impl ModelData for LinearRegressionData {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let weight_snapshot = self.snapshot_weights();
        let bias_snapshot = self.snapshot_bias();

        let payload = LinearRegressionDataSnapshot {
            weights: weight_snapshot.data,
            rows: weight_snapshot.rows,
            cols: weight_snapshot.cols,
            bias: bias_snapshot,
            feature_dimension: self.feature_dimension,
        };

        bincode::serialize(&payload)
            .map_err(|err| anyhow!("LinearRegressionData serialization failed: {err}"))
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let snapshot: LinearRegressionDataSnapshot = bincode::deserialize(bytes)
            .map_err(|err| anyhow!("LinearRegressionData deserialization failed: {err}"))?;

        let weights_matrix = Matrix::new(snapshot.weights, snapshot.rows, snapshot.cols);
        let bias_scalar = Scalar::new(snapshot.bias);

        Ok(Self {
            weights: Weights::from_tensor(Box::new(weights_matrix)),
            bias: Weights::from_tensor(Box::new(bias_scalar)),
            feature_dimension: snapshot.feature_dimension,
        })
    }

    fn num_features(&self) -> usize {
        self.feature_dimension
    }
}

#[derive(Serialize, Deserialize)]
struct LinearRegressionDataSnapshot {
    weights: Vec<f64>,
    rows: usize,
    cols: usize,
    bias: f64,
    feature_dimension: usize,
}

struct WeightSnapshot {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl LinearRegressionData {
    fn snapshot_weights(&self) -> WeightSnapshot {
        let matrix_ref = self.weights.borrow_matrix();
        WeightSnapshot {
            data: matrix_ref.data().to_vec(),
            rows: matrix_ref.rows(),
            cols: matrix_ref.cols(),
        }
    }

    fn snapshot_bias(&self) -> f64 {
        let scalar_ref = self.bias.borrow_scalar();
        scalar_ref.value()
    }
}

//! Linear Regressor implementation.
//!
//! Direct translation of `LinearRegressor.java` from Java GDS.

use crate::ml::core::functions::{
    ewise_add_matrix_scalar::EWiseAddMatrixScalar,
    matrix_multiply_with_transposed_second_operand::MatrixMultiplyWithTransposedSecondOperand,
};
use crate::ml::core::variable::Variable;
use crate::ml::models::Regressor;

use super::data::LinearRegressionData;

/// Prediction component for linear regression.
#[derive(Debug, Clone)]
pub struct LinearRegressor {
    data: LinearRegressionData,
}

impl LinearRegressor {
    /// Create a regressor backed by the supplied model data.
    pub fn new(data: LinearRegressionData) -> Self {
        Self { data }
    }

    /// Accessor for the underlying model data.
    pub fn data(&self) -> &LinearRegressionData {
        &self.data
    }

    /// Build the computation-graph variable producing predictions for a batch of features.
    /// Mirrors `LinearRegressor.predictionsVariable(Variable<Matrix> features)`.
    /// Returns a Variable<Matrix> containing predictions for the batch.
    pub fn predictions_variable(&self, features: Box<dyn Variable>) -> Box<dyn Variable> {
        let weighted_features = MatrixMultiplyWithTransposedSecondOperand::new(
            features,
            Box::new(self.data.weights().clone()),
        );

        Box::new(EWiseAddMatrixScalar::new(
            Box::new(weighted_features),
            Box::new(self.data.bias().clone()),
        ))
    }
}

impl Regressor for LinearRegressor {
    fn data(&self) -> &dyn crate::ml::models::RegressorData {
        &self.data
    }

    fn predict(&self, features: &[f64]) -> f64 {
        let matrix_ref = self.data.weights().borrow_matrix();
        let bias_ref = self.data.bias().borrow_scalar();

        let mut prediction = 0.0;
        for (i, &feature) in features.iter().enumerate() {
            prediction += matrix_ref[(0, i)] * feature;
        }

        prediction + bias_ref.value()
    }
}

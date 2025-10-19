//! Mean square error function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions MeanSquareError.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Scalar, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::abstract_variable::AbstractVariable;
use std::fmt;

/// Mean square error loss function.
///
/// Computes MSE between predictions and targets.
/// Corresponds to MeanSquareError in Java GDS.
/// Uses composition pattern: VariableBase holds parents [predictions, targets].
pub struct MeanSquareError {
    base: AbstractVariable,
}

impl MeanSquareError {
    pub fn new(predictions: Box<dyn Variable>, targets: Box<dyn Variable>) -> Self {
        Self::validate_dimensions(predictions.as_ref(), targets.as_ref());

        let parents = vec![predictions, targets];
        let dimensions = dimensions::scalar();
        let base = AbstractVariable::with_gradient_requirement(parents, dimensions, true);

        Self { base }
    }

    /// Helper to access predictions parent (index 0)
    fn predictions(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Helper to access targets parent (index 1)
    fn targets(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }

    fn validate_dimensions(predictions: &dyn Variable, targets: &dyn Variable) {
        let pred_size = dimensions::total_size(predictions.dimensions());
        let target_size = dimensions::total_size(targets.dimensions());

        if pred_size != target_size {
            panic!(
                "Targets and predictions must be of equal size. Got predictions: {}, targets: {}",
                dimensions::render(predictions.dimensions()),
                dimensions::render(targets.dimensions())
            );
        }
    }
}

impl Variable for MeanSquareError {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let predicted_data = ctx
            .data(self.predictions())
            .expect("Predictions not computed");

        let target_data = ctx.data(self.targets()).expect("Targets not computed");

        let mut sum_of_squares = 0.0;
        let length = predicted_data.total_size();

        for i in 0..length {
            let error = predicted_data.data()[i] - target_data.data()[i];
            sum_of_squares += error * error;
        }

        if !sum_of_squares.is_finite() {
            return Box::new(Scalar::new(f64::MAX));
        }

        Box::new(Scalar::new(sum_of_squares / length as f64))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_data = ctx.data(parent).expect("Parent data not computed");

        let other_parent_data = if std::ptr::eq(parent, self.predictions()) {
            ctx.data(self.targets()).expect("Targets not computed")
        } else {
            ctx.data(self.predictions())
                .expect("Predictions not computed")
        };

        let length = parent_data.data().len();

        let self_gradient = ctx
            .gradient(self)
            .expect("Self gradient not computed")
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let scale = 2.0 * self_gradient / length as f64;

        // Create gradient data vector
        let mut gradient_data = Vec::new();
        for i in 0..length {
            let error = parent_data.data()[i] - other_parent_data.data()[i];
            gradient_data.push(scale * error);
        }

        // Create gradient tensor based on parent type
        if let Some(matrix) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Matrix>() {
            Box::new(crate::ml::core::tensor::Matrix::new(gradient_data, matrix.rows(), matrix.cols())) as Box<dyn Tensor>
        } else if let Some(vector) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Vector>() {
            Box::new(crate::ml::core::tensor::Vector::new(gradient_data)) as Box<dyn Tensor>
        } else if let Some(scalar) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Scalar>() {
            Box::new(crate::ml::core::tensor::Scalar::new(gradient_data[0])) as Box<dyn Tensor>
        } else {
            panic!("Unknown tensor type");
        }
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for MeanSquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MeanSquareError")
    }
}

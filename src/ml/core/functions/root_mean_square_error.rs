//! Root mean square error function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions RootMeanSquareError.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::abstract_variable::AbstractVariable;
use std::fmt;

/// Root mean square error loss function.
///
/// Computes RMSE between predictions and targets.
/// Corresponds to RootMeanSquareError in Java GDS.
/// Uses composition pattern: VariableBase holds parents [predictions, targets].
pub struct RootMeanSquareError {
    base: AbstractVariable,
}

impl RootMeanSquareError {
    pub fn new(predictions: Box<dyn Variable>, targets: Box<dyn Variable>) -> Self {
        assert!(
            dimensions::is_vector(predictions.dimensions()),
            "Predictions need to be a vector"
        );
        assert!(dimensions::total_size(predictions.dimensions()) > 0);
        assert_eq!(
            dimensions::total_size(predictions.dimensions()),
            dimensions::total_size(targets.dimensions()),
            "Predictions and targets need to have the same total size"
        );

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
}

impl Variable for RootMeanSquareError {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let predictions_data = ctx
            .data(self.predictions())
            .expect("Predictions not computed");
        let predictions = predictions_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let targets_data = ctx.data(self.targets()).expect("Targets not computed");
        let targets = targets_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let mut squared_error_sum = 0.0;
        for idx in 0..targets.length() {
            // predictions is Matrix, targets is Vector - both use data_at(idx) for 1D access
            let error = predictions.data_at(idx, 0) - targets.data_at(idx);
            squared_error_sum += error * error;
        }

        if !squared_error_sum.is_finite() {
            return Box::new(Scalar::new(f64::MAX));
        }

        Box::new(Scalar::new(
            (squared_error_sum / targets.length() as f64).sqrt(),
        ))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.predictions()) {
            let predictions_data = ctx
                .data(self.predictions())
                .expect("Predictions not computed");
            let predictions = predictions_data
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("Predictions must be Matrix");

            let targets_data = ctx.data(self.targets()).expect("Targets not computed");
            let number_of_examples = targets_data
                .as_any()
                .downcast_ref::<Vector>()
                .expect("Targets must be Vector")
                .length();

            let self_data = ctx.data(self).expect("Self data not computed");
            let root_of_sum_of_square_error_over_n = self_data
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Self data must be Scalar");

            let parent_data = ctx.data(parent).expect("Parent data not computed");

            if root_of_sum_of_square_error_over_n.value() == 0.0 {
                return Box::new(Vector::with_size(number_of_examples));
            }

            let denominator = root_of_sum_of_square_error_over_n
                .scalar_multiply(number_of_examples as f64)
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Result must be Scalar")
                .value();

            let self_gradient_data = ctx.gradient(self).expect("Self gradient not computed");
            let self_gradient = self_gradient_data
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Self gradient must be Scalar")
                .value();

            let scale = self_gradient / denominator;

            let targets = targets_data
                .as_any()
                .downcast_ref::<Vector>()
                .expect("Targets must be Vector");

            // Create gradient data vector
            let mut gradient_data = Vec::new();
            for idx in 0..number_of_examples {
                // predictions is Matrix, targets is Vector
                let error = predictions.data_at(idx, 0) - targets.data_at(idx);
                gradient_data.push(error * scale);
            }

            return Box::new(Vector::new(gradient_data));
        }

        panic!(
            "The gradient should only be computed for the prediction parent, but got {}",
            parent
        );
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

impl fmt::Display for RootMeanSquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RootMeanSquareError")
    }
}

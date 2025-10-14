//! Logistic loss function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions LogisticLoss.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

/// Logistic loss function combining logistic regression prediction and cross-entropy loss.
///
/// This variable represents the composition of the logistic regression model's prediction function
/// and cross-entropy loss. This therefore represents a function from weights, features and targets
/// to a scalar loss value. Compared to using CrossEntropyLoss variable, composed with predictions from
/// the model, this variable does not register the predictions as a parent in the computation graph.
/// Rather, the gradient method directly computes the loss gradient for the weights and circumvents
/// the loss gradient for the predictions variable.
/// Another advantage of using LogisticLoss is that the expression for the gradient for the weights is
/// much simpler than the gradient obtained by back-propagating through the predictions variable.
/// In a compact form this gradient expression is just '(predictions - targets) * features'.
///
/// Uses composition pattern: VariableBase holds parents [weights, features, targets] or [weights, bias, features, targets].
/// Note: predictions is NOT a parent (graph optimization - bypasses predictions node).
pub struct LogisticLoss {
    base: VariableBase,
    predictions: Box<dyn Variable>, // NOT a parent - graph optimization
    has_bias: bool,                 // Track if bias is included
}

impl LogisticLoss {
    pub fn new(
        weights: Box<dyn Variable>,
        predictions: Box<dyn Variable>,
        features: Box<dyn Variable>,
        targets: Box<dyn Variable>,
    ) -> Self {
        Self::validate_vector_dimensions(
            weights.dimensions(),
            features.dimension(dimensions::COLUMNS_INDEX),
        );
        Self::validate_vector_dimensions(
            predictions.dimensions(),
            features.dimension(dimensions::ROWS_INDEX),
        );
        Self::validate_vector_dimensions(
            targets.dimensions(),
            features.dimension(dimensions::ROWS_INDEX),
        );

        // Parents: [weights, features, targets] - NOT predictions (graph optimization)
        let parents = vec![weights, features, targets];
        let base = VariableBase::new(parents, dimensions::scalar());

        Self {
            base,
            predictions,
            has_bias: false,
        }
    }

    pub fn new_with_bias(
        weights: Box<dyn Variable>,
        bias: Box<dyn Variable>,
        predictions: Box<dyn Variable>,
        features: Box<dyn Variable>,
        targets: Box<dyn Variable>,
    ) -> Self {
        Self::validate_vector_dimensions(
            weights.dimensions(),
            features.dimension(dimensions::COLUMNS_INDEX),
        );
        Self::validate_vector_dimensions(
            predictions.dimensions(),
            features.dimension(dimensions::ROWS_INDEX),
        );
        Self::validate_vector_dimensions(
            targets.dimensions(),
            features.dimension(dimensions::ROWS_INDEX),
        );

        // Parents: [weights, bias, features, targets] - NOT predictions (graph optimization)
        let parents = vec![weights, bias, features, targets];
        let base = VariableBase::new(parents, dimensions::scalar());

        Self {
            base,
            predictions,
            has_bias: true,
        }
    }

    /// Helper to access weights parent
    /// Without bias: index 0
    /// With bias: index 0
    fn weights(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Helper to access bias parent (only when has_bias = true)
    /// With bias: index 1
    fn bias(&self) -> &dyn Variable {
        assert!(self.has_bias, "Bias not available");
        self.base.parents()[1].as_ref()
    }

    /// Helper to access features parent
    /// Without bias: index 1
    /// With bias: index 2
    fn features(&self) -> &dyn Variable {
        let idx = if self.has_bias { 2 } else { 1 };
        self.base.parents()[idx].as_ref()
    }

    /// Helper to access targets parent
    /// Without bias: index 2
    /// With bias: index 3
    fn targets(&self) -> &dyn Variable {
        let idx = if self.has_bias { 3 } else { 2 };
        self.base.parents()[idx].as_ref()
    }

    fn validate_vector_dimensions(dimensions: &[usize], vector_length: usize) {
        if !dimensions::is_vector(dimensions) || dimensions::total_size(dimensions) != vector_length
        {
            panic!(
                "Expected a vector of size {}. Got {}",
                vector_length,
                dimensions::total_size(dimensions)
            );
        }
    }

    fn gradient_for_weights(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_gradient = ctx
            .gradient(self)
            .expect("Self gradient not computed")
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        ctx.forward(self.predictions.as_ref());

        let pred_data = ctx
            .data(self.predictions.as_ref())
            .expect("Predictions not computed");
        let pred_vector = pred_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let target_data = ctx.data(self.targets()).expect("Targets not computed");
        let target_vector = target_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let weights_data = ctx.data(self.weights()).expect("Weights not computed");
        let weights_vector = weights_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Weights must be Matrix");

        let features_data = ctx.data(self.features()).expect("Features not computed");
        let features_tensor = features_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Features must be Matrix");

        let feature_count = weights_vector.cols();
        let rows = weights_vector.rows();
        let mut gradient = Matrix::with_dimensions(rows, feature_count);
        let number_of_examples = target_vector.length();

        for idx in 0..number_of_examples {
            // pred_vector is Matrix, so use (row, col) indexing
            let error_per_example = (pred_vector.data_at(idx, 0) - target_vector.data_at(idx))
                / number_of_examples as f64;
            for feature in 0..feature_count {
                gradient.add_data_at(
                    0,
                    feature,
                    self_gradient * error_per_example * features_tensor.data_at(idx, feature),
                );
            }
        }

        Box::new(gradient)
    }

    fn gradient_for_bias(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_gradient_data = ctx.gradient(self).expect("Self gradient not computed");
        let self_gradient = self_gradient_data
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        ctx.forward(self.predictions.as_ref());

        let pred_data = ctx
            .data(self.predictions.as_ref())
            .expect("Predictions not computed");
        let pred_vector = pred_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let target_data = ctx.data(self.targets()).expect("Targets not computed");
        let target_vector = target_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let mut gradient = Box::new(Scalar::new(0.0));
        let number_of_examples = target_vector.length();

        for idx in 0..number_of_examples {
            // pred_vector is Matrix, so use (row, col) indexing
            let error_per_example = pred_vector.data_at(idx, 0) - target_vector.data_at(idx);
            let current = gradient.value();
            gradient.set_data_at(0, current + self_gradient * error_per_example);
        }

        gradient.scalar_multiply_mutate(1.0 / number_of_examples as f64);
        gradient
    }
}

impl Variable for LogisticLoss {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        ctx.forward(self.predictions.as_ref());

        let pred_tensor = ctx
            .data(self.predictions.as_ref())
            .expect("Predictions not computed");
        let pred_vector = pred_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let target_tensor = ctx.data(self.targets()).expect("Targets not computed");
        let target_vector = target_tensor
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let number_of_examples = target_vector.length();

        let mut result = 0.0;
        for idx in 0..number_of_examples {
            // pred_vector is Matrix, so use (row, col) indexing
            let predicted = pred_vector.data_at(idx, 0);
            let target = target_vector.data_at(idx);
            let v1 = target * predicted.ln();
            let v2 = (1.0 - target) * (1.0 - predicted).ln();

            if predicted == 0.0 {
                result += v2;
            } else if predicted == 1.0 {
                result += v1;
            } else {
                result += v1 + v2;
            }
        }

        Box::new(Scalar::new(-result / number_of_examples as f64))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.weights()) {
            self.gradient_for_weights(ctx)
        } else if self.has_bias && std::ptr::eq(parent, self.bias()) {
            self.gradient_for_bias(ctx)
        } else {
            panic!(
                "The gradient should only be computed for the bias and the weights parents, but got {}",
                parent
            );
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

impl fmt::Display for LogisticLoss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LogisticLoss")
    }
}

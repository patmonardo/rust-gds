//! Reduced focal loss function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ReducedFocalLoss.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::abstract_variable::AbstractVariable;
use std::fmt;

/// Computes focal loss given weights, bias, predictions, features and labels,
/// where it is assumed that predictions contain only values for all classes but the last one,
/// in practice, the output of ReducedSoftmax.
///
/// Corresponds to ReducedFocalLoss in Java GDS, extends ReducedCrossEntropyLoss.
/// Uses composition pattern: VariableBase holds parents [weights, features, labels, bias].
/// Note: predictions is NOT a parent for gradient tracking.
pub struct ReducedFocalLoss {
    base: AbstractVariable,
    predictions: Box<dyn Variable>,
    focus_weight: f64,
    class_weights: Vec<f64>,
}

impl ReducedFocalLoss {
    pub fn new(
        predictions: Box<dyn Variable>,
        weights: Box<dyn Variable>,
        bias: Box<dyn Variable>,
        features: Box<dyn Variable>,
        labels: Box<dyn Variable>,
        focus_weight: f64,
        class_weights: Vec<f64>,
    ) -> Self {
        // Parents are [weights, features, labels, bias] - NOT predictions
        let parents = vec![weights, features, labels, bias];
        let dimensions = dimensions::scalar();
        let base = AbstractVariable::with_gradient_requirement(parents, dimensions, true);

        Self {
            base,
            predictions,
            focus_weight,
            class_weights,
        }
    }

    /// Helper to access weights parent (index 0)
    fn weights(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Helper to access features parent (index 1)
    fn features(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }

    /// Helper to access labels parent (index 2)
    fn labels(&self) -> &dyn Variable {
        self.base.parents()[2].as_ref()
    }

    /// Helper to access bias parent (index 3)
    fn bias(&self) -> &dyn Variable {
        self.base.parents()[3].as_ref()
    }

    pub fn size_in_bytes() -> usize {
        crate::ml::core::tensor::size_in_bytes(&[1])
    }

    fn compute_individual_loss(
        &self,
        predicted_probability_for_true_class: f64,
        true_class: usize,
    ) -> f64 {
        let focal_factor = (1.0 - predicted_probability_for_true_class).powf(self.focus_weight);
        self.class_weights[true_class] * focal_factor * predicted_probability_for_true_class.ln()
    }

    fn compute_error_per_example(
        &self,
        number_of_examples: usize,
        predicted_class_probability: f64,
        indicator_is_true_class: f64,
        predicted_probability_for_true_class: f64,
        true_class: usize,
    ) -> f64 {
        let predicted_probability_for_wrong_classes = 1.0 - predicted_probability_for_true_class;
        let chain_rule_gradient =
            predicted_probability_for_wrong_classes.powf(self.focus_weight - 1.0);

        

        self.class_weights[true_class]
            * (self.focus_weight * chain_rule_gradient * predicted_probability_for_true_class.ln()
                - chain_rule_gradient * predicted_probability_for_wrong_classes
                    / predicted_probability_for_true_class)
            * (predicted_probability_for_true_class
                * (indicator_is_true_class - predicted_class_probability))
            / number_of_examples as f64
    }

    fn gradient_for_weights(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let pred_matrix = ctx.forward(self.predictions.as_ref());
        let pred_matrix = pred_matrix
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let labels_data = ctx.data(self.labels()).expect("Labels not computed");
        let labels_vector = labels_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Labels must be Vector");

        let number_of_examples = labels_vector.length();

        let self_gradient_data = ctx.gradient(self).expect("Self gradient not computed");
        let self_gradient = self_gradient_data
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let weights_data = ctx.data(self.weights()).expect("Weights not computed");
        let weights_matrix = weights_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Weights must be Matrix");

        let features_data = ctx.data(self.features()).expect("Features not computed");
        let feature_matrix = features_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Features must be Matrix");

        let feature_count = weights_matrix.cols();
        let reduced_class_count = weights_matrix.rows();
        let mut gradient = Matrix::with_dimensions(reduced_class_count, feature_count);

        for row in 0..number_of_examples {
            let true_class = labels_vector.data_at(row) as usize;
            for class_idx in 0..reduced_class_count {
                let predicted_class_probability = pred_matrix.data_at(row, class_idx);
                let predicted_probability_for_true_class = pred_matrix.data_at(row, true_class);
                let indicator_is_true_class = if true_class == class_idx { 1.0 } else { 0.0 };
                let error_per_example = self.compute_error_per_example(
                    number_of_examples,
                    predicted_class_probability,
                    indicator_is_true_class,
                    predicted_probability_for_true_class,
                    true_class,
                );
                for feature in 0..feature_count {
                    let grad_contribution =
                        self_gradient * error_per_example * feature_matrix.data_at(row, feature);
                    gradient.add_data_at(class_idx, feature, grad_contribution);
                }
            }
        }

        Box::new(gradient)
    }

    fn gradient_for_bias(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let pred_matrix = ctx.forward(self.predictions.as_ref());
        let pred_matrix = pred_matrix
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let labels_data = ctx.data(self.labels()).expect("Labels not computed");
        let labels_vector = labels_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Labels must be Vector");

        let number_of_examples = labels_vector.length();

        let self_gradient_data = ctx.gradient(self).expect("Self gradient not computed");
        let self_gradient = self_gradient_data
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let bias_data = ctx.data(self.bias()).expect("Bias not computed");
        let bias_vector = bias_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Bias must be Vector");

        let reduced_class_count = bias_vector.total_size();
        let mut gradient = Vector::with_size(reduced_class_count);

        for row in 0..number_of_examples {
            let true_class = labels_vector.data_at(row) as usize;
            for class_idx in 0..reduced_class_count {
                let predicted_class_probability = pred_matrix.data_at(row, class_idx);
                let predicted_probability_for_true_class = pred_matrix.data_at(row, true_class);
                let indicator_is_true_class = if true_class == class_idx { 1.0 } else { 0.0 };
                let error_per_example = self.compute_error_per_example(
                    number_of_examples,
                    predicted_class_probability,
                    indicator_is_true_class,
                    predicted_probability_for_true_class,
                    true_class,
                );
                let current = gradient.data()[class_idx];
                gradient.set_data_at(class_idx, current + self_gradient * error_per_example);
            }
        }

        Box::new(gradient)
    }
}

impl Variable for ReducedFocalLoss {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let predictions_tensor = ctx.forward(self.predictions.as_ref());
        let predictions_matrix = predictions_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let labels_tensor = ctx.data(self.labels()).expect("Labels not computed");
        let labels_vector = labels_tensor
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Labels must be Vector");

        let mut result = 0.0;
        for row in 0..labels_vector.total_size() {
            let true_class = labels_vector.data_at(row) as usize;
            let predicted_probability_for_true_class = predictions_matrix.data_at(row, true_class);
            if predicted_probability_for_true_class > 0.0 {
                result +=
                    self.compute_individual_loss(predicted_probability_for_true_class, true_class);
            }
        }

        Box::new(Scalar::new(-result / predictions_matrix.rows() as f64))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.weights()) {
            self.gradient_for_weights(ctx)
        } else if std::ptr::eq(parent, self.bias()) {
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

impl fmt::Display for ReducedFocalLoss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReducedFocalLoss(focusWeight={})", self.focus_weight)
    }
}

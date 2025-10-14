//! Focal loss function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions FocalLoss.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

const PREDICTED_PROBABILITY_THRESHOLD: f64 = 1e-50;

/// Focal loss function - a variant of cross entropy loss that focuses learning on hard examples.
///
/// Corresponds to FocalLoss in Java GDS, extends CrossEntropyLoss.
/// Uses composition pattern: VariableBase holds parents (predictions, targets).
pub struct FocalLoss {
    base: VariableBase,
    class_weights: Vec<f64>,
    focus_weight: f64,
}

impl FocalLoss {
    pub fn new(
        predictions: Box<dyn Variable>,
        targets: Box<dyn Variable>,
        focus_weight: f64,
        class_weights: Vec<f64>,
    ) -> Self {
        let parents = vec![predictions, targets];
        let dimensions = vec![1];
        let base = VariableBase::new(parents, dimensions);

        Self {
            base,
            class_weights,
            focus_weight,
        }
    }

    /// Helper to access predictions parent
    fn predictions(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Helper to access targets parent
    fn targets(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
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
            / number_of_examples as f64
    }

    fn gradient_for_predictions(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let predictions_data = ctx
            .data(self.predictions())
            .expect("Predictions not computed");
        let predictions_matrix = predictions_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let targets_data = ctx.data(self.targets()).expect("Targets not computed");
        let targets_vector = targets_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let self_gradient_data = ctx.gradient(self).expect("Self gradient not computed");
        let self_gradient = self_gradient_data
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let mut gradient =
            Matrix::with_dimensions(predictions_matrix.rows(), predictions_matrix.cols());

        for row in 0..gradient.rows() {
            let true_class = targets_vector.data_at(row) as usize;
            let predicted_probability_for_true_class = predictions_matrix.data_at(row, true_class);

            if predicted_probability_for_true_class > PREDICTED_PROBABILITY_THRESHOLD {
                let error = self_gradient
                    * self.compute_error_per_example(
                        gradient.rows(),
                        predicted_probability_for_true_class,
                        true_class,
                    );
                gradient.set_data_at(row, true_class, error);
            }
        }

        Box::new(gradient)
    }
}

impl Variable for FocalLoss {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let predictions_data = ctx
            .data(self.predictions())
            .expect("Predictions not computed");
        let predictions_matrix = predictions_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Predictions must be Matrix");

        let targets_data = ctx.data(self.targets()).expect("Targets not computed");
        let targets_vector = targets_data
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Targets must be Vector");

        let mut result = 0.0;
        for row in 0..targets_vector.total_size() {
            let true_class = targets_vector.data_at(row) as usize;
            let predicted_probability_for_true_class = predictions_matrix.data_at(row, true_class);

            if predicted_probability_for_true_class > 0.0 {
                result +=
                    self.compute_individual_loss(predicted_probability_for_true_class, true_class);
            }
        }

        Box::new(Scalar::new(-result / predictions_matrix.rows() as f64))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.predictions()) {
            self.gradient_for_predictions(ctx)
        } else {
            panic!(
                "The gradient should not be necessary for the targets. But got: {}",
                self.targets()
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

impl fmt::Display for FocalLoss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FocalLoss(focusWeight={})", self.focus_weight)
    }
}

//! Cross entropy loss function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions CrossEntropyLoss.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps a VariableBase (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: CrossEntropyLoss extends AbstractVariable<Scalar>
//!
//! - VariableBase provides: dimensions, parents, require_gradient tracking
//! - CrossEntropyLoss adds: predictions, targets, class weights, loss computation
//! - Delegates Variable trait methods to inner VariableBase
//!
//! This is the **origin of gradients** in the computational graph!

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

const PREDICTED_PROBABILITY_THRESHOLD: f64 = 1e-50;

/// Cross entropy loss for multi-class classification.
///
/// Computes: L = -Σ w_c * log(p_c) where p_c is predicted probability for true class c.
/// Corresponds to CrossEntropyLoss in Java GDS.
///
/// This is **protected** - FocalLoss extends this to add focal weighting.
pub struct CrossEntropyLoss {
    base: VariableBase, // COMPOSITION: wraps shared Variable logic (includes parents)
    pub(crate) class_weights: Vec<f64>, // Protected in Java - subclasses need access
}

impl CrossEntropyLoss {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new cross entropy loss.
    /// Java: `public CrossEntropyLoss(Variable<Matrix> predictions, Variable<Vector> targets, double[] classWeights)`
    pub fn new(
        predictions: Box<dyn Variable>,
        targets: Box<dyn Variable>,
        class_weights: Vec<f64>,
    ) -> Self {
        // Java: super(List.of(predictions, targets), Dimensions.scalar())
        let base = VariableBase::new(vec![predictions, targets], dimensions::scalar());

        Self {
            base,
            class_weights,
        }
    }

    /// Get predictions parent (first operand).
    fn predictions(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Get targets parent (second operand).
    fn targets(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }

    // ========================================================================
    // Utility Methods
    // ========================================================================

    /// Calculate size in bytes for scalar loss output.
    /// Java: `public static long sizeInBytes()`
    pub fn size_in_bytes() -> usize {
        crate::ml::core::tensor::size_in_bytes(&[1])
    }

    // ========================================================================
    // Loss Computation - Template Methods (can be overridden by subclasses)
    // ========================================================================

    /// Compute loss for a single example.
    /// Java: `double computeIndividualLoss(double predictedProbabilityForTrueClass, int trueClass)`
    /// Protected in Java - FocalLoss overrides this
    pub(crate) fn compute_individual_loss(
        &self,
        predicted_probability_for_true_class: f64,
        true_class: usize,
    ) -> f64 {
        self.class_weights[true_class] * predicted_probability_for_true_class.ln()
    }

    /// Compute gradient contribution for a single example.
    /// Java: `double computeErrorPerExample(int numberOfExamples, double predictedProbabilityForTrueClass, int trueClass)`
    /// Protected in Java - FocalLoss overrides this
    pub(crate) fn compute_error_per_example(
        &self,
        number_of_examples: usize,
        predicted_probability_for_true_class: f64,
        true_class: usize,
    ) -> f64 {
        -self.class_weights[true_class]
            / (predicted_probability_for_true_class * number_of_examples as f64)
    }

    // ========================================================================
    // Gradient Computation
    // ========================================================================

    /// Compute gradient with respect to predictions.
    /// Java: Called from `gradient(Variable<?> parent, ...)` when parent == predictions
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

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// CrossEntropyLoss delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where CrossEntropyLoss extends AbstractVariable.

impl Variable for CrossEntropyLoss {
    /// Compute cross entropy loss.
    /// Java: `public Scalar apply(ComputationContext ctx)`
    /// L = -1/n * Σ w_c * log(p_c)
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

    /// Compute gradient with respect to parent (predictions only).
    /// Java: `public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx)`
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

    // ========================================================================
    // DELEGATION: Forward to VariableBase
    // ========================================================================

    /// Check if gradient is required.
    /// Java: Inherited from `super(List.of(predictions, targets), ...)`
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Get parent variables (predictions, targets).
    /// Java: Inherited from `super(List.of(predictions, targets), ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    /// Get output dimensions (scalar).
    /// Java: Inherited from `super(..., Dimensions.scalar())`
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for CrossEntropyLoss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CrossEntropyLoss")
    }
}

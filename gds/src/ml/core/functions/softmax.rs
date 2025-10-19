//! Softmax activation function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Softmax.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps a VariableBase (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Softmax extends SingleParentVariable<Matrix, Matrix>
//!
//! - VariableBase provides: dimensions, parents, require_gradient tracking
//! - Softmax adds: softmax normalization logic (exp + row-wise normalization)
//! - Delegates Variable trait methods to inner VariableBase

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::abstract_variable::AbstractVariable;
use std::fmt;

/// Softmax activation function for multi-class classification.
///
/// Corresponds to Softmax in Java GDS.
/// Computes row-wise probability distributions: softmax(x_i) = exp(x_i) / Σ exp(x_j)
pub struct Softmax {
    base: AbstractVariable, // COMPOSITION: wraps shared Variable logic (includes parent)
}

impl Softmax {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new softmax activation.
    /// Java: `public Softmax(Variable<Matrix> parent) { super(parent, parent.dimensions()); }`
    pub fn new(parent: Box<dyn Variable>) -> Self {
        let dimensions = parent.dimensions().to_vec();

        // Java: super(parent, parent.dimensions())
        let base = AbstractVariable::with_gradient_requirement(vec![parent], dimensions, true);

        Self { base }
    }

    /// Get parent variable.
    fn parent(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    // ========================================================================
    // Utility Methods
    // ========================================================================

    /// Calculate size in bytes for matrix softmax output.
    /// Java: `public static long sizeInBytes(int rows, int cols)`
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[rows, cols])
    }

    /// Rescale softmax output to ensure numerical stability.
    /// Java: `private static void rescale(Matrix result)`
    fn rescale(result: &mut Matrix) {
        let rows = result.rows();
        let cols = result.cols();

        for row in 0..rows {
            let mut row_sum = 1e-15;
            for col in 0..cols {
                let current = result.data_at(row, col);
                row_sum += current;
            }
            for col in 0..cols {
                let current = result.data_at(row, col);
                result.set_data_at(row, col, current / row_sum);
            }
        }
    }

    // ========================================================================
    // Gradient Computation
    // ========================================================================

    /// Compute gradient with respect to parent.
    /// Java: `public Matrix gradientForParent(ComputationContext ctx)`
    /// Uses Jacobian of softmax: ∂softmax_i/∂x_j = softmax_i * (δ_ij - softmax_j)
    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_data_box = ctx.data(self).expect("Self data not computed");
        let self_data = self_data_box
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Self data must be Matrix");

        let self_gradient_box = ctx.gradient(self).expect("Self gradient not computed");
        let self_gradient = self_gradient_box
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Self gradient must be Matrix");

        let rows = self_data.rows();
        let cols = self_data.cols();

        let mut computed_gradient = Matrix::create(0.0, rows, cols);

        // result[row,col] = sum_{col2} s[row, col2] * (delta(col, col2) - s[row, col]) * grad[row, col2]
        for row in 0..rows {
            for col in 0..cols {
                let softmax_data = self_data.data_at(row, col);
                for softmax_col in 0..cols {
                    let delta = if col == softmax_col { 1.0 } else { 0.0 };
                    let gradient_contrib = self_data.data_at(row, softmax_col)
                        * (delta - softmax_data)
                        * self_gradient.data_at(row, softmax_col);
                    computed_gradient.add_data_at(row, col, gradient_contrib);
                }
            }
        }

        Box::new(computed_gradient)
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Softmax delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where Softmax extends SingleParentVariable.

impl Variable for Softmax {
    /// Apply softmax activation row-wise.
    /// Java: `public Matrix apply(ComputationContext ctx)`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let data_box = ctx.data(self.parent()).expect("Parent data not computed");
        let data = data_box
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let rows = data.rows();
        let cols = data.cols();

        let mut result = Matrix::with_dimensions(rows, cols);

        let mut rescale = false;
        for row in 0..rows {
            let mut row_sum = 1e-15;
            for col in 0..cols {
                let mut exp = data.data_at(row, col).exp();
                if exp.is_infinite() {
                    rescale = true;
                    exp = f64::MAX;
                }
                result.set_data_at(row, col, exp);
                row_sum += exp;
                if row_sum.is_infinite() {
                    rescale = true;
                    row_sum = f64::MAX;
                }
            }
            for col in 0..cols {
                let current = result.data_at(row, col);
                result.set_data_at(row, col, current / row_sum);
            }
        }

        if rescale {
            Self::rescale(&mut result);
        }

        Box::new(result)
    }

    /// Compute gradient with respect to parent.
    /// Java: Delegates to `gradientForParent(ctx)` from SingleParentVariable
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        assert!(
            std::ptr::eq(parent, self.parent()),
            "Gradient requested for unknown parent"
        );
        self.gradient_for_parent(ctx)
    }

    // ========================================================================
    // DELEGATION: Forward to VariableBase
    // ========================================================================

    /// Check if gradient is required.
    /// Java: Inherited from `super(parent, ...)`
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Get parent variables.
    /// Java: Inherited from `super(parent, ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    /// Get output dimensions (same as input).
    /// Java: Inherited from `super(..., parent.dimensions())`
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for Softmax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Softmax")
    }
}

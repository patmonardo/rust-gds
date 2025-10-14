//! Matrix vector sum function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions MatrixVectorSum.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps a VariableBase (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: MatrixVectorSum extends AbstractVariable<Matrix>
//!
//! - VariableBase provides: dimensions, parents, require_gradient tracking
//! - MatrixVectorSum adds: matrix, vector operands and broadcasting logic
//! - Delegates Variable trait methods to inner VariableBase

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::tensor::{Matrix, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

/// Adds a vector to each row of a matrix (broadcasting).
///
/// The vector is broadcast column-wise across the matrix rows.
/// Corresponds to MatrixVectorSum in Java GDS.
///
/// Note: Parents (matrix, vector) are stored in base.parents(). Access via matrix() and vector() helpers.
pub struct MatrixVectorSum {
    base: VariableBase, // COMPOSITION: wraps shared Variable logic (includes parents)
}

impl MatrixVectorSum {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new matrix-vector sum (broadcast addition).
    /// Java: `public MatrixVectorSum(Variable<Matrix> matrix, Variable<Vector> vector)`
    pub fn new(matrix: Box<dyn Variable>, vector: Box<dyn Variable>) -> Self {
        // Validate dimensions
        assert_eq!(
            matrix.dimension(COLUMNS_INDEX),
            vector.dimension(ROWS_INDEX),
            "Cannot broadcast vector with length {} to a matrix with {} columns",
            vector.dimension(ROWS_INDEX),
            matrix.dimension(COLUMNS_INDEX)
        );

        let dimensions = matrix.dimensions().to_vec();

        // Java: super(List.of(matrix, vector), matrix.dimensions())
        // Store parents [matrix, vector] in VariableBase
        let base = VariableBase::new(vec![matrix, vector], dimensions);

        Self { base }
    }

    /// Get matrix parent (first operand).
    fn matrix(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Get vector parent (second operand).
    fn vector(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// MatrixVectorSum delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where MatrixVectorSum extends AbstractVariable.

impl Variable for MatrixVectorSum {
    /// Broadcast vector addition to matrix.
    /// Java: `public Matrix apply(ComputationContext ctx) { return ctx.data(matrix).sumBroadcastColumnWise(ctx.data(vector)); }`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let matrix_tensor = ctx.data(self.matrix()).expect("Matrix data not computed");
        let matrix_data = matrix_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Matrix parent must be Matrix");

        let vector_tensor = ctx.data(self.vector()).expect("Vector data not computed");
        let vector_data = vector_tensor
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Vector parent must be Vector");

        matrix_data.sum_broadcast_column_wise(vector_data)
    }

    /// Compute gradient with respect to parent (matrix or vector).
    /// Java: `public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx)`
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.matrix()) {
            // Gradient for matrix: pass through
            // Java: `if (parent == matrix) return ctx.gradient(this);`
            let grad_tensor = ctx.gradient(self).expect("Gradient not computed");
            return grad_tensor.create_with_same_dimensions();
        } else if std::ptr::eq(parent, self.vector()) {
            // Gradient for vector: sum across columns
            // Java: `else return ctx.gradient(this).sumPerColumn();`
            let grad_tensor = ctx.gradient(self).expect("Gradient not computed");
            let grad = grad_tensor
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("Gradient must be Matrix");
            grad.sum_per_column()
        } else {
            panic!("Gradient requested for unknown parent");
        }
    }

    // ========================================================================
    // DELEGATION: Forward to VariableBase
    // ========================================================================

    /// Check if gradient is required.
    /// Java: Inherited from `super(List.of(matrix, vector), ...)`
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Get parent variables.
    /// Java: Inherited from `super(List.of(matrix, vector), ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    /// Get result dimensions (same as matrix dimensions).
    /// Java: Inherited from `super(..., matrix.dimensions())`
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for MatrixVectorSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MatrixVectorSum: {}, requireGradient: {}",
            self.base.render_dimensions(),
            self.require_gradient()
        )
    }
}

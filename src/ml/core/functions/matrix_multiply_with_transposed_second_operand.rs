//! Matrix multiplication with transposed second operand for ML in GDS.
//!
//! Translated from Java GDS ml-core functions MatrixMultiplyWithTransposedSecondOperand.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps a VariableBase (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: MatrixMultiplyWithTransposedSecondOperand extends AbstractVariable<Matrix>
//!
//! - VariableBase provides: dimensions, parents, require_gradient tracking
//! - This function adds: A, B operands and Ax=b computation logic
//! - Delegates Variable trait methods to inner VariableBase

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{self, COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

/// Matrix multiplication where the second operand is transposed: A * B^T
///
/// For dimensions (m, n) x (p, n)^T = (m, n) x (n, p) = (m, p)
///
/// This is the core linear algebra operation for solving Ax=b systems!
///
/// Note: Parents A and B are stored in base.parents(). Access via a() and b() helpers.
pub struct MatrixMultiplyWithTransposedSecondOperand {
    base: VariableBase, // COMPOSITION: wraps shared Variable logic (includes parents)
}

impl MatrixMultiplyWithTransposedSecondOperand {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new matrix multiplication A * B^T.
    /// Java: `public MatrixMultiplyWithTransposedSecondOperand(Variable<Matrix> A, Variable<Matrix> B)`
    pub fn new(a: Box<dyn Variable>, b: Box<dyn Variable>) -> Self {
        Self::assert_dimensions(a.as_ref(), b.as_ref());

        // Result dimensions: (m, n) x (p, n)^T = (m, p)
        let dimensions = dimensions::matrix(a.dimension(ROWS_INDEX), b.dimension(ROWS_INDEX));

        // Java: super(List.of(A, B), Dimensions.matrix(...))
        // Store parents [A, B] in VariableBase
        let base = VariableBase::new(vec![a, b], dimensions);

        Self { base }
    }

    /// Get parent A (first operand).
    fn a(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    /// Get parent B (second operand).
    fn b(&self) -> &dyn Variable {
        self.base.parents()[1].as_ref()
    }

    /// Factory method.
    /// Java: `public static MatrixMultiplyWithTransposedSecondOperand of(Variable<Matrix> A, Variable<Matrix> B)`
    pub fn of(a: Box<dyn Variable>, b: Box<dyn Variable>) -> Self {
        Self::new(a, b)
    }

    // ========================================================================
    // Utility Methods
    // ========================================================================

    /// Calculate size in bytes for result matrix.
    /// Java: `public static long sizeInBytes(int leftMatrixNumRows, int rightMatrixNumRows)`
    pub fn size_in_bytes(left_matrix_num_rows: usize, right_matrix_num_rows: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[left_matrix_num_rows, right_matrix_num_rows])
    }

    /// Validate that matrices can be multiplied with transpose.
    /// Java: `private void assertDimensions(Variable<Matrix> A, Variable<Matrix> B)`
    fn assert_dimensions(a: &dyn Variable, b: &dyn Variable) {
        assert_eq!(
            a.dimension(COLUMNS_INDEX),
            b.dimension(COLUMNS_INDEX),
            "Cannot multiply matrix having dimensions ({}, {}) with transposed matrix of dimensions ({}, {})",
            a.dimension(COLUMNS_INDEX),
            a.dimension(ROWS_INDEX),
            b.dimension(ROWS_INDEX),
            b.dimension(COLUMNS_INDEX)
        );
    }

    // ========================================================================
    // Gradient Helpers
    // ========================================================================

    /// Gradient with respect to A.
    /// Java: `if (parent == A) return gradient.multiply(ctx.data(B));`
    fn gradient_for_a(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let gradient_data = ctx.gradient(self).expect("Gradient not computed");
        let gradient = gradient_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Gradient must be Matrix");

        let b_tensor = ctx.data(self.b()).expect("B data not computed");
        let b_data = b_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("B must be Matrix");

        gradient.multiply(b_data)
    }

    /// Gradient with respect to B.
    /// Java: `else return gradient.multiplyTransA(ctx.data(A));`
    fn gradient_for_b(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let gradient_data = ctx.gradient(self).expect("Gradient not computed");
        let gradient = gradient_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Gradient must be Matrix");

        let a_tensor = ctx.data(self.a()).expect("A data not computed");
        let a_data = a_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("A must be Matrix");

        gradient.multiply_trans_a(a_data)
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// MatrixMultiply delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where MatrixMultiply extends AbstractVariable.

impl Variable for MatrixMultiplyWithTransposedSecondOperand {
    /// Compute A * B^T.
    /// Java: `public Matrix apply(ComputationContext ctx) { return ctx.data(A).multiplyTransB(ctx.data(B)); }`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let a_tensor = ctx.data(self.a()).expect("A data not computed");
        let a_matrix = a_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("A must be Matrix");

        let b_tensor = ctx.data(self.b()).expect("B data not computed");
        let b_matrix = b_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("B must be Matrix");

        a_matrix.multiply_trans_b(b_matrix)
    }

    /// Compute gradient with respect to parent (A or B).
    /// Java: `public Matrix gradient(Variable<?> parent, ComputationContext ctx)`
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        if std::ptr::eq(parent, self.a()) {
            self.gradient_for_a(ctx)
        } else if std::ptr::eq(parent, self.b()) {
            self.gradient_for_b(ctx)
        } else {
            panic!("Gradient requested for unknown parent");
        }
    }

    // ========================================================================
    // DELEGATION: Forward to VariableBase
    // ========================================================================

    /// Check if gradient is required.
    /// Java: Inherited from `super(List.of(A, B), ...)` which computes requireGradient from parents
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Get parent variables.
    /// Java: Inherited from `super(List.of(A, B), ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    /// Get result dimensions.
    /// Java: Inherited from `super(..., Dimensions.matrix(...))`
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for MatrixMultiplyWithTransposedSecondOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MatrixMultiplyWithTransposedSecondOperand: {}, requireGradient: {}",
            self.base.render_dimensions(),
            self.require_gradient()
        )
    }
}

//! Matrix sum function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions MatrixSum.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps an AbstractVariable (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: MatrixSum extends AbstractVariable<Matrix>
//!
//! - AbstractVariable provides: dimensions, parents, require_gradient tracking
//! - MatrixSum adds: element-wise summation logic
//! - Delegates Variable trait methods to inner AbstractVariable

use crate::ml::core::abstract_variable::AbstractVariable;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use std::fmt;

/// Sums multiple matrices element-wise.
///
/// All parent matrices must have the same dimensions.
/// Corresponds to MatrixSum in Java GDS.
pub struct MatrixSum {
    base: AbstractVariable, // COMPOSITION: wraps shared Variable logic
}

impl MatrixSum {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new matrix sum from multiple parent matrices.
    /// Java: `public MatrixSum(List<Variable<Matrix>> parents) { super(parents, validatedDimensions(parents)); }`
    pub fn new(parents: Vec<Box<dyn Variable>>) -> Self {
        let dimensions = Self::validated_dimensions(&parents);
        let base = AbstractVariable::new(parents, dimensions);
        Self { base }
    }

    // ========================================================================
    // Validation
    // ========================================================================

    /// Validate that all parents have the same dimensions.
    /// Java: `private static int[] validatedDimensions(List<Variable<Matrix>> parents)`
    fn validated_dimensions(parents: &[Box<dyn Variable>]) -> Vec<usize> {
        assert!(
            !parents.is_empty(),
            "MatrixSum requires at least one parent"
        );

        let first_dims = parents[0].dimensions();
        for parent in &parents[1..] {
            assert_eq!(
                parent.dimensions(),
                first_dims,
                "All parent matrices must have the same dimensions"
            );
        }

        first_dims.to_vec()
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// MatrixSum delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where MatrixSum extends AbstractVariable.

impl Variable for MatrixSum {
    /// Sum all parent matrices element-wise.
    /// Java: `public Matrix apply(ComputationContext ctx)`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let rows = self.base.dimension(ROWS_INDEX);
        let cols = self.base.dimension(COLUMNS_INDEX);

        let mut sum = Matrix::create(0.0, rows, cols);

        for parent in self.base.parents() {
            let parent_tensor = ctx.data(parent.as_ref()).expect("Parent data not computed");
            let parent_data = parent_tensor
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("Parent must be Matrix");

            sum.add_inplace(parent_data);
        }

        Box::new(sum)
    }

    /// Gradient for sum is just the gradient passed through.
    /// Java: `public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx) { return ctx.gradient(this); }`
    fn gradient(&self, _parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        ctx.gradient(self).expect("Gradient not computed")
    }

    // ========================================================================
    // DELEGATION: Forward to AbstractVariable
    // ========================================================================

    /// Check if gradient is required.
    /// Java: Inherited from `super(parents, ...)`
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Get parent variables.
    /// Java: Inherited from `super(parents, ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    /// Get result dimensions.
    /// Java: Inherited from `super(..., validatedDimensions(...))`
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for MatrixSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MatrixSum: {} parents, dimensions: {}",
            self.base.parents().len(),
            self.base.render_dimensions()
        )
    }
}

//! Constant function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Constant.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This Constant wraps an AbstractVariable (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Constant extends AbstractVariable<T>
//!
//! - AbstractVariable provides: dimensions, parents, require_gradient tracking
//! - Constant adds: data storage
//! - Constant delegates Variable trait methods to inner AbstractVariable

use crate::ml::core::abstract_variable::NotAFunctionException;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::abstract_variable::AbstractVariable;
use std::fmt;

/// A constant tensor value that doesn't depend on any parents.
///
/// This corresponds to the Constant class in Java GDS.
/// Uses type erasure - stores a boxed Tensor.
pub struct Constant {
    base: AbstractVariable, // COMPOSITION: wraps shared Variable logic
    data: Box<dyn Tensor>,
}

impl Constant {
    // ========================================================================
    // Constructors
    // ========================================================================

    /// Create a new constant from any tensor.
    pub fn new(data: Box<dyn Tensor>) -> Self {
        let dimensions = data.dimensions().to_vec();
        let base = AbstractVariable::with_gradient_requirement(
            vec![], // Constants have no parents
            dimensions,
            false, // Constants don't require gradients
        );
        Self { base, data }
    }

    /// Create a scalar constant.
    pub fn scalar(value: f64) -> Self {
        Self::new(Box::new(Scalar::new(value)))
    }

    /// Create a vector constant.
    pub fn vector(data: Vec<f64>) -> Self {
        Self::new(Box::new(Vector::new(data)))
    }

    /// Create a matrix constant.
    pub fn matrix(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self::new(Box::new(Matrix::new(data, rows, cols)))
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the underlying data (test-only in Java).
    #[cfg(test)]
    pub fn data(&self) -> &dyn Tensor {
        self.data.as_ref()
    }

    /// Calculate size in bytes for given dimensions.
    pub fn size_in_bytes(dimensions: &[usize]) -> usize {
        crate::ml::core::tensor::size_in_bytes(dimensions)
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Most methods delegate to the inner AbstractVariable.
// Constant-specific logic (apply, gradient) is implemented here.

impl Variable for Constant {
    /// Apply: Return the constant data.
    /// Constant-specific: No computation needed, just return stored data.
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        self.data.clone_box()
    }

    /// Gradient: Constants are not functions, so this panics.
    /// Constant-specific: Constants don't participate in backpropagation.
    fn gradient(&self, _parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("{}", NotAFunctionException);
    }

    // DELEGATION: Forward to AbstractVariable
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    // DELEGATION: Forward to AbstractVariable
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    // DELEGATION: Forward to AbstractVariable
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Constant: {}, requireGradient: {}",
            self.data,
            self.require_gradient()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_constant() {
        let c = Constant::scalar(42.0);
        assert_eq!(c.dimensions(), &[1, 1]);
        assert!(!c.require_gradient());
    }

    #[test]
    fn test_vector_constant() {
        let c = Constant::vector(vec![1.0, 2.0, 3.0]);
        assert_eq!(c.dimensions(), &[3, 1]);
    }

    #[test]
    fn test_matrix_constant() {
        let c = Constant::matrix(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_eq!(c.dimensions(), &[2, 2]);
    }

    #[test]
    #[should_panic]
    fn test_gradient_panics() {
        let c = Constant::scalar(1.0);
        let ctx = ComputationContext::new();
        let _ = c.gradient(&c, &ctx);
    }
}

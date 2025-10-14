//! Weights variable for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Weights.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This Weights wraps a VariableBase (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Weights<T> extends AbstractVariable<T>
//!
//! - VariableBase provides: dimensions, parents, require_gradient tracking
//! - Weights adds: data storage (trainable parameters)
//! - Weights delegates Variable trait methods to inner VariableBase

use crate::ml::core::abstract_variable::NotAFunctionException;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

/// Trainable weights that require gradient computation.
///
/// This corresponds to Weights<T> in Java GDS.
/// Uses type erasure - stores a boxed Tensor.
pub struct Weights {
    base: VariableBase, // COMPOSITION: wraps shared Variable logic
    data: Box<dyn Tensor>,
}

impl Weights {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create weights from any tensor.
    /// Java: `public Weights(T data) { super(List.of(), data.dimensions()); this.data = data; }`
    pub fn new(data: Box<dyn Tensor>) -> Self {
        let dimensions = data.dimensions().to_vec();
        let base = VariableBase::with_gradient_requirement(
            vec![], // Weights have no parents (leaf variables)
            dimensions,
            true, // Weights ALWAYS require gradients (trainable)
        );
        Self { base, data }
    }

    // ========================================================================
    // Factory Methods - match Java static constructors
    // ========================================================================

    /// Create matrix weights.
    /// Java: `public static Weights<Matrix> ofMatrix(int rows, int cols)`
    pub fn of_matrix(rows: usize, cols: usize) -> Self {
        Self::new(Box::new(Matrix::with_dimensions(rows, cols)))
    }

    /// Create vector weights from values.
    /// Java: `public static Weights<Vector> ofVector(double... values)`
    pub fn of_vector(values: Vec<f64>) -> Self {
        Self::new(Box::new(Vector::new(values)))
    }

    /// Create scalar weights.
    /// Java: `public static Weights<Scalar> ofScalar(double value)`
    pub fn of_scalar(value: f64) -> Self {
        Self::new(Box::new(Scalar::new(value)))
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the underlying data.
    /// Java: `public T data() { return data; }`
    pub fn data(&self) -> &dyn Tensor {
        self.data.as_ref()
    }

    /// Calculate size in bytes for matrix weights.
    /// Java: `public static long sizeInBytes(int rows, int cols)`
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[rows, cols])
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Weights delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where Weights extends AbstractVariable.

impl Variable for Weights {
    /// Return the stored data.
    /// Java: `public T apply(ComputationContext ctx) { return data; }`
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        self.data.clone_box()
    }

    /// Weights are leaf variables - gradient() should never be called.
    /// Java: `public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx) { throw new NotAFunctionException(); }`
    fn gradient(&self, _parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("{}", NotAFunctionException);
    }

    // ========================================================================
    // DELEGATION: Forward to VariableBase
    // ========================================================================

    /// Weights always require gradients (trainable parameters).
    /// Java: `public boolean requireGradient() { return true; }` (overrides AbstractVariable)
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Weights have no parents (leaf variables).
    /// Java: Inherited from `super(List.of(), ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    /// Get dimensions.
    /// Java: Inherited from `super(..., data.dimensions())`
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for Weights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Weights: {}, requireGradient: {}",
            self.data,
            self.require_gradient()
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_weights() {
        let w = Weights::of_matrix(2, 3);
        assert_eq!(w.dimensions(), &[2, 3]);
        assert!(w.require_gradient());
    }

    #[test]
    fn test_vector_weights() {
        let w = Weights::of_vector(vec![1.0, 2.0, 3.0]);
        assert_eq!(w.dimensions(), &[3]);
        assert!(w.require_gradient());
    }

    #[test]
    fn test_scalar_weights() {
        let w = Weights::of_scalar(42.0);
        assert_eq!(w.dimensions(), &[1]);
        assert!(w.require_gradient());
    }

    #[test]
    #[should_panic]
    fn test_gradient_panics() {
        let w = Weights::of_scalar(1.0);
        let ctx = ComputationContext::new();
        let _ = w.gradient(&w, &ctx);
    }
}

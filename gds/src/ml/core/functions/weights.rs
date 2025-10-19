//! Weights variable for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Weights.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This Weights wraps an AbstractVariable (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Weights<T> extends AbstractVariable<T>
//!
//! - AbstractVariable provides: dimensions, parents, require_gradient tracking
//! - Weights adds: data storage (trainable parameters)
//! - Weights delegates Variable trait methods to inner AbstractVariable

use crate::ml::core::abstract_variable::NotAFunctionException;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::{Matrix, Scalar, Tensor, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::core::abstract_variable::AbstractVariable;
use parking_lot::{
    MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::fmt;
use std::sync::Arc;

/// Trainable weights that require gradient computation.
///
/// This corresponds to Weights<T> in Java GDS.
/// Uses type erasure - stores a boxed Tensor protected by an `Arc<RwLock<…>>`
/// so that gradients can update the value concurrently.
pub struct Weights {
    base: AbstractVariable,                 // COMPOSITION: shared Variable behaviour
    data: Arc<RwLock<Box<dyn Tensor>>>, // Shared trainable tensor
}

impl Weights {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create weights from any tensor.
    /// Java: `public Weights(T data) { super(List.of(), data.dimensions()); this.data = data; }`
    pub fn new(data: Box<dyn Tensor>) -> Self {
        Self::from_tensor(data)
    }

    // ========================================================================
    // Factory Methods - match Java static constructors
    // ========================================================================

    /// Create matrix weights.
    /// Java: `public static Weights<Matrix> ofMatrix(int rows, int cols)`
    pub fn of_matrix(rows: usize, cols: usize) -> Self {
        Self::from_tensor(Box::new(Matrix::with_dimensions(rows, cols)))
    }

    /// Create vector weights from values.
    /// Java: `public static Weights<Vector> ofVector(double... values)`
    pub fn of_vector(values: Vec<f64>) -> Self {
        Self::from_tensor(Box::new(Vector::new(values)))
    }

    /// Create scalar weights.
    /// Java: `public static Weights<Scalar> ofScalar(double value)`
    pub fn of_scalar(value: f64) -> Self {
        Self::from_tensor(Box::new(Scalar::new(value)))
    }

    /// Construct weights from any tensor. Convenience for serialization paths.
    pub fn from_tensor(data: Box<dyn Tensor>) -> Self {
        let dimensions = data.dimensions().to_vec();
        let base = AbstractVariable::with_gradient_requirement(vec![], dimensions, true);
        Self {
            base,
            data: Arc::new(RwLock::new(data)),
        }
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Clone of internal tensor (read-only snapshot).
    pub fn snapshot(&self) -> Box<dyn Tensor> {
        self.data.read().clone_box()
    }

    /// Calculate size in bytes for matrix weights.
    /// Java: `public static long sizeInBytes(int rows, int cols)`
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[rows, cols])
    }

    /// Shared handle to the underlying tensor used by optimizers.
    pub fn handle(&self) -> Arc<RwLock<Box<dyn Tensor>>> {
        self.data.clone()
    }

    /// Borrow the tensor immutably.
    pub fn borrow(&self) -> RwLockReadGuard<'_, Box<dyn Tensor>> {
        self.data.read()
    }

    /// Borrow the tensor mutably.
    pub fn borrow_mut(&self) -> RwLockWriteGuard<'_, Box<dyn Tensor>> {
        self.data.write()
    }

    /// Borrow as matrix (panic if underlying tensor is not a Matrix).
    pub fn borrow_matrix(&self) -> MappedRwLockReadGuard<'_, Matrix> {
        RwLockReadGuard::map(self.data.read(), |tensor| {
            tensor
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("Weights tensor is not Matrix")
        })
    }

    /// Borrow as scalar (panic if not Scalar).
    pub fn borrow_scalar(&self) -> MappedRwLockReadGuard<'_, Scalar> {
        RwLockReadGuard::map(self.data.read(), |tensor| {
            tensor
                .as_any()
                .downcast_ref::<Scalar>()
                .expect("Weights tensor is not Scalar")
        })
    }

    /// Borrow as vector (panic if not Vector).
    pub fn borrow_vector(&self) -> MappedRwLockReadGuard<'_, Vector> {
        RwLockReadGuard::map(self.data.read(), |tensor| {
            tensor
                .as_any()
                .downcast_ref::<Vector>()
                .expect("Weights tensor is not Vector")
        })
    }

    /// Borrow as matrix mutably (panic if underlying tensor is not a Matrix).
    pub fn borrow_matrix_mut(&self) -> parking_lot::MappedRwLockWriteGuard<'_, Matrix> {
        RwLockWriteGuard::map(self.data.write(), |tensor| {
            tensor
                .as_any_mut()
                .downcast_mut::<Matrix>()
                .expect("Weights tensor is not Matrix")
        })
    }

    /// Borrow as scalar mutably (panic if not Scalar).
    pub fn borrow_scalar_mut(&self) -> parking_lot::MappedRwLockWriteGuard<'_, Scalar> {
        RwLockWriteGuard::map(self.data.write(), |tensor| {
            tensor
                .as_any_mut()
                .downcast_mut::<Scalar>()
                .expect("Weights tensor is not Scalar")
        })
    }

    /// Borrow as vector mutably (panic if not Vector).
    pub fn borrow_vector_mut(&self) -> parking_lot::MappedRwLockWriteGuard<'_, Vector> {
        RwLockWriteGuard::map(self.data.write(), |tensor| {
            tensor
                .as_any_mut()
                .downcast_mut::<Vector>()
                .expect("Weights tensor is not Vector")
        })
    }
}

impl Clone for Weights {
    fn clone(&self) -> Self {
        let base = AbstractVariable::with_gradient_requirement(
            vec![], 
            self.base.dimensions().to_vec(), 
            true
        );
        Self {
            base,
            data: self.data.clone(),
        }
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Weights delegates dimension/parent/gradient tracking to AbstractVariable.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where Weights extends AbstractVariable.

impl Variable for Weights {
    /// Return the stored data.
    /// Java: `public T apply(ComputationContext ctx) { return data; }`
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        self.data.read().clone_box()
    }

    /// Weights are leaf variables - gradient() should never be called.
    /// Java: `public Tensor<?> gradient(Variable<?> parent, ComputationContext ctx) { throw new NotAFunctionException(); }`
    fn gradient(&self, _parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("{}", NotAFunctionException);
    }

    // ========================================================================
    // DELEGATION: Forward to AbstractVariable
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
            self.base.render_dimensions(),
            self.require_gradient()
        )
    }
}

unsafe impl Send for Weights {}
unsafe impl Sync for Weights {}

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
        assert_eq!(w.dimensions(), &[3, 1]);
        assert!(w.require_gradient());
    }

    #[test]
    fn test_scalar_weights() {
        let w = Weights::of_scalar(42.0);
        assert_eq!(w.dimensions(), &[1, 1]);
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

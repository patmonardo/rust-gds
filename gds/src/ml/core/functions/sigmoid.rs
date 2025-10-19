//! Sigmoid activation function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Sigmoid.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps an AbstractVariable (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Sigmoid<T> extends SingleParentVariable<T, T>
//!
//! - AbstractVariable provides: dimensions, parents, require_gradient tracking
//! - Sigmoid adds: sigmoid activation logic (forward/backward)
//! - Delegates Variable trait methods to inner AbstractVariable

use crate::ml::core::abstract_variable::AbstractVariable;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::fmt;

/// Sigmoid activation function: σ(x) = 1 / (1 + e^(-x))
///
/// Corresponds to Sigmoid<T> in Java GDS.
/// Single-parent activation function with element-wise non-linearity.
pub struct Sigmoid {
    base: AbstractVariable, // COMPOSITION: wraps shared Variable logic
    parent: Box<dyn Variable>,
}

impl Sigmoid {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new sigmoid activation.
    /// Java: `public Sigmoid(Variable<T> parent) { super(parent, parent.dimensions()); }`
    pub fn new(parent: Box<dyn Variable>) -> Self {
        let dimensions = parent.dimensions().to_vec();
        let require_gradient = parent.require_gradient();
        let base = AbstractVariable::with_gradient_requirement(vec![], dimensions, require_gradient);
        Self { base, parent }
    }

    /// Get parent variable.
    fn parent(&self) -> &dyn Variable {
        self.parent.as_ref()
    }

    // ========================================================================
    // Utility Methods
    // ========================================================================

    /// Calculate size in bytes for matrix sigmoid output.
    /// Java: `public static long sizeInBytes(int rows, int cols)`
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[rows, cols])
    }

    /// Sigmoid function: σ(x) = 1 / (1 + e^(-x))
    /// Java: `public static double sigmoid(double x)`
    pub fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    // ========================================================================
    // Gradient Computation
    // ========================================================================

    /// Compute gradient with respect to parent.
    /// Java: `public T gradientForParent(ComputationContext ctx)`
    /// Gradient: σ'(x) = σ(x) * (1 - σ(x))
    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let result = ctx
            .data(self)
            .expect("Self data not computed")
            .map(|value| value * (1.0 - value));

        let self_gradient = ctx.gradient(self).expect("Self gradient not computed");

        result.elementwise_product(self_gradient.as_ref())
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Sigmoid delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where Sigmoid extends SingleParentVariable.

impl Variable for Sigmoid {
    /// Apply sigmoid activation element-wise.
    /// Java: `public T apply(ComputationContext ctx) { return ctx.data(parent).map(Sigmoid::sigmoid); }`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        ctx.data(self.parent())
            .expect("Parent data not computed")
            .map(Self::sigmoid)
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
    // DELEGATION: Forward to AbstractVariable
    // ========================================================================

    /// Check if gradient is required.
    /// Java: Inherited from `super(parent, ...)`
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    /// Get parent variables.
    /// Java: Inherited from `super(parent, ...)`
    fn parents(&self) -> &[Box<dyn Variable>] {
        std::slice::from_ref(&self.parent)
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

impl fmt::Display for Sigmoid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sigmoid")
    }
}

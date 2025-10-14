//! ReLU activation function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Relu.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps a VariableBase (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Relu<T> extends SingleParentVariable<T, T>
//!
//! - VariableBase provides: dimensions, parents, require_gradient tracking
//! - Relu adds: leaky ReLU activation logic with configurable alpha
//! - Delegates Variable trait methods to inner VariableBase

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

const ALPHA: f64 = 0.01;

/// Leaky ReLU activation function: f(x) = x if x > 0, else α·x
///
/// Corresponds to Relu<T> in Java GDS.
/// Single-parent activation with configurable leak factor α (default 0.01).
pub struct Relu {
    base: VariableBase, // COMPOSITION: wraps shared Variable logic (includes parent)
    alpha: f64,         // Leak factor for negative values
}

impl Relu {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new leaky ReLU with custom alpha.
    /// Java: `public Relu(Variable<T> parent, double alpha) { super(parent, parent.dimensions()); this.alpha = alpha; }`
    pub fn new(parent: Box<dyn Variable>, alpha: f64) -> Self {
        let dimensions = parent.dimensions().to_vec();

        // Java: super(parent, parent.dimensions())
        let base = VariableBase::new(vec![parent], dimensions);

        Self { base, alpha }
    }

    /// Create leaky ReLU with default alpha (0.01).
    /// Java: `public Relu(Variable<T> parent) { this(parent, ALPHA); }`
    pub fn with_default_alpha(parent: Box<dyn Variable>) -> Self {
        Self::new(parent, ALPHA)
    }

    /// Get parent variable.
    fn parent(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    // ========================================================================
    // Gradient Computation
    // ========================================================================

    /// Compute gradient with respect to parent.
    /// Java: `public T gradientForParent(ComputationContext ctx)`
    /// Gradient: f'(x) = 1 if x > 0, else α
    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let alpha = self.alpha;
        let parent_data = ctx.data(self.parent()).expect("Parent data not computed");

        // Manually create gradient tensor instead of using map() with closure
        let mut gradient = parent_data.create_with_same_dimensions();
        for (idx, &value) in parent_data.data().iter().enumerate() {
            let grad_value = if value > 0.0 { 1.0 } else { alpha };
            gradient.set_data_at(idx, grad_value);
        }

        let self_gradient = ctx.gradient(self).expect("Self gradient not computed");

        gradient.elementwise_product_mutate(self_gradient.as_ref());
        gradient
    }
}

// ============================================================================
// Variable Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Relu delegates dimension/parent/gradient tracking to VariableBase.
// Only implements function-specific logic (apply, gradient).
//
// This matches Java's inheritance where Relu extends SingleParentVariable.

impl Variable for Relu {
    /// Apply leaky ReLU activation element-wise.
    /// Java: `public T apply(ComputationContext ctx) { return ctx.data(parent).map(value -> (value > 0) ? value : (alpha * value)); }`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let alpha = self.alpha;
        let parent_data = ctx.data(self.parent()).expect("Parent data not computed");

        // Manually create result tensor instead of using map() with closure
        let mut result = parent_data.create_with_same_dimensions();
        for (idx, &value) in parent_data.data().iter().enumerate() {
            let output = if value > 0.0 { value } else { alpha * value };
            result.set_data_at(idx, output);
        }
        result
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

impl fmt::Display for Relu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Relu(alpha={})", self.alpha)
    }
}

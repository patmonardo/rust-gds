//! ReLU activation function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Relu.java.
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This function wraps an AbstractVariable (composition) to share dimension/parent tracking.
//! This matches Java's inheritance: Relu<T> extends SingleParentVariable<T, T>
//!
//! - AbstractVariable provides: dimensions, parents, require_gradient tracking
//! - Relu adds: leaky ReLU activation logic with configurable alpha
//! - Delegates Variable trait methods to inner AbstractVariable

use crate::ml::core::abstract_variable::AbstractVariable;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::fmt;

const ALPHA: f64 = 0.01;

/// Leaky ReLU activation function: f(x) = x if x > 0, else α·x
///
/// Corresponds to Relu<T> in Java GDS.
/// Single-parent activation with configurable leak factor α (default 0.01).
pub struct Relu {
    base: AbstractVariable, // COMPOSITION: wraps shared Variable logic
    parent: Box<dyn Variable>,
    alpha: f64,             // Leak factor for negative values
}

impl Relu {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create new leaky ReLU with custom alpha.
    /// Java: `public Relu(Variable<T> parent, double alpha) { super(parent, parent.dimensions()); this.alpha = alpha; }`
    pub fn new(parent: Box<dyn Variable>, alpha: f64) -> Self {
        let dimensions = parent.dimensions().to_vec();
        let require_gradient = parent.require_gradient();
        let base = AbstractVariable::with_gradient_requirement(vec![], dimensions, require_gradient);
        Self { base, parent, alpha }
    }

    /// Create leaky ReLU with default alpha (0.01).
    /// Java: `public Relu(Variable<T> parent) { this(parent, ALPHA); }`
    pub fn with_default_alpha(parent: Box<dyn Variable>) -> Self {
        Self::new(parent, ALPHA)
    }

    /// Get parent variable.
    fn parent(&self) -> &dyn Variable {
        self.parent.as_ref()
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
        let self_gradient = ctx.gradient(self).expect("Self gradient not computed");

        // Create gradient by manually iterating over data
        let mut gradient_data = Vec::new();
        for &value in parent_data.data() {
            gradient_data.push(if value > 0.0 { 1.0 } else { alpha });
        }
        
        // Create gradient tensor based on parent type
        let gradient = if let Some(matrix) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Matrix>() {
            Box::new(crate::ml::core::tensor::Matrix::new(gradient_data, matrix.rows(), matrix.cols())) as Box<dyn Tensor>
        } else if let Some(vector) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Vector>() {
            Box::new(crate::ml::core::tensor::Vector::new(gradient_data)) as Box<dyn Tensor>
        } else if let Some(scalar) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Scalar>() {
            Box::new(crate::ml::core::tensor::Scalar::new(gradient_data[0])) as Box<dyn Tensor>
        } else {
            panic!("Unknown tensor type");
        };
        
        // Element-wise product with self gradient
        gradient.elementwise_product(self_gradient.as_ref())
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

        // Create result by manually iterating over data
        let mut result_data = Vec::new();
        for &value in parent_data.data() {
            result_data.push(if value > 0.0 { value } else { alpha * value });
        }
        
        // Create result tensor based on parent type
        if let Some(matrix) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Matrix>() {
            Box::new(crate::ml::core::tensor::Matrix::new(result_data, matrix.rows(), matrix.cols())) as Box<dyn Tensor>
        } else if let Some(vector) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Vector>() {
            Box::new(crate::ml::core::tensor::Vector::new(result_data)) as Box<dyn Tensor>
        } else if let Some(scalar) = parent_data.as_any().downcast_ref::<crate::ml::core::tensor::Scalar>() {
            Box::new(crate::ml::core::tensor::Scalar::new(result_data[0])) as Box<dyn Tensor>
        } else {
            panic!("Unknown tensor type");
        }
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

impl fmt::Display for Relu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Relu(alpha={})", self.alpha)
    }
}

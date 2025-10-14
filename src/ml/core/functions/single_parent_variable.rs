//! Single parent variable base implementation for ML functions in GDS.
//!
//! Translated from Java GDS ml-core functions/SingleParentVariable.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! Uses type erasure pattern (Box<dyn Variable>) to match our architecture.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;

/// Abstract base for variables with a single parent.
///
/// Provides common functionality for functions that have exactly one input variable.
/// Uses type erasure pattern - parent is `Box<dyn Variable>`, not generic.
///
/// NOTE: This is rarely used directly. Most functions (Sigmoid, Relu, etc.)
/// directly embed the pattern instead of using this struct.
pub struct SingleParentVariable {
    parent: Box<dyn Variable>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl SingleParentVariable {
    /// Create a new single parent variable.
    pub fn new(parent: Box<dyn Variable>, dimensions: Vec<usize>) -> Self {
        let require_gradient = parent.require_gradient();
        Self {
            parent,
            dimensions,
            require_gradient,
        }
    }

    /// Get the parent variable.
    pub fn parent(&self) -> &dyn Variable {
        self.parent.as_ref()
    }

    /// Validate that the given variable is our parent.
    pub fn validate_parent(&self, variable: &dyn Variable) {
        let parent_ptr = self.parent.as_ref() as *const dyn Variable;
        let variable_ptr = variable as *const dyn Variable;

        if parent_ptr != variable_ptr {
            panic!("Calling gradient with a `parent` that was not expected");
        }
    }
}

impl Variable for SingleParentVariable {
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        unimplemented!("apply() must be implemented by concrete subclasses (Sigmoid, Relu, etc.)")
    }

    fn gradient(&self, parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        self.validate_parent(parent);
        unimplemented!("gradient_for_parent() must be implemented by concrete subclasses")
    }

    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    fn require_gradient(&self) -> bool {
        self.require_gradient
    }

    fn parents(&self) -> &[Box<dyn Variable>] {
        std::slice::from_ref(&self.parent)
    }
}

impl std::fmt::Display for SingleParentVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SingleParentVariable")
    }
}

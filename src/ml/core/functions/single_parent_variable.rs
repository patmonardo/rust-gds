//! Single parent variable base implementation for ML functions in GDS.
//!
//! Translated from Java GDS ml-core functions/SingleParentVariable.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! Uses type erasure pattern (Box<dyn Variable>) to match our architecture.

use crate::ml::core::abstract_variable::AbstractVariable;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;

/// Abstract base for variables with a single parent.
///
/// Provides common functionality for functions that have exactly one input variable.
/// Uses composition with AbstractVariable to match Java's inheritance pattern.
///
/// This corresponds to Java's `SingleParentVariable<P extends Tensor<P>, T extends Tensor<T>> extends AbstractVariable<T>`
pub struct SingleParentVariable {
    base: AbstractVariable,
    parent: Box<dyn Variable>,
}

impl SingleParentVariable {
    /// Create a new single parent variable.
    /// Java: `public SingleParentVariable(Variable<P> parent, int[] dimensions)`
    pub fn new(parent: Box<dyn Variable>, dimensions: Vec<usize>) -> Self {
        // We can't clone Box<dyn Variable>, so we'll create the AbstractVariable differently
        let require_gradient = parent.require_gradient();
        let base = AbstractVariable::with_gradient_requirement(vec![], dimensions, require_gradient);
        Self {
            base,
            parent,
        }
    }

    /// Get the parent variable.
    /// Java: `protected final Variable<P> parent`
    pub fn parent(&self) -> &dyn Variable {
        self.parent.as_ref()
    }

    /// Validate that the given variable is our parent.
    /// Java: `private void validateParent(Variable<?> variable)`
    pub fn validate_parent(&self, variable: &dyn Variable) {
        let parent_ptr = self.parent.as_ref() as *const dyn Variable;
        let variable_ptr = variable as *const dyn Variable;

        if parent_ptr != variable_ptr {
            panic!("Calling gradient with a `parent` that was not expected");
        }
    }

    /// Template method for gradient computation.
    /// Java: `protected abstract P gradientForParent(ComputationContext ctx);`
    /// 
    /// Concrete implementations must override this method.
    pub fn gradient_for_parent(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        unimplemented!("gradient_for_parent() must be implemented by concrete subclasses")
    }
}

impl Variable for SingleParentVariable {
    /// Apply: Must be implemented by concrete subclasses.
    /// Java: `public abstract T apply(ComputationContext ctx);`
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        unimplemented!("apply() must be implemented by concrete subclasses (Sigmoid, Relu, etc.)")
    }

    /// Gradient: Template method that validates parent and calls gradient_for_parent.
    /// Java: `public final Tensor<?> gradient(Variable<?> variable, ComputationContext ctx)`
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        self.validate_parent(parent);
        self.gradient_for_parent(ctx)
    }

    // DELEGATION: Forward to AbstractVariable
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }

    // DELEGATION: Forward to AbstractVariable
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    // DELEGATION: Forward to AbstractVariable
    fn parents(&self) -> &[Box<dyn Variable>] {
        // Return a slice containing our parent
        std::slice::from_ref(&self.parent)
    }
}

impl std::fmt::Display for SingleParentVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SingleParentVariable")
    }
}

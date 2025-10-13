//! Single parent variable base implementation for ML functions in GDS.
//!
//! Translated from Java GDS ml-core functions SingleParentVariable.
//! This is a literal 1:1 translation following repository translation policy.

use super::abstract_variable::AbstractVariable;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::{ComputationContext, Variable};
use std::rc::Rc;

/// Abstract base class for variables with a single parent.
///
/// This provides common functionality for functions that have exactly one input.
pub struct SingleParentVariable<P: Tensor, T: Tensor> {
    base: AbstractVariable<T>,
    parent: Rc<dyn Variable<P>>,
}

impl<P: Tensor, T: Tensor> SingleParentVariable<P, T> {
    /// Create a new single parent variable.
    pub fn new(parent: Rc<dyn Variable<P>>, dimensions: Vec<usize>) -> Self {
        let parents = vec![parent.clone()];
        Self {
            base: AbstractVariable::new(parents, dimensions),
            parent,
        }
    }

    /// Get the parent variable.
    pub fn parent(&self) -> &Rc<dyn Variable<P>> {
        &self.parent
    }

    /// Validate that the given variable is our parent.
    pub fn validate_parent(&self, variable: &dyn Variable<P>) {
        if !Rc::ptr_eq(&self.parent, &Rc::new(variable)) {
            panic!("Calling gradient with a `parent` that was not expected");
        }
    }
}

impl<P: Tensor, T: Tensor> Variable<T> for SingleParentVariable<P, T> {
    fn apply(&self, ctx: &ComputationContext) -> T {
        // This should be overridden by subclasses
        unimplemented!("apply should be implemented by subclasses")
    }

    fn gradient(&self, variable: &dyn Variable<T>, ctx: &ComputationContext) -> T {
        self.validate_parent(variable);
        self.gradient_for_parent(ctx)
    }

    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    fn parents(&self) -> &[Rc<dyn Variable<T>>] {
        self.base.parents()
    }
}

/// Trait for single parent variables to implement gradient computation.
pub trait SingleParentGradient<P: Tensor, T: Tensor> {
    /// Compute the gradient with respect to the parent.
    fn gradient_for_parent(&self, ctx: &ComputationContext) -> P;
}

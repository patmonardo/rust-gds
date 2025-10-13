//! Abstract variable base implementation for ML functions in GDS.
//!
//! Translated from Java GDS ml-core AbstractVariable.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::rc::Rc;

/// Abstract base implementation of Variable.
///
/// This provides common functionality for all variable implementations.
pub struct AbstractVariable<T: Tensor> {
    parents: Vec<Rc<dyn Variable<T>>>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

/// Abstract base implementation of Variable.
///
/// This provides common functionality for all variable implementations.
pub struct AbstractVariable<T: Tensor> {
    parents: Vec<Rc<dyn Variable<T>>>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl<T: Tensor> AbstractVariable<T> {
    /// Create a new abstract variable.
    pub fn new(parents: Vec<Rc<dyn Variable<T>>>, dimensions: Vec<usize>) -> Self {
        Self {
            parents,
            dimensions,
            require_gradient: true,
        }
    }

    /// Create a new abstract variable that doesn't require gradients.
    pub fn new_no_gradient(parents: Vec<Rc<dyn Variable<T>>>, dimensions: Vec<usize>) -> Self {
        Self {
            parents,
            dimensions,
            require_gradient: false,
        }
    }

    /// Get the parents.
    pub fn parents(&self) -> &[Rc<dyn Variable<T>>] {
        &self.parents
    }

    /// Get the dimensions.
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Check if gradients are required.
    pub fn require_gradient(&self) -> bool {
        self.require_gradient
    }
}

//! Abstract variable base implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core AbstractVariable.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::fmt;

/// Error type for functions that are not actually functions (like constants).
#[derive(Debug, Clone)]
pub struct NotAFunctionException;

impl fmt::Display for NotAFunctionException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not a function")
    }
}

impl std::error::Error for NotAFunctionException {}

/// Abstract base implementation of Variable trait.
///
/// Provides common functionality for dimension handling, parent tracking,
/// and gradient requirements. Uses type erasure - no generic parameter.
pub struct AbstractVariable {
    dimensions: Vec<usize>,
    require_gradient: bool,
    parents: Vec<Box<dyn Variable>>,
}

impl AbstractVariable {
    /// Create a new abstract variable.
    pub fn new(parents: Vec<Box<dyn Variable>>, dimensions: Vec<usize>) -> Self {
        let require_gradient = Self::any_parent_requires_gradient(&parents);
        Self {
            dimensions,
            require_gradient,
            parents,
        }
    }

    /// Check if any parent requires gradient.
    fn any_parent_requires_gradient(parents: &[Box<dyn Variable>]) -> bool {
        parents.iter().any(|parent| parent.require_gradient())
    }

    /// Get a specific dimension by index.
    pub fn dimension(&self, dimension_index: usize) -> usize {
        self.dimensions[dimension_index]
    }
}

impl Variable for AbstractVariable {
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("AbstractVariable::apply must be implemented by subclass")
    }

    fn gradient(&self, _parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("AbstractVariable::gradient must be implemented by subclass")
    }

    fn require_gradient(&self) -> bool {
        self.require_gradient
    }

    fn parents(&self) -> &[Box<dyn Variable>] {
        &self.parents
    }

    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }
}

impl fmt::Display for AbstractVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}, requireGradient: {}",
            std::any::type_name::<Self>()
                .split("::")
                .last()
                .unwrap_or("AbstractVariable"),
            dimensions::render(&self.dimensions),
            self.require_gradient
        )
    }
}

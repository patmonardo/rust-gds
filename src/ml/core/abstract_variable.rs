//! Abstract variable base implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core AbstractVariable.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::dimensions;
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

/// Abstract base implementation shared by all variables.
///
/// Matches Java's `AbstractVariable<T extends Tensor<T>>` by providing
/// parent tracking, dimension handling, and gradient requirements while
/// leaving function-specific behaviour to concrete types.
pub struct AbstractVariable {
    parents: Vec<Box<dyn Variable>>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl AbstractVariable {
    /// Java: `protected AbstractVariable(List<? extends Variable<?>> parents, int[] dimensions)`
    pub fn new(parents: Vec<Box<dyn Variable>>, dimensions: Vec<usize>) -> Self {
        let require_gradient = Self::any_parent_requires_gradient(&parents);
        Self {
            parents,
            dimensions,
            require_gradient,
        }
    }

    /// Convenience constructor for leaf variables that control the gradient flag.
    pub fn with_gradient_requirement(
        parents: Vec<Box<dyn Variable>>,
        dimensions: Vec<usize>,
        require_gradient: bool,
    ) -> Self {
        Self {
            parents,
            dimensions,
            require_gradient,
        }
    }

    /// Java: `public List<? extends Variable<?>> parents()`
    pub fn parents(&self) -> &[Box<dyn Variable>] {
        &self.parents
    }

    /// Java: `public int[] dimensions()`
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Java: `public int dimension(int dimensionIndex)`
    pub fn dimension(&self, dimension_index: usize) -> usize {
        self.dimensions[dimension_index]
    }

    /// Java: `public boolean requireGradient()`
    pub fn require_gradient(&self) -> bool {
        self.require_gradient
    }

    /// Helper used by constructors.
    fn any_parent_requires_gradient(parents: &[Box<dyn Variable>]) -> bool {
        parents.iter().any(|parent| parent.require_gradient())
    }

    /// Helper for toString-style formatting. Mirrors `Dimensions.render` usage.
    pub fn render_dimensions(&self) -> String {
        dimensions::render(&self.dimensions)
    }
}

impl fmt::Display for AbstractVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AbstractVariable: {}, requireGradient: {}",
            self.render_dimensions(),
            self.require_gradient
        )
    }
}

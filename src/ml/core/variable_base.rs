//! VariableBase - shared storage and logic for all variable types.
//!
//! This struct replaces Java's abstract AbstractVariable<T> base class.
//! It contains the storage (dimensions, parents, require_gradient) and all shared methods
//! that were inherited in Java's class hierarchy.
//!
//! Design Pattern: Composition + Delegation
//! - Functions (Constant, Weights, etc.) WRAP a VariableBase (composition)
//! - They delegate dimension/parent/gradient tracking to their inner VariableBase
//! - They implement only function-specific logic themselves
//!
//! This achieves the same goal as Java's inheritance but using Rust idioms.
//!
//! Translated from Java GDS ml-core AbstractVariable.java

use crate::ml::core::variable::Variable;
use std::fmt;

/// Shared storage and logic for all variable types.
///
/// This is the "base class" equivalent from Java's AbstractVariable<T>.
/// Contains the protected fields (dimensions, parents, require_gradient) and all
/// concrete methods that were inherited by Constant, Weights, and ML functions.
///
/// Note: Not Clone - Variable trait objects can't be cloned generically.
/// Functions that wrap VariableBase must implement their own Clone logic.
pub struct VariableBase {
    dimensions: Vec<usize>,
    require_gradient: bool,
    parents: Vec<Box<dyn Variable>>,
}

impl VariableBase {
    /// Create new VariableBase.
    ///
    /// Automatically determines require_gradient from parents.
    /// Java: `protected AbstractVariable(List<? extends Variable<?>> parents, int[] dimensions)`
    pub fn new(parents: Vec<Box<dyn Variable>>, dimensions: Vec<usize>) -> Self {
        let require_gradient = Self::any_parent_requires_gradient(&parents);
        Self {
            dimensions,
            require_gradient,
            parents,
        }
    }

    /// Create VariableBase with explicit gradient requirement.
    ///
    /// Use this for leaf variables (Constants, Weights) that may need gradients
    /// regardless of parents.
    pub fn with_gradient_requirement(
        parents: Vec<Box<dyn Variable>>,
        dimensions: Vec<usize>,
        require_gradient: bool,
    ) -> Self {
        Self {
            dimensions,
            require_gradient,
            parents,
        }
    }

    // ========================================================================
    // Accessors - match Java's private fields with public getters
    // ========================================================================

    /// Get reference to dimensions.
    /// Java: `public int[] dimensions()`
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Get a specific dimension by index.
    /// Java: `public int dimension(int dimensionIndex)`
    pub fn dimension(&self, dimension_index: usize) -> usize {
        self.dimensions[dimension_index]
    }

    /// Get reference to parent variables.
    /// Java: `public List<? extends Variable<?>> parents()`
    pub fn parents(&self) -> &[Box<dyn Variable>] {
        &self.parents
    }

    /// Check if this variable requires gradient computation.
    /// Java: `public boolean requireGradient()`
    pub fn require_gradient(&self) -> bool {
        self.require_gradient
    }

    // ========================================================================
    // Shared Logic - from Java AbstractVariable
    // ========================================================================

    /// Check if any parent requires gradient computation.
    /// Java: `private boolean anyParentRequiresGradient()`
    fn any_parent_requires_gradient(parents: &[Box<dyn Variable>]) -> bool {
        parents.iter().any(|parent| parent.require_gradient())
    }

    /// Render dimensions for display.
    /// Helper for toString() implementations.
    pub fn render_dimensions(&self) -> String {
        crate::ml::core::dimensions::render(&self.dimensions)
    }
}

impl fmt::Debug for VariableBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariableBase")
            .field("dimensions", &self.dimensions)
            .field("require_gradient", &self.require_gradient)
            .field("parents_count", &self.parents.len())
            .finish()
    }
}

impl fmt::Display for VariableBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VariableBase: {}, requireGradient: {}",
            self.render_dimensions(),
            self.require_gradient
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::dimensions;

    #[test]
    fn test_variable_base_creation_no_parents() {
        let base = VariableBase::new(vec![], dimensions::vector(3));
        assert_eq!(base.dimensions(), &[3, 1]);
        assert!(!base.require_gradient());
        assert_eq!(base.parents().len(), 0);
    }

    #[test]
    fn test_variable_base_with_explicit_gradient() {
        let base = VariableBase::with_gradient_requirement(vec![], dimensions::matrix(2, 3), true);
        assert_eq!(base.dimensions(), &[2, 3]);
        assert!(base.require_gradient());
    }

    #[test]
    fn test_dimension_access() {
        let base = VariableBase::new(vec![], dimensions::matrix(2, 3));
        assert_eq!(base.dimension(0), 2);
        assert_eq!(base.dimension(1), 3);
    }

    #[test]
    fn test_render_dimensions() {
        let base = VariableBase::new(vec![], dimensions::vector(5));
        let rendered = base.render_dimensions();
        assert_eq!(rendered, "Matrix(5, 1)");
    }

    // Note: Testing with actual Variable parents requires creating mock Variables,
    // which we'll test in integration tests after refactoring functions.
}

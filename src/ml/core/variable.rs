//! Variable trait for ML computations in GDS.
//!
//! Translated from Java GDS ml-core Variable.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! ## Type Erasure Design
//!
//! This implementation uses full type erasure (Option C) to match Java's runtime behavior.
//! The Variable trait has NO generic parameter, and all Tensor returns are boxed.
//! This makes the trait fully object-safe, enabling heterogeneous variable trees.
//!
//! Java's `Variable<T extends Tensor<T>>` uses wildcards (`Variable<?>`) which erase
//! to just `Variable` at runtime. Our Rust equivalent: `Box<dyn Variable>` works
//! identically, with `Box<dyn Tensor>` for tensor values.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use std::any::Any;
use std::fmt;

/// Trait for variables in ML computations.
///
/// This corresponds to the Variable interface in Java GDS.
/// All tensor values are type-erased via `Box<dyn Tensor>`.
/// Extends Any for downcasting and pointer-based keying.
pub trait Variable: fmt::Display + Any {
    /// Apply this variable to get its tensor value.
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;

    /// Compute gradient with respect to a parent variable.
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor>;

    /// Whether this variable requires gradient computation.
    fn require_gradient(&self) -> bool;

    /// Get parent variables.
    fn parents(&self) -> &[Box<dyn Variable>];

    /// Get dimensions of this variable.
    fn dimensions(&self) -> &[usize];

    /// Get a specific dimension by index.
    fn dimension(&self, i: usize) -> usize {
        self.dimensions()[i]
    }

    /// Renders the variable into a human readable representation.
    fn render(&self) -> String
    where
        Self: Sized,
    {
        let mut sb = String::new();
        render_recursive(&mut sb, self, 0);
        sb
    }
}

/// Helper function for recursive rendering (static method in Java).
///
/// This is a free function following the established pattern.
/// With type erasure, this is now straightforward - all variables are `dyn Variable`.
pub fn render_recursive(sb: &mut String, variable: &dyn Variable, depth: usize) {
    // Render indentation
    for _ in 0..depth.saturating_sub(1) {
        sb.push('\t');
    }

    if depth > 0 {
        sb.push_str("|-- ");
    }

    sb.push_str(&variable.to_string());
    sb.push('\n');

    for parent in variable.parents() {
        render_recursive(sb, parent.as_ref(), depth + 1);
    }
}

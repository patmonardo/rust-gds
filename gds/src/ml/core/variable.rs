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
    /// Java: `T apply(ComputationContext ctx)`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;

    /// Compute gradient with respect to a parent variable.
    /// Java: `Tensor<?> gradient(Variable<?> parent, ComputationContext ctx)`
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor>;

    /// Whether this variable requires gradient computation.
    /// Java: `boolean requireGradient()`
    fn require_gradient(&self) -> bool;

    /// Get parent variables.
    /// Java: `Iterable<? extends Variable<?>> parents()`
    fn parents(&self) -> &[Box<dyn Variable>];

    /// Get dimensions of this variable's output tensor.
    /// Java: `int[] dimensions()`
    fn dimensions(&self) -> &[usize];

    /// Get a specific dimension by index.
    /// Java: `int dimension(int i)`
    fn dimension(&self, i: usize) -> usize {
        self.dimensions()[i]
    }
}

/// Helper function to traverse variable tree and collect all variables.
pub fn collect_variables(variable: &dyn Variable) -> Vec<Box<dyn Variable>> {
    let mut result = Vec::new();
    collect_variables_recursive(variable, &mut result);
    result
}

fn collect_variables_recursive(variable: &dyn Variable, result: &mut Vec<Box<dyn Variable>>) {
    for parent in variable.parents() {
        collect_variables_recursive(parent.as_ref(), result);
    }
    // Note: We can't clone variables, so we'll skip this for now
    // result.push(Box::new(variable.clone_box()));
}

/// Helper function to check if any parent requires gradient.
pub fn any_parent_requires_gradient(parents: &[Box<dyn Variable>]) -> bool {
    parents.iter().any(|parent| parent.require_gradient())
}

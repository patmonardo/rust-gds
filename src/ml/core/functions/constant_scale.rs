//! Constant scale function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ConstantScale.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::abstract_variable::AbstractVariable;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::fmt;

/// A function that scales a tensor by a constant value.
///
/// Corresponds to ConstantScale in Java GDS.
/// Uses composition with AbstractVariable to match Java's inheritance pattern.
pub struct ConstantScale {
    base: AbstractVariable,
    parent: Box<dyn Variable>,
    constant: f64,
}

impl ConstantScale {
    pub fn new(parent: Box<dyn Variable>, constant: f64) -> Self {
        let dimensions = parent.dimensions().to_vec();
        let require_gradient = parent.require_gradient();
        let base = AbstractVariable::with_gradient_requirement(vec![], dimensions, require_gradient);
        Self {
            base,
            parent,
            constant,
        }
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let gradient = ctx.gradient(self).expect("Gradient must be computed");
        gradient.scalar_multiply(self.constant)
    }
}

impl Variable for ConstantScale {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_data = ctx.data(self.parent.as_ref()).expect("Parent not computed");
        parent_data.scalar_multiply(self.constant)
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        // Verify parent matches
        assert!(
            std::ptr::eq(parent, self.parent.as_ref()),
            "Gradient requested for unknown parent"
        );
        self.gradient_for_parent(ctx)
    }

    // DELEGATION: Forward to AbstractVariable
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    // DELEGATION: Forward to AbstractVariable
    fn parents(&self) -> &[Box<dyn Variable>] {
        std::slice::from_ref(&self.parent)
    }

    // DELEGATION: Forward to AbstractVariable
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for ConstantScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConstantScale: {}, requireGradient: {}",
            self.constant, self.require_gradient()
        )
    }
}

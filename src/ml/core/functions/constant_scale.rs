//! Constant scale function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ConstantScale.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::fmt;

/// A function that scales a tensor by a constant value.
///
/// Corresponds to ConstantScale in Java GDS.
pub struct ConstantScale {
    parent: Box<dyn Variable>,
    constant: f64,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl ConstantScale {
    pub fn new(parent: Box<dyn Variable>, constant: f64) -> Self {
        let dimensions = parent.dimensions().to_vec();
        let require_gradient = parent.require_gradient();
        Self {
            parent,
            constant,
            dimensions,
            require_gradient,
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

    fn require_gradient(&self) -> bool {
        self.require_gradient
    }

    fn parents(&self) -> &[Box<dyn Variable>] {
        std::slice::from_ref(&self.parent)
    }

    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }
}

impl fmt::Display for ConstantScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConstantScale: {}, requireGradient: {}",
            self.constant, self.require_gradient
        )
    }
}

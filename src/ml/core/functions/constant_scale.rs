//! Constant scale function implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ConstantScale.
//! This is a literal 1:1 translation following repository translation policy.

use super::single_parent_variable::{SingleParentGradient, SingleParentVariable};
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::{ComputationContext, Variable};
use std::rc::Rc;

/// A function that scales a tensor by a constant value.
pub struct ConstantScale<T: Tensor> {
    base: SingleParentVariable<T, T>,
    constant: f64,
}

impl<T: Tensor + Clone> ConstantScale<T> {
    /// Create a new constant scale function.
    pub fn new(parent: Rc<dyn Variable<T>>, constant: f64) -> Self {
        let dimensions = parent.dimensions().to_vec();
        Self {
            base: SingleParentVariable::new(parent, dimensions),
            constant,
        }
    }

    /// Get the constant scaling factor.
    pub fn constant(&self) -> f64 {
        self.constant
    }
}

impl<T: Tensor + Clone> Variable<T> for ConstantScale<T> {
    fn apply(&self, ctx: &ComputationContext) -> T {
        let parent_data = ctx.data(self.base.parent().as_ref());
        parent_data.scalar_multiply(self.constant)
    }

    fn gradient(&self, variable: &dyn Variable<T>, ctx: &ComputationContext) -> T {
        self.base.gradient(variable, ctx)
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

impl<T: Tensor + Clone> SingleParentGradient<T, T> for ConstantScale<T> {
    fn gradient_for_parent(&self, ctx: &ComputationContext) -> T {
        let gradient = ctx.gradient(self);
        gradient.scalar_multiply(self.constant)
    }
}

impl<T: Tensor + Clone> std::fmt::Display for ConstantScale<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConstantScale: scale by {}, requireGradient: {}",
            self.constant,
            self.require_gradient()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::functions::Constant;
    use crate::ml::core::tensor::{Scalar, Vector};

    #[test]
    fn test_scalar_constant_scale() {
        let parent = Rc::new(Constant::scalar(2.0));
        let scale = ConstantScale::new(parent, 3.0);
        let mut ctx = ComputationContext::new();

        // Set up parent data in context
        ctx.set_data(scale.parents()[0].as_ref(), Scalar::new(2.0));

        let result = scale.apply(&ctx);
        assert_eq!(result.value(), 6.0); // 2.0 * 3.0

        assert_eq!(scale.dimensions(), &[]);
        assert!(scale.require_gradient());
    }

    #[test]
    fn test_vector_constant_scale() {
        let data = [1.0, 2.0, 3.0];
        let parent = Rc::new(Constant::vector(&data));
        let scale = ConstantScale::new(parent, 2.0);
        let mut ctx = ComputationContext::new();

        // Set up parent data in context
        ctx.set_data(scale.parents()[0].as_ref(), Vector::new(&data));

        let result = scale.apply(&ctx);
        assert_eq!(result.data(), &[2.0, 4.0, 6.0]); // [1,2,3] * 2

        assert_eq!(scale.dimensions(), &[3]);
        assert!(scale.require_gradient());
    }

    #[test]
    fn test_constant_scale_gradient() {
        let parent = Rc::new(Constant::scalar(2.0));
        let scale = ConstantScale::new(parent.clone(), 3.0);
        let mut ctx = ComputationContext::new();

        // Set up gradient in context
        ctx.set_gradient(&scale, Scalar::new(5.0));

        let gradient = scale.gradient_for_parent(&ctx);
        assert_eq!(gradient.value(), 15.0); // 5.0 * 3.0
    }
}

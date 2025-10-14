//! L2 norm squared function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions L2NormSquared.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Scalar, Tensor};
use crate::ml::core::variable::Variable;
use std::fmt;

/// L2 norm squared of a tensor.
///
/// Computes the sum of squared elements. Corresponds to L2NormSquared in Java GDS.
pub struct L2NormSquared {
    parent: Box<dyn Variable>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl L2NormSquared {
    pub fn new(parent: Box<dyn Variable>) -> Self {
        let require_gradient = parent.require_gradient();
        Self {
            parent,
            dimensions: dimensions::scalar(),
            require_gradient,
        }
    }

    pub fn size_in_bytes_of_apply() -> usize {
        crate::ml::core::tensor::size_in_bytes(&[1])
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_gradient = ctx
            .gradient(self)
            .expect("Self gradient not computed")
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let parent_data = ctx
            .data(self.parent.as_ref())
            .expect("Parent data not computed");

        parent_data.scalar_multiply(2.0 * self_gradient)
    }
}

impl Variable for L2NormSquared {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_matrix = ctx
            .data(self.parent.as_ref())
            .expect("Parent data not computed");

        let length = parent_matrix.total_size();
        let mut l2_norm = 0.0;

        for idx in 0..length {
            let value = parent_matrix.data()[idx];
            l2_norm += value * value;
        }

        Box::new(Scalar::new(l2_norm))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
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

impl fmt::Display for L2NormSquared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L2NormSquared")
    }
}

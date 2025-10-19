//! Element sum function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ElementSum.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::tensor::{Scalar, Tensor};
use crate::ml::core::variable::Variable;
use std::fmt;

/// Sums all elements across multiple parent tensors into a scalar.
///
/// Corresponds to ElementSum in Java GDS.
pub struct ElementSum {
    parents: Vec<Box<dyn Variable>>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl ElementSum {
    pub fn new(parents: Vec<Box<dyn Variable>>) -> Self {
        let require_gradient = parents.iter().any(|p| p.require_gradient());
        Self {
            parents,
            dimensions: dimensions::scalar(),
            require_gradient,
        }
    }
}

impl Variable for ElementSum {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let mut sum = 0.0;
        for parent in &self.parents {
            let parent_data = ctx.data(parent.as_ref()).expect("Parent data not computed");
            sum += parent_data.aggregate_sum();
        }
        Box::new(Scalar::new(sum))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_gradient = ctx
            .gradient(self)
            .expect("Self gradient not computed")
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let parent_data = ctx.data(parent).expect("Parent data not computed");

        // Create a tensor of same shape filled with self_gradient value
        parent_data.ones_like().scalar_multiply(self_gradient)
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

impl fmt::Display for ElementSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ElementSum: {}, requireGradient: {}",
            dimensions::render(&self.dimensions),
            self.require_gradient
        )
    }
}

//! Normalize rows function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions NormalizeRows.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

const EPSILON: f64 = 1e-10;

/// Normalizes each row of a matrix by its L2 norm.
///
/// Corresponds to NormalizeRows in Java GDS, extends SingleParentVariable.
/// Uses composition pattern: VariableBase holds parent (matrix to normalize).
pub struct NormalizeRows {
    base: VariableBase,
}

impl NormalizeRows {
    pub fn new(parent: Box<dyn Variable>) -> Self {
        let dimensions = parent.dimensions().to_vec();
        let base = VariableBase::new(vec![parent], dimensions);
        Self { base }
    }

    /// Helper to access parent matrix
    fn parent(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_data = parent_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let gradient_tensor = ctx.gradient(self).expect("Gradient not computed");
        let normalize_rows_gradient = gradient_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Gradient must be Matrix");

        let rows = parent_data.rows();
        let cols = parent_data.cols();
        let mut parent_grad_matrix = Matrix::with_dimensions(rows, cols);

        for row in 0..rows {
            let mut l2_squared = 0.0;
            for col in 0..cols {
                let cell_value = parent_data.data_at(row, col);
                l2_squared += cell_value * cell_value;
            }
            let l2 = l2_squared.sqrt();
            let l2_cubed = l2 * l2_squared;

            if l2_cubed == 0.0 {
                continue;
            }

            for col in 0..cols {
                let parent_cell_value = parent_data.data_at(row, col);
                for grad_col in 0..cols {
                    let partial_gradient = if col == grad_col {
                        normalize_rows_gradient.data_at(row, col)
                            * (l2_squared - parent_cell_value * parent_cell_value)
                    } else {
                        -normalize_rows_gradient.data_at(row, grad_col)
                            * (parent_cell_value * parent_data.data_at(row, grad_col))
                    };
                    parent_grad_matrix.add_data_at(row, col, partial_gradient);
                }

                let current = parent_grad_matrix.data_at(row, col);
                parent_grad_matrix.set_data_at(row, col, current / l2_cubed);
            }
        }

        Box::new(parent_grad_matrix)
    }
}

impl Variable for NormalizeRows {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_matrix = parent_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let rows = parent_matrix.rows();
        let cols = parent_matrix.cols();

        let mut result = parent_matrix.create_with_same_dimensions();
        let result_matrix = result
            .as_any_mut()
            .downcast_mut::<Matrix>()
            .expect("Result must be Matrix");

        for row in 0..rows {
            let mut squared_sum = 0.0;
            for col in 0..cols {
                let value = parent_matrix.data_at(row, col);
                squared_sum += value * value;
            }

            // Adding EPSILON to avoid division by zero
            let l2 = squared_sum.sqrt() + EPSILON;
            for col in 0..cols {
                result_matrix.set_data_at(row, col, parent_matrix.data_at(row, col) / l2);
            }
        }

        result
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        assert!(
            std::ptr::eq(parent, self.parent()),
            "Gradient requested for unknown parent"
        );
        self.gradient_for_parent(ctx)
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    fn parents(&self) -> &[Box<dyn Variable>] {
        self.base.parents()
    }

    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for NormalizeRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NormalizeRows")
    }
}

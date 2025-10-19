//! Reduced softmax function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ReducedSoftmax.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use std::fmt;

/// Computes the softmax for all classes except the last one which is
/// implicitly 1 - sum(output[i]) where i goes over all the other classes.
///
/// Corresponds to ReducedSoftmax in Java GDS.
pub struct ReducedSoftmax {
    parent: Box<dyn Variable>,
    dimensions: Vec<usize>,
    require_gradient: bool,
}

impl ReducedSoftmax {
    pub fn new(parent: Box<dyn Variable>) -> Self {
        let dimensions = vec![
            parent.dimension(ROWS_INDEX),
            parent.dimension(COLUMNS_INDEX) + 1,
        ];
        let require_gradient = parent.require_gradient();

        Self {
            parent,
            dimensions,
            require_gradient,
        }
    }

    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[rows, cols - 1])
    }

    fn rescale(result: &mut Matrix) {
        let rows = result.rows();
        let cols = result.cols();

        for row in 0..rows {
            let mut row_sum = 1e-15;
            for col in 0..cols {
                let current = result.data_at(row, col);
                row_sum += current;
            }
            for col in 0..cols {
                let current = result.data_at(row, col);
                result.set_data_at(row, col, current / row_sum);
            }
        }
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_data_tensor = ctx.data(self).expect("Self data not computed");
        let self_data = self_data_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Self data must be Matrix");

        let self_gradient_tensor = ctx.gradient(self).expect("Self gradient not computed");
        let self_gradient = self_gradient_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Self gradient must be Matrix");

        let rows = self_data.rows();
        let cols = self_data.cols();

        let mut computed_gradient = Matrix::create(0.0, rows, cols - 1);

        // result[row,col] = sum_{col2} s[row, col2] * (delta(col, col2) - s[row, col]) * grad[row, col2]
        for row in 0..rows {
            for col in 0..(cols - 1) {
                let softmax_data = self_data.data_at(row, col);
                for softmax_col in 0..cols {
                    let value_at_other_column = self_data.data_at(row, softmax_col);
                    let gradient_at_other_column = self_gradient.data_at(row, softmax_col);
                    let impact_of_changing_any_column =
                        if col == softmax_col { 1.0 } else { 0.0 } - softmax_data;

                    let gradient_value = value_at_other_column
                        * impact_of_changing_any_column
                        * gradient_at_other_column;
                    computed_gradient.add_data_at(row, col, gradient_value);
                }
            }
        }

        Box::new(computed_gradient)
    }
}

impl Variable for ReducedSoftmax {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let data_tensor = ctx
            .data(self.parent.as_ref())
            .expect("Parent data not computed");
        let data = data_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let rows = data.rows();
        let cols = data.cols() + 1;

        let mut result = Matrix::create(0.0, rows, cols);
        let mut rescale = false;

        for row in 0..rows {
            let mut row_sum = 0.0;
            for col in 0..cols {
                let exp = if col == cols - 1 {
                    1.0
                } else {
                    data.data_at(row, col).exp()
                };

                let exp = if exp.is_infinite() {
                    rescale = true;
                    f64::MAX
                } else {
                    exp
                };

                result.set_data_at(row, col, exp);
                row_sum += exp;

                if row_sum.is_infinite() {
                    rescale = true;
                    row_sum = f64::MAX;
                }
            }

            for col in 0..cols {
                let current = result.data_at(row, col);
                result.set_data_at(row, col, current / row_sum);
            }
        }

        if rescale {
            Self::rescale(&mut result);
        }

        Box::new(result)
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

impl fmt::Display for ReducedSoftmax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReducedSoftmax")
    }
}

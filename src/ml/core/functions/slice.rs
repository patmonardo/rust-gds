//! Slice function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions Slice.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{self, COLUMNS_INDEX};
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

/// Slices a matrix by selecting specific rows via batch IDs.
///
/// Corresponds to Slice in Java GDS, extends SingleParentVariable<Matrix, Matrix>.
/// Uses composition pattern: VariableBase holds parent (matrix to slice).
pub struct Slice {
    base: VariableBase,
    batch_ids: Vec<usize>,
}

impl Slice {
    pub fn new(parent: Box<dyn Variable>, batch_ids: Vec<usize>) -> Self {
        let dimensions = dimensions::matrix(batch_ids.len(), parent.dimension(COLUMNS_INDEX));
        let base = VariableBase::new(vec![parent], dimensions);

        Self { base, batch_ids }
    }

    /// Helper to access parent matrix
    fn parent(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let gradient_tensor = ctx.gradient(self).expect("Gradient not computed");
        let this_gradient = gradient_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Gradient must be Matrix");

        let parent_data_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_data = parent_data_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let mut result_matrix = Matrix::with_dimensions(parent_data.rows(), parent_data.cols());

        let rows = self.batch_ids.len();
        let cols = this_gradient.cols();

        for row in 0..rows {
            let child_row = self.batch_ids[row];
            for col in 0..cols {
                result_matrix.add_data_at(child_row, col, this_gradient.data_at(row, col));
            }
        }

        Box::new(result_matrix)
    }
}

impl Variable for Slice {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_data = parent_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let rows = self.batch_ids.len();
        let mut result = Matrix::create(0.0, rows, parent_data.cols());

        for row in 0..rows {
            result.set_row(row, parent_data, self.batch_ids[row]);
        }

        Box::new(result)
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

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slice")
    }
}

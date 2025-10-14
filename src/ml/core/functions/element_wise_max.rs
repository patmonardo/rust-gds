//! Element-wise max function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions ElementWiseMax.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::subgraph::BatchNeighbors;
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

const INVALID_NEIGHBOR: i32 = -1;

/// Computes element-wise maximum of node features and their neighbors' features.
///
/// For each batch node, finds the maximum value among weighted neighbor features.
/// Corresponds to ElementWiseMax in Java GDS, extends SingleParentVariable.
/// Uses composition pattern: VariableBase holds parent (matrix to aggregate).
pub struct ElementWiseMax {
    base: VariableBase,
    batch_neighbors: Box<dyn BatchNeighbors>,
}

impl ElementWiseMax {
    pub fn new(parent: Box<dyn Variable>, batch_neighbors: Box<dyn BatchNeighbors>) -> Self {
        assert!(
            parent.dimension(ROWS_INDEX) >= batch_neighbors.node_count(),
            "Expecting a row for each node in the subgraph"
        );

        let dimensions = vec![
            batch_neighbors.batch_size(),
            parent.dimension(COLUMNS_INDEX),
        ];
        let base = VariableBase::new(vec![parent], dimensions);

        Self {
            base,
            batch_neighbors,
        }
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

        let mut result_matrix = Matrix::with_dimensions(parent_data.rows(), parent_data.cols());
        let cols = result_matrix.cols();

        let gradient_tensor = ctx.gradient(self).expect("Gradient not computed");
        let element_wise_max_gradient = gradient_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Gradient must be Matrix");

        let self_tensor = ctx.data(self).expect("Self data not computed");
        let element_wise_max_data = self_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Self data must be Matrix");

        let batch_ids = self.batch_neighbors.batch_ids();

        for (batch_idx, &source_id) in batch_ids.iter().enumerate() {
            let neighbors = self.batch_neighbors.neighbors(source_id);
            let degree = neighbors.len();
            let mut cached_weights = vec![0.0; degree];

            for (neighbor_index, &neighbor) in neighbors.iter().enumerate() {
                cached_weights[neighbor_index] = self
                    .batch_neighbors
                    .relationship_weight(source_id, neighbor);
            }

            for col in 0..cols {
                let this_cell_data = element_wise_max_data.data_at(batch_idx, col);

                let mut min_diff_to_cell_data = f64::MAX;
                let mut max_neighbor = INVALID_NEIGHBOR;
                let mut max_neighbor_weight = f64::NAN;

                // Find neighbor that contributed the max value
                for (neighbor_index, &neighbor) in neighbors.iter().enumerate() {
                    let relationship_weight = cached_weights[neighbor_index];

                    let diff_to_cell_data = (this_cell_data
                        - (parent_data.data_at(neighbor, col) * relationship_weight))
                        .abs();

                    if diff_to_cell_data < min_diff_to_cell_data {
                        min_diff_to_cell_data = diff_to_cell_data;
                        max_neighbor = neighbor as i32;
                        max_neighbor_weight = relationship_weight;
                    }
                }

                if max_neighbor == INVALID_NEIGHBOR {
                    assert_eq!(degree, 0);
                    continue;
                }

                // Propagate gradient to the neighbor's cell
                result_matrix.add_data_at(
                    max_neighbor as usize,
                    col,
                    element_wise_max_gradient.data_at(batch_idx, col) * max_neighbor_weight,
                );
            }
        }

        Box::new(result_matrix)
    }
}

impl Variable for ElementWiseMax {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_data = parent_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let rows = self.batch_neighbors.batch_size();
        let cols = parent_data.cols();
        let batch_ids = self.batch_neighbors.batch_ids();

        let mut max = Matrix::create(f64::NEG_INFINITY, rows, cols);

        for (batch_idx, &batch_node_id) in batch_ids.iter().enumerate() {
            // Find the maximum value among the neighbors' data for each cell in the row
            for &neighbor in self.batch_neighbors.neighbors(batch_node_id) {
                let relationship_weight = self
                    .batch_neighbors
                    .relationship_weight(batch_node_id, neighbor);
                for col in 0..cols {
                    let neighbor_value = parent_data.data_at(neighbor, col) * relationship_weight;
                    if neighbor_value >= max.data_at(batch_idx, col) {
                        max.set_data_at(batch_idx, col, neighbor_value);
                    }
                }
            }

            // Avoid Double.NEGATIVE_INFINITY entries for isolated batchNodes
            if self.batch_neighbors.degree(batch_node_id) == 0 {
                for col in 0..cols {
                    max.set_data_at(batch_idx, col, 0.0);
                }
            }
        }

        Box::new(max)
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

impl fmt::Display for ElementWiseMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElementWiseMax")
    }
}

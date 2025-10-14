//! Multi mean function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions MultiMean.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions::{COLUMNS_INDEX, ROWS_INDEX};
use crate::ml::core::subgraph::BatchNeighbors;
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::ml::core::variable::Variable;
use crate::ml::core::variable_base::VariableBase;
use std::fmt;

/// Computes the mean of node features and their neighbors' features.
///
/// For each batch node, aggregates its own features with weighted neighbor features.
/// Corresponds to MultiMean in Java GDS, extends SingleParentVariable.
/// Uses composition pattern: VariableBase holds parent (matrix to aggregate).
pub struct MultiMean {
    base: VariableBase,
    sub_graph: Box<dyn BatchNeighbors>,
}

impl MultiMean {
    pub fn new(parent: Box<dyn Variable>, sub_graph: Box<dyn BatchNeighbors>) -> Self {
        assert!(
            parent.dimension(ROWS_INDEX) >= sub_graph.node_count(),
            "Expecting a row for each node in the subgraph"
        );

        let dimensions = vec![sub_graph.batch_size(), parent.dimension(COLUMNS_INDEX)];
        let base = VariableBase::new(vec![parent], dimensions);

        Self { base, sub_graph }
    }

    /// Helper to access parent matrix
    fn parent(&self) -> &dyn Variable {
        self.base.parents()[0].as_ref()
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let gradient_tensor = ctx.gradient(self).expect("Gradient not computed");
        let multi_mean_gradient = gradient_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Gradient must be Matrix");

        let parent_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_data = parent_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let mut result_matrix = Matrix::with_dimensions(parent_data.rows(), parent_data.cols());
        let cols = result_matrix.cols();
        let batch_ids = self.sub_graph.batch_ids();

        for (batch_idx, &batch_node_id) in batch_ids.iter().enumerate() {
            let neighbors = self.sub_graph.neighbors(batch_node_id);
            let closed_neighborhood_degree = neighbors.len() + 1;

            // Pass gradient to batch node's data
            for col in 0..cols {
                let normalized_gradient =
                    multi_mean_gradient.data_at(batch_idx, col) / closed_neighborhood_degree as f64;
                result_matrix.add_data_at(batch_node_id, col, normalized_gradient);
            }

            // Propagate gradient to neighbors' data
            for &neighbor in neighbors {
                let relationship_weight =
                    self.sub_graph.relationship_weight(batch_node_id, neighbor);

                for col in 0..cols {
                    let neighbor_gradient =
                        multi_mean_gradient.data_at(batch_idx, col) * relationship_weight;
                    // add_data_at takes (row, col, value) for 2D indexing
                    result_matrix.add_data_at(
                        neighbor,
                        col,
                        neighbor_gradient / closed_neighborhood_degree as f64,
                    );
                }
            }
        }

        Box::new(result_matrix)
    }
}

impl Variable for MultiMean {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_tensor = ctx.data(self.parent()).expect("Parent data not computed");
        let parent_data = parent_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Parent must be Matrix");

        let batch_ids = self.sub_graph.batch_ids();
        let batch_size = batch_ids.len();
        let cols = parent_data.cols();

        let mut result_means = Matrix::create(0.0, batch_size, cols);

        for (batch_idx, &batch_node_id) in batch_ids.iter().enumerate() {
            let neighbors = self.sub_graph.neighbors(batch_node_id);
            // TODO: Replace with sum of weights to normalize the weights
            let closed_neighborhood_degree = neighbors.len() + 1;

            // Initialize mean row with parent row for nodeId in batch
            for col in 0..cols {
                let source_col_entry = parent_data.data_at(batch_node_id, col);
                result_means.add_data_at(
                    batch_idx,
                    col,
                    source_col_entry / closed_neighborhood_degree as f64,
                );
            }

            // Fetch rows from neighbors and update mean
            for &neighbor in neighbors {
                let relationship_weight =
                    self.sub_graph.relationship_weight(batch_node_id, neighbor);
                for col in 0..cols {
                    let neighbor_col_data =
                        parent_data.data_at(neighbor, col) * relationship_weight;
                    result_means.add_data_at(
                        batch_idx,
                        col,
                        neighbor_col_data / closed_neighborhood_degree as f64,
                    );
                }
            }
        }

        // TODO: Try to divide by numberOfEntries once instead of on every update

        Box::new(result_means)
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

impl fmt::Display for MultiMean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MultiMean")
    }
}

// BatchNeighbors trait is now available from crate::ml::core::subgraph
// No placeholder needed - use the trait bound directly

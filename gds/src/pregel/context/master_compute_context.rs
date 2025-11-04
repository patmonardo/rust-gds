//! MasterComputeContext - API for global coordination between supersteps
//!
//! Provides the master compute API for algorithm-level coordination.

use crate::core::utils::progress::tasks::LeafTask;
use crate::pregel::{NodeValue, PregelRuntimeConfig};
use crate::types::graph::Graph;
use std::sync::Arc;

/// Context for the master compute step.
///
/// The `MasterComputeContext` runs once per superstep (not per vertex) and provides:
/// - Global statistics and convergence checking
/// - Access to aggregated values across all vertices
/// - Ability to signal early termination
///
/// # Master Compute Phase
///
/// Unlike regular compute() which runs per-node, masterCompute() runs once after
/// each superstep completes. It's useful for:
/// - Checking convergence criteria
/// - Gathering global statistics
/// - Deciding whether to terminate early
/// - Logging progress
///
/// # TODO
///
/// This is a stub interface. Full implementation will include:
/// - Superstep number
/// - Node value access (read-only)
/// - Aggregator access
/// - Early termination signaling
/// - Parallel execution support via executor service
#[allow(dead_code)] // TODO: Remove once fully implemented
pub struct MasterComputeContext<C: PregelRuntimeConfig> {
    config: C,
    graph: Arc<dyn Graph>,
    iteration: usize,
    node_values: Arc<parking_lot::RwLock<NodeValue>>,
    progress_task: Option<Arc<LeafTask>>,
}

impl<C: PregelRuntimeConfig> MasterComputeContext<C> {
    /// Create a new MasterComputeContext.
    pub fn new(
        config: C,
        graph: Arc<dyn Graph>,
        iteration: usize,
        node_values: Arc<parking_lot::RwLock<NodeValue>>,
        progress_task: Option<Arc<LeafTask>>,
    ) -> Self {
        Self {
            config,
            graph,
            iteration,
            node_values,
            progress_task,
        }
    }
    /// Get the current superstep number (0-indexed).
    pub fn superstep(&self) -> usize {
        self.iteration
    }

    /// Returns true if this is the initial superstep (superstep 0).
    pub fn is_initial_superstep(&self) -> bool {
        self.superstep() == 0
    }

    /// Get the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the total number of relationships in the graph.
    pub fn relationship_count(&self) -> usize {
        self.graph.relationship_count()
    }

    /// Get the configuration.
    pub fn config(&self) -> &C {
        &self.config
    }

    /// Read a double node value by ID and schema key.
    ///
    /// This is the infrastructure for accessing node values during master compute.
    /// The actual algorithm logic (convergence checking, normalization) belongs in
    /// the `master_compute()` method of `PregelComputation`.
    pub fn double_node_value(&self, node_id: usize, key: &str) -> f64 {
        self.node_values.read().double_value(key, node_id)
    }

    /// Read a long node value by ID and schema key.
    pub fn long_node_value(&self, node_id: usize, key: &str) -> i64 {
        self.node_values.read().long_value(key, node_id)
    }

    /// Set a double node value by ID and schema key.
    ///
    /// Used by master compute to atomically update all nodes after normalization.
    pub fn set_double_node_value(&mut self, node_id: usize, key: &str, value: f64) {
        self.node_values.write().set(key, node_id, value);
    }

    /// Set a long node value by ID and schema key.
    pub fn set_long_node_value(&mut self, node_id: usize, key: &str, value: i64) {
        self.node_values.write().set_long(key, node_id, value);
    }

    /// Iterate over all nodes with a consumer function.
    ///
    /// The consumer receives one node ID at a time and returns true to continue
    /// or false to stop iteration. Used for parallel processing in master compute.
    ///
    /// # Example: Convergence Checking
    ///
    /// ```rust
    /// let mut did_converge = true;
    /// context.for_each_node(|node_id| {
    ///     let current = context.double_node_value(node_id, "rank");
    ///     let next = context.double_node_value(node_id, "next_rank");
    ///     if (current - next).abs() > tolerance {
    ///         did_converge = false;
    ///         false // Stop iteration early (optimization)
    ///     } else {
    ///         true // Continue
    ///     }
    /// });
    /// ```
    pub fn for_each_node<F>(&self, mut consumer: F)
    where
        F: FnMut(usize) -> bool,
    {
        let node_count = self.node_count();
        for node_id in 0..node_count {
            if !consumer(node_id) {
                break; // Consumer returned false, stop iteration
            }
        }
    }
}

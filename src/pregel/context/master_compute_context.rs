//! MasterComputeContext - API for global coordination between supersteps
//!
//! Provides the master compute API for algorithm-level coordination.

use crate::core::utils::progress::tasks::LeafTask;
use crate::pregel::{NodeValue, PregelConfig};
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
pub struct MasterComputeContext<C: PregelConfig> {
    config: C,
    graph: Arc<dyn Graph>,
    iteration: usize,
    node_values: Arc<parking_lot::RwLock<NodeValue>>,
    progress_task: Option<Arc<LeafTask>>,
}

impl<C: PregelConfig> MasterComputeContext<C> {
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

    /// Access a node's value by ID and schema key.
    ///
    /// # TODO
    ///
    /// Stub - will read from node value storage
    pub fn node_value<V>(&self, _node_id: usize, _key: &str) -> Option<V> {
        None
    }

    /// Iterate over all nodes with a consumer function.
    ///
    /// The consumer receives one node ID at a time and returns true to continue
    /// or false to stop iteration.
    ///
    /// # TODO
    ///
    /// Stub - will iterate over actual graph nodes
    pub fn for_each_node<F>(&self, mut _consumer: F)
    where
        F: FnMut(usize) -> bool,
    {
        // Stub
    }
}

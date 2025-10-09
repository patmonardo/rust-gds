//! NodeCentricContext - Base context for node-centric operations
//!
//! Provides the foundation for InitContext and ComputeContext with common
//! node-centric operations like setting node values, accessing neighbors, etc.

use crate::pregel::{NodeValue, PregelConfig};

/// Base context class providing node-centric access to the graph.
///
/// This serves as the foundation for `InitContext` and `ComputeContext`,
/// providing common operations that work on a per-node basis.
///
/// # Node-Centric Operations
///
/// - **Node ID Management**: Track which node is currently being processed
/// - **Node Values**: Set properties for the current node
/// - **Graph Queries**: Degree, neighbors, node existence checks
/// - **ID Translation**: Convert between internal and original node IDs
///
/// # TODO
///
/// This is a foundational stub. Full implementation will include:
/// - Graph reference for topology access
/// - NodeValue reference for property storage
/// - Neighbor iteration (forEachNeighbor)
/// - Degree queries (degree, incomingDegree for bidirectional)
/// - ID translation (toOriginalId, toInternalId)
pub struct NodeCentricContext<C: PregelConfig> {
    node_id: u64,
    config: std::marker::PhantomData<C>,
    // TODO: Add fields when implementing
    // graph: &'a Graph,
    // node_value: &'a mut NodeValue,
    // progress_tracker: &'a ProgressTracker,
}

impl<C: PregelConfig> NodeCentricContext<C> {
    /// Create a new node-centric context (stub).
    ///
    /// # TODO
    ///
    /// Add actual parameters: graph, config, node_value, progress_tracker
    pub fn stub() -> Self {
        Self {
            node_id: 0,
            config: std::marker::PhantomData,
        }
    }

    /// Set the node ID for this context.
    ///
    /// Called by the framework before each init() or compute() invocation
    /// to indicate which node is being processed.
    pub fn set_node_id(&mut self, node_id: u64) {
        self.node_id = node_id;
    }

    /// Get the node ID currently being processed.
    pub fn node_id(&self) -> u64 {
        self.node_id
    }

    /// Get the number of nodes in the graph.
    ///
    /// # TODO
    ///
    /// Stub - will return graph.node_count()
    pub fn node_count(&self) -> u64 {
        0
    }

    /// Check if a node exists in the graph.
    ///
    /// # TODO
    ///
    /// Stub - will check against graph node count
    pub fn node_exists(&self, node_id: u64) -> bool {
        let _ = node_id;
        false
    }

    /// Set a double node value for the given property key.
    ///
    /// # Parameters
    ///
    /// - `key`: The property key from the PregelSchema
    /// - `value`: The double value to set
    ///
    /// # TODO
    ///
    /// Stub - will write to node_value.set(key, self.node_id, value)
    pub fn set_node_value(&mut self, _key: &str, _value: f64) {
        // Stub
    }

    /// Set a long node value for the given property key.
    ///
    /// # TODO
    ///
    /// Stub - will write to node_value.set_long(key, self.node_id, value)
    pub fn set_node_value_long(&mut self, _key: &str, _value: i64) {
        // Stub
    }

    /// Set a long array node value for the given property key.
    ///
    /// # TODO
    ///
    /// Stub - will write to node_value.set_long_array(key, self.node_id, value)
    pub fn set_node_value_long_array(&mut self, _key: &str, _value: Vec<i64>) {
        // Stub
    }

    /// Set a double array node value for the given property key.
    ///
    /// # TODO
    ///
    /// Stub - will write to node_value.set_double_array(key, self.node_id, value)
    pub fn set_node_value_double_array(&mut self, _key: &str, _value: Vec<f64>) {
        // Stub
    }

    /// Get the degree (number of outgoing relationships) of the current node.
    ///
    /// # TODO
    ///
    /// Stub - will return graph.degree(self.node_id)
    pub fn degree(&self) -> usize {
        0
    }

    /// Convert internal node ID to original graph node ID.
    ///
    /// # TODO
    ///
    /// Stub - will call graph.to_original_node_id()
    pub fn to_original_id(&self, internal_node_id: u64) -> u64 {
        internal_node_id
    }

    /// Convert original graph node ID to internal node ID.
    ///
    /// # TODO
    ///
    /// Stub - will call graph.to_internal_node_id()
    pub fn to_internal_id(&self, original_node_id: u64) -> u64 {
        original_node_id
    }

    /// Iterate over neighbors of the current node.
    ///
    /// # TODO
    ///
    /// Stub - will call graph.for_each_neighbor(self.node_id, consumer)
    pub fn for_each_neighbor<F>(&self, _consumer: F)
    where
        F: FnMut(u64),
    {
        // Stub
    }

    /// Iterate over distinct neighbors (each neighbor only once).
    ///
    /// # TODO
    ///
    /// Stub - will use a bitset to track visited neighbors
    pub fn for_each_distinct_neighbor<F>(&self, _consumer: F)
    where
        F: FnMut(u64),
    {
        // Stub
    }
}

/// Trait for bidirectional context operations.
///
/// Provides access to incoming edges in addition to outgoing edges.
/// This is only available when using `BidirectionalPregelComputation`.
pub trait BidirectionalNodeCentricContext {
    /// Get the incoming degree (number of incoming relationships) of the current node.
    ///
    /// # TODO
    ///
    /// Stub - will return graph.incoming_degree(node_id)
    fn incoming_degree(&self) -> usize {
        0
    }

    /// Iterate over incoming neighbors of the current node.
    ///
    /// # TODO
    ///
    /// Stub - will call graph.for_each_incoming_neighbor(node_id, consumer)
    fn for_each_incoming_neighbor<F>(&self, _consumer: F)
    where
        F: FnMut(u64),
    {
        // Stub
    }

    /// Iterate over distinct incoming neighbors (each neighbor only once).
    ///
    /// # TODO
    ///
    /// Stub - will use a bitset to track visited neighbors
    fn for_each_distinct_incoming_neighbor<F>(&self, _consumer: F)
    where
        F: FnMut(u64),
    {
        // Stub
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;
    impl PregelConfig for TestConfig {
        fn max_iterations(&self) -> usize {
            10
        }
    }

    #[test]
    fn test_node_centric_context_creation() {
        let ctx: NodeCentricContext<TestConfig> = NodeCentricContext::stub();
        assert_eq!(ctx.node_id(), 0);
    }

    #[test]
    fn test_set_node_id() {
        let mut ctx: NodeCentricContext<TestConfig> = NodeCentricContext::stub();
        ctx.set_node_id(42);
        assert_eq!(ctx.node_id(), 42);
    }

    #[test]
    fn test_id_translation() {
        let ctx: NodeCentricContext<TestConfig> = NodeCentricContext::stub();
        // Stub returns same ID
        assert_eq!(ctx.to_original_id(123), 123);
        assert_eq!(ctx.to_internal_id(456), 456);
    }
}

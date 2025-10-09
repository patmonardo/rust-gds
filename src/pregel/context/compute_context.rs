//! ComputeContext - API for vertex computation phase
//!
//! Provides the complete API for vertices to interact with the Pregel framework
//! during the compute phase of each superstep.

use crate::pregel::PregelConfig;

/// Context provided to vertices during the compute phase.
///
/// The `ComputeContext` gives vertices the full Pregel API:
/// - Read/write node values
/// - Send messages to other nodes
/// - Vote to halt
/// - Access superstep number
/// - Query node degree and neighbors
///
/// # Message Sending
///
/// Vertices can send messages to:
/// - All neighbors: `send_to_neighbors(msg)`
/// - Specific node: `send_to(node_id, msg)`
/// - Multiple nodes: `send_to_many(&[node_ids], msg)`
///
/// Messages sent in superstep N are delivered in superstep N+1.
///
/// # Voting to Halt
///
/// A vertex can vote to halt by calling `vote_to_halt()`. Once halted:
/// - No more `compute()` calls until a message is received
/// - Receiving a message reactivates the vertex
/// - Computation ends when all vertices halt with no messages in flight
///
/// # TODO
///
/// This is a stub interface. Full implementation will include:
/// - Message sending API
/// - Node value get/set
/// - Voting to halt
/// - Superstep number
/// - Degree and neighbor access
/// - Aggregator access
pub struct ComputeContext<C: PregelConfig> {
    config: std::marker::PhantomData<C>,
}

impl<C: PregelConfig> ComputeContext<C> {
    /// Get the current superstep number (0-indexed).
    ///
    /// # TODO
    ///
    /// Stub - will return actual superstep from framework
    pub fn superstep(&self) -> usize {
        0
    }

    /// Returns true if this is the initial superstep (superstep 0).
    pub fn is_initial_superstep(&self) -> bool {
        self.superstep() == 0
    }

    /// Get the total number of nodes in the graph.
    ///
    /// # TODO
    ///
    /// Stub - will return actual node count from graph
    pub fn node_count(&self) -> usize {
        0
    }

    /// Get the current node's value.
    ///
    /// # TODO
    ///
    /// Stub - will read from node value storage
    pub fn node_value<V>(&self) -> V
    where
        V: Default,
    {
        V::default()
    }

    /// Set the current node's value.
    ///
    /// # TODO
    ///
    /// Stub - will write to node value storage
    pub fn set_node_value<V>(&mut self, _value: V) {
        // Stub
    }

    /// Get the out-degree of the current node.
    ///
    /// # TODO
    ///
    /// Stub - will query graph topology
    pub fn degree(&self) -> usize {
        0
    }

    /// Send a message to all neighbors of the current node.
    ///
    /// # TODO
    ///
    /// Stub - will enqueue messages to outgoing message buffer
    pub fn send_to_neighbors<M>(&mut self, _message: M) {
        // Stub
    }

    /// Send a message to a specific node.
    ///
    /// # TODO
    ///
    /// Stub - will enqueue message to target node
    pub fn send_to<M>(&mut self, _target: usize, _message: M) {
        // Stub
    }

    /// Vote to halt this node.
    ///
    /// The node will not receive further compute() calls unless it receives
    /// a message, which will reactivate it.
    ///
    /// # TODO
    ///
    /// Stub - will mark node as halted in framework state
    pub fn vote_to_halt(&mut self) {
        // Stub
    }
}

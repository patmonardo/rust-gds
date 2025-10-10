//! ComputeContext - API for vertex computation phase
//!
//! Provides the complete API for vertices to interact with the Pregel framework
//! during the compute phase of each superstep.

use crate::pregel::{NodeValue, PregelConfig};
use crate::types::graph::Graph;
use parking_lot::RwLock;
use std::sync::Arc;

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
/// # Translation from Java/TS
///
/// Follows Java constructor:
/// ```java
/// ComputeContext(Graph graph, CONFIG config, BasePregelComputation computation,
///                NodeValue nodeValue, Messenger<?> messenger, HugeAtomicBitSet voteBits,
///                MutableInt iteration, Optional<MutableBoolean> hasSendMessage,
///                ProgressTracker progressTracker)
/// ```
pub struct ComputeContext<C: PregelConfig, I: crate::pregel::MessageIterator> {
    base: super::NodeCentricContext<C>,
    iteration: usize,
    messenger: Arc<dyn crate::pregel::Messenger<I>>,
    vote_bits: Arc<crate::collections::HugeAtomicBitSet>,
    has_sent_message: Arc<std::sync::atomic::AtomicBool>,
}

impl<C: PregelConfig, I: crate::pregel::MessageIterator> ComputeContext<C, I> {
    /// Create a new compute context.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph topology
    /// * `config` - The Pregel configuration
    /// * `node_value` - The node property storage
    /// * `iteration` - The current superstep number (0-indexed)
    /// * `messenger` - Message passing system
    /// * `vote_bits` - Vote-to-halt tracking
    /// * `has_sent_message` - Flag for tracking if any message was sent
    pub fn new(
        graph: Arc<dyn Graph>,
        config: C,
        node_value: Arc<RwLock<NodeValue>>,
        iteration: usize,
        messenger: Arc<dyn crate::pregel::Messenger<I>>,
        vote_bits: Arc<crate::collections::HugeAtomicBitSet>,
        has_sent_message: Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        Self {
            base: super::NodeCentricContext::new(graph, config, node_value),
            iteration,
            messenger,
            vote_bits,
            has_sent_message,
        }
    }

    /// Set the node ID for this context.
    ///
    /// Delegates to the base NodeCentricContext.
    pub fn set_node_id(&mut self, node_id: u64) {
        self.base.set_node_id(node_id);
    }

    /// Get the node ID currently being processed.
    pub fn node_id(&self) -> u64 {
        self.base.node_id()
    }

    /// Get the current superstep number (0-indexed).
    pub fn superstep(&self) -> usize {
        self.iteration
    }

    /// Returns true if this is the initial superstep (superstep 0).
    pub fn is_initial_superstep(&self) -> bool {
        self.iteration == 0
    }

    /// Get the total number of nodes in the graph.
    pub fn node_count(&self) -> u64 {
        self.base.node_count()
    }

    /// Get the configuration.
    pub fn config(&self) -> &C {
        self.base.config()
    }

    /// Returns the degree (number of relationships) of the currently processed node.
    pub fn degree(&self) -> usize {
        self.base.degree()
    }

    /// Read a double node value for the given property key.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// double doubleNodeValue(String key)
    /// ```
    pub fn double_node_value(&self, key: &str) -> f64 {
        self.base.double_node_value(key)
    }

    /// Read a long node value for the given property key.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// long longNodeValue(String key)
    /// ```
    pub fn long_node_value(&self, key: &str) -> i64 {
        self.base.long_node_value(key)
    }

    /// Set a double node value for the given property key.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void setNodeValue(String key, double value)
    /// ```
    pub fn set_node_value(&mut self, key: &str, value: f64) {
        self.base.set_node_value(key, value);
    }

    /// Set a long node value for the given property key.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void setNodeValue(String key, long value)
    /// ```
    pub fn set_node_value_long(&mut self, key: &str, value: i64) {
        self.base.set_node_value_long(key, value);
    }

    /// Iterate over neighbors of the current node.
    pub fn for_each_neighbor<F>(&self, consumer: F)
    where
        F: FnMut(u64),
    {
        self.base.for_each_neighbor(consumer);
    }

    /// Send a message to all neighbors of the current node.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void sendToNeighbors(double message)
    /// ```
    pub fn send_to_neighbors(&mut self, message: f64) {
        // Collect neighbors first (we need &mut self for send_to)
        let mut neighbors = Vec::new();
        self.base.for_each_neighbor(|neighbor_id| {
            neighbors.push(neighbor_id);
        });

        // Now send messages to all neighbors
        for target in neighbors {
            self.send_to(target, message);
        }
    }

    /// Send a message to a specific node.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void sendTo(long targetNodeId, double message)
    /// ```
    pub fn send_to(&mut self, target: u64, message: f64) {
        let source = self.base.node_id();
        self.messenger.send_to(source, target, message);
        self.has_sent_message
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Vote to halt this node.
    ///
    /// The node will not receive further compute() calls unless it receives
    /// a message, which will reactivate it.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void voteToHalt()
    /// ```
    pub fn vote_to_halt(&mut self) {
        let node_id = self.base.node_id();
        self.vote_bits.set(node_id as usize);
    }
}

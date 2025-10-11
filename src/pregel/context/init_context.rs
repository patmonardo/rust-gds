//! InitContext - API for vertex initialization phase
//!
//! Provides access to node properties and configuration during the init() phase.

use crate::pregel::{NodeValue, PregelRuntimeConfig};
use crate::types::graph::Graph;
use parking_lot::RwLock;
use std::sync::Arc;

/// Context provided to vertices during the initialization phase.
///
/// The `InitContext` gives vertices access to:
/// - Node properties from the input graph
/// - Algorithm configuration
/// - API to set initial node values
///
/// # Lifecycle
///
/// - Created once per vertex before the first superstep
/// - Used only during the `PregelComputation::init()` call
/// - Provides read-only access to input graph properties
///
/// # Translation from Java/TS
///
/// Follows Java constructor:
/// ```java
/// InitContext(Graph graph, CONFIG config, NodeValue nodeValue, ProgressTracker progressTracker)
/// ```
pub struct InitContext<C: PregelRuntimeConfig> {
    base: super::NodeCentricContext<C>,
}

impl<C: PregelRuntimeConfig> InitContext<C> {
    /// Create a new initialization context.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph topology
    /// * `config` - The Pregel configuration
    /// * `node_value` - The node property storage
    pub fn new(graph: Arc<dyn Graph>, config: C, node_value: Arc<RwLock<NodeValue>>) -> Self {
        Self {
            base: super::NodeCentricContext::new(graph, config, node_value),
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

    /// Get the total number of nodes in the graph.
    pub fn node_count(&self) -> u64 {
        self.base.node_count()
    }

    /// Get the configuration.
    pub fn config(&self) -> &C {
        self.base.config()
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

    /// Set a long array node value for the given property key.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void setNodeValue(String key, long[] value)
    /// ```
    pub fn set_node_value_long_array(&mut self, key: &str, value: Vec<i64>) {
        self.base.set_node_value_long_array(key, value);
    }

    /// Set a double array node value for the given property key.
    ///
    /// # Java equivalent
    ///
    /// ```java
    /// void setNodeValue(String key, double[] value)
    /// ```
    pub fn set_node_value_double_array(&mut self, key: &str, value: Vec<f64>) {
        self.base.set_node_value_double_array(key, value);
    }

    /// Returns the degree (number of relationships) of the currently processed node.
    pub fn degree(&self) -> usize {
        self.base.degree()
    }

    /// Iterate over neighbors of the current node.
    pub fn for_each_neighbor<F>(&self, consumer: F)
    where
        F: FnMut(u64),
    {
        self.base.for_each_neighbor(consumer);
    }
}

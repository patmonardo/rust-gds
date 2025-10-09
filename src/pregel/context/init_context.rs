//! InitContext - API for vertex initialization phase
//!
//! Provides access to node properties and configuration during the init() phase.

use crate::pregel::PregelConfig;

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
/// # TODO
///
/// This is a stub interface. Full implementation will include:
/// - Access to node properties from the graph
/// - Ability to set initial node values
/// - Configuration access
/// - Node degree information
pub struct InitContext<C: PregelConfig> {
    base: super::NodeCentricContext<C>,
}

impl<C: PregelConfig> InitContext<C> {
    /// Create a new initialization context.
    pub fn new(config: C) -> Self {
        Self {
            base: super::NodeCentricContext::new(config),
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
    ///
    /// # TODO
    ///
    /// Stub - will return actual node count from graph
    pub fn node_count(&self) -> usize {
        0
    }

    /// Set the initial value for this node.
    ///
    /// # TODO
    ///
    /// Stub - will write to node value storage
    pub fn set_node_value<V>(&mut self, _value: V) {
        // Stub
    }

    /// Get a node property by name.
    ///
    /// # TODO
    ///
    /// Stub - will read from graph property storage
    pub fn node_property<V>(&self, _key: &str) -> Option<V> {
        None
    }

    /// Get the configuration.
    ///
    /// # TODO
    ///
    /// Will return actual config reference when wired
    pub fn config(&self) -> C
    where
        C: Clone,
    {
        // Stub - need to store config in base context
        unimplemented!("Config access not yet wired")
    }
}

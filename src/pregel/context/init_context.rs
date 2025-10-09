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
    config: std::marker::PhantomData<C>,
}

impl<C: PregelConfig> InitContext<C> {
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
}

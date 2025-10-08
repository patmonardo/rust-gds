use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;

/// Interface for retrieving node property values by key.
///
/// This mirrors the TypeScript `NodePropertyContainer` contract and provides
/// a lightweight accessor for property values while allowing implementations to
/// decide how to store and cache the underlying data.
pub trait NodePropertyContainer {
    /// Returns the property values associated with the provided key, if they are available.
    fn node_properties(&self, property_key: &str) -> Option<Arc<dyn NodePropertyValues>>;

    /// Returns the set of all available node property keys.
    fn available_node_properties(&self) -> HashSet<String>;
}

/// Helper trait that offers utility functions mirroring the TypeScript namespace helpers.
pub trait NodePropertyContainerExt: NodePropertyContainer {
    /// Convenience method that returns the property values for `property_key` and
    /// falls back to the provided default when the key is missing.
    fn node_properties_or<'a>(
        &'a self,
        property_key: &str,
        fallback: &'a Arc<dyn NodePropertyValues>,
    ) -> Arc<dyn NodePropertyValues> {
        self.node_properties(property_key)
            .unwrap_or_else(|| Arc::clone(fallback))
    }

    /// Returns `true` when the container exposes the given property key.
    fn has_node_property(&self, property_key: &str) -> bool {
        self.available_node_properties().contains(property_key)
    }
}

impl<T> NodePropertyContainerExt for T where T: NodePropertyContainer + ?Sized {}

/// Trivial implementation of [`NodePropertyContainer`] that always returns an empty result.
#[derive(Debug, Default, Clone, Copy)]
pub struct EmptyNodePropertyContainer;

impl NodePropertyContainer for EmptyNodePropertyContainer {
    fn node_properties(&self, _property_key: &str) -> Option<Arc<dyn NodePropertyValues>> {
        None
    }

    fn available_node_properties(&self) -> HashSet<String> {
        HashSet::new()
    }
}

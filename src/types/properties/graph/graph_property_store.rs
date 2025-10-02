use super::graph_property::GraphProperty;
use super::graph_property_values::GraphPropertyValues;
use crate::types::properties::property_store::GenericPropertyStore;
use std::collections::HashMap;

/// Represents a store for graph properties.
/// Each property is identified by a string key and holds GraphPropertyValues.
pub type GraphPropertyStore = GenericPropertyStore<Box<dyn GraphPropertyValues>>;

impl GraphPropertyStore {
    /// Creates a builder for constructing a GraphPropertyStore.
    pub fn builder() -> GraphPropertyStoreBuilder {
        GraphPropertyStoreBuilder::new()
    }
}

/// Builder for GraphPropertyStore.
#[derive(Debug)]
pub struct GraphPropertyStoreBuilder {
    properties: HashMap<String, GraphProperty>,
}

impl GraphPropertyStoreBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        GraphPropertyStoreBuilder {
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the store.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Box<dyn GraphPropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let property = GraphProperty::of(key_str.clone(), values);
        self.properties.insert(key_str, property);
        self
    }

    /// Adds an already constructed property to the store.
    pub fn put(mut self, key: impl Into<String>, property: GraphProperty) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    /// Builds the GraphPropertyStore.
    pub fn build(self) -> GraphPropertyStore {
        GraphPropertyStore::new(self.properties)
    }
}

impl Default for GraphPropertyStoreBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::graph::{
        DefaultDoubleGraphPropertyValues, DefaultLongGraphPropertyValues,
    };
    use crate::types::properties::property_store::PropertyStore;

    #[test]
    fn test_empty_graph_property_store() {
        let store = GraphPropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_graph_property_store_builder() {
        let node_count = DefaultLongGraphPropertyValues::singleton(1000);
        let avg_degree = DefaultDoubleGraphPropertyValues::singleton(5.2);

        let store = GraphPropertyStore::builder()
            .put_property("node_count", Box::new(node_count))
            .put_property("avg_degree", Box::new(avg_degree))
            .build();

        assert!(!store.is_empty());
        assert_eq!(store.len(), 2);
        assert!(store.contains_key("node_count"));
        assert!(store.contains_key("avg_degree"));
    }
}

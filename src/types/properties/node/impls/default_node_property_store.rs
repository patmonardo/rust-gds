use crate::types::properties::node::NodeProperty;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::property_store::GenericPropertyStore;
use crate::types::property::PropertyState;
use std::collections::HashMap;

/// Represents a store for node properties.
/// Each property is identified by a string key and holds NodePropertyValues.
pub type NodePropertyStore = GenericPropertyStore<Box<dyn NodePropertyValues>>;

impl NodePropertyStore {
    /// Creates a builder for constructing a NodePropertyStore.
    pub fn builder() -> NodePropertyStoreBuilder {
        NodePropertyStoreBuilder::new()
    }
}

/// Builder for NodePropertyStore.
#[derive(Debug)]
pub struct NodePropertyStoreBuilder {
    properties: HashMap<String, NodeProperty>,
}

impl NodePropertyStoreBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        NodePropertyStoreBuilder {
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the store.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Box<dyn NodePropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let property = NodeProperty::of(key_str.clone(), PropertyState::Normal, values);
        self.properties.insert(key_str, property);
        self
    }

    /// Adds a property with a specific state to the store.
    pub fn put_property_with_state(
        mut self,
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn NodePropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let property = NodeProperty::of(key_str.clone(), state, values);
        self.properties.insert(key_str, property);
        self
    }

    /// Adds an already constructed property to the store.
    pub fn put(mut self, key: impl Into<String>, property: NodeProperty) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    /// Builds the NodePropertyStore.
    pub fn build(self) -> NodePropertyStore {
        NodePropertyStore::new(self.properties)
    }
}

impl Default for NodePropertyStoreBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::{
        DefaultDoubleNodePropertyValues, DefaultLongNodePropertyValues,
    };
    use crate::types::properties::property::PropertyTrait;
    use crate::types::properties::property_store::PropertyStore;

    #[test]
    fn test_empty_node_property_store() {
        let store = NodePropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_node_property_store_builder() {
        let age_values = DefaultLongNodePropertyValues::new(vec![25, 30, 35], 3);
        let score_values = DefaultDoubleNodePropertyValues::new(vec![1.5, 2.0, 2.5], 3);

        let store = NodePropertyStore::builder()
            .put_property("age", Box::new(age_values))
            .put_property("score", Box::new(score_values))
            .build();

        assert!(!store.is_empty());
        assert_eq!(store.len(), 2);
        assert!(store.contains_key("age"));
        assert!(store.contains_key("score"));

        let age_prop = store.get("age").unwrap();
        assert_eq!(age_prop.key(), "age");
    }
}

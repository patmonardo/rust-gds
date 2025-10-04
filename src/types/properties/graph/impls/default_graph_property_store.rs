use super::default_graph_property::DefaultGraphProperty;
use crate::types::properties::graph::graph_property_values::GraphPropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// Concrete graph property store mirroring the relationship/node store layout.
#[derive(Debug, Clone, Default)]
pub struct DefaultGraphPropertyStore {
    properties: HashMap<String, DefaultGraphProperty>,
}

impl DefaultGraphPropertyStore {
    /// Creates an empty graph property store.
    pub fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Creates a graph property store from a map of properties.
    pub fn new(properties: HashMap<String, DefaultGraphProperty>) -> Self {
        Self { properties }
    }

    /// Returns a builder for constructing a graph property store.
    pub fn builder() -> GraphPropertyStoreBuilder {
        GraphPropertyStoreBuilder::new()
    }

    /// Returns a reference to the underlying property map.
    pub fn graph_properties(&self) -> &HashMap<String, DefaultGraphProperty> {
        &self.properties
    }

    /// Retrieves a property by key.
    pub fn get(&self, key: &str) -> Option<&DefaultGraphProperty> {
        self.properties.get(key)
    }

    /// Returns `true` when the store contains no properties.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns the number of properties contained in the store.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns whether the store contains the provided key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Returns a vector of property keys.
    pub fn key_set(&self) -> Vec<&str> {
        self.properties.keys().map(|k| k.as_str()).collect()
    }

    /// Returns the properties as an iterator-backed vector for convenience.
    pub fn values(&self) -> Vec<&DefaultGraphProperty> {
        self.properties.values().collect()
    }

    /// Inserts or replaces the provided property using its key.
    pub fn put_property_entry(
        &mut self,
        property: DefaultGraphProperty,
    ) -> Option<DefaultGraphProperty> {
        self.properties.insert(property.key().to_string(), property)
    }

    /// Removes a property by key if present.
    pub fn remove_property(&mut self, key: &str) -> Option<DefaultGraphProperty> {
        self.properties.remove(key)
    }
}

/// Builder for `DefaultGraphPropertyStore`.
#[derive(Debug, Clone, Default)]
pub struct GraphPropertyStoreBuilder {
    properties: HashMap<String, DefaultGraphProperty>,
}

impl GraphPropertyStoreBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the store using the default state and schema.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Arc<dyn GraphPropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let property = DefaultGraphProperty::of(key_str.clone(), values);
        self.properties.insert(key_str, property);
        self
    }

    /// Inserts an already constructed property into the builder.
    pub fn put(mut self, key: impl Into<String>, property: DefaultGraphProperty) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    /// Builds the graph property store.
    pub fn build(self) -> DefaultGraphPropertyStore {
        DefaultGraphPropertyStore::new(self.properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::graph::{
        DefaultDoubleGraphPropertyValues, DefaultLongGraphPropertyValues,
    };
    use crate::types::properties::property_values::PropertyValues;

    #[test]
    fn empty_store_builder() {
        let store = DefaultGraphPropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.key_set().is_empty());
    }

    #[test]
    fn builder_puts_properties() {
        let node_count: Arc<dyn GraphPropertyValues> =
            Arc::new(DefaultLongGraphPropertyValues::singleton(1_000));
        let avg_degree: Arc<dyn GraphPropertyValues> =
            Arc::new(DefaultDoubleGraphPropertyValues::singleton(5.2));

        let store = DefaultGraphPropertyStore::builder()
            .put_property("node_count", node_count.clone())
            .put_property("avg_degree", avg_degree.clone())
            .build();

        assert!(!store.is_empty());
        assert_eq!(store.len(), 2);
        assert!(store.contains_key("node_count"));
        assert!(store.contains_key("avg_degree"));

        let node_count_prop = store.get("node_count").unwrap();
        assert_eq!(
            node_count_prop.property_schema().value_type(),
            node_count.value_type()
        );
    }
}

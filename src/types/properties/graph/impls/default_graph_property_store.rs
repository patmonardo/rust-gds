use crate::types::properties::graph::graph_property::GraphProperty;
use crate::types::properties::graph::graph_property_store::{
    GraphPropertyStore, GraphPropertyStoreBuilder,
};
use crate::types::properties::graph::graph_property_values::GraphPropertyValues;
use crate::types::properties::property_store::PropertyStore;
use std::collections::HashMap;
use std::sync::Arc;

/// Concrete implementation backing the GraphPropertyStore trait.
#[derive(Debug, Clone, Default)]
pub struct DefaultGraphPropertyStore {
    properties: HashMap<String, GraphProperty>,
}

/// Builder for DefaultGraphPropertyStore.
#[derive(Debug, Clone, Default)]
pub struct DefaultGraphPropertyStoreBuilder {
    properties: HashMap<String, GraphProperty>,
}

/* Base PropertyStore implementation - only properties() needed */
impl PropertyStore for DefaultGraphPropertyStore {
    type Property = GraphProperty;

    fn properties(&self) -> &HashMap<String, Self::Property> {
        &self.properties
    }
}

/* Domain-specific GraphPropertyStore implementation */
impl GraphPropertyStore for DefaultGraphPropertyStore {
    type Builder = DefaultGraphPropertyStoreBuilder;

    fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn new(properties: HashMap<String, Self::Property>) -> Self {
        Self { properties }
    }

    fn builder() -> Self::Builder {
        DefaultGraphPropertyStoreBuilder {
            properties: HashMap::new(),
        }
    }

    fn get_all_properties(&self) -> Vec<&Self::Property> {
        self.properties.values().collect()
    }

    fn get_property_values(&self, property_key: &str) -> Option<&dyn GraphPropertyValues> {
        self.properties
            .get(property_key)
            .map(|property| property.values())
    }

    fn to_builder(&self) -> Self::Builder {
        DefaultGraphPropertyStoreBuilder {
            properties: self.properties.clone(),
        }
    }
}

/* Builder trait implementation */
impl GraphPropertyStoreBuilder for DefaultGraphPropertyStoreBuilder {
    type Store = DefaultGraphPropertyStore;
    type Property = GraphProperty;

    fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn from_store(store: &Self::Store) -> Self {
        Self {
            properties: store.properties.clone(),
        }
    }

    fn properties(mut self, props: HashMap<String, Self::Property>) -> Self {
        self.properties = props;
        self
    }

    fn put_if_absent(mut self, key: impl Into<String>, property: Self::Property) -> Self {
        let k = key.into();
        self.properties.entry(k).or_insert(property);
        self
    }

    fn put(mut self, key: impl Into<String>, property: Self::Property) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    fn remove_property(mut self, key: &str) -> Self {
        self.properties.remove(key);
        self
    }

    fn build(self) -> Self::Store {
        DefaultGraphPropertyStore::new(self.properties)
    }
}

/* Inherent convenience methods for the store (ergonomics without trait import) */
impl DefaultGraphPropertyStore {
    /// Returns the number of properties in this store.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns whether this store is empty.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns a reference to the property with the given key.
    pub fn get(&self, key: &str) -> Option<&GraphProperty> {
        self.properties.get(key)
    }

    /// Returns whether the store contains a property with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Returns a reference to the underlying properties map.
    pub fn graph_properties(&self) -> &HashMap<String, GraphProperty> {
        &self.properties
    }
}

/* Inherent convenience methods for the builder (do not belong to the trait) */
impl DefaultGraphPropertyStoreBuilder {
    /// Convenience method to add a property by supplying property values directly.
    /// This reduces imports for callers who just want to add values.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Arc<dyn GraphPropertyValues>,
    ) -> Self {
        use crate::types::property_state::PropertyState;

        let key_str = key.into();
        let prop = GraphProperty::with_state(key_str.clone(), PropertyState::Persistent, values);
        self.properties.insert(key_str, prop);
        self
    }
}

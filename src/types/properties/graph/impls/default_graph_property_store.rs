use crate::types::properties::graph::graph_property::GraphProperty;
use crate::types::properties::graph::graph_property_store::{
    GraphPropertyStore, GraphPropertyStoreBuilder,
};
use crate::types::properties::graph::graph_property_values::GraphPropertyValues;
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

impl GraphPropertyStore for DefaultGraphPropertyStore {
    type Property = GraphProperty;
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

    fn has_property(&self, property_key: &str) -> bool {
        self.properties.contains_key(property_key)
    }

    fn property_key_set(&self) -> Vec<&str> {
        self.properties.keys().map(|k| k.as_str()).collect()
    }

    fn get_property(&self, property_key: &str) -> Option<&Self::Property> {
        self.properties.get(property_key)
    }

    fn get_all_properties(&self) -> Vec<&Self::Property> {
        self.properties.values().collect()
    }

    fn get_property_values(&self, property_key: &str) -> Option<&dyn GraphPropertyValues> {
        self.properties
            .get(property_key)
            .map(|p| p.values().as_ref())
    }

    fn size(&self) -> usize {
        self.properties.len()
    }

    fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    fn to_builder(&self) -> Self::Builder {
        DefaultGraphPropertyStoreBuilder {
            properties: self.properties.clone(),
        }
    }
}

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

    fn put_property(
        mut self,
        key: impl Into<String>,
        values: Arc<dyn GraphPropertyValues>,
    ) -> Self {
        use crate::types::properties::property::Property;
        use crate::types::property::PropertyState;

        let key_str = key.into();
        let prop = Property::of(key_str.clone(), PropertyState::Normal, values);
        self.properties.insert(key_str, prop);
        self
    }

    fn put(mut self, key: impl Into<String>, property: Self::Property) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    fn build(self) -> Self::Store {
        DefaultGraphPropertyStore::new(self.properties)
    }
}

use crate::collections::backends::arrow::{ArrowDoubleArray, ArrowLongArray};
use crate::collections::backends::factory::{
    create_double_backend_from_config, create_long_backend_from_config, DoubleCollection,
    LongCollection,
};
use crate::collections::backends::vec::{VecDouble, VecLong};
use crate::types::properties::graph::impls::default_graph_property_values::{
    DefaultDoubleGraphPropertyValues, DefaultLongGraphPropertyValues,
};
use crate::types::properties::graph::GraphProperty;
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::graph::{GraphPropertyStore, GraphPropertyStoreBuilder};
use crate::types::properties::PropertyStore;
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
        use crate::types::PropertyState;

        let key_str = key.into();
        let prop = GraphProperty::with_state(key_str.clone(), PropertyState::Persistent, values);
        self.properties.insert(key_str, prop);
        self
    }

    /// Create and put a Long graph property using CollectionsConfig for backend selection.
    pub fn put_long_with_config(
        mut self,
        config: &crate::config::CollectionsConfig<i64>,
        key: impl Into<String>,
        values: Vec<i64>,
    ) -> Self {
        let backend = create_long_backend_from_config(config, values);
        let pv = build_long_graph_property_values(backend);
        use crate::types::PropertyState;
        let key_str = key.into();
        let prop = GraphProperty::with_state(key_str.clone(), PropertyState::Persistent, pv);
        self.properties.insert(key_str, prop);
        self
    }

    /// Convenience: create and put Long graph property from Vec using Vec-backed defaults.
    pub fn put_long_from_vec(self, key: impl Into<String>, values: Vec<i64>) -> Self {
        // Default to Vec backend
        let default_config = crate::config::CollectionsConfig::<i64>::default();
        self.put_long_with_config(&default_config, key, values)
    }

    /// Create and put a Double graph property using CollectionsConfig for backend selection.
    pub fn put_double_with_config(
        mut self,
        config: &crate::config::CollectionsConfig<f64>,
        key: impl Into<String>,
        values: Vec<f64>,
    ) -> Self {
        let backend = create_double_backend_from_config(config, values);
        let pv = build_double_graph_property_values(backend);
        use crate::types::PropertyState;
        let key_str = key.into();
        let prop = GraphProperty::with_state(key_str.clone(), PropertyState::Persistent, pv);
        self.properties.insert(key_str, prop);
        self
    }

    /// Convenience: create and put Double graph property from Vec using Vec-backed defaults.
    pub fn put_double_from_vec(self, key: impl Into<String>, values: Vec<f64>) -> Self {
        // Default to Vec backend
        let default_config = crate::config::CollectionsConfig::<f64>::default();
        self.put_double_with_config(&default_config, key, values)
    }
}

fn build_long_graph_property_values(backend: LongCollection) -> Arc<dyn GraphPropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongGraphPropertyValues::<VecLong>::from_collection(collection),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(DefaultLongGraphPropertyValues::<VecLong>::from_collection(
                vec_backend,
            ))
        }
        LongCollection::Arrow(collection) => {
            Arc::new(DefaultLongGraphPropertyValues::<ArrowLongArray>::from_collection(collection))
        }
    }
}

fn build_double_graph_property_values(backend: DoubleCollection) -> Arc<dyn GraphPropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => {
            Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(collection))
        }
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(vec_backend))
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleGraphPropertyValues::<
            ArrowDoubleArray,
        >::from_collection(collection)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::arrow::{ArrowDoubleArray, ArrowLongArray};

    #[test]
    fn builds_arrow_backed_long_graph_values() {
        let backend = LongCollection::Arrow(ArrowLongArray::from_vec(vec![3, 4, 5]));
        let values = build_long_graph_property_values(backend);
        let collected: Vec<i64> = values.long_values().collect();
        assert_eq!(collected, vec![3, 4, 5]);
    }

    #[test]
    fn builds_arrow_backed_double_graph_values() {
        let backend = DoubleCollection::Arrow(ArrowDoubleArray::from_vec(vec![0.5, 0.75]));
        let values = build_double_graph_property_values(backend);
        let collected: Vec<f64> = values.double_values().collect();
        assert_eq!(collected, vec![0.5, 0.75]);
    }
}

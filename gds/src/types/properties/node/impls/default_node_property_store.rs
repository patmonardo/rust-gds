use crate::types::properties::node::NodeProperty;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::node::{NodePropertyStore, NodePropertyStoreBuilder};
use crate::types::properties::PropertyStore;
use crate::types::PropertyState;
// Note: The generated property value types are now generic over Collections backend.
// They should be used as DefaultLongNodePropertyValues<C>, DefaultDoubleNodePropertyValues<C>, etc.
// where C is a Collections implementation like VecLong, VecDouble, HugeLong, etc.
use crate::types::properties::node::impls::default_node_property_values::{
    DefaultLongNodePropertyValues,
    DefaultDoubleNodePropertyValues,
};
use crate::collections::backends::vec::{VecLong, VecDouble};
use std::collections::HashMap;
use std::sync::Arc;

/// Concrete node property store implementation.
#[derive(Debug, Clone, Default)]
pub struct DefaultNodePropertyStore {
    properties: HashMap<String, NodeProperty>,
}

/// Builder for DefaultNodePropertyStore.
#[derive(Debug, Clone, Default)]
pub struct DefaultNodePropertyStoreBuilder {
    properties: HashMap<String, NodeProperty>,
}

/* Base PropertyStore implementation - only properties() needed */
impl PropertyStore for DefaultNodePropertyStore {
    type Property = NodeProperty;

    fn properties(&self) -> &HashMap<String, Self::Property> {
        &self.properties
    }
}

/* Domain-specific NodePropertyStore implementation */
impl NodePropertyStore for DefaultNodePropertyStore {
    type Builder = DefaultNodePropertyStoreBuilder;

    fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn new(properties: HashMap<String, Self::Property>) -> Self {
        Self { properties }
    }

    fn builder() -> Self::Builder {
        DefaultNodePropertyStoreBuilder {
            properties: HashMap::new(),
        }
    }

    fn get_all_properties(&self) -> Vec<&Self::Property> {
        self.properties.values().collect()
    }

    fn get_property_values(&self, property_key: &str) -> Option<&dyn NodePropertyValues> {
        self.properties
            .get(property_key)
            .map(|property| property.values())
    }

    fn to_builder(&self) -> Self::Builder {
        DefaultNodePropertyStoreBuilder {
            properties: self.properties.clone(),
        }
    }
}

/* Builder trait implementation */
impl NodePropertyStoreBuilder for DefaultNodePropertyStoreBuilder {
    type Store = DefaultNodePropertyStore;
    type Property = NodeProperty;

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

    // ...existing code...

    fn remove_property(mut self, key: &str) -> Self {
        self.properties.remove(key);
        self
    }

    fn build(self) -> Self::Store {
        DefaultNodePropertyStore::new(self.properties)
    }
}

/* Inherent convenience methods for the store (ergonomics without trait import) */
impl DefaultNodePropertyStore {
    /// Returns the number of properties in this store.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns whether this store is empty.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns a reference to the property with the given key.
    pub fn get(&self, key: &str) -> Option<&NodeProperty> {
        self.properties.get(key)
    }

    /// Returns whether the store contains a property with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Returns a reference to the underlying properties map.
    pub fn node_properties(&self) -> &HashMap<String, NodeProperty> {
        &self.properties
    }
}

/* Inherent convenience methods for the builder (do not belong to the trait) */
impl DefaultNodePropertyStoreBuilder {
    /// Convenience method to add a property by supplying property values directly.
    /// This mirrors `GraphPropertyStoreBuilder::put_property` and reduces imports for callers.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let prop = NodeProperty::with_state(key_str.clone(), PropertyState::Persistent, values);
        self.properties.insert(key_str, prop);
        self
    }

    /// Create and put a Long property using CollectionsConfig for backend selection.
    pub fn put_long_with_config(
        mut self,
        config: &crate::config::CollectionsConfig<i64>,
        key: impl Into<String>,
        values: Vec<i64>,
    ) -> Self {
        let node_count = values.len();
        
        // Use config to select backend
        let backend = crate::collections::backends::factory::create_long_backend_from_config(config, values);
        
        let pv: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultLongNodePropertyValues::<VecLong>::from_collection(backend, node_count)
        );
        let prop = NodeProperty::with_state(key.into(), PropertyState::Persistent, pv);
        self.properties.insert(prop.key().to_string(), prop);
        self
    }

    /// Convenience: create and put a Long property from a Vec using Vec-backed defaults.
    pub fn put_long_from_vec(
        self,
        key: impl Into<String>,
        values: Vec<i64>,
    ) -> Self {
        // Default to Vec backend
        let default_config = crate::config::CollectionsConfig::<i64>::default();
        self.put_long_with_config(&default_config, key, values)
    }

    /// Create and put a Double property using CollectionsConfig for backend selection.
    pub fn put_double_with_config(
        mut self,
        config: &crate::config::CollectionsConfig<f64>,
        key: impl Into<String>,
        values: Vec<f64>,
    ) -> Self {
        let node_count = values.len();
        
        // Use config to select backend
        let backend = crate::collections::backends::factory::create_double_backend_from_config(config, values);
        
        let pv: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(backend, node_count)
        );
        let prop = NodeProperty::with_state(key.into(), PropertyState::Persistent, pv);
        self.properties.insert(prop.key().to_string(), prop);
        self
    }

    /// Convenience: create and put a Double property from a Vec using Vec-backed defaults.
    pub fn put_double_from_vec(
        self,
        key: impl Into<String>,
        values: Vec<f64>,
    ) -> Self {
        // Default to Vec backend
        let default_config = crate::config::CollectionsConfig::<f64>::default();
        self.put_double_with_config(&default_config, key, values)
    }

    /// Convenience: create and put a DoubleArray property from a Vec<Option<Vec<f64>>>.
    /// NOTE: Array types not yet implemented with universal adapters - this method is temporarily disabled.
    #[allow(dead_code)]
    pub fn put_double_array_from_vec(
        self,
        key: impl Into<String>,
        _values: Vec<Option<Vec<f64>>>,
    ) -> Self {
        // TODO: Implement when array types are added to universal adapters
        unimplemented!("DoubleArray not yet migrated to universal adapters");
    }

    /// Convenience: create and put a LongArray property from a Vec<Option<Vec<i64>>>.
    /// NOTE: Array types not yet implemented with universal adapters - this method is temporarily disabled.
    #[allow(dead_code)]
    pub fn put_long_array_from_vec(
        self,
        key: impl Into<String>,
        _values: Vec<Option<Vec<i64>>>,
    ) -> Self {
        // TODO: Implement when array types are added to universal adapters
        unimplemented!("LongArray not yet migrated to universal adapters");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::DefaultValue;
    use crate::types::PropertyState;

    fn sample_prop(key: &str) -> NodeProperty {
        use crate::types::properties::PropertyValues;
        use std::sync::Arc;

        let values: Arc<dyn NodePropertyValues> =
            Arc::new(DefaultLongNodePropertyValues::from_collection(crate::collections::backends::vec::VecLong::from(vec![1, 2, 3]), 3));
        let default_value = DefaultValue::of(values.value_type());
        NodeProperty::with_default(
            key.to_string(),
            PropertyState::Persistent,
            values,
            default_value,
        )
    }

    #[test]
    fn empty_builder() {
        let store = DefaultNodePropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn put_and_get() {
        let p1 = sample_prop("age");
        let store = DefaultNodePropertyStore::builder().put("age", p1).build();
        assert!(store.contains_key("age"));
        assert_eq!(store.key_set(), vec!["age"]);
        assert!(store.get_property_values("age").is_some());
    }

    #[test]
    fn put_if_absent_behavior() {
        let p1 = sample_prop("level");
        let p2 = sample_prop("level");
        let store = DefaultNodePropertyStore::builder()
            .put_if_absent("level", p1)
            .put_if_absent("level", p2) // ignored
            .build();
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn remove_property() {
        let p1 = sample_prop("rank");
        let store = DefaultNodePropertyStore::builder()
            .put("rank", p1)
            .remove_property("rank")
            .build();
        assert!(store.is_empty());
    }

    #[test]
    fn to_builder_round_trip() {
        let p1 = sample_prop("score");
        let store = DefaultNodePropertyStore::builder().put("score", p1).build();
        let rebuilt = store.to_builder().build();
        assert!(rebuilt.contains_key("score"));
    }
}

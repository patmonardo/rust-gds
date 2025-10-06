use crate::types::properties::node::node_property::NodeProperty;
use crate::types::properties::node::node_property_store::{
    NodePropertyStore, NodePropertyStoreBuilder,
};
use crate::types::properties::node::node_property_values::NodePropertyValues;
use crate::types::properties::property::Property;
use crate::types::property::PropertyState;
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

/* Store trait implementation */
impl NodePropertyStore for DefaultNodePropertyStore {
    type Property = NodeProperty;
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

    fn get_property_values(&self, property_key: &str) -> Option<&dyn NodePropertyValues> {
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
        let prop = Property::of(key_str.clone(), PropertyState::Normal, values);
        self.properties.insert(key_str, prop);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::properties::property::Property;
    use crate::types::property::PropertyState;
    use crate::types::schema::DefaultValue;

    fn sample_prop(key: &str) -> NodeProperty {
        use crate::types::properties::property_values::PropertyValues;
        use std::sync::Arc;

        let values: Arc<dyn NodePropertyValues> =
            Arc::new(DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3));
        let default_value = DefaultValue::of(values.value_type());
        Property::with_default(
            key.to_string(),
            PropertyState::Normal,
            values,
            default_value,
        )
    }

    #[test]
    fn empty_builder() {
        let store = DefaultNodePropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.size(), 0);
    }

    #[test]
    fn put_and_get() {
        let p1 = sample_prop("age");
        let store = DefaultNodePropertyStore::builder().put("age", p1).build();
        assert!(store.has_property("age"));
        assert_eq!(store.property_key_set(), vec!["age"]);
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
        assert_eq!(store.size(), 1);
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
        assert!(rebuilt.has_property("score"));
    }
}

use super::property::{Property, PropertyTrait};
use super::property_values::PropertyValues;
use std::collections::HashMap;
use thiserror::Error;

/// Error type for property store operations.
#[derive(Error, Debug, Clone)]
pub enum PropertyStoreError {
    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    #[error("Property already exists: {0}")]
    PropertyAlreadyExists(String),

    #[error("Invalid property key: {0}")]
    InvalidPropertyKey(String),
}

pub type PropertyStoreResult<T> = Result<T, PropertyStoreError>;

/// A store that contains properties indexed by their property key.
///
/// This mirrors the TypeScript PropertyStore interface.
pub trait PropertyStore: Send + Sync {
    type Values: PropertyValues;
    type Property: PropertyTrait<Values = Self::Values>;

    /// Returns a reference to the properties map.
    fn properties(&self) -> &HashMap<String, Self::Property>;

    /// Gets a property by key.
    fn get(&self, property_key: &str) -> Option<&Self::Property> {
        self.properties().get(property_key)
    }

    /// Checks if the property store is empty.
    fn is_empty(&self) -> bool {
        self.properties().is_empty()
    }

    /// Returns the number of properties in the store.
    fn len(&self) -> usize {
        self.properties().len()
    }

    /// Returns the set of property keys in this store.
    fn key_set(&self) -> Vec<&str> {
        self.properties().keys().map(|s| s.as_str()).collect()
    }

    /// Checks if the store contains a property with the given key.
    fn contains_key(&self, property_key: &str) -> bool {
        self.properties().contains_key(property_key)
    }
}

/// Generic property store implementation.
#[derive(Debug, Clone)]
pub struct GenericPropertyStore<V: PropertyValues> {
    properties: HashMap<String, Property<V>>,
}

impl<V: PropertyValues> GenericPropertyStore<V> {
    /// Creates an empty property store.
    pub fn empty() -> Self {
        GenericPropertyStore {
            properties: HashMap::new(),
        }
    }

    /// Creates a property store from a map of properties.
    pub fn new(properties: HashMap<String, Property<V>>) -> Self {
        GenericPropertyStore { properties }
    }

}

impl<V: PropertyValues> PropertyStore for GenericPropertyStore<V> {
    type Values = V;
    type Property = Property<V>;

    fn properties(&self) -> &HashMap<String, Self::Property> {
        &self.properties
    }
}

impl<V: PropertyValues> Default for GenericPropertyStore<V> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::property::{PropertyState, ValueType};
    use crate::types::schema::{DefaultValue, PropertySchema};

    #[test]
    fn test_empty_store() {
        let store: GenericPropertyStore<DefaultLongNodePropertyValues> =
            GenericPropertyStore::empty();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_store_operations() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3);
        let schema = PropertySchema::new(
            "test_prop",
            ValueType::Long,
            DefaultValue::Long(0),
            PropertyState::Normal,
        );
        let prop = Property::new(values, schema);

        let mut props = HashMap::new();
        props.insert("test_prop".to_string(), prop);

        let store = GenericPropertyStore::new(props);
        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
        assert!(store.contains_key("test_prop"));
        assert!(store.get("test_prop").is_some());

        let keys = store.key_set();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "test_prop");
    }
}

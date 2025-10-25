//! Monadic PropertyStore: Simple Collections First PropertyStore
//!
//! A simplified property store implementation that works directly with Collections,
//! independent of graph/node/relationship complexity.

use crate::types::properties::monadic_property::MonadicProperty;
use crate::types::properties::PropertyStore;
use std::collections::HashMap;
use std::sync::Arc;

/// Monadic property store: Simple Collections First implementation
#[derive(Debug, Clone, Default)]
pub struct MonadicPropertyStore {
    properties: HashMap<String, MonadicProperty>,
}

/// Builder for MonadicPropertyStore
#[derive(Debug, Clone, Default)]
pub struct MonadicPropertyStoreBuilder {
    properties: HashMap<String, MonadicProperty>,
}

impl MonadicPropertyStore {
    /// Create an empty property store
    pub fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Create a property store with the given properties
    pub fn new(properties: HashMap<String, MonadicProperty>) -> Self {
        Self { properties }
    }

    /// Get a builder for this property store
    pub fn builder() -> MonadicPropertyStoreBuilder {
        MonadicPropertyStoreBuilder {
            properties: HashMap::new(),
        }
    }

    /// Get all properties as a vector
    pub fn get_all_properties(&self) -> Vec<&MonadicProperty> {
        self.properties.values().collect()
    }

    /// Get property values by key
    pub fn get_property_values(&self, property_key: &str) -> Option<Arc<dyn crate::types::properties::PropertyValues>> {
        self.properties
            .get(property_key)
            .map(|property| property.values_arc())
    }

    /// Convert to builder
    pub fn to_builder(&self) -> MonadicPropertyStoreBuilder {
        MonadicPropertyStoreBuilder {
            properties: self.properties.clone(),
        }
    }

    /// Get a property by key
    pub fn get(&self, key: &str) -> Option<&MonadicProperty> {
        self.properties.get(key)
    }

    /// Check if a property exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Get the number of properties
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Check if the store is empty
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Get all property keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.properties.keys()
    }
}

impl PropertyStore for MonadicPropertyStore {
    type Property = MonadicProperty;

    fn properties(&self) -> &HashMap<String, Self::Property> {
        &self.properties
    }
}

impl MonadicPropertyStoreBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Create a builder from an existing store
    pub fn from_store(store: &MonadicPropertyStore) -> Self {
        Self {
            properties: store.properties.clone(),
        }
    }

    /// Set all properties at once
    pub fn properties(mut self, props: HashMap<String, MonadicProperty>) -> Self {
        self.properties = props;
        self
    }

    /// Add a property if it doesn't exist
    pub fn put_if_absent(mut self, key: impl Into<String>, property: MonadicProperty) -> Self {
        let k = key.into();
        self.properties.entry(k).or_insert(property);
        self
    }

    /// Add or replace a property
    pub fn put(mut self, key: impl Into<String>, property: MonadicProperty) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    /// Remove a property
    pub fn remove(mut self, key: &str) -> Self {
        self.properties.remove(key);
        self
    }

    /// Build the property store
    pub fn build(self) -> MonadicPropertyStore {
        MonadicPropertyStore {
            properties: self.properties,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::{VecLong, VecDouble};
    use crate::types::properties::monadic_property_values::{LongPropertyValues, DoublePropertyValues};
    use crate::types::properties::PropertyValues;
    use crate::types::ValueType;

    #[test]
    fn empty_property_store() {
        let store = MonadicPropertyStore::empty();
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn property_store_with_properties() {
        // Create long property
        let vec_long = VecLong::from(vec![1, 2, 3]);
        let long_values = LongPropertyValues::new(vec_long, 0);
        let age_property = MonadicProperty::of("age", Arc::new(long_values));

        // Create double property
        let vec_double = VecDouble::from(vec![1.5, 2.5, 3.5]);
        let double_values = DoublePropertyValues::new(vec_double, 0.0);
        let score_property = MonadicProperty::of("score", Arc::new(double_values));

        // Create store with properties
        let mut properties = HashMap::new();
        properties.insert("age".to_string(), age_property);
        properties.insert("score".to_string(), score_property);

        let store = MonadicPropertyStore::new(properties);

        assert_eq!(store.len(), 2);
        assert!(store.contains_key("age"));
        assert!(store.contains_key("score"));
        assert!(!store.is_empty());
    }

    #[test]
    fn property_store_builder() {
        // Create properties
        let vec_long = VecLong::from(vec![10, 20, 30]);
        let long_values = LongPropertyValues::new(vec_long, 0);
        let rank_property = MonadicProperty::of("rank", Arc::new(long_values));

        let vec_double = VecDouble::from(vec![5.0, 6.0]);
        let double_values = DoublePropertyValues::new(vec_double, 0.0);
        let rating_property = MonadicProperty::of("rating", Arc::new(double_values));

        // Build store
        let store = MonadicPropertyStore::builder()
            .put("rank", rank_property)
            .put("rating", rating_property)
            .build();

        assert_eq!(store.len(), 2);
        assert!(store.contains_key("rank"));
        assert!(store.contains_key("rating"));
    }

    #[test]
    fn property_store_get_values() {
        let vec_long = VecLong::from(vec![100, 200]);
        let long_values = LongPropertyValues::new(vec_long, 0);
        let property = MonadicProperty::of("count", Arc::new(long_values));

        let store = MonadicPropertyStore::builder()
            .put("count", property)
            .build();

        let values = store.get_property_values("count").unwrap();
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.element_count(), 2);
    }

    #[test]
    fn property_store_builder_put_if_absent() {
        let vec_long1 = VecLong::from(vec![1, 2]);
        let long_values1 = LongPropertyValues::new(vec_long1, 0);
        let property1 = MonadicProperty::of("value", Arc::new(long_values1));

        let vec_long2 = VecLong::from(vec![3, 4]);
        let long_values2 = LongPropertyValues::new(vec_long2, 0);
        let property2 = MonadicProperty::of("value", Arc::new(long_values2));

        let store = MonadicPropertyStore::builder()
            .put("value", property1)
            .put_if_absent("value", property2)  // Should not replace
            .build();

        let values = store.get_property_values("value").unwrap();
        assert_eq!(values.element_count(), 2);  // Original property retained
    }

    #[test]
    fn property_store_to_builder() {
        let vec_long = VecLong::from(vec![1, 2, 3]);
        let long_values = LongPropertyValues::new(vec_long, 0);
        let property = MonadicProperty::of("test", Arc::new(long_values));

        let store1 = MonadicPropertyStore::builder()
            .put("test", property)
            .build();

        let vec_double = VecDouble::from(vec![4.0, 5.0]);
        let double_values = DoublePropertyValues::new(vec_double, 0.0);
        let new_property = MonadicProperty::of("new", Arc::new(double_values));

        let store2 = store1
            .to_builder()
            .put("new", new_property)
            .build();

        assert_eq!(store2.len(), 2);
        assert!(store2.contains_key("test"));
        assert!(store2.contains_key("new"));
    }
}


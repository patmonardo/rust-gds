use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use crate::types::property::PropertyState;
use crate::types::schema::{Aggregation, DefaultValue};
use std::collections::HashMap;
use std::sync::Arc;

/// Concrete implementation of a relationship property store.
/// Stores properties in an owned `HashMap`, mirroring the default
/// TypeScript implementation while embracing Rust ergonomics.
#[derive(Debug, Clone, Default)]
pub struct DefaultRelationshipPropertyStore {
    properties: HashMap<String, RelationshipProperty>,
}

impl DefaultRelationshipPropertyStore {
    /// Creates an empty relationship property store.
    pub fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Creates a relationship property store from a map of properties.
    pub fn new(properties: HashMap<String, RelationshipProperty>) -> Self {
        Self { properties }
    }

    /// Creates a builder for constructing a relationship property store.
    pub fn builder() -> RelationshipPropertyStoreBuilder {
        RelationshipPropertyStoreBuilder::new()
    }

    /// Returns a reference to the underlying property map.
    pub fn relationship_properties(&self) -> &HashMap<String, RelationshipProperty> {
        &self.properties
    }

    /// Gets a property by key.
    pub fn get(&self, property_key: &str) -> Option<&RelationshipProperty> {
        self.properties.get(property_key)
    }

    /// Checks if the property store is empty.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns the number of properties in the store.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns the set of property keys in this store.
    pub fn key_set(&self) -> Vec<&str> {
        self.properties.keys().map(|s| s.as_str()).collect()
    }

    /// Returns an iterator over all properties in this store.
    pub fn values(&self) -> Vec<&RelationshipProperty> {
        self.properties.values().collect()
    }

    /// Checks if the store contains a property with the given key.
    pub fn contains_key(&self, property_key: &str) -> bool {
        self.properties.contains_key(property_key)
    }

    /// Inserts or replaces the provided property using its key.
    pub fn put_property_entry(
        &mut self,
        property: RelationshipProperty,
    ) -> Option<RelationshipProperty> {
        self.properties.insert(property.key().to_string(), property)
    }

    /// Removes the property matching the provided key and returns it when present.
    pub fn remove_property(&mut self, property_key: &str) -> Option<RelationshipProperty> {
        self.properties.remove(property_key)
    }

    /// Creates a new store containing only the property referenced by `property_key`.
    /// Returns an empty store when the key is not present.
    pub fn filter(&self, property_key: &str) -> DefaultRelationshipPropertyStore {
        match self.properties.get(property_key) {
            Some(property) => {
                let mut filtered = HashMap::new();
                filtered.insert(property_key.to_string(), property.clone());
                DefaultRelationshipPropertyStore::new(filtered)
            }
            None => DefaultRelationshipPropertyStore::empty(),
        }
    }
}

/// Builder for `DefaultRelationshipPropertyStore`.
#[derive(Debug, Default, Clone)]
pub struct RelationshipPropertyStoreBuilder {
    properties: HashMap<String, RelationshipProperty>,
}

impl RelationshipPropertyStoreBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the store using the default aggregation strategy.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let property = RelationshipProperty::with_default_aggregation(
            key_str.clone(),
            PropertyState::Normal,
            values,
        );
        self.properties.insert(key_str, property);
        self
    }

    /// Adds a property with an explicit aggregation strategy.
    pub fn put_property_with_aggregation(
        mut self,
        key: impl Into<String>,
        values: Arc<dyn RelationshipPropertyValues>,
        aggregation: Aggregation,
    ) -> Self {
        let key_str = key.into();
        let value_type = values.value_type();
        let property = RelationshipProperty::of(
            key_str.clone(),
            PropertyState::Normal,
            values,
            DefaultValue::of(value_type),
            aggregation,
        );
        self.properties.insert(key_str, property);
        self
    }

    /// Adds an already constructed property to the store.
    pub fn put(mut self, key: impl Into<String>, property: RelationshipProperty) -> Self {
        self.properties.insert(key.into(), property);
        self
    }

    /// Builds the relationship property store.
    pub fn build(self) -> DefaultRelationshipPropertyStore {
        DefaultRelationshipPropertyStore::new(self.properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
    use crate::types::schema::Aggregation;

    #[test]
    fn test_empty_relationship_property_store() {
        let store = DefaultRelationshipPropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.key_set().is_empty());
        assert!(!store.contains_key("weight"));
    }

    #[test]
    fn test_relationship_property_store_builder() {
        let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(
            vec![1.0, 2.5, 3.7],
            0.0,
            3,
        ));
        let cost_values = Arc::new(DefaultRelationshipPropertyValues::new(
            vec![10.0, 20.0, 30.0],
            0.0,
            3,
        ));

        let store = DefaultRelationshipPropertyStore::builder()
            .put_property("weight", weight_values)
            .put_property_with_aggregation("cost", cost_values, Aggregation::Sum)
            .build();

        assert!(!store.is_empty());
        assert_eq!(store.len(), 2);
        assert!(store.contains_key("weight"));
        assert!(store.contains_key("cost"));

        let cost_prop = store.get("cost").unwrap();
        assert_eq!(cost_prop.aggregation(), Aggregation::Sum);

        let filtered = store.filter("weight");
        assert_eq!(filtered.len(), 1);
        assert!(filtered.contains_key("weight"));
        assert!(!filtered.contains_key("cost"));
    }
}

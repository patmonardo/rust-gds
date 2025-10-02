use super::relationship_property::RelationshipProperty;
use super::relationship_property_values::RelationshipPropertyValues;
use crate::types::property::PropertyState;
use crate::types::schema::Aggregation;
use std::collections::HashMap;

/// Represents a store for relationship properties.
/// Each property is identified by a string key and holds RelationshipPropertyValues.
#[derive(Debug)]
pub struct RelationshipPropertyStore {
    properties: HashMap<String, RelationshipProperty>,
}

impl RelationshipPropertyStore {
    /// Creates an empty relationship property store.
    pub fn empty() -> Self {
        RelationshipPropertyStore {
            properties: HashMap::new(),
        }
    }

    /// Creates a relationship property store from a map of properties.
    pub fn new(properties: HashMap<String, RelationshipProperty>) -> Self {
        RelationshipPropertyStore { properties }
    }

    /// Creates a builder for constructing a RelationshipPropertyStore.
    pub fn builder() -> RelationshipPropertyStoreBuilder {
        RelationshipPropertyStoreBuilder::new()
    }

    /// Returns a reference to the properties map.
    pub fn properties(&self) -> &HashMap<String, RelationshipProperty> {
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

    /// Checks if the store contains a property with the given key.
    pub fn contains_key(&self, property_key: &str) -> bool {
        self.properties.contains_key(property_key)
    }
}

impl Default for RelationshipPropertyStore {
    fn default() -> Self {
        Self::empty()
    }
}

/// Builder for RelationshipPropertyStore.
#[derive(Debug)]
pub struct RelationshipPropertyStoreBuilder {
    properties: HashMap<String, RelationshipProperty>,
}

impl RelationshipPropertyStoreBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        RelationshipPropertyStoreBuilder {
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the store with default aggregation.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: Box<dyn RelationshipPropertyValues>,
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

    /// Adds a property with a specific aggregation strategy.
    pub fn put_property_with_aggregation(
        mut self,
        key: impl Into<String>,
        values: Box<dyn RelationshipPropertyValues>,
        aggregation: Aggregation,
    ) -> Self {
        let key_str = key.into();
        let value_type = values.value_type();
        let property = RelationshipProperty::of(
            key_str.clone(),
            PropertyState::Normal,
            values,
            crate::types::schema::DefaultValue::of(value_type),
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

    /// Builds the RelationshipPropertyStore.
    pub fn build(self) -> RelationshipPropertyStore {
        RelationshipPropertyStore::new(self.properties)
    }
}

impl Default for RelationshipPropertyStoreBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;

    #[test]
    fn test_empty_relationship_property_store() {
        let store = RelationshipPropertyStore::builder().build();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_relationship_property_store_builder() {
        let weight_values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.5, 3.7], 0.0, 3);
        let cost_values = DefaultRelationshipPropertyValues::new(vec![10.0, 20.0, 30.0], 0.0, 3);

        let store = RelationshipPropertyStore::builder()
            .put_property("weight", Box::new(weight_values))
            .put_property_with_aggregation("cost", Box::new(cost_values), Aggregation::Sum)
            .build();

        assert!(!store.is_empty());
        assert_eq!(store.len(), 2);
        assert!(store.contains_key("weight"));
        assert!(store.contains_key("cost"));

        let cost_prop = store.get("cost").unwrap();
        assert_eq!(cost_prop.aggregation(), Aggregation::Sum);
    }
}

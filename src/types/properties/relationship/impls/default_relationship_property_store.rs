use crate::types::properties::property_store::PropertyStore;
use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::{
    relationship_property_store::{RelationshipPropertyStore, RelationshipPropertyStoreBuilder},
    relationship_property_values::RelationshipPropertyValues,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Default implementation of RelationshipPropertyStore.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipPropertyStore {
    properties: HashMap<String, RelationshipProperty>,
}

/// Builder for DefaultRelationshipPropertyStore.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipPropertyStoreBuilder {
    properties: HashMap<String, RelationshipProperty>,
}

/* Base PropertyStore implementation - only properties() needed */
impl PropertyStore for DefaultRelationshipPropertyStore {
    type Property = RelationshipProperty;

    fn properties(&self) -> &HashMap<String, Self::Property> {
        &self.properties
    }
}

/* Domain-specific RelationshipPropertyStore implementation */
impl RelationshipPropertyStore for DefaultRelationshipPropertyStore {
    type Builder = DefaultRelationshipPropertyStoreBuilder;

    fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn new(properties: HashMap<String, Self::Property>) -> Self {
        Self { properties }
    }

    fn builder() -> Self::Builder {
        DefaultRelationshipPropertyStoreBuilder::new()
    }

    fn get_all_properties(&self) -> Vec<&Self::Property> {
        self.properties.values().collect()
    }

    fn get_property_values(&self, property_key: &str) -> Option<&dyn RelationshipPropertyValues> {
        self.properties
            .get(property_key)
            .map(|property| property.values())
    }

    fn to_builder(&self) -> Self::Builder {
        DefaultRelationshipPropertyStoreBuilder::from_store(self)
    }
}

/* Builder trait implementation */
impl RelationshipPropertyStoreBuilder for DefaultRelationshipPropertyStoreBuilder {
    type Store = DefaultRelationshipPropertyStore;
    type Property = RelationshipProperty;

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
        let key = key.into();
        self.properties.entry(key).or_insert(property);
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
        DefaultRelationshipPropertyStore {
            properties: self.properties,
        }
    }
}

/* Inherent convenience methods for the store (ergonomics without trait import) */
impl DefaultRelationshipPropertyStore {
    /// Returns the number of properties in this store.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns whether this store is empty.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns a reference to the property with the given key.
    pub fn get(&self, key: &str) -> Option<&RelationshipProperty> {
        self.properties.get(key)
    }

    /// Returns whether the store contains a property with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Returns a reference to the underlying properties map.
    /// Note: In TypeScript, this is called `relationshipProperties()`.
    pub fn relationship_properties(&self) -> &HashMap<String, RelationshipProperty> {
        &self.properties
    }
}

/* Inherent convenience methods for the builder (do not belong to the trait) */
impl DefaultRelationshipPropertyStoreBuilder {
    /// Convenience method to add a property by supplying property values directly.
    /// Reduces imports for callers who just want to add a property by values.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: impl Into<Arc<dyn RelationshipPropertyValues>>,
    ) -> Self {
        let key_str = key.into();
        let values = values.into();
        use crate::types::property_state::PropertyState;

        let prop =
            RelationshipProperty::with_state(key_str.clone(), PropertyState::Persistent, values);
        self.properties.insert(key_str, prop);
        self
    }
}

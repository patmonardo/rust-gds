use super::graph_property_values::GraphPropertyValues;
use crate::types::properties::PropertyStore;
use std::collections::HashMap;

/// Abstraction describing the behaviour expected from any graph property store.
/// Extends PropertyStore to inherit common map-like operations.
pub trait GraphPropertyStore: PropertyStore {
    type Builder: GraphPropertyStoreBuilder<Store = Self, Property = Self::Property>;

    fn empty() -> Self
    where
        Self: Sized;

    fn new(properties: HashMap<String, Self::Property>) -> Self
    where
        Self: Sized;

    fn builder() -> Self::Builder
    where
        Self: Sized;

    // Domain-specific methods
    fn get_all_properties(&self) -> Vec<&Self::Property>;
    fn get_property_values(&self, property_key: &str) -> Option<&dyn GraphPropertyValues>;
    fn to_builder(&self) -> Self::Builder;
}

/// Builder abstraction for graph property stores.
pub trait GraphPropertyStoreBuilder {
    type Store: GraphPropertyStore<Builder = Self, Property = Self::Property>;
    type Property;

    fn new() -> Self;
    fn from_store(store: &Self::Store) -> Self;

    fn properties(self, props: HashMap<String, Self::Property>) -> Self;
    fn put_if_absent(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn put(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn remove_property(self, key: &str) -> Self;

    fn build(self) -> Self::Store;
}

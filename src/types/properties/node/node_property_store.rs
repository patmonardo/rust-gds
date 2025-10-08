use super::node_property_values::NodePropertyValues;
use crate::types::properties::PropertyStore;
use std::collections::HashMap;

/// Trait describing a store of node properties (traits-first pattern).
/// Extends PropertyStore to inherit common map-like operations.
pub trait NodePropertyStore: PropertyStore {
    type Builder: NodePropertyStoreBuilder<Store = Self, Property = Self::Property>;

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
    fn get_property_values(&self, property_key: &str) -> Option<&dyn NodePropertyValues>;
    fn to_builder(&self) -> Self::Builder;
}

/// Builder trait for constructing node property stores.
pub trait NodePropertyStoreBuilder {
    type Store: NodePropertyStore<Builder = Self, Property = Self::Property>;
    type Property;

    fn new() -> Self;
    fn from_store(store: &Self::Store) -> Self;

    fn properties(self, props: HashMap<String, Self::Property>) -> Self;
    fn put_if_absent(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn put(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn remove_property(self, key: &str) -> Self;

    fn build(self) -> Self::Store;
}

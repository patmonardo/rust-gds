use super::node_property_values::NodePropertyValues;
use std::collections::HashMap;

/// Trait describing a store of node properties (traits-first pattern).
pub trait NodePropertyStore {
    type Property;
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

    fn has_property(&self, property_key: &str) -> bool;
    fn property_key_set(&self) -> Vec<&str>;
    fn get_property(&self, property_key: &str) -> Option<&Self::Property>;
    fn get_all_properties(&self) -> Vec<&Self::Property>;
    fn get_property_values(&self, property_key: &str) -> Option<&dyn NodePropertyValues>;
    fn count(&self) -> usize;
    fn is_empty(&self) -> bool;
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

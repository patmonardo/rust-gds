use super::graph_property_values::GraphPropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// Abstraction describing the behaviour expected from any graph property store.
pub trait GraphPropertyStore {
    type Property;
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

    fn has_property(&self, property_key: &str) -> bool;
    fn property_key_set(&self) -> Vec<&str>;
    fn get_property(&self, property_key: &str) -> Option<&Self::Property>;

    fn get_property_or_null(&self, property_key: &str) -> Option<&Self::Property> {
        self.get_property(property_key)
    }

    fn get_all_properties(&self) -> Vec<&Self::Property>;

    // Return a borrowed reference to the underlying values. Callers that need ownership
    // can wrap it themselves if the concrete type exposes cloning.
    fn get_property_values(&self, property_key: &str) -> Option<&dyn GraphPropertyValues>;

    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_builder(&self) -> Self::Builder;
}

/// Builder abstraction for graph property stores.
pub trait GraphPropertyStoreBuilder {
    type Store: GraphPropertyStore<Builder = Self, Property = Self::Property>;
    type Property;

    fn new() -> Self;
    fn from_store(store: &Self::Store) -> Self;

    fn put_property(self, key: impl Into<String>, values: Arc<dyn GraphPropertyValues>) -> Self;

    fn put(self, key: impl Into<String>, property: Self::Property) -> Self;

    fn build(self) -> Self::Store;
}

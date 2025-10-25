//! Triadic PropertyStore: Composing Three MonadicPropertyStores
//!
//! This module demonstrates how three independent MonadicPropertyStores can be
//! composed into a unified three-level system, each with separate key spaces
//! and independent backend choices.

use crate::types::properties::monadic::{MonadicPropertyStore, MonadicProperty};
use crate::types::properties::PropertyValues;
use std::sync::Arc;

/// Triadic PropertyStore: Three-Level Composition
///
/// Composes three independent MonadicPropertyStores for:
/// - Level 0: Meta properties (graph/system metadata)
/// - Level 1: Node properties (entity/element properties)
/// - Level 2: Link properties (connection/relationship properties)
///
/// Each level maintains its own key space and can use different Collections backends.
#[derive(Debug, Clone, Default)]
pub struct TriadicPropertyStore {
    /// Meta-level properties (graph/system metadata)
    meta_properties: MonadicPropertyStore,
    
    /// Node-level properties (entity/element properties)
    node_properties: MonadicPropertyStore,
    
    /// Link-level properties (connection/relationship properties)
    link_properties: MonadicPropertyStore,
}

impl TriadicPropertyStore {
    /// Create an empty triadic property store
    pub fn empty() -> Self {
        Self {
            meta_properties: MonadicPropertyStore::empty(),
            node_properties: MonadicPropertyStore::empty(),
            link_properties: MonadicPropertyStore::empty(),
        }
    }

    /// Create a triadic property store with explicit stores for each level
    pub fn new(
        meta_properties: MonadicPropertyStore,
        node_properties: MonadicPropertyStore,
        link_properties: MonadicPropertyStore,
    ) -> Self {
        Self {
            meta_properties,
            node_properties,
            link_properties,
        }
    }

    /// Get a builder for this property store
    pub fn builder() -> TriadicPropertyStoreBuilder {
        TriadicPropertyStoreBuilder::new()
    }

    // ========================================================================
    // Meta Property Access (Level 0)
    // ========================================================================

    /// Get a meta property by key
    pub fn get_meta_property(&self, key: &str) -> Option<&MonadicProperty> {
        self.meta_properties.get(key)
    }

    /// Get meta property values by key
    pub fn get_meta_property_values(&self, key: &str) -> Option<Arc<dyn PropertyValues>> {
        self.meta_properties.get_property_values(key)
    }

    /// Get all meta property keys
    pub fn meta_property_keys(&self) -> impl Iterator<Item = &String> {
        self.meta_properties.keys()
    }

    /// Check if a meta property exists
    pub fn contains_meta_property(&self, key: &str) -> bool {
        self.meta_properties.contains_key(key)
    }

    /// Get the number of meta properties
    pub fn meta_property_count(&self) -> usize {
        self.meta_properties.len()
    }

    /// Get a reference to the meta properties store
    pub fn meta_properties(&self) -> &MonadicPropertyStore {
        &self.meta_properties
    }

    // ========================================================================
    // Node Property Access (Level 1)
    // ========================================================================

    /// Get a node property by key
    pub fn get_node_property(&self, key: &str) -> Option<&MonadicProperty> {
        self.node_properties.get(key)
    }

    /// Get node property values by key
    pub fn get_node_property_values(&self, key: &str) -> Option<Arc<dyn PropertyValues>> {
        self.node_properties.get_property_values(key)
    }

    /// Get all node property keys
    pub fn node_property_keys(&self) -> impl Iterator<Item = &String> {
        self.node_properties.keys()
    }

    /// Check if a node property exists
    pub fn contains_node_property(&self, key: &str) -> bool {
        self.node_properties.contains_key(key)
    }

    /// Get the number of node properties
    pub fn node_property_count(&self) -> usize {
        self.node_properties.len()
    }

    /// Get a reference to the node properties store
    pub fn node_properties(&self) -> &MonadicPropertyStore {
        &self.node_properties
    }

    // ========================================================================
    // Link Property Access (Level 2)
    // ========================================================================

    /// Get a link property by key
    pub fn get_link_property(&self, key: &str) -> Option<&MonadicProperty> {
        self.link_properties.get(key)
    }

    /// Get link property values by key
    pub fn get_link_property_values(&self, key: &str) -> Option<Arc<dyn PropertyValues>> {
        self.link_properties.get_property_values(key)
    }

    /// Get all link property keys
    pub fn link_property_keys(&self) -> impl Iterator<Item = &String> {
        self.link_properties.keys()
    }

    /// Check if a link property exists
    pub fn contains_link_property(&self, key: &str) -> bool {
        self.link_properties.contains_key(key)
    }

    /// Get the number of link properties
    pub fn link_property_count(&self) -> usize {
        self.link_properties.len()
    }

    /// Get a reference to the link properties store
    pub fn link_properties(&self) -> &MonadicPropertyStore {
        &self.link_properties
    }

    // ========================================================================
    // Unified Access Patterns
    // ========================================================================

    /// Get the total number of properties across all levels
    pub fn total_property_count(&self) -> usize {
        self.meta_property_count() + self.node_property_count() + self.link_property_count()
    }

    /// Check if the store is completely empty
    pub fn is_empty(&self) -> bool {
        self.meta_properties.is_empty()
            && self.node_properties.is_empty()
            && self.link_properties.is_empty()
    }

    /// Convert to builder for modification
    pub fn to_builder(&self) -> TriadicPropertyStoreBuilder {
        TriadicPropertyStoreBuilder {
            meta_properties_builder: Some(self.meta_properties.to_builder()),
            node_properties_builder: Some(self.node_properties.to_builder()),
            link_properties_builder: Some(self.link_properties.to_builder()),
        }
    }
}

/// Builder for TriadicPropertyStore
#[derive(Debug, Clone, Default)]
pub struct TriadicPropertyStoreBuilder {
    meta_properties_builder: Option<crate::types::properties::monadic::MonadicPropertyStoreBuilder>,
    node_properties_builder: Option<crate::types::properties::monadic::MonadicPropertyStoreBuilder>,
    link_properties_builder: Option<crate::types::properties::monadic::MonadicPropertyStoreBuilder>,
}

impl TriadicPropertyStoreBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            meta_properties_builder: Some(MonadicPropertyStore::builder()),
            node_properties_builder: Some(MonadicPropertyStore::builder()),
            link_properties_builder: Some(MonadicPropertyStore::builder()),
        }
    }

    /// Set meta properties store directly
    pub fn meta_properties(mut self, store: MonadicPropertyStore) -> Self {
        self.meta_properties_builder = Some(store.to_builder());
        self
    }

    /// Set node properties store directly
    pub fn node_properties(mut self, store: MonadicPropertyStore) -> Self {
        self.node_properties_builder = Some(store.to_builder());
        self
    }

    /// Set link properties store directly
    pub fn link_properties(mut self, store: MonadicPropertyStore) -> Self {
        self.link_properties_builder = Some(store.to_builder());
        self
    }

    /// Add a meta property
    pub fn put_meta(mut self, key: impl Into<String>, property: MonadicProperty) -> Self {
        if let Some(builder) = self.meta_properties_builder.take() {
            self.meta_properties_builder = Some(builder.put(key, property));
        }
        self
    }

    /// Add a node property
    pub fn put_node(mut self, key: impl Into<String>, property: MonadicProperty) -> Self {
        if let Some(builder) = self.node_properties_builder.take() {
            self.node_properties_builder = Some(builder.put(key, property));
        }
        self
    }

    /// Add a link property
    pub fn put_link(mut self, key: impl Into<String>, property: MonadicProperty) -> Self {
        if let Some(builder) = self.link_properties_builder.take() {
            self.link_properties_builder = Some(builder.put(key, property));
        }
        self
    }

    /// Build the triadic property store
    pub fn build(self) -> TriadicPropertyStore {
        TriadicPropertyStore {
            meta_properties: self
                .meta_properties_builder
                .unwrap_or_else(|| MonadicPropertyStore::builder())
                .build(),
            node_properties: self
                .node_properties_builder
                .unwrap_or_else(|| MonadicPropertyStore::builder())
                .build(),
            link_properties: self
                .link_properties_builder
                .unwrap_or_else(|| MonadicPropertyStore::builder())
                .build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::monadic::property_values::{MonadicLongPropertyValues, MonadicDoublePropertyValues};
    use crate::collections::backends::vec::{VecLong, VecDouble};
    use crate::types::ValueType;

    #[test]
    fn empty_triadic_store() {
        let store = TriadicPropertyStore::empty();
        assert_eq!(store.meta_property_count(), 0);
        assert_eq!(store.node_property_count(), 0);
        assert_eq!(store.link_property_count(), 0);
        assert_eq!(store.total_property_count(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn triadic_store_with_properties() {
        // Create meta property (scalar value for graph metadata)
        let vec_long = VecLong::from(vec![42]);
        let meta_values = MonadicLongPropertyValues::new(vec_long, 0);
        let graph_id = MonadicProperty::of("graph_id", Arc::new(meta_values));

        // Create node property (one value per node)
        let vec_long = VecLong::from(vec![25, 30, 35]);
        let node_values = MonadicLongPropertyValues::new(vec_long, 0);
        let age = MonadicProperty::of("age", Arc::new(node_values));

        // Create link property (one value per relationship)
        let vec_double = VecDouble::from(vec![0.8, 0.6, 0.9, 0.7]);
        let link_values = MonadicDoublePropertyValues::new(vec_double, 0.0);
        let weight = MonadicProperty::of("weight", Arc::new(link_values));

        // Build triadic store
        let store = TriadicPropertyStore::builder()
            .put_meta("graph_id", graph_id)
            .put_node("age", age)
            .put_link("weight", weight)
            .build();

        assert_eq!(store.meta_property_count(), 1);
        assert_eq!(store.node_property_count(), 1);
        assert_eq!(store.link_property_count(), 1);
        assert_eq!(store.total_property_count(), 3);
        assert!(!store.is_empty());
    }

    #[test]
    fn triadic_store_separate_key_spaces() {
        // All three levels can have "name" property with different meanings
        let vec_long = VecLong::from(vec![1]);
        let meta_name = MonadicProperty::of("name", Arc::new(
            MonadicLongPropertyValues::new(vec_long, 0)
        ));

        let vec_long = VecLong::from(vec![2, 3]);
        let node_name = MonadicProperty::of("name", Arc::new(
            MonadicLongPropertyValues::new(vec_long, 0)
        ));

        let vec_long = VecLong::from(vec![4, 5, 6]);
        let link_name = MonadicProperty::of("name", Arc::new(
            MonadicLongPropertyValues::new(vec_long, 0)
        ));

        let store = TriadicPropertyStore::builder()
            .put_meta("name", meta_name)
            .put_node("name", node_name)
            .put_link("name", link_name)
            .build();

        // All three "name" properties coexist
        assert!(store.contains_meta_property("name"));
        assert!(store.contains_node_property("name"));
        assert!(store.contains_link_property("name"));

        // Each has different element counts
        let meta_values = store.get_meta_property_values("name").unwrap();
        assert_eq!(meta_values.element_count(), 1);

        let node_values = store.get_node_property_values("name").unwrap();
        assert_eq!(node_values.element_count(), 2);

        let link_values = store.get_link_property_values("name").unwrap();
        assert_eq!(link_values.element_count(), 3);
    }

    #[test]
    fn triadic_store_access_patterns() {
        let vec_long = VecLong::from(vec![100, 200]);
        let node_values = MonadicLongPropertyValues::new(vec_long, 0);
        let rank = MonadicProperty::of("rank", Arc::new(node_values));

        let store = TriadicPropertyStore::builder()
            .put_node("rank", rank)
            .build();

        // Access via get_node_property
        assert!(store.get_node_property("rank").is_some());
        assert!(store.get_meta_property("rank").is_none());
        assert!(store.get_link_property("rank").is_none());

        // Access via get_node_property_values
        let values = store.get_node_property_values("rank").unwrap();
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.element_count(), 2);

        // Access keys
        let keys: Vec<&String> = store.node_property_keys().collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "rank");
    }

    #[test]
    fn triadic_store_to_builder() {
        let vec_long = VecLong::from(vec![1]);
        let meta_prop = MonadicProperty::of("id", Arc::new(
            MonadicLongPropertyValues::new(vec_long, 0)
        ));

        let store1 = TriadicPropertyStore::builder()
            .put_meta("id", meta_prop)
            .build();

        // Convert to builder and add more
        let vec_double = VecDouble::from(vec![3.14, 2.71]);
        let node_prop = MonadicProperty::of("value", Arc::new(
            MonadicDoublePropertyValues::new(vec_double, 0.0)
        ));

        let store2 = store1.to_builder()
            .put_node("value", node_prop)
            .build();

        assert_eq!(store2.meta_property_count(), 1);
        assert_eq!(store2.node_property_count(), 1);
        assert_eq!(store2.total_property_count(), 2);
    }
}


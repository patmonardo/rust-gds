use super::PropertyCursor;
use crate::types::graph::id_map::MappedNodeId;
use std::fmt::Debug;

/// Accessor for relationship property values.
///
/// The trait mirrors the JavaScript `RelationshipProperties` interface while
/// adopting Rust idioms. Implementations can back their data from a wide range
/// of storage engines (CSR blocks, Arrow IPC buffers, memory-mapped pages,
/// etc.) as long as they can answer the fundamental "give me the value for the
/// relationship between these two mapped node ids" query.
pub trait RelationshipProperties: Debug {
    /// Returns the default property value used when a relationship is missing
    /// a concrete value in the underlying store.
    fn default_property_value(&self) -> f64;

    /// Returns the property value for the relationship identified by the
    /// `(source_id, target_id)` pair. When the underlying storage does not have
    /// a value for the relationship the provided `fallback_value` should be
    /// returned instead.
    fn relationship_property(
        &self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        fallback_value: f64,
    ) -> f64;

    /// Optional hook to obtain a property cursor positioned for the provided
    /// source node. Implementations that do not support cursored access can
    /// return `None`.
    fn property_cursor_for(
        &self,
        _source_id: MappedNodeId,
        _reuse: Option<&mut dyn PropertyCursor>,
    ) -> Option<Box<dyn PropertyCursor>> {
        None
    }
}

/// Extension helpers that provide ergonomic adapters on top of the base trait.
pub trait RelationshipPropertiesExt: RelationshipProperties {
    /// Returns the property value using this collection's default fallback.
    fn relationship_property_or_default(
        &self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
    ) -> f64 {
        self.relationship_property(source_id, target_id, self.default_property_value())
    }

    /// Returns the property value with an explicit fallback, mirroring the
    /// TypeScript overload that omits the fallback parameter.
    fn relationship_property_or(
        &self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        fallback_value: f64,
    ) -> f64 {
        self.relationship_property(source_id, target_id, fallback_value)
    }
}

impl<T: RelationshipProperties + ?Sized> RelationshipPropertiesExt for T {}

/// Relationship properties implementation that always returns the same value.
#[derive(Debug, Clone, Copy)]
pub struct ConstantRelationshipProperties {
    value: f64,
}

impl ConstantRelationshipProperties {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl RelationshipProperties for ConstantRelationshipProperties {
    fn default_property_value(&self) -> f64 {
        self.value
    }

    fn relationship_property(
        &self,
        _source_id: MappedNodeId,
        _target_id: MappedNodeId,
        _fallback_value: f64,
    ) -> f64 {
        self.value
    }
}

/// Relationship properties implementation that delegates to the supplied
/// fallback value. This is the Rust analogue of the TypeScript
/// `RelationshipProperties.empty` helper.
#[derive(Debug, Clone, Copy)]
pub struct EmptyRelationshipProperties {
    default_value: f64,
}

impl EmptyRelationshipProperties {
    pub fn new(default_value: f64) -> Self {
        Self { default_value }
    }
}

impl Default for EmptyRelationshipProperties {
    fn default() -> Self {
        Self { default_value: 0.0 }
    }
}

impl RelationshipProperties for EmptyRelationshipProperties {
    fn default_property_value(&self) -> f64 {
        self.default_value
    }

    fn relationship_property(
        &self,
        _source_id: MappedNodeId,
        _target_id: MappedNodeId,
        fallback_value: f64,
    ) -> f64 {
        fallback_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_relationship_properties_returns_same_value() {
        let props = ConstantRelationshipProperties::new(42.0);
        assert_eq!(props.default_property_value(), 42.0);
        assert_eq!(props.relationship_property_or_default(1, 2), 42.0);
        assert_eq!(props.relationship_property(1, 2, 99.0), 42.0);
    }

    #[test]
    fn empty_relationship_properties_respects_fallback() {
        let props = EmptyRelationshipProperties::new(0.5);
        assert_eq!(props.default_property_value(), 0.5);
        assert_eq!(props.relationship_property(1, 2, 1.5), 1.5);
        assert_eq!(props.relationship_property_or_default(3, 4), 0.5);
    }
}

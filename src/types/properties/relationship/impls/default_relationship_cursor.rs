use crate::types::graph::id_map::MappedNodeId;
use crate::types::properties::relationship::traits::{
    ModifiableRelationshipCursor, PropertyValue, RelationshipCursor,
};

/// Immutable relationship cursor mirroring the TypeScript primitive
/// implementation. Stores the source node id, target node id and an
/// associated property value.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultRelationshipCursor {
    source_id: MappedNodeId,
    target_id: MappedNodeId,
    property: PropertyValue,
}

impl DefaultRelationshipCursor {
    /// Construct a new immutable cursor instance.
    pub fn new(source_id: MappedNodeId, target_id: MappedNodeId, property: PropertyValue) -> Self {
        Self {
            source_id,
            target_id,
            property,
        }
    }

    /// Create a modifiable cursor seeded with the same values.
    pub fn to_modifiable(self) -> DefaultModifiableRelationshipCursor {
        DefaultModifiableRelationshipCursor::new(self.source_id, self.target_id, self.property)
    }
}

impl RelationshipCursor for DefaultRelationshipCursor {
    fn source_id(&self) -> MappedNodeId {
        self.source_id
    }

    fn target_id(&self) -> MappedNodeId {
        self.target_id
    }

    fn property(&self) -> PropertyValue {
        self.property
    }
}

/// Mutable cursor implementation used by iterators that reuse a single
/// cursor instance while traversing relationships.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultModifiableRelationshipCursor {
    source_id: MappedNodeId,
    target_id: MappedNodeId,
    property: PropertyValue,
}

impl DefaultModifiableRelationshipCursor {
    /// Creates a new modifiable cursor with the provided initial values.
    pub fn new(source_id: MappedNodeId, target_id: MappedNodeId, property: PropertyValue) -> Self {
        Self {
            source_id,
            target_id,
            property,
        }
    }

    /// Consume the modifiable cursor and return an immutable view.
    pub fn freeze(self) -> DefaultRelationshipCursor {
        DefaultRelationshipCursor::new(self.source_id, self.target_id, self.property)
    }
}

impl RelationshipCursor for DefaultModifiableRelationshipCursor {
    fn source_id(&self) -> MappedNodeId {
        self.source_id
    }

    fn target_id(&self) -> MappedNodeId {
        self.target_id
    }

    fn property(&self) -> PropertyValue {
        self.property
    }
}

impl ModifiableRelationshipCursor for DefaultModifiableRelationshipCursor {
    fn set_source_id(&mut self, source_id: MappedNodeId) {
        self.source_id = source_id;
    }

    fn set_target_id(&mut self, target_id: MappedNodeId) {
        self.target_id = target_id;
    }

    fn set_property(&mut self, property: PropertyValue) {
        self.property = property;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immutable_cursor_exposes_values() {
        let cursor = DefaultRelationshipCursor::new(1, 2, 3.5);
        assert_eq!(cursor.source_id(), 1);
        assert_eq!(cursor.target_id(), 2);
        assert_eq!(cursor.property(), 3.5);
    }

    #[test]
    fn modifiable_cursor_updates_in_place() {
        let mut cursor = DefaultModifiableRelationshipCursor::new(0, 0, 0.0);
        cursor.set_source_id(10);
        cursor.set_target_id(20);
        cursor.set_property(2.5);

        assert_eq!(cursor.source_id(), 10);
        assert_eq!(cursor.target_id(), 20);
        assert_eq!(cursor.property(), 2.5);
    }

    #[test]
    fn freeze_returns_immutable_snapshot() {
        let cursor = DefaultModifiableRelationshipCursor::new(3, 4, 5.5).freeze();
        assert_eq!(cursor.source_id(), 3);
        assert_eq!(cursor.target_id(), 4);
        assert_eq!(cursor.property(), 5.5);
    }

    #[test]
    fn to_modifiable_round_trips() {
        let original = DefaultRelationshipCursor::new(7, 8, 9.0);
        let mut modifiable = original.to_modifiable();
        modifiable.set_property(1.5);
        assert_eq!(modifiable.source_id(), 7);
        assert_eq!(modifiable.target_id(), 8);
        assert_eq!(modifiable.property(), 1.5);

        let frozen = modifiable.freeze();
        assert_eq!(frozen.source_id(), 7);
        assert_eq!(frozen.target_id(), 8);
        assert_eq!(frozen.property(), 1.5);
    }
}

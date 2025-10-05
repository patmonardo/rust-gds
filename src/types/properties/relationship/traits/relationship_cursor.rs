use super::property_cursor::PropertyValue;
use crate::types::graph::id_map::MappedNodeId;
use std::fmt::Debug;

/// Represents a relationship between two nodes with an associated property value.
pub trait RelationshipCursor: Debug {
    fn source_id(&self) -> MappedNodeId;
    fn target_id(&self) -> MappedNodeId;
    fn property(&self) -> PropertyValue;
}

/// Mutable variant of the relationship cursor used by iterator implementations
/// that reuse a single cursor instance.
pub trait ModifiableRelationshipCursor: RelationshipCursor {
    fn set_source_id(&mut self, source_id: MappedNodeId);
    fn set_target_id(&mut self, target_id: MappedNodeId);
    fn set_property(&mut self, property: PropertyValue);
}

/// Convenient alias for passing cursor trait objects around.
pub type RelationshipCursorBox = Box<dyn RelationshipCursor + Send>;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Clone, Copy)]
    struct SimpleCursor {
        source: MappedNodeId,
        target: MappedNodeId,
        property: PropertyValue,
    }

    impl RelationshipCursor for SimpleCursor {
        fn source_id(&self) -> MappedNodeId {
            self.source
        }

        fn target_id(&self) -> MappedNodeId {
            self.target
        }

        fn property(&self) -> PropertyValue {
            self.property
        }
    }

    impl ModifiableRelationshipCursor for SimpleCursor {
        fn set_source_id(&mut self, source_id: MappedNodeId) {
            self.source = source_id;
        }

        fn set_target_id(&mut self, target_id: MappedNodeId) {
            self.target = target_id;
        }

        fn set_property(&mut self, property: PropertyValue) {
            self.property = property;
        }
    }

    #[test]
    fn modifiable_cursor_updates_values() {
        let mut cursor = SimpleCursor::default();
        cursor.set_source_id(1);
        cursor.set_target_id(2);
        cursor.set_property(3.5);

        assert_eq!(cursor.source_id(), 1);
        assert_eq!(cursor.target_id(), 2);
        assert_eq!(cursor.property(), 3.5);
    }
}

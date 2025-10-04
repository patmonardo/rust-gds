use super::relationship_cursor::RelationshipCursorBox;
use super::relationship_predicate::RelationshipPredicate;
use crate::types::graph::id_map::MappedNodeId;
use crate::types::properties::relationship::PropertyValue;

/// Relationship traversal exposed through cursor streams.
pub trait RelationshipIterator: RelationshipPredicate + Send + Sync {
    /// Stream the outgoing relationships for `node_id`, yielding cursor snapshots that
    /// include the property value or `fallback_value` when none exists.
    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a>;

    /// Stream the incoming relationships for `node_id` with the same semantics as
    /// [`stream_relationships`].
    fn stream_inverse_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a>;

    /// Create a concurrent-safe copy of this iterator.
    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator>;
}

/// Boxed iterator over relationship cursors.
pub type RelationshipStream<'a> = Box<dyn Iterator<Item = RelationshipCursorBox> + Send + 'a>;

#[cfg(test)]
mod tests {
    use super::super::relationship_cursor;
    use super::*;

    #[derive(Clone)]
    struct TestIterator {
        edges: Vec<(MappedNodeId, MappedNodeId, PropertyValue)>,
    }

    impl RelationshipPredicate for TestIterator {
        fn exists(&self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
            self.edges
                .iter()
                .any(|(s, t, _)| *s == source_id && *t == target_id)
        }
    }

    impl RelationshipIterator for TestIterator {
        fn stream_relationships<'a>(
            &'a self,
            node_id: MappedNodeId,
            _fallback_value: PropertyValue,
        ) -> RelationshipStream<'a> {
            let iter = self
                .edges
                .iter()
                .cloned()
                .filter(move |(s, _, _)| *s == node_id)
                .map(|(s, t, p)| {
                    Box::new(SimpleCursor {
                        source: s,
                        target: t,
                        property: p,
                    }) as RelationshipCursorBox
                });
            Box::new(iter)
        }

        fn stream_inverse_relationships<'a>(
            &'a self,
            node_id: MappedNodeId,
            _fallback_value: PropertyValue,
        ) -> RelationshipStream<'a> {
            let iter = self
                .edges
                .iter()
                .cloned()
                .filter(move |(_, t, _)| *t == node_id)
                .map(|(s, t, p)| {
                    Box::new(SimpleCursor {
                        source: s,
                        target: t,
                        property: p,
                    }) as RelationshipCursorBox
                });
            Box::new(iter)
        }

        fn concurrent_copy(&self) -> Box<dyn RelationshipIterator> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug)]
    struct SimpleCursor {
        source: MappedNodeId,
        target: MappedNodeId,
        property: PropertyValue,
    }

    impl relationship_cursor::RelationshipCursor for SimpleCursor {
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

    #[test]
    fn streams_yield_relationships() {
        let iter = TestIterator {
            edges: vec![(1, 2, 1.5), (1, 3, 2.5)],
        };
        let collected: Vec<(MappedNodeId, MappedNodeId, PropertyValue)> = iter
            .stream_relationships(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.property()))
            .collect();
        assert_eq!(collected, vec![(1, 2, 1.5), (1, 3, 2.5)]);
    }

    #[test]
    fn inverse_streams_filter_by_target() {
        let iter = TestIterator {
            edges: vec![(0, 1, 0.2), (2, 1, 0.4), (2, 3, 0.6)],
        };
        let collected: Vec<(MappedNodeId, MappedNodeId, PropertyValue)> = iter
            .stream_inverse_relationships(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.property()))
            .collect();
        assert_eq!(collected, vec![(0, 1, 0.2), (2, 1, 0.4)]);
    }

    #[test]
    fn streams_respect_existence_checks() {
        let iter = TestIterator {
            edges: vec![(5, 6, 1.0)],
        };
        assert!(iter.exists(5, 6));
        assert!(iter.stream_relationships(4, 0.0).next().is_none());
    }

    #[test]
    fn concurrent_copy_clones_state() {
        let iter = TestIterator {
            edges: vec![(1, 2, 3.0)],
        };
        let clone = iter.concurrent_copy();
        let collected: Vec<(MappedNodeId, MappedNodeId, PropertyValue)> = clone
            .stream_relationships(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.property()))
            .collect();
        assert_eq!(collected, vec![(1, 2, 3.0)]);
    }
}

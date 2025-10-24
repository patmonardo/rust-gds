use super::relationship_cursor::{RelationshipCursorBox, WeightedRelationshipCursorBox};
use super::relationship_predicate::RelationshipPredicate;
use crate::types::graph::id_map::NodeId;

/// Relationship traversal exposed through cursor streams.
pub trait RelationshipIterator: RelationshipPredicate + Send + Sync {
    /// Stream the outgoing relationships for `node_id`, yielding cursor snapshots that
    /// include the property value or `fallback_value` when none exists.
    fn stream_relationships<'a>(
        &'a self,
        node_id: NodeId,
        fallback_value: f64,
    ) -> RelationshipStream<'a>;

    /// Stream the incoming relationships for `node_id` with the same semantics as
    /// [`stream_relationships`].
    fn stream_inverse_relationships<'a>(
        &'a self,
        node_id: NodeId,
        fallback_value: f64,
    ) -> RelationshipStream<'a>;

    /// Create a concurrent-safe copy of this iterator.
    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator>;

    // === Phase 2C: Weighted Relationship Streams ===
    
    /// Stream the outgoing relationships for `node_id`, yielding weighted cursor snapshots
    /// optimized for algorithms requiring direct f64 weight access.
    /// 
    /// This method provides high-performance access to relationship weights without
    /// type conversion overhead, designed for pathfinding algorithms.
    fn stream_relationships_weighted<'a>(
        &'a self,
        node_id: NodeId,
        fallback_value: f64,
    ) -> WeightedRelationshipStream<'a>;

    /// Stream the incoming relationships for `node_id` with the same semantics as
    /// [`stream_relationships_weighted`].
    fn stream_inverse_relationships_weighted<'a>(
        &'a self,
        node_id: NodeId,
        fallback_value: f64,
    ) -> WeightedRelationshipStream<'a>;
}

/// Boxed iterator over relationship cursors.
pub type RelationshipStream<'a> = Box<dyn Iterator<Item = RelationshipCursorBox> + Send + 'a>;

/// Boxed iterator over weighted relationship cursors.
pub type WeightedRelationshipStream<'a> = Box<dyn Iterator<Item = WeightedRelationshipCursorBox> + Send + 'a>;

#[cfg(test)]
mod tests {
    use super::super::relationship_cursor;
    use super::*;

    #[derive(Clone)]
    struct TestIterator {
        edges: Vec<(NodeId, NodeId, f64)>,
    }

    impl RelationshipPredicate for TestIterator {
        fn exists(&self, source_id: NodeId, target_id: NodeId) -> bool {
            self.edges
                .iter()
                .any(|(s, t, _)| *s == source_id && *t == target_id)
        }
    }

    impl RelationshipIterator for TestIterator {
        fn stream_relationships<'a>(
            &'a self,
            node_id: NodeId,
            _fallback_value: f64,
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
            node_id: NodeId,
            _fallback_value: f64,
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

        // === Phase 2C: Weighted Stream Implementations ===
        
        fn stream_relationships_weighted<'a>(
            &'a self,
            node_id: NodeId,
            _fallback_value: f64,
        ) -> WeightedRelationshipStream<'a> {
            let iter = self
                .edges
                .iter()
                .cloned()
                .filter(move |(s, _, _)| *s == node_id)
                .map(|(s, t, p)| {
                    // Direct f64 weight access - no conversion needed
                    Box::new(WeightedCursor {
                        source: s,
                        target: t,
                        weight: p,
                    }) as WeightedRelationshipCursorBox
                });
            Box::new(iter)
        }

        fn stream_inverse_relationships_weighted<'a>(
            &'a self,
            node_id: NodeId,
            _fallback_value: f64,
        ) -> WeightedRelationshipStream<'a> {
            let iter = self
                .edges
                .iter()
                .cloned()
                .filter(move |(_, t, _)| *t == node_id)
                .map(|(s, t, p)| {
                    // Direct f64 weight access - no conversion needed
                    Box::new(WeightedCursor {
                        source: s,
                        target: t,
                        weight: p,
                    }) as WeightedRelationshipCursorBox
                });
            Box::new(iter)
        }
    }

    #[derive(Debug)]
    struct SimpleCursor {
        source: NodeId,
        target: NodeId,
        property: f64,
    }

    impl relationship_cursor::RelationshipCursor for SimpleCursor {
        fn source_id(&self) -> NodeId {
            self.source
        }

        fn target_id(&self) -> NodeId {
            self.target
        }

        fn property(&self) -> f64 {
            self.property
        }
    }

    // === Phase 2C: WeightedRelationshipCursor Implementation ===
    
    #[derive(Debug)]
    struct WeightedCursor {
        source: NodeId,
        target: NodeId,
        weight: f64,
    }

    impl relationship_cursor::WeightedRelationshipCursor for WeightedCursor {
        fn source_id(&self) -> NodeId {
            self.source
        }

        fn target_id(&self) -> NodeId {
            self.target
        }

        fn weight(&self) -> f64 {
            self.weight
        }
    }

    #[test]
    fn streams_yield_relationships() {
        let iter = TestIterator {
            edges: vec![(1, 2, 1.0), (1, 3, 2.0)],
        };
        let collected: Vec<(NodeId, NodeId, f64)> = iter
            .stream_relationships(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.property()))
            .collect();
        assert_eq!(collected, vec![(1, 2, 1.0), (1, 3, 2.0)]);
    }

    #[test]
    fn inverse_streams_filter_by_target() {
        let iter = TestIterator {
            edges: vec![(0, 1, 2.0), (2, 1, 4.0), (2, 3, 6.0)],
        };
        let collected: Vec<(NodeId, NodeId, f64)> = iter
            .stream_inverse_relationships(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.property()))
            .collect();
        assert_eq!(collected, vec![(0, 1, 2.0), (2, 1, 4.0)]);
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
        let collected: Vec<(NodeId, NodeId, f64)> = clone
            .stream_relationships(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.property()))
            .collect();
        assert_eq!(collected, vec![(1, 2, 3.0)]);
    }

    // === Phase 2C: Weighted Stream Tests ===
    
    #[test]
    fn weighted_streams_yield_f64_weights() {
        let iter = TestIterator {
            edges: vec![(1, 2, 1.0), (1, 3, 2.0)],
        };
        let collected: Vec<(NodeId, NodeId, f64)> = iter
            .stream_relationships_weighted(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.weight()))
            .collect();
        assert_eq!(collected, vec![(1, 2, 1.0), (1, 3, 2.0)]);
    }

    #[test]
    fn weighted_inverse_streams_filter_by_target() {
        let iter = TestIterator {
            edges: vec![(0, 1, 2.0), (2, 1, 4.0), (2, 3, 6.0)],
        };
        let collected: Vec<(NodeId, NodeId, f64)> = iter
            .stream_inverse_relationships_weighted(1, 0.0)
            .map(|cursor| (cursor.source_id(), cursor.target_id(), cursor.weight()))
            .collect();
        assert_eq!(collected, vec![(0, 1, 2.0), (2, 1, 4.0)]);
    }

    #[test]
    fn weighted_streams_provide_direct_f64_access() {
        let iter = TestIterator {
            edges: vec![(1, 2, 42.0)],
        };
        let cursor = iter.stream_relationships_weighted(1, 0.0).next().unwrap();
        
        // Direct f64 access - no conversion overhead
        let weight = cursor.weight();
        assert_eq!(weight, 42.0);
        
        // Can be used directly in algorithms without casting
        let distance = weight + 10.0;
        assert_eq!(distance, 52.0);
    }
}

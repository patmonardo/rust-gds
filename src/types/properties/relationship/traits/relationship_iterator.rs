use super::relationship_consumer::RelationshipConsumer;
use super::relationship_cursor::RelationshipCursorBox;
use super::relationship_predicate::RelationshipPredicate;
use super::relationship_with_property_consumer::RelationshipWithPropertyConsumer;
use crate::types::graph::id_map::MappedNodeId;
use crate::types::properties::relationship::PropertyValue;

/// Iterator abstraction for traversing relationships with optional property values.
pub trait RelationshipIterator: RelationshipPredicate + Send + Sync {
    /// Iterate over the relationships of `node_id` without exposing property values.
    fn for_each_relationship(&self, node_id: MappedNodeId, consumer: &mut dyn RelationshipConsumer);

    /// Iterate over the relationships of `node_id`, providing property values or
    /// `fallback_value` when none exist.
    fn for_each_relationship_with_properties(
        &self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
        consumer: &mut dyn RelationshipWithPropertyConsumer,
    );

    /// Iterate over the inverse relationships (incoming edges) of `node_id`.
    fn for_each_inverse_relationship(
        &self,
        node_id: MappedNodeId,
        consumer: &mut dyn RelationshipConsumer,
    );

    /// Iterate over inverse relationships providing property values.
    fn for_each_inverse_relationship_with_properties(
        &self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
        consumer: &mut dyn RelationshipWithPropertyConsumer,
    );

    /// Stream relationships starting from `node_id` as an iterator of cursors.
    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a>;

    /// Create a concurrent-safe copy of this iterator.
    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator>;
}

/// Boxed iterator over relationship cursors.
pub type RelationshipStream<'a> = Box<dyn Iterator<Item = RelationshipCursorBox> + Send + 'a>;

/// Convenience helpers to bridge between the property-aware and property-less flows.
pub trait RelationshipIteratorExt: RelationshipIterator {
    fn for_each_relationship_with_fallback(
        &self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
        consumer: &mut dyn RelationshipConsumer,
    ) {
        let mut adapter = ConsumerAdapter { inner: consumer };
        self.for_each_relationship_with_properties(node_id, fallback_value, &mut adapter);
    }

    fn for_each_inverse_relationship_with_fallback(
        &self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
        consumer: &mut dyn RelationshipConsumer,
    ) {
        let mut adapter = ConsumerAdapter { inner: consumer };
        self.for_each_inverse_relationship_with_properties(node_id, fallback_value, &mut adapter);
    }
}

impl<T> RelationshipIteratorExt for T where T: RelationshipIterator + ?Sized {}

struct ConsumerAdapter<'a> {
    inner: &'a mut dyn RelationshipConsumer,
}

impl RelationshipWithPropertyConsumer for ConsumerAdapter<'_> {
    fn accept(
        &mut self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        _property: PropertyValue,
    ) -> bool {
        self.inner.accept(source_id, target_id)
    }
}

#[cfg(test)]
mod tests {
    use super::super::relationship_cursor;
    use super::*;
    use crate::types::properties::relationship::traits::relationship_consumer::for_each_relationship;
    use crate::types::properties::relationship::traits::RelationshipPredicate;
    use std::cell::RefCell;
    use std::rc::Rc;

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
        fn for_each_relationship(
            &self,
            node_id: MappedNodeId,
            consumer: &mut dyn RelationshipConsumer,
        ) {
            for (s, t, _) in &self.edges {
                if *s == node_id {
                    if !consumer.accept(*s, *t) {
                        break;
                    }
                }
            }
        }

        fn for_each_relationship_with_properties(
            &self,
            node_id: MappedNodeId,
            _fallback_value: PropertyValue,
            consumer: &mut dyn RelationshipWithPropertyConsumer,
        ) {
            for (s, t, p) in &self.edges {
                if *s == node_id {
                    let property = *p;
                    if !consumer.accept(*s, *t, property) {
                        break;
                    }
                }
            }
        }

        fn for_each_inverse_relationship(
            &self,
            node_id: MappedNodeId,
            consumer: &mut dyn RelationshipConsumer,
        ) {
            for (s, t, _) in &self.edges {
                if *t == node_id {
                    if !consumer.accept(*s, *t) {
                        break;
                    }
                }
            }
        }

        fn for_each_inverse_relationship_with_properties(
            &self,
            node_id: MappedNodeId,
            _fallback_value: PropertyValue,
            consumer: &mut dyn RelationshipWithPropertyConsumer,
        ) {
            for (s, t, p) in &self.edges {
                if *t == node_id {
                    if !consumer.accept(*s, *t, *p) {
                        break;
                    }
                }
            }
        }

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

        fn concurrent_copy(&self) -> Box<dyn RelationshipIterator> {
            Box::new(TestIterator {
                edges: self.edges.clone(),
            })
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
    fn iterator_ext_adapts_property_flow() {
        let iter = TestIterator {
            edges: vec![(1, 2, 1.5)],
        };
        let values: Rc<RefCell<Vec<(MappedNodeId, MappedNodeId)>>> = Rc::new(RefCell::new(vec![]));
        let values_ref = Rc::clone(&values);
        let mut consumer = for_each_relationship(move |s, t| {
            values_ref.borrow_mut().push((s, t));
        });

        iter.for_each_relationship_with_fallback(1, 0.0, &mut consumer);

        assert_eq!(values.borrow().as_slice(), &[(1, 2)]);
    }
}

use crate::types::id_map::MappedNodeId;

/// Consumer interface for iterating relationships without attached properties.
pub trait RelationshipConsumer {
    /// Called for every edge that matches a given relation constraint.
    /// Return `true` to continue iteration or `false` to stop early.
    fn accept(&mut self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool;
}

impl<F> RelationshipConsumer for F
where
    F: FnMut(MappedNodeId, MappedNodeId) -> bool,
{
    fn accept(&mut self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
        self(source_id, target_id)
    }
}

/// Helper struct that executes two consumers in sequence.
pub struct AndThenRelationshipConsumer<First, Second> {
    first: First,
    second: Second,
}

impl<First, Second> AndThenRelationshipConsumer<First, Second> {
    pub fn new(first: First, second: Second) -> Self {
        Self { first, second }
    }
}

impl<First, Second> RelationshipConsumer for AndThenRelationshipConsumer<First, Second>
where
    First: RelationshipConsumer,
    Second: RelationshipConsumer,
{
    fn accept(&mut self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
        if !self.first.accept(source_id, target_id) {
            return false;
        }
        self.second.accept(source_id, target_id)
    }
}

/// Extension helpers mirroring the TypeScript namespace utilities.
pub trait RelationshipConsumerExt: RelationshipConsumer + Sized {
    /// Chain another consumer that is executed after `self`.
    fn and_then<After>(self, after: After) -> AndThenRelationshipConsumer<Self, After>
    where
        After: RelationshipConsumer,
    {
        AndThenRelationshipConsumer::new(self, after)
    }
}

impl<T> RelationshipConsumerExt for T where T: RelationshipConsumer + Sized {}

/// Creates a consumer that always continues iteration and invokes the provided callback.
pub fn for_each_relationship<F>(mut callback: F) -> impl RelationshipConsumer
where
    F: FnMut(MappedNodeId, MappedNodeId) + 'static,
{
    move |source_id: MappedNodeId, target_id: MappedNodeId| {
        callback(source_id, target_id);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn and_then_chains_consumers() {
        let values: Rc<RefCell<Vec<(MappedNodeId, MappedNodeId)>>> = Rc::new(RefCell::new(vec![]));
        let values_first = Rc::clone(&values);
        let values_second = Rc::clone(&values);

        let mut consumer = for_each_relationship(move |s, t| {
            values_first.borrow_mut().push((s, t));
        })
        .and_then(move |s: MappedNodeId, t: MappedNodeId| {
            values_second.borrow_mut().push((t, s));
            true
        });

        assert!(consumer.accept(1, 2));
        assert_eq!(values.borrow().as_slice(), &[(1, 2), (2, 1)]);
    }

    #[test]
    fn consumer_can_stop_iteration() {
        let count = Rc::new(RefCell::new(0));
        let count_ref = Rc::clone(&count);
        let mut consumer = move |_: MappedNodeId, _: MappedNodeId| {
            *count_ref.borrow_mut() += 1;
            false
        };

        assert!(!consumer.accept(1, 2));
        assert_eq!(*count.borrow(), 1);
    }
}

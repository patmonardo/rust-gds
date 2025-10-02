use crate::types::id_map::MappedNodeId;
use crate::types::properties::relationship::PropertyValue;

/// Consumer interface for relationships with property values attached.
pub trait RelationshipWithPropertyConsumer {
    fn accept(
        &mut self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        property: PropertyValue,
    ) -> bool;
}

impl<F> RelationshipWithPropertyConsumer for F
where
    F: FnMut(MappedNodeId, MappedNodeId, PropertyValue) -> bool,
{
    fn accept(
        &mut self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        property: PropertyValue,
    ) -> bool {
        self(source_id, target_id, property)
    }
}

pub struct AndThenRelationshipWithPropertyConsumer<First, Second> {
    first: First,
    second: Second,
}

impl<First, Second> AndThenRelationshipWithPropertyConsumer<First, Second> {
    pub fn new(first: First, second: Second) -> Self {
        Self { first, second }
    }
}

impl<First, Second> RelationshipWithPropertyConsumer
    for AndThenRelationshipWithPropertyConsumer<First, Second>
where
    First: RelationshipWithPropertyConsumer,
    Second: RelationshipWithPropertyConsumer,
{
    fn accept(
        &mut self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        property: PropertyValue,
    ) -> bool {
        if !self.first.accept(source_id, target_id, property) {
            return false;
        }
        self.second.accept(source_id, target_id, property)
    }
}

pub trait RelationshipWithPropertyConsumerExt: RelationshipWithPropertyConsumer + Sized {
    fn and_then<After>(self, after: After) -> AndThenRelationshipWithPropertyConsumer<Self, After>
    where
        After: RelationshipWithPropertyConsumer,
    {
        AndThenRelationshipWithPropertyConsumer::new(self, after)
    }
}

impl<T> RelationshipWithPropertyConsumerExt for T where T: RelationshipWithPropertyConsumer + Sized {}

pub fn for_each_relationship_with_property<F>(
    mut callback: F,
) -> impl RelationshipWithPropertyConsumer
where
    F: FnMut(MappedNodeId, MappedNodeId, PropertyValue) + 'static,
{
    move |source_id: MappedNodeId, target_id: MappedNodeId, property: PropertyValue| {
        callback(source_id, target_id, property);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn property_consumer_chain_executes_both() {
        let values: Rc<RefCell<Vec<(MappedNodeId, MappedNodeId, PropertyValue)>>> =
            Rc::new(RefCell::new(vec![]));
        let values_first = Rc::clone(&values);
        let values_second = Rc::clone(&values);

        let mut consumer = for_each_relationship_with_property(move |s, t, p| {
            values_first.borrow_mut().push((s, t, p));
        })
        .and_then(move |s, t, p| {
            values_second.borrow_mut().push((t, s, p));
            true
        });

        assert!(consumer.accept(1, 2, 3.5));
        assert_eq!(values.borrow().as_slice(), &[(1, 2, 3.5), (2, 1, 3.5)]);
    }

    #[test]
    fn property_consumer_can_stop_iteration() {
        let count = Rc::new(RefCell::new(0));
        let count_ref = Rc::clone(&count);
        let mut consumer = move |_: MappedNodeId, _: MappedNodeId, _: PropertyValue| {
            *count_ref.borrow_mut() += 1;
            false
        };

        assert!(!consumer.accept(1, 2, 0.0));
        assert_eq!(*count.borrow(), 1);
    }
}

use crate::types::graph::id_map::MappedNodeId;

/// Predicate for checking the existence of relationships between nodes.
pub trait RelationshipPredicate {
    fn exists(&self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool;
}

impl<F> RelationshipPredicate for F
where
    F: Fn(MappedNodeId, MappedNodeId) -> bool,
{
    fn exists(&self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
        self(source_id, target_id)
    }
}

pub fn all_relationships() -> impl RelationshipPredicate {
    |_source_id: MappedNodeId, _target_id: MappedNodeId| true
}

pub fn no_relationships() -> impl RelationshipPredicate {
    |_source_id: MappedNodeId, _target_id: MappedNodeId| false
}

pub fn not_relationships<P>(predicate: P) -> impl RelationshipPredicate
where
    P: RelationshipPredicate,
{
    move |source_id: MappedNodeId, target_id: MappedNodeId| !predicate.exists(source_id, target_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_predicate_accepts_everything() {
        let pred = all_relationships();
        assert!(pred.exists(1, 2));
    }

    #[test]
    fn none_predicate_rejects_everything() {
        let pred = no_relationships();
        assert!(!pred.exists(1, 2));
    }

    #[test]
    fn not_predicate_inverts() {
        let pred = not_relationships(no_relationships());
        assert!(pred.exists(1, 2));
    }
}

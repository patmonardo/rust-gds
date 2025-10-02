use std::collections::HashSet;

use crate::types::schema::NodeLabel;

use super::MappedNodeId;

/// Consumer trait mirroring the TypeScript callback signatures while remaining object safe.
pub trait NodeConsumer {
    fn accept(&mut self, node_id: MappedNodeId) -> bool;
}

impl<F> NodeConsumer for F
where
    F: FnMut(MappedNodeId) -> bool,
{
    fn accept(&mut self, node_id: MappedNodeId) -> bool {
        self(node_id)
    }
}

/// Iterator utilities for traversing mapped node identifiers.
pub trait NodeIterator: Send + Sync {
    /// Applies the consumer to each mapped node identifier until either all
    /// nodes are visited or the consumer returns `false`.
    fn for_each_node(&self, consumer: &mut dyn NodeConsumer);

    /// Returns an iterator over all mapped node identifiers.
    fn iter(&self) -> NodeIdIterator<'_>;

    /// Returns an iterator over mapped nodes that carry any of the provided labels.
    fn iter_with_labels<'a>(&'a self, labels: &'a HashSet<NodeLabel>) -> NodeIdIterator<'a>;
}

/// Type-erased iterator over mapped node identifiers.
pub type NodeIdIterator<'a> = Box<dyn Iterator<Item = MappedNodeId> + 'a>;

/// Extension helpers for [`NodeIterator`].
pub trait NodeIteratorExt: NodeIterator {
    /// Collects the iterator into a vector.
    fn to_vec(&self) -> Vec<MappedNodeId> {
        self.iter().collect()
    }

    /// Collects the label-filtered iterator into a vector.
    fn to_vec_with_labels(&self, labels: &HashSet<NodeLabel>) -> Vec<MappedNodeId> {
        self.iter_with_labels(labels).collect()
    }
}

impl<T> NodeIteratorExt for T where T: NodeIterator + ?Sized {}

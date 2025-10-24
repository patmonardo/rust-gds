use crate::types::graph::id_map::NodeId;

/// Interface for retrieving degree information about nodes in a graph.
pub trait Degrees {
    /// Returns the number of outgoing (or undirected) relationships for the node.
    fn degree(&self, node_id: NodeId) -> usize;

    /// Returns the number of incoming relationships for the node.
    /// Implementations may return `None` when inverse indexing is unsupported.
    fn degree_inverse(&self, node_id: NodeId) -> Option<usize>;

    /// Returns the number of relationships for `node_id` without counting parallel edges.
    fn degree_without_parallel_relationships(&self, node_id: NodeId) -> usize;
}

/// Blanket implementation for references.
impl<T: Degrees + ?Sized> Degrees for &T {
    fn degree(&self, node_id: NodeId) -> usize {
        (**self).degree(node_id)
    }

    fn degree_inverse(&self, node_id: NodeId) -> Option<usize> {
        (**self).degree_inverse(node_id)
    }

    fn degree_without_parallel_relationships(&self, node_id: NodeId) -> usize {
        (**self).degree_without_parallel_relationships(node_id)
    }
}

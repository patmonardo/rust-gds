use crate::types::graph::id_map::MappedNodeId;

/// In-memory adjacency representation used by the default graph implementation.
#[derive(Debug, Clone)]
pub struct RelationshipTopology {
    outgoing: Vec<Vec<MappedNodeId>>,
    incoming: Option<Vec<Vec<MappedNodeId>>>,
    relationship_count: usize,
    has_parallel_edges: bool,
}

impl RelationshipTopology {
    /// Creates a new topology from outgoing adjacency lists.
    pub fn new(outgoing: Vec<Vec<MappedNodeId>>, incoming: Option<Vec<Vec<MappedNodeId>>>) -> Self {
        let relationship_count = outgoing.iter().map(|adj| adj.len()).sum();
        let has_parallel_edges = outgoing.iter().any(|adj| {
            let mut sorted = adj.clone();
            sorted.sort_unstable();
            sorted.windows(2).any(|window| window[0] == window[1])
        });

        Self {
            outgoing,
            incoming,
            relationship_count,
            has_parallel_edges,
        }
    }

    /// Returns the number of relationships encoded in this topology.
    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    /// Returns true when this topology may contain parallel edges.
    pub fn has_parallel_edges(&self) -> bool {
        self.has_parallel_edges
    }

    /// Returns the outgoing adjacency for the given node, if available.
    pub fn outgoing(&self, node: MappedNodeId) -> Option<&[MappedNodeId]> {
        self.outgoing
            .get(node as usize)
            .map(|neighbors| neighbors.as_slice())
    }

    /// Returns the incoming adjacency for the given node when an inverse index exists.
    pub fn incoming(&self, node: MappedNodeId) -> Option<&[MappedNodeId]> {
        self.incoming
            .as_ref()
            .and_then(|lists| lists.get(node as usize))
            .map(|neighbors| neighbors.as_slice())
    }

    /// Returns true when an inverse index is available.
    pub fn is_inverse_indexed(&self) -> bool {
        self.incoming.is_some()
    }

    /// Returns the total number of nodes tracked by this topology.
    pub fn node_capacity(&self) -> usize {
        self.outgoing.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_parallel_edges() {
        let topology = RelationshipTopology::new(vec![vec![1, 1], vec![]], None);
        assert!(topology.has_parallel_edges());
    }

    #[test]
    fn counts_relationships() {
        let topology = RelationshipTopology::new(vec![vec![1, 2], vec![0]], None);
        assert_eq!(topology.relationship_count(), 3);
    }
}

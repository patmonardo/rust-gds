use super::AdjacencyCursor;
use crate::types::graph::MappedNodeId;

/// Read-only access to the adjacency list of a mono-partite graph.
///
/// The trait is intentionally low-level and mirrors the semantics of the upstream GDS API,
/// while allowing implementors to provide custom cursor pooling or reuse strategies.
pub trait AdjacencyList: Send + Sync + std::fmt::Debug {
    /// Returns the degree (number of outgoing relationships) of the given node.
    fn degree(&self, node: MappedNodeId) -> usize;

    /// Acquire a reusable cursor object. Implementations may recycle internal instances
    /// or allocate a fresh cursor for each call.
    fn raw_adjacency_cursor(&self) -> Box<dyn AdjacencyCursor>;

    /// Initialise the provided `cursor` for iterating over the adjacency list of `node`.
    fn init_cursor(&self, cursor: &mut dyn AdjacencyCursor, node: MappedNodeId);

    /// Optional hint describing the expected in-memory and off-heap footprint (bytes).
    /// Implementations dealing with disk backed or compressed representations may return
    /// `None` when a cheap estimate is unavailable.
    fn memory_usage_bytes(&self) -> Option<usize> {
        None
    }
}

/// Extension helpers building on top of [`AdjacencyList`].
pub trait AdjacencyListExt: AdjacencyList {
    /// Convenience method creating a fully initialised cursor for the given node.
    fn adjacency_cursor(&self, node: MappedNodeId) -> Box<dyn AdjacencyCursor> {
        let mut cursor = self.raw_adjacency_cursor();
        self.init_cursor(cursor.as_mut(), node);
        cursor
    }

    /// Attempt to reuse the provided cursor. When `None`, falls back to [`AdjacencyListExt::adjacency_cursor`].
    fn adjacency_cursor_with_reuse(
        &self,
        reuse: Option<Box<dyn AdjacencyCursor>>,
        node: MappedNodeId,
    ) -> Box<dyn AdjacencyCursor> {
        if let Some(mut cursor) = reuse {
            self.init_cursor(cursor.as_mut(), node);
            cursor
        } else {
            self.adjacency_cursor(node)
        }
    }
}

impl<T> AdjacencyListExt for T where T: AdjacencyList + ?Sized {}

use crate::types::graph::MappedNodeId;

/// Special target identifier returned when a cursor cannot produce a valid value.
pub const NOT_FOUND_TARGET: MappedNodeId = MappedNodeId::MAX;

/// Low-level cursor iterating over the target ids of a single adjacency list.
///
/// The contract mirrors the semantics of the TypeScript/Java GDS `AdjacencyCursor` while
/// embracing idiomatic Rust ergonomics. Implementations are free to expose more ergonomic
/// iterator adapters via the provided extension trait.
pub trait AdjacencyCursor: Send + Sync + std::fmt::Debug {
    /// (Re-)initialise the cursor so that it produces the targets for the adjacency list
    /// identified by `index`. The `degree` is provided as a hint and should describe the
    /// number of targets that can be decoded.
    fn init(&mut self, index: usize, degree: usize);

    /// Total number of targets this cursor will yield for the current adjacency list.
    fn size(&self) -> usize;

    /// Returns the number of targets that have not been produced yet.
    fn remaining(&self) -> usize;

    /// Returns `true` when at least one more target id can be decoded.
    fn has_next(&self) -> bool {
        self.remaining() > 0
    }

    /// Decode the next target node identifier. Returns `None` when the cursor is exhausted.
    fn next_vlong(&mut self) -> Option<MappedNodeId>;

    /// Peek at the next target node identifier without advancing the cursor.
    fn peek_vlong(&self) -> Option<MappedNodeId>;

    /// Consume target ids until a value strictly larger than `node_id` is encountered.
    /// Returns the matching target or `None` if the cursor is exhausted before such value appears.
    fn skip_until(&mut self, node_id: MappedNodeId) -> Option<MappedNodeId>;

    /// Consume target ids until a value larger than or equal to `node_id` is encountered.
    /// Returns the matching target or `None` if the cursor is exhausted.
    fn advance(&mut self, node_id: MappedNodeId) -> Option<MappedNodeId>;

    /// Advance the cursor by `n` elements. Returns the target id observed at the new position,
    /// or `None` if advancing moves beyond the end of the underlying adjacency list.
    fn advance_by(&mut self, n: usize) -> Option<MappedNodeId>;
}

/// Convenience helpers for working with [`AdjacencyCursor`] values.
pub trait AdjacencyCursorExt: AdjacencyCursor {
    /// Drain the cursor, collecting all remaining target node identifiers into a new vector.
    fn collect_remaining(&mut self) -> Vec<MappedNodeId> {
        let mut out = Vec::with_capacity(self.remaining());
        while let Some(target) = self.next_vlong() {
            out.push(target);
        }
        out
    }

    /// Return the next target id or [`NOT_FOUND_TARGET`] when the cursor is exhausted.
    fn next_or_not_found(&mut self) -> MappedNodeId {
        self.next_vlong().unwrap_or(NOT_FOUND_TARGET)
    }
}

impl<T> AdjacencyCursorExt for T where T: AdjacencyCursor + ?Sized {}

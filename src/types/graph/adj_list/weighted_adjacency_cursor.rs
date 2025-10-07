use super::{AdjacencyCursor, NOT_FOUND_TARGET};
use crate::types::graph::MappedNodeId;

/// Type alias describing the scalar value associated with a weighted relationship.
pub type EdgeWeight = f64;

/// Cursor iterating over weighted relationships.
///
/// The cursor returns `(target_id, weight)` pairs while preserving the low-level controls
/// exposed by [`AdjacencyCursor`]. Implementors are expected to keep `next_vlong` and
/// `next_weighted` in sync: calling `next_weighted` should advance the cursor exactly once.
pub trait WeightedAdjacencyCursor: AdjacencyCursor {
    /// Produce the next weighted edge, returning `None` when the cursor is exhausted.
    fn next_weighted(&mut self) -> Option<(MappedNodeId, EdgeWeight)>;

    /// Peek at the next weighted edge without advancing the cursor.
    fn peek_weighted(&self) -> Option<(MappedNodeId, EdgeWeight)>;

    /// Fallback weight returned when an implementation cannot provide an explicit value.
    fn fallback_weight(&self) -> EdgeWeight;

    /// Produce the next weighted edge or a tuple containing [`NOT_FOUND_TARGET`] and the
    /// fallback weight when the cursor is exhausted.
    fn next_weighted_or_fallback(&mut self) -> (MappedNodeId, EdgeWeight) {
        self.next_weighted()
            .unwrap_or((NOT_FOUND_TARGET, self.fallback_weight()))
    }
}

/// Helper providing high-level collection utilities for [`WeightedAdjacencyCursor`].
pub trait WeightedAdjacencyCursorExt: WeightedAdjacencyCursor {
    /// Collect all remaining weighted edges into a vector.
    fn collect_edges(&mut self) -> Vec<(MappedNodeId, EdgeWeight)> {
        let mut out = Vec::with_capacity(self.remaining());
        while let Some(edge) = self.next_weighted() {
            out.push(edge);
        }
        out
    }
}

impl<T> WeightedAdjacencyCursorExt for T where T: WeightedAdjacencyCursor + ?Sized {}

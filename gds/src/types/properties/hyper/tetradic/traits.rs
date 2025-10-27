use super::types::{Concepts, DyadId};

/// Public trait for Tetradic HyperStore implementations.
pub trait TetradicHyperStore: Send + Sync {
    /// Insert or replace a concepts pair identified by `id`.
    fn insert(&self, id: DyadId, concepts: Concepts);

    /// Retrieve the concepts for `id`.
    fn get(&self, id: &DyadId) -> Option<Concepts>;

    /// Optional: compact or optimize storage. Left as a no-op in the
    /// in-memory reference implementation.
    fn compact(&self) {}
}

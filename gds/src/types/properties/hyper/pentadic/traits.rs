use super::types::{AssertionId, Concepts};

/// Pentadic HyperStore trait. Mirrors the tetradic trait naming and `Concepts`
/// alias to keep the module surfaces identical.
pub trait PentadicHyperStore: Send + Sync {
    /// Insert a pentadic assertion mapping an `id` to `concepts`.
    fn insert(&self, id: AssertionId, concepts: Concepts);

    /// Retrieve the concepts for an assertion id.
    fn get(&self, id: &AssertionId) -> Option<Concepts>;
}

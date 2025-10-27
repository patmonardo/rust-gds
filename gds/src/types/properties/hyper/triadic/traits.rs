use super::types::{Concepts, TriadId};

/// Triadic HyperStore trait using the `Concepts` alias for consistent naming.
pub trait TriadicHyperStore: Send + Sync {
    /// Insert or replace the triad identified by `id`.
    fn insert(&self, id: TriadId, concepts: Concepts);

    /// Retrieve the triad for `id`.
    fn get(&self, id: &TriadId) -> Option<Concepts>;
}

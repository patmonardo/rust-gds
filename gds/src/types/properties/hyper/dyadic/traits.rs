use super::types::{Concepts, DyadId};

/// Dyadic HyperStore contract.
pub trait DyadicHyperStore: Send + Sync {
    fn insert(&self, id: DyadId, concepts: Concepts);
    fn get(&self, id: &DyadId) -> Option<Concepts>;
}

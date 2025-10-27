pub use super::in_memory::InMemoryTetradicStore;
pub use super::traits::TetradicHyperStore;
pub use super::types::{ConceptPayload, Concepts, DyadId};

// Backwards-compatible convenience alias used by examples
pub use super::types::TetradicProperty;
pub type MLPipelinePropertyStore = InMemoryTetradicStore;

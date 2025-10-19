//! Computation Descriptor registry — The Five-Fold Synthesis in Action
//!
//! ComputationDescriptor exemplifies the Five-Fold Synthesis of Projection:
//!
//! 1. TRANSFORM (Ground): ComputationDescriptor ≡ Computer (via Transform<D, R>)
//! 2. DESCRIPTOR (Identity): ComputationDescriptor { id, name, species, pattern }
//! 3. MEMBERSHIP (Inherence): ComputationMembership { constraints linking Property & Storage }
//! 4. RUNTIME (Difference): Computer trait { init/step/finalize }
//! 5. CONSEQUENCE (Entailment): ConsequenceRule determines runtime strategy from membership
//!
//! Once we POSSESS this Five-Fold Concept, it can be APPLIED in two ways:
//! - eval: Analyze the descriptor → extract knowledge of its constraints
//! - factory: Generate the runtime → manifest omnipotence
//!
//! This module shows: Descriptor IS Runtime through Transform.
//! No factories hidden in trait objects, no reflection, no indirection.
//! Pure concept → pure manifestation.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputationSpecies {
    Bsp,
    MapReduce,
    Dataflow,
    Actor,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputationPattern {
    VertexCentric,
    EdgeCentric,
    Global,
    Custom(String),
}

/// Minimal descriptor describing a computation species and pattern.
/// Keep small so macro-generated registration is trivial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationDescriptor {
    /// numeric id for runtime registry (macro should pick stable ids)
    pub id: u32,
    /// human name
    pub name: String,
    /// species (BSP, MapReduce, ...)
    pub species: ComputationSpecies,
    /// pattern (VertexCentric, etc.)
    pub pattern: ComputationPattern,
    /// optional short description
    pub description: Option<String>,
}

impl ComputationDescriptor {
    pub fn new(
        id: u32,
        name: impl Into<String>,
        species: ComputationSpecies,
        pattern: ComputationPattern,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            species,
            pattern,
            description: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

lazy_static::lazy_static! {
    static ref COMPUTATION_REGISTRY: std::sync::RwLock<HashMap<u32, ComputationDescriptor>> =
        std::sync::RwLock::new(HashMap::new());
}

/// Register a computation descriptor at runtime. Returns true if newly inserted.
pub fn register_computation_descriptor(desc: ComputationDescriptor) -> bool {
    use std::collections::hash_map::Entry;
    let mut reg = COMPUTATION_REGISTRY.write().unwrap();
    match reg.entry(desc.id) {
        Entry::Vacant(e) => {
            e.insert(desc);
            true
        }
        Entry::Occupied(_) => false,
    }
}

/// Get a computation descriptor by id.
pub fn get_computation_descriptor(id: u32) -> Option<ComputationDescriptor> {
    let reg = COMPUTATION_REGISTRY.read().unwrap();
    reg.get(&id).cloned()
}

/// Clear the registry (test helper)
#[cfg(test)]
pub fn clear_computation_registry() {
    let mut reg = COMPUTATION_REGISTRY.write().unwrap();
    reg.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_lookup() {
        clear_computation_registry();
        let desc = ComputationDescriptor::new(
            1,
            "pregel_bsp",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );
        assert!(register_computation_descriptor(desc.clone()));
        assert!(!register_computation_descriptor(desc.clone())); // already present
        let got = get_computation_descriptor(1).expect("found");
        assert_eq!(got.name, "pregel_bsp");
        assert_eq!(got.species, ComputationSpecies::Bsp);
    }
}

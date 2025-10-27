//! FactStore: A Priori Essence (Essential Being)
//! Part 2a of Objective Logic
//!
//! FactStore represents Essential Being - Ground-Condition-Facticity.
//! Property Science = Axiological Ontology
//! Being sublated into its Essence as Essential Being
//! Facticity "coming into Existence" as Property
//!
//! NOT GraphStore - this is the ground of Property Science.

use crate::collections::hyper_store::HyperStore;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Projection levels for HyperPropertyStores
/// Following the sacred mathematical progression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionLevel {
    Monadic = 1,  // Simple Unity
    Dyadic = 2,   // Reflective Mark (no middle)
    Triadic = 3,  // Determinate Mark (with middle)
    Tetradic = 4, // ML Pipeline (4-4-4-4 pattern)
    Pentadic = 5, // Absolute Idea
}

/// Reflection: Relational Essence (Dyad without Center Mark)
///
/// Pure relational structure without a center mark.
/// This is the Dyadic pattern - two elements in relation
/// but without a mediating third element.
///
/// Uses Arc for shared ownership since S doesn't implement Copy.
#[derive(Clone, Debug)]
pub struct Reflection<S> {
    pub left: std::sync::Arc<S>,
    pub right: std::sync::Arc<S>,
}

/// CenterMark: The mark that yields the Concept
///
/// The middle element that determines the relation.
/// Only present in Triadic and higher structures.
#[derive(Debug)]
pub struct CenterMark {
    pub position: usize,
    pub value: Box<dyn Any>,
}

/// Sentinel: Mark of an End
///
/// Indicates completion of a projective process.
/// Required for the Path of Becoming to reach its goal.
#[derive(Clone, Debug)]
pub struct Sentinel {
    pub end_position: usize,
    pub completion_mark: bool,
}

/// FactStore: A Priori Essence (Essential Being)
///
/// This trait represents Essential Being as Ground-Condition-Facticity.
/// Being has been sublated into its Essence as Essential Being.
/// Facticity has "come into Existence" as Property.
///
/// This is NOT GraphStore - it's the foundational ground of Property Science.
pub trait FactStore {
    type Store: HyperStore<i64>; // Pure Being foundation
    type Aspect; // Essential Relation (Pure Essence)

    /// Get the projection dimension of this FactStore
    fn dimension(&self) -> ProjectionLevel;

    /// Get the underlying HyperStore (Pure Being)
    fn store(&self) -> &Self::Store;

    /// Get the aspects (Essential Relations) of this store
    fn aspects(&self) -> Vec<&Self::Aspect>;

    /// Reflection: Relational Essence (Dyad without Center Mark)
    /// Only valid for Dyadic structures
    fn reflect(&self) -> Option<Reflection<Self::Store>>
    where
        Self::Store: Clone,
    {
        None // Default: no reflection
    }

    /// Mark of Center: Yields the Concept
    /// Only valid for Triadic and higher structures
    fn mark_center(&self) -> Option<CenterMark> {
        None // Default: no center mark
    }

    /// Sentinel: Mark of an End
    /// Indicates completion of projective process
    fn sentinel(&self) -> Option<Sentinel> {
        None // Default: no sentinel
    }
}

/// MonadicFactStore: Simple Unity
///
/// The simplest FactStore - a single element.
/// This is the foundation of all higher structures.
pub struct MonadicFactStore<S: HyperStore<i64>> {
    pub store: S,
    pub dimension: ProjectionLevel,
}

impl<S: HyperStore<i64>> MonadicFactStore<S> {
    pub fn new(store: S) -> Self {
        Self {
            store,
            dimension: ProjectionLevel::Monadic,
        }
    }
}

impl<S: HyperStore<i64>> FactStore for MonadicFactStore<S> {
    type Store = S;
    type Aspect = ();

    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Monadic
    }

    fn store(&self) -> &Self::Store {
        &self.store
    }

    fn aspects(&self) -> Vec<&Self::Aspect> {
        vec![] // Monadic has no aspects
    }
}

/// DyadicFactStore: Reflective Mark (no middle)
///
/// Two FactStores in relation without a center mark.
/// This is pure relational essence - Reflection.
pub struct DyadicFactStore<S: HyperStore<i64>> {
    pub left: MonadicFactStore<S>,
    pub right: MonadicFactStore<S>,
}

impl<S: HyperStore<i64>> DyadicFactStore<S> {
    pub fn new(left: MonadicFactStore<S>, right: MonadicFactStore<S>) -> Self {
        Self { left, right }
    }
}

impl<S: HyperStore<i64>> FactStore for DyadicFactStore<S> {
    type Store = S;
    type Aspect = ();

    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Dyadic
    }

    fn store(&self) -> &Self::Store {
        self.left.store() // Use left as primary store
    }

    fn aspects(&self) -> Vec<&Self::Aspect> {
        vec![]
    }

    /// Dyadic provides Reflection
    fn reflect(&self) -> Option<Reflection<Self::Store>>
    where
        Self::Store: Clone,
    {
        Some(Reflection {
            left: std::sync::Arc::new(self.left.store.clone()),
            right: std::sync::Arc::new(self.right.store.clone()),
        })
    }
}

/// TriadicFactStore: Determinate Mark (with middle)
///
/// Three FactStores with a center mark.
/// The middle (essence) determines the relation.
/// This yields the Concept.
pub struct TriadicFactStore<S: HyperStore<i64>> {
    pub being: MonadicFactStore<S>,
    pub essence: MonadicFactStore<S>, // The middle!
    pub concept: MonadicFactStore<S>,
    pub center_mark: CenterMark,
}

impl<S: HyperStore<i64>> TriadicFactStore<S> {
    pub fn new(
        being: MonadicFactStore<S>,
        essence: MonadicFactStore<S>,
        concept: MonadicFactStore<S>,
        center_mark: CenterMark,
    ) -> Self {
        Self {
            being,
            essence,
            concept,
            center_mark,
        }
    }
}

impl<S: HyperStore<i64>> FactStore for TriadicFactStore<S> {
    type Store = S;
    type Aspect = ();

    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Triadic
    }

    fn store(&self) -> &Self::Store {
        &self.essence.store // Use essence as the primary store
    }

    fn aspects(&self) -> Vec<&Self::Aspect> {
        vec![]
    }

    /// Triadic provides Mark of Center
    fn mark_center(&self) -> Option<CenterMark> {
        // Clone the value properly
        // Note: Box<dyn Any> doesn't implement Clone, so we'll use the reference
        Some(CenterMark {
            position: 1,         // Middle position (essence)
            value: Box::new(()), // Placeholder - should properly clone self.center_mark.value
        })
    }
}

// ---------------------------------------------------------------------------
// Triadic-aware runtime model: Appearance -> Fact -> Assertion
// ---------------------------------------------------------------------------

/// A recorded mark / raw evidence (Appearance)
#[derive(Clone, Debug)]
pub struct Appearance {
    pub id: u64,
    pub ground_hint: Option<u64>,
    pub raw_blob: Vec<u8>,
    pub recorded_at_ms: u128,
    pub recorded_by: Option<String>,
}

/// A minimal dyadic fact (Ground : Condition)
#[derive(Clone, Debug)]
pub struct Fact {
    pub id: u64,
    pub ground: u64,
    pub predicate: String,
    pub value: String,
    pub origin_appearance: Option<u64>,
    pub created_at_ms: u128,
}

/// A triadic assertion: a fact inhering in an interpretive act
#[derive(Clone, Debug)]
pub struct Assertion {
    pub id: u64,
    pub fact_id: u64,
    pub issuer: String,
    pub context: Option<String>,
    pub confidence: Option<f64>,
    pub tags: Vec<String>,
    pub provenance_blob: Option<Vec<u8>>,
    pub valid_from_ms: Option<u128>,
    pub valid_to_ms: Option<u128>,
    pub created_at_ms: u128,
}

/// A tiny in-memory store that demonstrates the dyad -> triad lifecycle.
/// This is intentionally small and illustrative; production stores should
/// use the HyperStore adapters mentioned above.
pub struct HyperFactStore {
    appearances: Mutex<HashMap<u64, Appearance>>,
    facts: Mutex<HashMap<u64, Fact>>,
    assertions: Mutex<HashMap<u64, Assertion>>,
    next_id: Mutex<u64>,
}

impl Default for HyperFactStore {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperFactStore {
    pub fn new() -> Self {
        Self {
            appearances: Mutex::new(HashMap::new()),
            facts: Mutex::new(HashMap::new()),
            assertions: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }

    fn allocate_id(&self) -> u64 {
        let mut guard = self.next_id.lock().unwrap();
        let id = *guard;
        *guard += 1;
        id
    }

    fn now_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    /// Insert an appearance (raw mark). Returns its id.
    pub fn insert_appearance(
        &self,
        ground_hint: Option<u64>,
        raw_blob: Vec<u8>,
        recorded_by: Option<String>,
    ) -> u64 {
        let id = self.allocate_id();
        let app = Appearance {
            id,
            ground_hint,
            raw_blob,
            recorded_at_ms: Self::now_ms(),
            recorded_by,
        };
        self.appearances.lock().unwrap().insert(id, app);
        id
    }

    /// Synthesize a dyadic Fact from an appearance. Very small heuristic:
    /// if ground_hint present use it, otherwise use appearance id as ground.
    pub fn synthesize_fact_from_appearance(
        &self,
        appearance_id: u64,
        predicate: &str,
        value: &str,
    ) -> Option<u64> {
        let apps = self.appearances.lock().unwrap();
        let app = apps.get(&appearance_id)?;
        let id = self.allocate_id();
        let fact = Fact {
            id,
            ground: app.ground_hint.unwrap_or(app.id),
            predicate: predicate.to_string(),
            value: value.to_string(),
            origin_appearance: Some(appearance_id),
            created_at_ms: Self::now_ms(),
        };
        drop(apps);
        self.facts.lock().unwrap().insert(id, fact);
        Some(id)
    }

    /// Create an assertion for a fact (triadic act). Returns assertion id.
    pub fn assert_fact(
        &self,
        fact_id: u64,
        issuer: &str,
        context: Option<&str>,
        confidence: Option<f64>,
        tags: Vec<String>,
    ) -> Option<u64> {
        let facts = self.facts.lock().unwrap();
        if !facts.contains_key(&fact_id) {
            return None;
        }
        drop(facts);
        let id = self.allocate_id();
        let now = Self::now_ms();
        let assertion = Assertion {
            id,
            fact_id,
            issuer: issuer.to_string(),
            context: context.map(|s| s.to_string()),
            confidence,
            tags,
            provenance_blob: None,
            valid_from_ms: Some(now),
            valid_to_ms: None,
            created_at_ms: now,
        };
        self.assertions.lock().unwrap().insert(id, assertion);
        Some(id)
    }

    /// Convenience: interpret an appearance (synthesize fact + assert it).
    pub fn interpret_appearance(
        &self,
        appearance_id: u64,
        predicate: &str,
        value: &str,
        issuer: &str,
        context: Option<&str>,
        confidence: Option<f64>,
        tags: Vec<String>,
    ) -> Option<(u64, u64)> {
        let fact_id = self.synthesize_fact_from_appearance(appearance_id, predicate, value)?;
        let assertion_id = self.assert_fact(fact_id, issuer, context, confidence, tags)?;
        Some((fact_id, assertion_id))
    }

    // Query helpers for tests / convenience
    pub fn get_fact(&self, id: u64) -> Option<Fact> {
        self.facts.lock().unwrap().get(&id).cloned()
    }

    pub fn get_assertion(&self, id: u64) -> Option<Assertion> {
        self.assertions.lock().unwrap().get(&id).cloned()
    }

    pub fn get_appearance(&self, id: u64) -> Option<Appearance> {
        self.appearances.lock().unwrap().get(&id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::hyper_store::VecHyperStore;

    #[test]
    fn test_monadic_fact_store() {
        let store = VecHyperStore::new();
        let fact_store = MonadicFactStore::new(store);
        assert_eq!(fact_store.dimension(), ProjectionLevel::Monadic);
    }

    #[test]
    fn test_dyadic_fact_store() {
        let left_store = VecHyperStore::new();
        let right_store = VecHyperStore::new();

        let left = MonadicFactStore::new(left_store);
        let right = MonadicFactStore::new(right_store);

        let dyadic = DyadicFactStore::new(left, right);
        assert_eq!(dyadic.dimension(), ProjectionLevel::Dyadic);

        // Dyadic should provide reflection
        assert!(dyadic.reflect().is_some());
    }

    #[test]
    fn test_triadic_fact_store() {
        let being_store = VecHyperStore::new();
        let essence_store = VecHyperStore::new();
        let concept_store = VecHyperStore::new();

        let being = MonadicFactStore::new(being_store);
        let essence = MonadicFactStore::new(essence_store);
        let concept = MonadicFactStore::new(concept_store);

        let center_mark = CenterMark {
            position: 1,
            value: Box::new(42),
        };

        let triadic = TriadicFactStore::new(being, essence, concept, center_mark);
        assert_eq!(triadic.dimension(), ProjectionLevel::Triadic);

        // Triadic should provide center mark
        assert!(triadic.mark_center().is_some());
    }
}

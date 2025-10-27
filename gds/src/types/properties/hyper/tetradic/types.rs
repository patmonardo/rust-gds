//! Tetradic types (shared aliases)

/// Identifier for a tetradic dyad entry.
pub type DyadId = String;

/// A generic payload representing a concept (opaque bytes). Backends may
/// replace this with richer typed encodings.
pub type ConceptPayload = Vec<u8>;

/// The canonical `Concepts` alias used across HyperStore submodules. For the
/// Tetradic layer a `Concepts` value represents the pair (left, right) dyads
/// — e.g. (features, labels).
pub type Concepts = (ConceptPayload, ConceptPayload);

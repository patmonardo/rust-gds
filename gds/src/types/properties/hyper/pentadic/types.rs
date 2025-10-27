//! Pentadic types (shared aliases)

/// Identifier for a pentadic assertion.
pub type AssertionId = String;

/// Reference to a triadic entity (opaque id for now).
pub type TriadRef = String;

/// A simple certainty metric.
pub type Certainty = f32;

/// Canonical `Concepts` alias for the Pentadic layer. Here it represents
/// (primary_triad_ref, secondary_triad_ref, certainty).
pub type Concepts = (TriadRef, TriadRef, Certainty);

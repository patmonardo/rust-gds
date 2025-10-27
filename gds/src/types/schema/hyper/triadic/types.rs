//! Triadic schema types: The Schema of Conceiving (Concepts)
//!
//! The third schema is the schema of Concepts - where the system conceives.
//! Triadic represents the movement: "The system conceives X AS Y based on Z"
//!
//! THE AS: The moment of Objectivity
//! - AS brings in Objectivity (the active mediation)
//! - IS brings in Subjectivity (X is, Y is)
//! - "Conceives X AS Y" involves the objective shaping of the relationship
//!
//! Structure:
//! - being: What the system conceives (X)
//! - essence: How the system conceives it (the mediating mark - AS)
//! - concept: The resulting concept (Y)
//!
//! This introduces the moment of Objectivity in the logical progression.
//!
//! # Self-Certainty
//! 
//! This is the schema of Self-Certainty (the third moment of self-certification).
//! The AS mediates between Self-Evidence (IS) and Self-Assertion (the claim).
//! The mediating mark (essence) introduces Self-Certainty through Objectivity.

use crate::types::schema::hyper::monadic::MonadicSchema;
use crate::types::schema::HyperSchemaLevel;

/// TriadicSchemaId: identifier for triadic schema entries
pub type TriadicSchemaId = String;

/// CenterMark: The mark that yields the Concept
/// The middle element that determines the relation.
/// Only present in Triadic and higher structures.
#[derive(Debug, Clone)]
pub struct CenterMark {
    pub position: usize,
    pub value_type: crate::types::ValueType,
}

/// TriadicSchema: schema for graph structure with center mark
#[derive(Clone, Debug)]
pub struct TriadicSchema {
    level: HyperSchemaLevel,
    key: String,
    being: MonadicSchema,
    essence: MonadicSchema, // The middle!
    concept: MonadicSchema,
    center_mark: CenterMark,
    value_type: crate::types::ValueType,
}

impl TriadicSchema {
    /// Create a new triadic schema
    pub fn new(
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        being: MonadicSchema,
        essence: MonadicSchema,
        concept: MonadicSchema,
        center_mark: CenterMark,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Triadic,
            key: key.into(),
            being,
            essence,
            concept,
            center_mark,
            value_type,
        }
    }

    /// Get the being schema
    pub fn being(&self) -> &MonadicSchema {
        &self.being
    }

    /// Get the essence schema (the middle)
    pub fn essence(&self) -> &MonadicSchema {
        &self.essence
    }

    /// Get the concept schema
    pub fn concept(&self) -> &MonadicSchema {
        &self.concept
    }

    /// Get the center mark
    pub fn center_mark(&self) -> &CenterMark {
        &self.center_mark
    }
}

impl crate::types::schema::HyperSchema for TriadicSchema {
    fn level(&self) -> HyperSchemaLevel {
        HyperSchemaLevel::Triadic
    }

    fn value_type(&self) -> crate::types::ValueType {
        self.value_type
    }

    fn key(&self) -> &str {
        &self.key
    }
}


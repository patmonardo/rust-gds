//! Pentadic schema types: assertion and certification schema

use crate::types::schema::hyper::triadic::TriadicSchema;
use crate::types::schema::HyperSchemaLevel;

/// PentadicSchemaId: identifier for pentadic schema entries
pub type PentadicSchemaId = String;

/// Reference to a triadic entity (opaque id for now)
pub type TriadRef = String;

/// A simple certainty metric
pub type Certainty = f32;

/// PentadicSchema: The Schema of Knowledge (The Idea)
/// 
/// The fifth schema is the schema of Knowledge itself.
/// Pentadic represents the system "knowing" - the full knowing, raised from Concept to Idea.
/// 
/// THE FROM: The Idea stage (after Objectivity)
/// - FROM shows provenance, grounding, object-relation
/// - "Knows X FROM Y" indicates the Idea stage of Logic as Science
/// - This is key to the Idea of Cognition and the Idea of the True
/// 
/// Structure: "The system knows X FROM Y"
/// - primary_triad: What the system knows (X - the knowledge content)
/// - secondary_triad: From what (Y - the source, evidence, provenance)
/// - knowledge_assertion: The knowing statement (what we know)
/// - learning_chain_ref: The learning path that produced this knowing
/// 
/// THE SUBLATION OF MEDIACY:
/// The Learning Pipeline (Tetradic: Dyad × Dyad) approaches the Sublation of Mediacy.
/// Dyad × Dyad raised into the Pentad sheds all Perceptive Root into a Pure Conceptual framework.
/// 
/// This IS Learning:
/// - Tetradic: The perceptual chain (Dyad × Dyad with mediacy)
/// - Pentadic: Sublates the mediacy, sheds perceptive root, becomes pure concept
/// - FROM replaces AS: no longer "conceives X AS Y" but directly "knows X FROM Y"
/// - The mediating mark (essence, AS) is sublated into the object-relation (FROM)
/// 
/// The Path from Ignorance to Knowledge (Logic as Science):
/// - Monadic: Being
/// - Dyadic: Relationship (ground ↔ condition)
/// - Triadic: Conceiving (X AS Y - introduces Objectivity with mediacy)
/// - Tetradic: Learning (the chain of dyads - approaching sublation of mediacy)
/// - Pentadic: Knowledge (X FROM Y - sublated mediacy, pure concept)
#[derive(Clone, Debug)]
pub struct PentadicSchema {
    level: HyperSchemaLevel,
    key: String,
    /// What the system knows (the knowledge itself)
    primary_triad: TriadicSchema,
    /// From what the system knows it (source, evidence, provenance)
    secondary_triad: TriadicSchema,
    value_type: crate::types::ValueType,
    /// How certain the system is of this knowledge
    confidence: Option<Certainty>,
    /// Reference to the learning chain that produced this knowing
    learning_chain_ref: Option<String>,
    /// The knowledge assertion: "The system knows X from Y"
    knowledge_assertion: Option<String>,
}

impl PentadicSchema {
    /// Create a new pentadic schema for assertions
    pub fn new(
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        primary_triad: TriadicSchema,
        secondary_triad: TriadicSchema,
        confidence: Option<Certainty>,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Pentadic,
            key: key.into(),
            primary_triad,
            secondary_triad,
            value_type,
            confidence,
            learning_chain_ref: None,
            knowledge_assertion: None,
        }
    }

    /// Create a pentadic knowing from a learning chain
    /// This represents what the system knows as a result of the chain of dyads
    pub fn from_learning_chain(
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        primary_triad: TriadicSchema,
        secondary_triad: TriadicSchema,
        confidence: Option<Certainty>,
        learning_chain_ref: String,
        knowledge_assertion: String,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Pentadic,
            key: key.into(),
            primary_triad,
            secondary_triad,
            value_type,
            confidence,
            learning_chain_ref: Some(learning_chain_ref),
            knowledge_assertion: Some(knowledge_assertion),
        }
    }

    /// Raise the Concept (Triadic) to the Idea (Pentadic)
    /// 
    /// The movement from Conceiving to Knowing:
    /// - Triadic: "Conceives X AS Y" (Objectivity - AS introduces the mediating mark)
    /// - Pentadic: "Knows X FROM Y" (Idea - FROM shows the grounding/provenance)
    /// 
    /// This is the Idea stage (after Objectivity) in Logic as Science.
    /// Key to the Idea of Cognition and the Idea of the True.
    /// 
    /// Structure:
    /// - concept_triad: What we conceive (the concept with AS)
    /// - ground_triad: From what we conceive it (the ground - IS statements)
    /// - Assertion: The knowing that results (raised from Concept to Idea)
    /// 
    /// The FROM marks the object-relation that distinguishes Knowing from Conceiving.
    pub fn raise_concept_to_idea(
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        concept_triad: TriadicSchema,      // What the system conceives
        ground_triad: TriadicSchema,      // From what it conceives (ground)
        confidence: Option<Certainty>,
        learning_chain_ref: String,
        knowledge_assertion: String,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Pentadic,
            key: key.into(),
            primary_triad: concept_triad,  // The conceived knowledge
            secondary_triad: ground_triad,  // From what we know it
            value_type,
            confidence,
            learning_chain_ref: Some(learning_chain_ref),
            knowledge_assertion: Some(knowledge_assertion),
        }
    }

    /// Raise Dyad × Dyad (Tetradic Learning) into Pentadic (Pure Concept)
    /// This sublates the mediacy (sheds perceptive root, becomes pure concept)
    /// 
    /// The Sublation of Mediacy:
    /// - Tetradic (Dyad × Dyad) = Learning with perceptive mediacy
    /// - Raising to Pentadic sheds the perceptive root
    /// - Becomes pure conceptual framework
    /// - FROM replaces the mediating AS
    /// 
    /// This IS Learning: shedding all perceptive root into pure concept
    pub fn sublate_learning_pipeline(
        learning_chain_key: String,
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        primary_triad: TriadicSchema,
        secondary_triad: TriadicSchema,
        confidence: Option<Certainty>,
        knowledge_assertion: String,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Pentadic,
            key: key.into(),
            primary_triad,
            secondary_triad,
            value_type,
            confidence,
            learning_chain_ref: Some(learning_chain_key),
            knowledge_assertion: Some(knowledge_assertion),
        }
    }

    /// Get the primary triad
    pub fn primary_triad(&self) -> &TriadicSchema {
        &self.primary_triad
    }

    /// Get the secondary triad
    pub fn secondary_triad(&self) -> &TriadicSchema {
        &self.secondary_triad
    }

    /// Get the confidence level
    pub fn confidence(&self) -> Option<Certainty> {
        self.confidence
    }

    /// Get the learning chain reference
    pub fn learning_chain_ref(&self) -> Option<&String> {
        self.learning_chain_ref.as_ref()
    }

    /// Get the knowledge assertion (what we know)
    pub fn knowledge_assertion(&self) -> Option<&String> {
        self.knowledge_assertion.as_ref()
    }

    /// Check if this represents Knowing from a learning chain
    pub fn is_knowing(&self) -> bool {
        self.learning_chain_ref.is_some() && self.knowledge_assertion.is_some()
    }

    /// Get the full self-certification statement
    /// Format: "The system knows X FROM Y" - the complete Idea stage
    pub fn self_certification(&self) -> String {
        let assertion = self.knowledge_assertion.as_ref().cloned().unwrap_or_else(|| "Unknown".to_string());
        let source = self.learning_chain_ref.as_ref().cloned().unwrap_or_else(|| "Unknown source".to_string());
        format!("The system knows '{}' FROM '{}'", assertion, source)
    }
}

impl crate::types::schema::HyperSchema for PentadicSchema {
    fn level(&self) -> HyperSchemaLevel {
        HyperSchemaLevel::Pentadic
    }

    fn value_type(&self) -> crate::types::ValueType {
        self.value_type
    }

    fn key(&self) -> &str {
        &self.key
    }
}


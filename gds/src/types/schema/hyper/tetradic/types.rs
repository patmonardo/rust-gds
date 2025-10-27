//! Tetradic schema types: ML Pipeline schema (Dyadic²)

use crate::types::schema::hyper::dyadic::{DyadicSchema, DyadicSchemaId};
use crate::types::schema::HyperSchemaLevel;

/// TetradicSchemaId: identifier for tetradic schema entries
pub type TetradicSchemaId = String;

/// TetradicSchema: The Schema of Learning
/// 
/// The fourth schema is the schema of Learning - the Path from Ignorance to Knowledge.
/// Tetradic represents ML pipelines as chains of dyads: **Dyad × Dyad**
/// 
/// Learning is: a chain of relationships (dyads) that leads from not knowing to knowing.
/// Each link in the chain: ground ↔ condition (features ↔ labels, input ↔ output)
/// The chain itself: the ML pipeline that transforms ignorance into knowledge.
/// 
/// THE APPROACHING SUBLATION OF MEDIACY:
/// - Tetradic (Dyad × Dyad) still has perceptual root (mediacy)
/// - It approaches but hasn't yet shed the mediating mark
/// - The chain is still tied to perceptive/empirical grounding
/// - **Raising Dyad × Dyad into Pentadic** = the sublation of this mediacy
/// 
/// The Chain of Dyads:
/// - Each dyad is a relationship that builds toward knowing
/// - The chain connects dyads in sequence (with mediacy)
/// - Raising to Pentadic: sheds perceptive root, becomes pure concept
/// - FROM replaces AS: the mediating mark is sublated into object-relation
#[derive(Clone, Debug)]
pub struct TetradicSchema {
    level: HyperSchemaLevel,
    key: String,
    left_dyad: DyadicSchema,
    right_dyad: DyadicSchema,
    value_type: crate::types::ValueType,
    // Pipeline metadata
    pipeline_metadata: Option<String>,
    // Chain of dyads: the sequence of relationships that lead to Knowing
    chain_length: Option<usize>,
    // Next dyad in the chain (for building learning sequences)
    next_dyad_id: Option<crate::types::schema::hyper::dyadic::DyadicSchemaId>,
}

impl TetradicSchema {
    /// Create a new tetradic schema for ML pipelines
    /// 
    /// The Path from Ignorance to Knowledge:
    /// Each ML pipeline is a chain of relationships (dyads) that leads to knowing.
    /// Learning is simply this chain - from not knowing to knowing.
    pub fn new(
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        left_dyad: DyadicSchema,
        right_dyad: DyadicSchema,
        pipeline_metadata: Option<String>,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Tetradic,
            key: key.into(),
            left_dyad,
            right_dyad,
            value_type,
            pipeline_metadata,
            chain_length: None,
            next_dyad_id: None,
        }
    }

    /// Create a chain link in the learning sequence
    /// Each link connects to the next dyad in the chain
    pub fn as_chain_link(
        key: impl Into<String>,
        value_type: crate::types::ValueType,
        left_dyad: DyadicSchema,
        right_dyad: DyadicSchema,
        chain_length: usize,
        next_dyad_id: Option<DyadicSchemaId>,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Tetradic,
            key: key.into(),
            left_dyad,
            right_dyad,
            value_type,
            pipeline_metadata: None,
            chain_length: Some(chain_length),
            next_dyad_id,
        }
    }

    /// Get the left dyad (features)
    pub fn left_dyad(&self) -> &DyadicSchema {
        &self.left_dyad
    }

    /// Get the right dyad (labels)
    pub fn right_dyad(&self) -> &DyadicSchema {
        &self.right_dyad
    }

    /// Get pipeline metadata
    pub fn pipeline_metadata(&self) -> Option<&String> {
        self.pipeline_metadata.as_ref()
    }

    /// Get the chain length (how many links in this learning chain)
    pub fn chain_length(&self) -> Option<usize> {
        self.chain_length
    }

    /// Get the next dyad ID in the chain
    pub fn next_dyad_id(&self) -> Option<&DyadicSchemaId> {
        self.next_dyad_id.as_ref()
    }

    /// Check if this is a chain link (part of a learning sequence)
    pub fn is_chain_link(&self) -> bool {
        self.chain_length.is_some()
    }
}

impl crate::types::schema::HyperSchema for TetradicSchema {
    fn level(&self) -> HyperSchemaLevel {
        HyperSchemaLevel::Tetradic
    }

    fn value_type(&self) -> crate::types::ValueType {
        self.value_type
    }

    fn key(&self) -> &str {
        &self.key
    }
}


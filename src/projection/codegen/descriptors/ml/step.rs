//! Step descriptors for ML pipelines.
//!
//! Maps to Java's step types:
//! - ExecutableNodePropertyStep
//! - FeatureStep
//! - NodePropertyStep

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Single step in an ML pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepDescriptor {
    /// Node property mutation step
    NodeProperty(NodePropertyStepDescriptor),

    /// Feature extraction step
    Feature(FeatureStepDescriptor),
}

/// Descriptor for a node property mutation step.
///
/// Maps to Java's `org.neo4j.gds.ml.pipeline.ExecutableNodePropertyStep`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePropertyStepDescriptor {
    /// Step name/identifier
    pub name: String,

    /// Algorithm to execute (e.g., "pageRank", "fastRP")
    pub algorithm: String,

    /// Property name to write results to
    pub property_name: String,

    /// Algorithm-specific configuration
    pub config: NodePropertyStepConfig,
}

/// Configuration for node property step algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePropertyStepConfig {
    /// Algorithm parameters (algorithm-specific key-value pairs)
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Descriptor for a feature extraction step.
///
/// Maps to Java's `org.neo4j.gds.ml.pipeline.FeatureStep`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStepDescriptor {
    /// Step name/identifier
    pub name: String,

    /// Feature type (e.g., "scalar", "array", "embedding")
    pub feature_type: FeatureType,

    /// Source properties to extract features from
    pub source_properties: Vec<String>,

    /// Target feature dimension (for dimensionality reduction/projection)
    pub target_dimension: Option<usize>,
}

/// Type of feature extraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureType {
    /// Single scalar value per node
    Scalar,

    /// Fixed-size array per node
    Array,

    /// Variable-size embedding per node
    Embedding,
}

impl NodePropertyStepDescriptor {
    /// Create a new node property step descriptor.
    pub fn new(name: String, algorithm: String, property_name: String) -> Self {
        Self {
            name,
            algorithm,
            property_name,
            config: NodePropertyStepConfig {
                parameters: HashMap::new(),
            },
        }
    }
}

impl FeatureStepDescriptor {
    /// Create a new feature step descriptor.
    pub fn new(name: String, feature_type: FeatureType, source_properties: Vec<String>) -> Self {
        Self {
            name,
            feature_type,
            source_properties,
            target_dimension: None,
        }
    }

    /// Set target dimension for feature projection.
    pub fn with_target_dimension(mut self, dimension: usize) -> Self {
        self.target_dimension = Some(dimension);
        self
    }
}

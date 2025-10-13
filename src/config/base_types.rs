//! Base configuration types and traits
//!
//! Provides core configuration interfaces that other configs extend.

use crate::projection::{NodeLabel, RelationshipType};

/// Marker trait for all configuration types
pub trait Config: Send + Sync {}

/// Configuration with concurrency settings
pub trait ConcurrencyConfig: Config {
    fn concurrency(&self) -> usize;
}

/// Configuration with write concurrency settings
pub trait WriteConfig: Config {
    fn write_concurrency(&self) -> usize;
}

/// Configuration with relationship weight property
pub trait RelationshipWeightConfig: Config {
    fn relationship_weight_property(&self) -> Option<&str>;
}

/// Configuration with write property
pub trait WritePropertyConfig: WriteConfig {
    fn write_property(&self) -> &str;
}

/// Configuration with iteration settings
pub trait IterationsConfig: Config {
    fn max_iterations(&self) -> usize;
    fn tolerance(&self) -> Option<f64>;
}

/// Configuration with embedding dimension
pub trait EmbeddingDimensionConfig: Config {
    fn embedding_dimension(&self) -> usize;
}

/// Configuration with feature properties
pub trait FeaturePropertiesConfig: Config {
    fn feature_properties(&self) -> &[String];
}

/// Configuration with deduplication settings
pub trait DeduplicationConfig: Config {
    fn deduplicate_ids(&self) -> bool;
}

/// Base configuration for algorithms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AlgoBaseConfig {
    pub concurrency: usize,
    pub node_labels: Vec<NodeLabel>,
    pub relationship_types: Vec<RelationshipType>,
}

impl Default for AlgoBaseConfig {
    fn default() -> Self {
        Self {
            concurrency: num_cpus::get(),
            node_labels: vec![NodeLabel::all_nodes()],
            relationship_types: vec![RelationshipType::all_relationships()],
        }
    }
}

impl Config for AlgoBaseConfig {}

impl ConcurrencyConfig for AlgoBaseConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

/// Base configuration for mutate operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MutateConfig {
    pub write_concurrency: usize,
    pub mutate_property: String,
}

impl Default for MutateConfig {
    fn default() -> Self {
        Self {
            write_concurrency: num_cpus::get(),
            mutate_property: String::from("mutated"),
        }
    }
}

impl Config for MutateConfig {}

impl WriteConfig for MutateConfig {
    fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }
}

/// Base configuration for builder operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuilderConfig {
    pub use_pooled_builder_provider: bool,
    pub max_original_id: i64,
    pub max_intermediate_id: i64,
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self {
            use_pooled_builder_provider: false,
            max_original_id: -1,
            max_intermediate_id: -1,
        }
    }
}

impl Config for BuilderConfig {}

/// Base configuration interface for ML models
pub trait BaseConfig: Config {
    fn parameters(&self) -> std::collections::HashMap<String, serde_json::Value>;
}

/// CustomInfo trait for model custom information.
pub trait CustomInfo: ToMapConvertible + serde::Serialize + serde::de::DeserializeOwned {
    /// Optional training method.
    fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        None
    }
}

/// Training method enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TrainingMethod {
    /// Supervised training
    Supervised,
    /// Unsupervised training
    Unsupervised,
    /// Semi-supervised training
    SemiSupervised,
}

/// Graph schema for model training
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphSchema {
    pub node_labels: Vec<String>,
    pub relationship_types: Vec<String>,
    pub node_properties: Vec<String>,
    pub relationship_properties: Vec<String>,
}

impl GraphSchema {
    pub fn new() -> Self {
        Self {
            node_labels: Vec::new(),
            relationship_types: Vec::new(),
            node_properties: Vec::new(),
            relationship_properties: Vec::new(),
        }
    }
}

impl Default for GraphSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that can be converted to a map
pub trait ToMapConvertible {
    fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value>;
}

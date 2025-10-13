//! ModelMetaData interface for ML model metadata in GDS.
//!
//! Translated from Java GDS ModelMetaData.java in model-catalog-api.
//! This is a literal 1:1 translation following repository translation policy.

use std::time::SystemTime;

use crate::types::prelude::*;

/// ModelMetaData interface for ML model metadata.
///
/// Generic parameters:
/// - CONFIG: Configuration type extending ModelConfig
/// - INFO: Custom info type extending CustomInfo
pub trait ModelMetaData<CONFIG, INFO>
where
    CONFIG: ModelConfig,
    INFO: CustomInfo,
{
    /// The creator of the model.
    fn creator(&self) -> &str;

    /// Users with whom the model is shared.
    fn shared_with(&self) -> &[String];

    /// The name of the model.
    fn name(&self) -> &str;

    /// The algorithm type.
    fn algo_type(&self) -> &str;

    /// The graph schema used for training.
    fn graph_schema(&self) -> &GraphSchema;

    /// The training configuration.
    fn train_config(&self) -> &CONFIG;

    /// The creation time.
    fn creation_time(&self) -> SystemTime;

    /// Custom information about the model.
    fn custom_info(&self) -> &INFO;

    /// The GDS version used to create the model.
    fn gds_version(&self) -> &str;

    /// Whether the model is published (shared with all users).
    fn is_published(&self) -> bool {
        self.shared_with().contains(&"ALL_USERS".to_string())
    }
}

/// Concrete implementation of ModelMetaData trait.
pub struct ConcreteModelMetaData<C, I>
where
    C: ModelConfig,
    I: CustomInfo,
{
    creator: String,
    shared_with: Vec<String>,
    name: String,
    algo_type: String,
    graph_schema: GraphSchema,
    train_config: C,
    creation_time: SystemTime,
    custom_info: I,
    gds_version: String,
}

impl<C, I> ModelMetaData<C, I> for ConcreteModelMetaData<C, I>
where
    C: ModelConfig,
    I: CustomInfo,
{
    fn creator(&self) -> &str {
        &self.creator
    }

    fn shared_with(&self) -> &[String] {
        &self.shared_with
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn algo_type(&self) -> &str {
        &self.algo_type
    }

    fn graph_schema(&self) -> &GraphSchema {
        &self.graph_schema
    }

    fn train_config(&self) -> &C {
        &self.train_config
    }

    fn creation_time(&self) -> SystemTime {
        self.creation_time
    }

    fn custom_info(&self) -> &I {
        &self.custom_info
    }

    fn gds_version(&self) -> &str {
        &self.gds_version
    }
}

impl<C, I> ConcreteModelMetaData<C, I>
where
    C: ModelConfig,
    I: CustomInfo,
{
    /// Create a new ConcreteModelMetaData instance.
    pub fn new(
        creator: String,
        shared_with: Vec<String>,
        name: String,
        algo_type: String,
        graph_schema: GraphSchema,
        train_config: C,
        creation_time: SystemTime,
        custom_info: I,
        gds_version: String,
    ) -> Self {
        Self {
            creator,
            shared_with,
            name,
            algo_type,
            graph_schema,
            train_config,
            creation_time,
            custom_info,
            gds_version,
        }
    }
}

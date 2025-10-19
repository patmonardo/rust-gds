//! ModelMetaData interface for ML model metadata in GDS.
//!
//! Translated from Java GDS ModelMetaData.java in model-catalog-api.
//! This is a literal 1:1 translation following repository translation policy.

use std::time::SystemTime;

use crate::config::{Config, GraphSchema};
use crate::core::model::model::CustomInfo;

/// ModelMetaData interface for ML model metadata.
///
/// Generic parameters:
/// - CONFIG: Configuration type extending ModelConfig
/// - INFO: Custom info type extending CustomInfo
pub trait ModelMetaData<CONFIG, INFO>
where
    CONFIG: Config,
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


//! Model interface for ML models in GDS.
//!
//! Translated from Java GDS Model.java in model-catalog-api.
//! This is a literal 1:1 translation following repository translation policy.

use std::path::PathBuf;
use std::time::SystemTime;

use crate::config::{BaseConfig, ToMapConvertible, GraphSchema};
// use crate::ml::TrainingMethod;  // Commented out - ml module deactivated
use crate::core::model::types::ModelConfig;

/// TrainingMethod - Stub type (ml module deactivated)
#[derive(Debug, Clone)]
pub enum TrainingMethod {
    // Stub implementation - ml module deactivated
    Stub,
}

/// Model interface for ML models.
///
/// Generic parameters:
/// - DATA: The model data type
/// - CONFIG: Configuration type extending ModelConfig and BaseConfig
/// - INFO: Custom info type extending CustomInfo
pub trait Model<DATA, CONFIG, INFO>
where
    CONFIG: ModelConfig + BaseConfig,
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

    /// The model data (nullable).
    fn data(&self) -> Option<&DATA>;

    /// The training configuration.
    fn train_config(&self) -> &CONFIG;

    /// The creation time.
    fn creation_time(&self) -> SystemTime;

    /// The GDS version used to create the model.
    fn gds_version(&self) -> &str;

    /// Custom information about the model.
    fn custom_info(&self) -> &INFO;

    /// File location if stored (optional).
    fn file_location(&self) -> Option<&PathBuf>;

    /// Whether the model is loaded (data is not null).
    fn loaded(&self) -> bool {
        self.data().is_some()
    }

    /// Whether the model is stored (file location is present).
    fn stored(&self) -> bool {
        self.file_location().is_some()
    }

    /// Whether the model is published (shared with all users).
    fn is_published(&self) -> bool {
        self.shared_with().contains(&"ALL_USERS".to_string())
    }
}

/// Static factory methods for Model.
pub mod model_factory {
    // Factory methods can be added here when needed
}

/// CustomInfo trait for model custom information.
pub trait CustomInfo: ToMapConvertible + serde::Serialize + serde::de::DeserializeOwned {
    /// Optional training method.
    fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        None
    }
}


//! Model interface for ML models in GDS.
//!
//! Translated from Java GDS Model.java in model-catalog-api.
//! This is a literal 1:1 translation following repository translation policy.

use std::path::PathBuf;
use std::time::SystemTime;

use crate::types::prelude::*;

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
    use super::*;

    /// All users constant.
    pub const ALL_USERS: &str = "*";

    /// Public model suffix constant.
    pub const PUBLIC_MODEL_SUFFIX: &str = "_public";

    /// Create a new Model instance.
    pub fn of<D, C, I>(
        gds_version: String,
        algo_type: String,
        graph_schema: GraphSchema,
        model_data: D,
        train_config: C,
        custom_info: I,
    ) -> impl Model<D, C, I>
    where
        C: ModelConfig + BaseConfig,
        I: CustomInfo,
    {
        ConcreteModel {
            creator: train_config.username().to_string(),
            shared_with: Vec::new(),
            name: train_config.model_name().to_string(),
            algo_type,
            graph_schema,
            data: Some(model_data),
            train_config,
            creation_time: SystemTime::now(),
            gds_version,
            custom_info,
            file_location: None,
        }
    }

    /// Create a new Model instance (test-only version).
    #[cfg(test)]
    pub fn of_test<D, C, I>(
        algo_type: String,
        graph_schema: GraphSchema,
        model_data: D,
        train_config: C,
        custom_info: I,
    ) -> impl Model<D, C, I>
    where
        C: ModelConfig + BaseConfig,
        I: CustomInfo,
    {
        of(
            "default".to_string(),
            algo_type,
            graph_schema,
            model_data,
            train_config,
            custom_info,
        )
    }
}

/// CustomInfo trait for model custom information.
pub trait CustomInfo: ToMapConvertible + serde::Serialize + serde::de::DeserializeOwned {
    /// Optional training method.
    fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        None
    }
}

/// Concrete implementation of Model trait.
struct ConcreteModel<D, C, I>
where
    C: ModelConfig + BaseConfig,
    I: CustomInfo,
{
    creator: String,
    shared_with: Vec<String>,
    name: String,
    algo_type: String,
    graph_schema: GraphSchema,
    data: Option<D>,
    train_config: C,
    creation_time: SystemTime,
    gds_version: String,
    custom_info: I,
    file_location: Option<PathBuf>,
}

impl<D, C, I> Model<D, C, I> for ConcreteModel<D, C, I>
where
    C: ModelConfig + BaseConfig,
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

    fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    fn train_config(&self) -> &C {
        &self.train_config
    }

    fn creation_time(&self) -> SystemTime {
        self.creation_time
    }

    fn gds_version(&self) -> &str {
        &self.gds_version
    }

    fn custom_info(&self) -> &I {
        &self.custom_info
    }

    fn file_location(&self) -> Option<&PathBuf> {
        self.file_location.as_ref()
    }
}

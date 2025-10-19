use crate::config::BaseConfig;
use crate::types::schema::GraphSchema;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

pub const ALL_USERS: &str = "*";
// pub const PUBLIC_MODEL_SUFFIX: &str = "_public";

/// Core model trait for serializable model data
pub trait ModelData: Send + Sync + Debug + Serialize {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Custom info attached to models
pub trait CustomInfo: Send + Sync + Debug + Serialize {
    fn to_map(&self) -> serde_json::Value;
    fn training_method(&self) -> Option<String> {
        None
    }
}

/// Configuration for model training/creation
pub trait ModelConfig: Send + Sync + Debug + Serialize + BaseConfig {
    const MODEL_NAME_KEY: &'static str = "modelName";
    const MODEL_TYPE_KEY: &'static str = "modelType";

    fn model_name(&self) -> &str;
    fn model_user(&self) -> &str;
    fn username(&self) -> String {
        self.username_override()
            .unwrap_or_else(|| self.model_user().to_string())
    }
    fn username_override(&self) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model<D, C, I>
where
    D: ModelData,
    C: ModelConfig,
    I: CustomInfo,
{
    creator: String,
    shared_with: Vec<String>,
    name: String,
    algo_type: String,
    graph_schema: GraphSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<D>,
    train_config: C,
    creation_time: DateTime<Utc>,
    gds_version: String,
    custom_info: I,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_location: Option<PathBuf>,
}

impl<D, C, I> Model<D, C, I>
where
    D: ModelData,
    C: ModelConfig,
    I: CustomInfo,
{
    pub fn new(
        creator: String,
        name: String,
        algo_type: String,
        graph_schema: GraphSchema,
        data: Option<D>,
        train_config: C,
        gds_version: String,
        custom_info: I,
    ) -> Self {
        Self {
            creator,
            shared_with: Vec::new(),
            name,
            algo_type,
            graph_schema,
            data,
            train_config,
            creation_time: Utc::now(),
            gds_version,
            custom_info,
            file_location: None,
        }
    }

    pub fn creator(&self) -> &str {
        &self.creator
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn algo_type(&self) -> &str {
        &self.algo_type
    }

    pub fn graph_schema(&self) -> &GraphSchema {
        &self.graph_schema
    }

    pub fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    pub fn train_config(&self) -> &C {
        &self.train_config
    }

    pub fn creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }

    pub fn custom_info(&self) -> &I {
        &self.custom_info
    }

    pub fn file_location(&self) -> Option<&PathBuf> {
        self.file_location.as_ref()
    }

    pub fn is_loaded(&self) -> bool {
        self.data.is_some()
    }

    pub fn is_stored(&self) -> bool {
        self.file_location.is_some()
    }

    pub fn is_published(&self) -> bool {
        self.shared_with.contains(&ALL_USERS.to_string())
    }

    pub fn with_file_location(mut self, location: PathBuf) -> Self {
        self.file_location = Some(location);
        self
    }

    pub fn share_with(&mut self, username: &str) {
        if !self.shared_with.contains(&username.to_string()) {
            self.shared_with.push(username.to_string());
        }
    }

    pub fn publish(&mut self) {
        self.share_with(ALL_USERS);
    }
}

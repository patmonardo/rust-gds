use crate::ml::models::{TrainerConfig, TrainingMethod};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shared configuration for random forest models
/// 1:1 with RandomForestTrainerConfig.java interface from Java GDS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestConfig {
    /// Maximum ratio of features to consider for splits
    #[serde(default = "default_max_features_ratio")]
    pub max_features_ratio: Option<f64>,

    /// Ratio of training samples to use for each tree
    #[serde(default = "default_num_samples_ratio")]
    pub num_samples_ratio: f64,

    /// Number of trees in the forest
    #[serde(default = "default_num_trees")]
    pub num_decision_trees: usize,

    /// Maximum depth of each tree
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,

    /// Minimum samples required to split
    #[serde(default = "default_min_samples_split")]
    pub min_samples_split: usize,

    /// Minimum samples required in a leaf
    #[serde(default = "default_min_samples_leaf")]
    pub min_samples_leaf: usize,
}

impl RandomForestConfig {
    /// Get max features ratio with default based on feature dimension
    /// 1:1 with maxFeaturesRatio(int featureDimension) in Java
    pub fn max_features_ratio(&self, feature_dimension: usize) -> f64 {
        self.max_features_ratio
            .unwrap_or_else(|| 1.0 / (feature_dimension as f64).sqrt())
    }
}

impl Default for RandomForestConfig {
    fn default() -> Self {
        Self {
            max_features_ratio: None,
            num_samples_ratio: default_num_samples_ratio(),
            num_decision_trees: default_num_trees(),
            max_depth: default_max_depth(),
            min_samples_split: default_min_samples_split(),
            min_samples_leaf: default_min_samples_leaf(),
        }
    }
}

fn default_max_features_ratio() -> Option<f64> {
    None
}
fn default_num_samples_ratio() -> f64 {
    1.0
}
fn default_num_trees() -> usize {
    100
}
fn default_max_depth() -> usize {
    0
} // 0 means unlimited
fn default_min_samples_split() -> usize {
    2
}
fn default_min_samples_leaf() -> usize {
    1
}

/// Configuration for random forest classifier trainer
/// 1:1 with RandomForestClassifierTrainerConfig.java from Java GDS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestClassifierTrainerConfig {
    #[serde(flatten)]
    pub forest: RandomForestConfig,
}

impl TrainerConfig for RandomForestClassifierTrainerConfig {
    fn method(&self) -> TrainingMethod {
        TrainingMethod::RandomForestClassification
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "maxFeaturesRatio".to_string(),
            serde_json::json!(self.forest.max_features_ratio),
        );
        map.insert(
            "numberOfSamplesRatio".to_string(),
            serde_json::json!(self.forest.num_samples_ratio),
        );
        map.insert(
            "numberOfDecisionTrees".to_string(),
            serde_json::json!(self.forest.num_decision_trees),
        );
        map.insert(
            "maxDepth".to_string(),
            serde_json::json!(self.forest.max_depth),
        );
        map.insert(
            "minSamplesSplit".to_string(),
            serde_json::json!(self.forest.min_samples_split),
        );
        map.insert(
            "minSamplesLeaf".to_string(),
            serde_json::json!(self.forest.min_samples_leaf),
        );
        map
    }
}

/// Configuration for random forest regressor trainer
/// 1:1 with RandomForestRegressorTrainerConfig.java from Java GDS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestRegressorTrainerConfig {
    #[serde(flatten)]
    pub forest: RandomForestConfig,
}

impl TrainerConfig for RandomForestRegressorTrainerConfig {
    fn method(&self) -> TrainingMethod {
        TrainingMethod::RandomForestRegression
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "maxFeaturesRatio".to_string(),
            serde_json::json!(self.forest.max_features_ratio),
        );
        map.insert(
            "numberOfSamplesRatio".to_string(),
            serde_json::json!(self.forest.num_samples_ratio),
        );
        map.insert(
            "numberOfDecisionTrees".to_string(),
            serde_json::json!(self.forest.num_decision_trees),
        );
        map.insert(
            "maxDepth".to_string(),
            serde_json::json!(self.forest.max_depth),
        );
        map.insert(
            "minSamplesSplit".to_string(),
            serde_json::json!(self.forest.min_samples_split),
        );
        map.insert(
            "minSamplesLeaf".to_string(),
            serde_json::json!(self.forest.min_samples_leaf),
        );
        map
    }
}

use crate::ml::models::{TrainerConfig, TrainingMethod};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for logistic regression training
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogisticRegressionTrainConfig {
    /// L2 regularization penalty
    #[serde(default = "default_penalty")]
    pub penalty: f64,

    /// Batch size for training
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Learning rate for gradient descent
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f64,

    /// Maximum number of epochs
    #[serde(default = "default_max_epochs")]
    pub max_epochs: usize,

    /// Minimum change in loss to continue training
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    /// Weight for focal loss
    #[serde(default = "default_focus_weight")]
    pub focus_weight: f64,

    /// Class weights for handling imbalanced data
    #[serde(default)]
    pub class_weights: Option<Vec<f64>>,
}

fn default_penalty() -> f64 {
    0.0
}
fn default_batch_size() -> usize {
    100
}
fn default_learning_rate() -> f64 {
    0.001
}
fn default_max_epochs() -> usize {
    100
}
fn default_tolerance() -> f64 {
    1e-4
}
fn default_focus_weight() -> f64 {
    0.0
}

impl Default for LogisticRegressionTrainConfig {
    fn default() -> Self {
        Self {
            penalty: default_penalty(),
            batch_size: default_batch_size(),
            learning_rate: default_learning_rate(),
            max_epochs: default_max_epochs(),
            tolerance: default_tolerance(),
            focus_weight: default_focus_weight(),
            class_weights: None,
        }
    }
}

impl TrainerConfig for LogisticRegressionTrainConfig {
    fn method(&self) -> TrainingMethod {
        TrainingMethod::LogisticRegression
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "method".to_string(),
            serde_json::Value::String("LogisticRegression".to_string()),
        );
        map.insert(
            "penalty".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.penalty).unwrap()),
        );
        map.insert(
            "batch_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.batch_size)),
        );
        map.insert(
            "learning_rate".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.learning_rate).unwrap()),
        );
        map.insert(
            "max_epochs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.max_epochs)),
        );
        map.insert(
            "tolerance".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.tolerance).unwrap()),
        );
        map.insert(
            "focus_weight".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.focus_weight).unwrap()),
        );

        if let Some(ref class_weights) = self.class_weights {
            let weights_array: Vec<serde_json::Value> = class_weights
                .iter()
                .map(|&w| serde_json::Value::Number(serde_json::Number::from_f64(w).unwrap()))
                .collect();
            map.insert(
                "class_weights".to_string(),
                serde_json::Value::Array(weights_array),
            );
        }

        map
    }
}

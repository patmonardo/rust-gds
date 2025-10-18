use crate::ml::{
    gradient_descent::GradientDescentConfig, models::ClassAwareTrainerConfig, TrainingMethod,
};
use serde::{Deserialize, Serialize};

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

impl GradientDescentConfig for LogisticRegressionTrainConfig {
    fn batch_size(&self) -> usize {
        self.batch_size
    }

    fn learning_rate(&self) -> f64 {
        self.learning_rate
    }

    fn tolerance(&self) -> f64 {
        self.tolerance
    }

    fn max_epochs(&self) -> usize {
        self.max_epochs
    }
}

impl ClassAwareTrainerConfig for LogisticRegressionTrainConfig {
    fn method(&self) -> TrainingMethod {
        TrainingMethod::LogisticRegression
    }

    fn class_weights(&self) -> Option<&[f64]> {
        self.class_weights.as_deref()
    }
}

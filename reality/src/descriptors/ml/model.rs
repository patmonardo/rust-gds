//! Model descriptor for ML architectures.
//!
//! Maps to Java ML model classes in ml-algo package.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model architecture and hyperparameters.
///
/// Maps to Java model types (LogisticRegression, RandomForest, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescriptor {
    /// Model type (e.g., "logisticRegression", "randomForest")
    pub model_type: String,

    /// Model-specific hyperparameters
    pub hyperparameters: HashMap<String, serde_json::Value>,
}

impl ModelDescriptor {
    /// Create a new model descriptor.
    pub fn new(model_type: String) -> Self {
        Self {
            model_type,
            hyperparameters: HashMap::new(),
        }
    }

    /// Add a hyperparameter.
    pub fn with_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.hyperparameters.insert(key, value);
        self
    }
}

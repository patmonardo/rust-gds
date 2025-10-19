//! Linear Regression training configuration.
//!
//! Translation of `LinearRegressionTrainConfig.java` from Java GDS.

use crate::ml::{
    gradient_descent::GradientDescentConfig,
    models::{TrainerConfig, TrainingMethod},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration container for linear regression training.
/// Matches the Java `LinearRegressionTrainConfig` interface which combines
/// gradient descent parameters with the L2 penalty term.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinearRegressionTrainConfig {
    /// Gradient descent hyper-parameters (batch size, learning rate, epochs, tolerance).
    #[serde(default)]
    gradient: GradientDescentConfig,

    /// L2 regularization penalty (a.k.a. ridge penalty).
    #[serde(default)]
    penalty: f64,
}

impl Default for LinearRegressionTrainConfig {
    fn default() -> Self {
        Self {
            gradient: GradientDescentConfig::default(),
            penalty: 0.0,
        }
    }
}

impl LinearRegressionTrainConfig {
    /// Create a new configuration with explicit values.
    pub fn new(gradient: GradientDescentConfig, penalty: f64) -> Self {
        Self { gradient, penalty }
    }

    /// Accessor mirroring Java's `gradientDescentConfig()` getter.
    pub fn gradient(&self) -> &GradientDescentConfig {
        &self.gradient
    }

    /// Mutable accessor used when overriding sub-fields.
    pub fn gradient_mut(&mut self) -> &mut GradientDescentConfig {
        &mut self.gradient
    }

    /// L2 regularization penalty.
    pub fn penalty(&self) -> f64 {
        self.penalty
    }

    /// Update the L2 penalty.
    pub fn set_penalty(&mut self, penalty: f64) {
        self.penalty = penalty;
    }

    /// Convenience re-export of the learning rate for 1:1 parity with Java config.
    pub fn learning_rate(&self) -> f64 {
        self.gradient.learning_rate()
    }

    /// Convenience re-export of the batch size (mini-batch size).
    pub fn batch_size(&self) -> usize {
        self.gradient.batch_size()
    }

    /// Convenience re-export of the maximum epochs.
    pub fn max_epochs(&self) -> usize {
        self.gradient.max_epochs()
    }

    /// Convenience re-export of the convergence tolerance.
    pub fn tolerance(&self) -> f64 {
        self.gradient.tolerance()
    }

    /// Training method marker, mirroring Java's `method()` default implementation.
    pub fn method(&self) -> TrainingMethod {
        TrainingMethod::LinearRegression
    }
}

impl TrainerConfig for LinearRegressionTrainConfig {
    fn method(&self) -> TrainingMethod {
        TrainingMethod::LinearRegression
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "method".to_string(),
            serde_json::Value::String("LinearRegression".to_string()),
        );
        map.insert(
            "penalty".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.penalty).unwrap()),
        );

        // Add gradient descent config fields
        map.insert(
            "batch_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.gradient.batch_size())),
        );
        map.insert(
            "learning_rate".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.gradient.learning_rate()).unwrap(),
            ),
        );
        map.insert(
            "max_epochs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.gradient.max_epochs())),
        );
        map.insert(
            "tolerance".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.gradient.tolerance()).unwrap(),
            ),
        );
        map.insert(
            "min_epochs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.gradient.min_epochs())),
        );
        map.insert(
            "patience".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.gradient.patience())),
        );

        map
    }
}

//! Training method enumeration for ML models.
//!
//! This enum represents the different machine learning training methods
//! available in the GDS ML system. Each variant corresponds to a specific
//! algorithm that can be trained on graph data.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Enumeration of supported machine learning training methods.
///
/// This corresponds to the TrainingMethod enum in Java GDS ml-api.
/// Each variant represents a different ML algorithm that can be trained.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingMethod {
    /// Logistic regression for binary/multiclass classification.
    LogisticRegression,

    /// Linear regression for continuous prediction.
    LinearRegression,

    /// Random forest classifier.
    RandomForestClassification,

    /// Random forest regressor.
    RandomForestRegression,

    /// Multilayer perceptron (neural network) classifier.
    MLPClassification,
}

impl fmt::Display for TrainingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainingMethod::LogisticRegression => write!(f, "LogisticRegression"),
            TrainingMethod::LinearRegression => write!(f, "LinearRegression"),
            TrainingMethod::RandomForestClassification => write!(f, "RandomForest"),
            TrainingMethod::RandomForestRegression => write!(f, "RandomForest"),
            TrainingMethod::MLPClassification => write!(f, "MultilayerPerceptron"),
        }
    }
}

impl TrainingMethod {
    /// Check if this is a classification method.
    pub fn is_classification(&self) -> bool {
        matches!(
            self,
            TrainingMethod::LogisticRegression
                | TrainingMethod::RandomForestClassification
                | TrainingMethod::MLPClassification
        )
    }

    /// Check if this is a regression method.
    pub fn is_regression(&self) -> bool {
        matches!(
            self,
            TrainingMethod::LinearRegression | TrainingMethod::RandomForestRegression
        )
    }

    /// Check if this is a random forest method.
    pub fn is_random_forest(&self) -> bool {
        matches!(
            self,
            TrainingMethod::RandomForestClassification | TrainingMethod::RandomForestRegression
        )
    }

    /// Check if this is a neural network method.
    pub fn is_neural_network(&self) -> bool {
        matches!(self, TrainingMethod::MLPClassification)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_logistic_regression() {
        assert_eq!(
            TrainingMethod::LogisticRegression.to_string(),
            "LogisticRegression"
        );
    }

    #[test]
    fn test_display_linear_regression() {
        assert_eq!(
            TrainingMethod::LinearRegression.to_string(),
            "LinearRegression"
        );
    }

    #[test]
    fn test_display_random_forest_classification() {
        assert_eq!(
            TrainingMethod::RandomForestClassification.to_string(),
            "RandomForest"
        );
    }

    #[test]
    fn test_display_random_forest_regression() {
        assert_eq!(
            TrainingMethod::RandomForestRegression.to_string(),
            "RandomForest"
        );
    }

    #[test]
    fn test_display_mlp_classification() {
        assert_eq!(
            TrainingMethod::MLPClassification.to_string(),
            "MultilayerPerceptron"
        );
    }

    #[test]
    fn test_is_classification() {
        assert!(TrainingMethod::LogisticRegression.is_classification());
        assert!(TrainingMethod::RandomForestClassification.is_classification());
        assert!(TrainingMethod::MLPClassification.is_classification());
        assert!(!TrainingMethod::LinearRegression.is_classification());
        assert!(!TrainingMethod::RandomForestRegression.is_classification());
    }

    #[test]
    fn test_is_regression() {
        assert!(TrainingMethod::LinearRegression.is_regression());
        assert!(TrainingMethod::RandomForestRegression.is_regression());
        assert!(!TrainingMethod::LogisticRegression.is_regression());
        assert!(!TrainingMethod::RandomForestClassification.is_regression());
        assert!(!TrainingMethod::MLPClassification.is_regression());
    }

    #[test]
    fn test_is_random_forest() {
        assert!(TrainingMethod::RandomForestClassification.is_random_forest());
        assert!(TrainingMethod::RandomForestRegression.is_random_forest());
        assert!(!TrainingMethod::LogisticRegression.is_random_forest());
        assert!(!TrainingMethod::LinearRegression.is_random_forest());
        assert!(!TrainingMethod::MLPClassification.is_random_forest());
    }

    #[test]
    fn test_is_neural_network() {
        assert!(TrainingMethod::MLPClassification.is_neural_network());
        assert!(!TrainingMethod::LogisticRegression.is_neural_network());
        assert!(!TrainingMethod::LinearRegression.is_neural_network());
        assert!(!TrainingMethod::RandomForestClassification.is_neural_network());
        assert!(!TrainingMethod::RandomForestRegression.is_neural_network());
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(
            TrainingMethod::LogisticRegression,
            TrainingMethod::LogisticRegression
        );
        assert_ne!(
            TrainingMethod::LogisticRegression,
            TrainingMethod::LinearRegression
        );
    }

    #[test]
    fn test_enum_copy() {
        let method1 = TrainingMethod::LogisticRegression;
        let method2 = method1; // Copy
        assert_eq!(method1, method2);
    }
}

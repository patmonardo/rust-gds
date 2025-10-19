//! Training method enumeration
//!
//! 1:1 translation of TrainingMethod.java from Java GDS

use serde::{Deserialize, Serialize};
use std::fmt;

/// Training method enum matching Java GDS TrainingMethod
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingMethod {
    /// Logistic Regression classifier
    LogisticRegression,
    /// Random Forest classifier
    RandomForestClassification,
    /// Multi-Layer Perceptron classifier
    MLPClassification,
    /// Linear Regression
    LinearRegression,
    /// Random Forest regressor
    RandomForestRegression,
}

impl fmt::Display for TrainingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainingMethod::LogisticRegression => write!(f, "LogisticRegression"),
            TrainingMethod::RandomForestClassification => write!(f, "RandomForestClassification"),
            TrainingMethod::MLPClassification => write!(f, "MLPClassification"),
            TrainingMethod::LinearRegression => write!(f, "LinearRegression"),
            TrainingMethod::RandomForestRegression => write!(f, "RandomForestRegression"),
        }
    }
}

impl TrainingMethod {
    /// Check if this is a classification method
    pub fn is_classification(&self) -> bool {
        matches!(
            self,
            TrainingMethod::LogisticRegression
                | TrainingMethod::RandomForestClassification
                | TrainingMethod::MLPClassification
        )
    }

    /// Check if this is a regression method
    pub fn is_regression(&self) -> bool {
        matches!(
            self,
            TrainingMethod::LinearRegression | TrainingMethod::RandomForestRegression
        )
    }
}

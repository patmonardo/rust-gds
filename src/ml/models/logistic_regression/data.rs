use crate::ml::{
    core::{
        dimensions::Dimensions,
        functions::weights::Weights,
        tensor::{Matrix, Vector},
    },
    TrainingMethod,
};
use serde::{Deserialize, Serialize};

/// Data structure holding the parameters of a logistic regression model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogisticRegressionData {
    weights: Weights<Matrix>,
    bias: Weights<Vector>,
    number_of_classes: usize,
}

impl LogisticRegressionData {
    /// Creates a standard logistic regression model
    pub fn standard(feature_count: usize, number_of_classes: usize) -> Self {
        Self::create(number_of_classes, feature_count, false)
    }

    /// Creates a logistic regression model with reduced class count
    /// This is an optimization where we "virtually" add a weight of 0.0 for the last class
    pub fn with_reduced_class_count(feature_count: usize, number_of_classes: usize) -> Self {
        Self::create(number_of_classes, feature_count, true)
    }

    fn create(class_count: usize, feature_count: usize, skip_last_class: bool) -> Self {
        let rows = if skip_last_class {
            class_count - 1
        } else {
            class_count
        };

        let weights = Weights::of_matrix(rows, feature_count);
        let bias = Weights::new(Vector::zeros(rows));

        Self {
            weights,
            bias,
            number_of_classes: class_count,
        }
    }

    /// Returns the weights matrix
    pub fn weights(&self) -> &Weights<Matrix> {
        &self.weights
    }

    /// Returns the bias vector
    pub fn bias(&self) -> &Weights<Vector> {
        &self.bias
    }

    /// Returns the number of classes
    pub fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }

    /// Returns the feature dimension
    pub fn feature_dimension(&self) -> usize {
        self.weights.dimension(Dimensions::COLUMNS_INDEX)
    }

    /// Returns the training method
    pub fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::LogisticRegression
    }
}

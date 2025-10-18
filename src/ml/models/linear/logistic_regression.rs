// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Logistic Regression implementation.
//!
//! 1:1 translation of LogisticRegressionClassifier.java from Java GDS.

use crate::ml::core::tensor::{Matrix, Vector};
use crate::ml::models::{
    BaseModelData, Classifier, ClassifierData, Features, ModelData, TrainingMethod,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Logistic regression classifier.
/// 1:1 translation of LogisticRegressionClassifier.java from Java GDS.
#[derive(Debug)]
pub struct LogisticRegressionClassifier {
    data: LogisticRegressionData,
}

impl LogisticRegressionClassifier {
    pub fn new(data: LogisticRegressionData) -> Self {
        Self { data }
    }

    pub fn weights(&self) -> &Matrix {
        &self.data.weights
    }

    pub fn bias(&self) -> &Vector {
        &self.data.bias
    }
}

impl Classifier for LogisticRegressionClassifier {
    fn data(&self) -> &dyn ClassifierData {
        &self.data
    }

    /// Predict class probabilities for a single feature vector
    /// 1:1 with Classifier.predictProbabilities(double[]) in Java
    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        if features.len() != self.data.num_features {
            panic!(
                "Feature dimension mismatch: got {}, expected {}",
                features.len(),
                self.data.num_features
            );
        }

        let num_classes = self.data.num_classes;
        let mut logits = vec![0.0; num_classes];

        // Calculate logits: logits[c] = weights[c] · features + bias[c]
        for c in 0..num_classes {
            let mut logit = self.data.bias[c];
            for i in 0..self.data.num_features {
                logit += self.data.weights[(c, i)] * features[i];
            }
            logits[c] = logit;
        }

        // Stabilized softmax computation
        let max_logit = logits.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let mut probs = vec![0.0; num_classes];

        let mut sum = 0.0;
        for c in 0..num_classes {
            probs[c] = (logits[c] - max_logit).exp();
            sum += probs[c];
        }

        // Normalize to get probabilities
        if sum > 0.0 {
            probs.iter_mut().for_each(|p| *p /= sum);
        }

        probs
    }

    /// Predict class probabilities for a batch of features
    /// 1:1 with Classifier.predictProbabilities(Batch, Features) in Java
    fn predict_probabilities_batch(&self, batch: &[usize], features: &dyn Features) -> Matrix {
        let batch_size = batch.len();
        let num_classes = self.data.num_classes;
        let mut result = Matrix::zeros(batch_size, num_classes);

        for (i, &sample_id) in batch.iter().enumerate() {
            let feature_vec = features.get(sample_id);
            let probs = self.predict_probabilities(feature_vec);
            for c in 0..num_classes {
                result[(i, c)] = probs[c];
            }
        }

        result
    }
}

/// Logistic Regression Model Data.
/// 1:1 translation of LogisticRegressionData.java from Java GDS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticRegressionData {
    pub weights: Matrix, // Shape: (num_classes, num_features)
    pub bias: Vector,    // Shape: (num_classes,)
    pub num_classes: usize,
    pub num_features: usize,
}

impl BaseModelData for LogisticRegressionData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::LogisticRegression
    }

    fn feature_dimension(&self) -> usize {
        self.num_features
    }
}

impl ClassifierData for LogisticRegressionData {
    fn number_of_classes(&self) -> usize {
        self.num_classes
    }
}

impl LogisticRegressionData {
    /// Create new LogisticRegressionData.
    pub fn new(num_classes: usize, num_features: usize) -> Self {
        Self {
            weights: Matrix::zeros(num_classes, num_features),
            bias: Vector::zeros(num_classes),
            num_classes,
            num_features,
        }
    }
}

impl ModelData for LogisticRegressionData {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization failed: {}", e))
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization failed: {}", e))
    }

    fn num_features(&self) -> usize {
        self.num_features
    }
}

/// Logistic Regression Training Configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticRegressionConfig {
    /// L2 regularization penalty
    #[serde(default)]
    pub penalty: f64,

    /// Maximum number of iterations
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Learning rate
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f64,

    /// Batch size for gradient descent
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

impl Default for LogisticRegressionConfig {
    fn default() -> Self {
        Self {
            penalty: 0.0,
            max_iterations: default_max_iterations(),
            learning_rate: default_learning_rate(),
            batch_size: default_batch_size(),
        }
    }
}

fn default_max_iterations() -> usize {
    100
}

fn default_learning_rate() -> f64 {
    0.001
}

fn default_batch_size() -> usize {
    100
}

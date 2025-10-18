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

//! Random Forest Classifier implementation.
//!
//! 1:1 translation of RandomForestClassifier.java from Java GDS.

use crate::ml::core::tensor::Matrix;
use crate::ml::models::{BaseModelData, Classifier, ClassifierData, Features, TrainingMethod};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::config::RandomForestConfig;

/// Trait for decision tree predictors.
/// 1:1 translation of DecisionTreePredictor<T> interface from Java GDS.
pub trait DecisionTreePredictor<T>: Send + Sync {
    fn predict(&self, features: &[f64]) -> T;
}

/// Random forest classifier model.
/// 1:1 translation of RandomForestClassifier.java from Java GDS.
pub struct RandomForestClassifier {
    data: RandomForestClassifierData,
}

impl RandomForestClassifier {
    pub fn new(data: RandomForestClassifierData) -> Self {
        Self { data }
    }

    pub fn num_trees(&self) -> usize {
        self.data.decision_trees.len()
    }

    /// Helper method to gather tree predictions (votes per class)
    /// 1:1 with gatherTreePredictions() in Java
    fn gather_tree_predictions(&self, features: &[f64]) -> Vec<i32> {
        let mut predictions_per_class = vec![0i32; self.data.number_of_classes()];

        for tree in &self.data.decision_trees {
            let predicted_class = tree.predict(features);
            predictions_per_class[predicted_class] += 1;
        }

        predictions_per_class
    }
}

impl Classifier for RandomForestClassifier {
    fn data(&self) -> &dyn ClassifierData {
        &self.data
    }

    /// Predict class probabilities for a single feature vector
    /// 1:1 with predictProbabilities(double[] features) in Java
    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        let votes_per_class = self.gather_tree_predictions(features);
        let number_of_trees = self.data.decision_trees.len() as f64;

        let mut probabilities = vec![0.0; self.data.number_of_classes()];

        for (class_idx, &vote_for_class) in votes_per_class.iter().enumerate() {
            probabilities[class_idx] = vote_for_class as f64 / number_of_trees;
        }

        probabilities
    }

    /// Predict class probabilities for a batch of features
    /// 1:1 with predictProbabilities(Batch batch, Features features) in Java
    fn predict_probabilities_batch(&self, batch: &[usize], features: &dyn Features) -> Matrix {
        let mut predicted_probabilities = Matrix::zeros(batch.len(), self.data.number_of_classes());

        for (offset, &id) in batch.iter().enumerate() {
            let feature_vec = features.get(id);
            let probabilities = self.predict_probabilities(feature_vec);
            for (class_idx, probability) in probabilities.iter().enumerate() {
                predicted_probabilities[(offset, class_idx)] = *probability;
            }
        }

        predicted_probabilities
    }
}

/// Random Forest Classifier Data.
/// 1:1 translation of RandomForestClassifierData.java from Java GDS.
///
/// NOTE: Serialization deferred - trait objects require special handling
pub struct RandomForestClassifierData {
    pub decision_trees: Vec<Box<dyn DecisionTreePredictor<usize>>>,
    pub num_classes: usize,
    pub num_features: usize,
}

impl fmt::Debug for RandomForestClassifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomForestClassifier")
            .field("num_trees", &self.data.decision_trees.len())
            .field("num_classes", &self.data.num_classes)
            .field("num_features", &self.data.num_features)
            .finish()
    }
}

impl fmt::Debug for RandomForestClassifierData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomForestClassifierData")
            .field("num_trees", &self.decision_trees.len())
            .field("num_classes", &self.num_classes)
            .field("num_features", &self.num_features)
            .finish()
    }
}

impl BaseModelData for RandomForestClassifierData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::RandomForestClassification
    }

    fn feature_dimension(&self) -> usize {
        self.num_features
    }
}

impl ClassifierData for RandomForestClassifierData {
    fn number_of_classes(&self) -> usize {
        self.num_classes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestClassifierConfig {
    #[serde(flatten)]
    pub forest: RandomForestConfig,

    /// Number of classes
    pub num_classes: usize,

    /// Optional class weights for imbalanced problems
    #[serde(default)]
    pub class_weights: Option<Vec<f64>>,
}

//! Parallel node classifier for batch prediction
//! 1:1 translation of ParallelNodeClassifier.java

use crate::{
    collections::{HugeLongArray, HugeObjectArray},
    ml::models::{Classifier, Features},
};
use std::sync::Arc;

/// Parallel node classifier that can process predictions in batches
/// 1:1 translation of ParallelNodeClassifier.java
pub struct ParallelNodeClassifier {
    classifier: Arc<dyn Classifier>,
    features: Arc<dyn Features>,
    batch_size: usize,
}

impl ParallelNodeClassifier {
    /// Creates a new ParallelNodeClassifier
    /// Simplified constructor matching Java's ParallelNodeClassifier
    pub fn new(
        classifier: Arc<dyn Classifier>,
        features: Arc<dyn Features>,
        batch_size: usize,
    ) -> Self {
        Self {
            classifier,
            features,
            batch_size,
        }
    }

    /// Predicts classes for an evaluation set
    /// 1:1 with predict(ReadOnlyHugeLongArray) in Java
    pub fn predict(&self, evaluation_set: &[u64]) -> HugeLongArray {
        self.predict_internal(evaluation_set.len(), |i| evaluation_set[i], None)
    }

    /// Predicts with optional probabilities output
    /// 1:1 with predict(HugeObjectArray<double[]>) in Java
    pub fn predict_with_probabilities(
        &self,
        predicted_probabilities: Option<&mut HugeObjectArray<Vec<f64>>>,
    ) -> HugeLongArray {
        let size = self.features.size();
        self.predict_internal(size, |i| i as u64, predicted_probabilities)
    }

    /// Internal prediction method
    /// 1:1 with private predict() in Java
    fn predict_internal(
        &self,
        evaluation_set_size: usize,
        node_id_mapper: impl Fn(usize) -> u64,
        mut predicted_probabilities: Option<&mut HugeObjectArray<Vec<f64>>>,
    ) -> HugeLongArray {
        let mut predicted_classes = HugeLongArray::new(evaluation_set_size);

        // Process in batches
        for batch_start in (0..evaluation_set_size).step_by(self.batch_size) {
            let batch_end = (batch_start + self.batch_size).min(evaluation_set_size);

            for i in batch_start..batch_end {
                let node_id = node_id_mapper(i) as usize;
                let feature_vec = self.features.get(node_id);
                let probs = self.classifier.predict_probabilities(feature_vec);

                // Store probabilities if requested
                if let Some(prob_array) = predicted_probabilities.as_mut() {
                    prob_array.set(node_id, probs.clone());
                }

                // Find class with max probability
                let best_class = probs
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(idx, _)| idx as i64)
                    .unwrap_or(0);

                predicted_classes.set(i, best_class);
            }
        }

        predicted_classes
    }
}

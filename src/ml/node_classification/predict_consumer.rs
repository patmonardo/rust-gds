//! Consumer for node classification predictions
//! 1:1 translation of NodeClassificationPredictConsumer.java

use crate::{
    collections::{HugeLongArray, HugeObjectArray},
    ml::models::{Classifier, Features},
};
use std::sync::{Arc, Mutex};

/// Consumer for node classification predictions
/// 1:1 translation of NodeClassificationPredictConsumer.java
pub struct NodeClassificationPredictConsumer {
    features: Arc<dyn Features>,
    classifier: Arc<dyn Classifier>,
    predicted_probabilities: Option<Arc<Mutex<HugeObjectArray<Vec<f64>>>>>,
    predicted_classes: Arc<Mutex<HugeLongArray>>,
}

impl NodeClassificationPredictConsumer {
    /// Creates a new NodeClassificationPredictConsumer
    pub fn new(
        features: Arc<dyn Features>,
        classifier: Arc<dyn Classifier>,
        predicted_probabilities: Option<Arc<Mutex<HugeObjectArray<Vec<f64>>>>>,
        predicted_classes: Arc<Mutex<HugeLongArray>>,
    ) -> Self {
        Self {
            features,
            classifier,
            predicted_probabilities,
            predicted_classes,
        }
    }

    /// Accepts a batch for processing
    /// 1:1 with accept(Batch) in Java
    pub fn accept(&self, batch_start: usize, batch_end: usize, node_ids: &[u64]) {
        let number_of_classes = self.classifier.number_of_classes();

        // Create batch indices for this batch
        let batch_indices: Vec<usize> = (batch_start..batch_end)
            .map(|i| node_ids[i] as usize)
            .collect();

        // Get probability matrix for entire batch
        let probability_matrix = self
            .classifier
            .predict_probabilities_batch(&batch_indices, &*self.features);

        // Process each element in the batch
        for (row, &node_id) in batch_indices.iter().enumerate() {
            // Find best class for current row
            let mut best_class = 0;
            let mut max_prob = probability_matrix[(row, 0)];

            for class in 1..number_of_classes {
                let prob = probability_matrix[(row, class)];
                if prob > max_prob {
                    max_prob = prob;
                    best_class = class;
                }
            }

            // Store results
            let mut predicted_classes = self
                .predicted_classes
                .lock()
                .expect("predicted_classes mutex poisoned");
            predicted_classes.set(node_id, best_class as i64);

            if let Some(ref probs) = self.predicted_probabilities {
                let mut class_probs = vec![0.0; number_of_classes];
                for class in 0..number_of_classes {
                    class_probs[class] = probability_matrix[(row, class)];
                }
                let mut probabilities = probs
                    .lock()
                    .expect("predicted_probabilities mutex poisoned");
                probabilities.set(node_id, class_probs);
            }
        }
    }
}

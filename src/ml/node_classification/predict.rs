//! Node classification prediction
//! 1:1 translation of NodeClassificationPredict.java

use crate::{
    collections::{HugeLongArray, HugeObjectArray},
    ml::models::{Classifier, Features},
};
use std::{fmt, sync::Arc};

/// Result of node classification prediction
/// 1:1 with NodeClassificationResult in Java
pub struct NodeClassificationPredictResult {
    predicted_classes: Arc<HugeLongArray>,
    predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
}

impl fmt::Debug for NodeClassificationPredictResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeClassificationPredictResult")
            .field("predicted_classes_len", &self.predicted_classes.size())
            .field(
                "probabilities_size",
                &self.predicted_probabilities.as_ref().map(|arr| arr.size()),
            )
            .finish()
    }
}

impl NodeClassificationPredictResult {
    /// Creates a new NodeClassificationPredictResult
    pub fn new(
        predicted_classes: Arc<HugeLongArray>,
        predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
    ) -> Self {
        Self {
            predicted_classes,
            predicted_probabilities,
        }
    }

    /// Returns the predicted classes
    pub fn predicted_classes(&self) -> &Arc<HugeLongArray> {
        &self.predicted_classes
    }

    /// Returns the predicted probabilities if available
    pub fn predicted_probabilities(&self) -> Option<&Arc<HugeObjectArray<Vec<f64>>>> {
        self.predicted_probabilities.as_ref()
    }
}

/// Node classification prediction algorithm
/// 1:1 translation of NodeClassificationPredict.java
pub struct NodeClassificationPredict {
    classifier: Arc<dyn Classifier>,
    features: Arc<dyn Features>,
    batch_size: usize,
    produce_probabilities: bool,
}

impl NodeClassificationPredict {
    /// Creates a new NodeClassificationPredict
    /// Simplified constructor matching Java's NodeClassificationPredict
    pub fn new(
        classifier: Arc<dyn Classifier>,
        features: Arc<dyn Features>,
        batch_size: usize,
        produce_probabilities: bool,
    ) -> Self {
        Self {
            classifier,
            features,
            batch_size,
            produce_probabilities,
        }
    }

    /// Computes predictions for all nodes
    /// 1:1 with compute() in Java
    pub fn compute(&self) -> NodeClassificationPredictResult {
        let node_count = self.features.size();

        // Initialize probabilities if requested
        let mut predicted_probabilities = if self.produce_probabilities {
            let num_classes = self.classifier.number_of_classes();
            let mut predictions = HugeObjectArray::new(node_count);
            // Initialize with zero vectors
            for i in 0..node_count {
                predictions.set(i, vec![0.0; num_classes]);
            }
            Some(predictions)
        } else {
            None
        };

        let parallel_classifier = super::parallel_classifier::ParallelNodeClassifier::new(
            self.classifier.clone(),
            self.features.clone(),
            self.batch_size,
        );

        // Predict both classes and probabilities in a single pass
        let predicted_classes =
            parallel_classifier.predict_with_probabilities(predicted_probabilities.as_mut());

        let predicted_probabilities = predicted_probabilities.map(Arc::new);

        NodeClassificationPredictResult::new(Arc::new(predicted_classes), predicted_probabilities)
    }
}

/// Memory estimation for node classification prediction
/// 1:1 with memoryEstimation() in Java
pub fn estimate_predict_memory(
    node_count: usize,
    predict_probabilities: bool,
    number_of_classes: usize,
) -> usize {
    let mut memory = std::mem::size_of::<i64>() * node_count; // predicted classes

    if predict_probabilities {
        memory += std::mem::size_of::<f64>() * node_count * number_of_classes; // probabilities
    }

    memory
}

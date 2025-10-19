use crate::{
    collections::HugeLongArray,
    ml::{
        metrics::classification::ClassificationMetric,
        models::{Classifier, Features},
    },
};
use std::sync::Arc;

/// Computer for classification metrics
/// 1:1 translation of ClassificationMetricComputer.java
pub struct ClassificationMetricComputer {
    predicted_classes: Arc<HugeLongArray>,
    labels: Arc<HugeLongArray>,
}

impl ClassificationMetricComputer {
    /// Creates a new instance from predicted classes and actual labels
    pub fn new(predicted_classes: Arc<HugeLongArray>, labels: Arc<HugeLongArray>) -> Self {
        Self {
            predicted_classes,
            labels,
        }
    }

    /// Creates a new instance for evaluating metrics on a validation set
    /// 1:1 with ClassificationMetricComputer.forEvaluationSet() in Java
    pub fn for_evaluation_set(
        features: Arc<dyn Features>,
        labels: Arc<HugeLongArray>,
        evaluation_set: Arc<Vec<u64>>, // ReadOnlyHugeLongArray
        classifier: Arc<dyn Classifier>,
    ) -> Self {
        // Predict classes for evaluation set
        let predictor = ParallelNodeClassifier::new(classifier, features, 100);

        let predicted_classes = predictor.predict(&evaluation_set);
        let local_labels = Self::make_local_targets(&evaluation_set, &labels);

        Self {
            predicted_classes: Arc::new(predicted_classes),
            labels: Arc::new(local_labels),
        }
    }

    /// Computes a score using the given metric
    /// 1:1 with score() in Java
    pub fn score(&self, metric: &dyn ClassificationMetric) -> f64 {
        // Compute metric directly on predicted vs actual
        metric.compute(&self.labels, &self.predicted_classes)
    }

    /// Make local targets array aligned with evaluation set
    /// 1:1 with makeLocalTargets() in Java
    fn make_local_targets(node_ids: &[u64], targets: &HugeLongArray) -> HugeLongArray {
        let mut local_targets = HugeLongArray::new(node_ids.len());
        for (i, &node_id) in node_ids.iter().enumerate() {
            local_targets.set(i, targets.get(node_id as usize));
        }
        local_targets
    }
}

/// Simple parallel classifier stub (minimal implementation)
/// 
/// Note: Currently processes nodes one-by-one for simplicity.
/// The `_batch_size` field is reserved for future batch processing optimization.
struct ParallelNodeClassifier {
    classifier: Arc<dyn Classifier>,
    features: Arc<dyn Features>,
    _batch_size: usize, // Reserved for future batch processing implementation
}

impl ParallelNodeClassifier {
    fn new(
        classifier: Arc<dyn Classifier>,
        features: Arc<dyn Features>,
        batch_size: usize,
    ) -> Self {
        Self {
            classifier,
            features,
            _batch_size: batch_size,
        }
    }

    fn predict(&self, evaluation_set: &[u64]) -> HugeLongArray {
        let mut predictions = HugeLongArray::new(evaluation_set.len());

        // TODO: Implement batch processing using self._batch_size for better performance
        // Currently processing nodes one-by-one for simplicity
        for (i, &node_id) in evaluation_set.iter().enumerate() {
            let feature_vec = self.features.get(node_id as usize);
            let probs = self.classifier.predict_probabilities(feature_vec);

            // Find class with max probability
            let predicted_class = probs
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx as i64)
                .unwrap_or(0);

            predictions.set(i, predicted_class);
        }

        predictions
    }
}

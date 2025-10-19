// Phase 1.5: LinkPredictionTrainResult - Training output for link prediction

use std::marker::PhantomData;

// TODO: Replace with real types when available
pub type Classifier = PhantomData<()>;
pub type TrainingStatistics = PhantomData<()>;

/// Result of link prediction model training.
///
/// Contains:
/// - **Classifier**: Trained binary classification model (LogisticRegression, RandomForest, etc.)
/// - **TrainingStatistics**: Training metrics, convergence history, hyperparameters
///
/// # Link Prediction Training
///
/// Link prediction trains a binary classifier to distinguish:
/// - Positive examples: Existing relationships (label = 1)
/// - Negative examples: Non-existent relationships from negative sampling (label = 0)
///
/// The classifier learns from link features (Hadamard, Cosine, L2, etc.) computed
/// on node property pairs.
///
/// # Example Flow
///
/// ```text
/// LinkPredictionTrain::compute()
///   ↓
/// 1. Extract features & labels → FeaturesAndLabels
/// 2. Train classifier → Classifier
/// 3. Track statistics → TrainingStatistics
/// 4. Return → LinkPredictionTrainResult
/// ```
#[derive(Debug, Clone)]
pub struct LinkPredictionTrainResult {
    /// Trained binary classifier
    classifier: Classifier,

    /// Training metrics and convergence history
    training_statistics: TrainingStatistics,
}

impl LinkPredictionTrainResult {
    /// Creates a new LinkPredictionTrainResult.
    ///
    /// # Arguments
    ///
    /// * `classifier` - Trained binary classification model
    /// * `training_statistics` - Training metrics and history
    pub fn new(classifier: Classifier, training_statistics: TrainingStatistics) -> Self {
        Self {
            classifier,
            training_statistics,
        }
    }

    /// Returns the trained classifier.
    pub fn classifier(&self) -> &Classifier {
        &self.classifier
    }

    /// Returns the training statistics.
    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_result_creation() {
        let classifier = PhantomData;
        let stats = PhantomData;
        let result = LinkPredictionTrainResult::new(classifier, stats);

        let _classifier = result.classifier();
        let _stats = result.training_statistics();
    }

    #[test]
    fn test_accessors() {
        let result = LinkPredictionTrainResult::new(PhantomData, PhantomData);

        assert!(result.classifier().is::<()>());
        assert!(result.training_statistics().is::<()>());
    }

    #[test]
    fn test_clone() {
        let result1 = LinkPredictionTrainResult::new(PhantomData, PhantomData);
        let result2 = result1.clone();

        // Both accessible after clone
        let _c1 = result1.classifier();
        let _c2 = result2.classifier();
    }

    #[test]
    fn test_multiple_results() {
        // Simulating multiple training runs (e.g., hyperparameter search)
        let results: Vec<LinkPredictionTrainResult> = (0..5)
            .map(|_| LinkPredictionTrainResult::new(PhantomData, PhantomData))
            .collect();

        assert_eq!(results.len(), 5);
    }
}

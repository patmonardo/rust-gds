/// Training executor for ML pipelines.
///
/// Orchestrates model training with hyperparameter search and model selection.
/// Node-centric: trains models to predict node properties/labels.
use crate::projection::codegen::descriptors::ml::{ModelCandidate, ValidationMetric};
use crate::projection::eval::ml::pipeline_state::DatasetSplits;
use crate::types::properties::PropertyValues;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Errors during training.
#[derive(Debug)]
pub enum TrainingError {
    NoCandidates,
    NoFeatures,
    TargetNotFound(String),
    TrainingFailed(String),
    EvaluationFailed(String),
}

impl fmt::Display for TrainingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainingError::NoCandidates => write!(f, "No model candidates provided"),
            TrainingError::NoFeatures => write!(f, "No features provided"),
            TrainingError::TargetNotFound(s) => write!(f, "Target property not found: {}", s),
            TrainingError::TrainingFailed(s) => write!(f, "Training failed: {}", s),
            TrainingError::EvaluationFailed(s) => write!(f, "Evaluation failed: {}", s),
        }
    }
}

impl std::error::Error for TrainingError {}

/// Result of training a single model candidate.
#[derive(Debug, Clone)]
pub struct TrainingResult {
    pub candidate: ModelCandidate,
    pub validation_score: f64,
    pub training_time_ms: u64,
}

/// Training statistics for model selection.
///
/// Tracks validation scores for all model candidates.
/// Node-centric: scores based on node prediction accuracy.
#[derive(Debug, Default, Clone)]
pub struct TrainingStatistics {
    results: Vec<TrainingResult>,
}

impl TrainingStatistics {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Record training result for a candidate.
    pub fn record(&mut self, candidate: ModelCandidate, score: f64, time_ms: u64) {
        self.results.push(TrainingResult {
            candidate,
            validation_score: score,
            training_time_ms: time_ms,
        });
    }

    /// Get best model candidate based on validation score.
    pub fn best_candidate(&self) -> Result<&ModelCandidate, TrainingError> {
        self.results
            .iter()
            .max_by(|a, b| {
                a.validation_score
                    .partial_cmp(&b.validation_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| &r.candidate)
            .ok_or(TrainingError::NoCandidates)
    }

    /// Get best validation score.
    pub fn best_score(&self) -> Option<f64> {
        self.results
            .iter()
            .map(|r| r.validation_score)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Get all training results.
    pub fn results(&self) -> &[TrainingResult] {
        &self.results
    }

    /// Get number of candidates trained.
    pub fn candidate_count(&self) -> usize {
        self.results.len()
    }
}

/// Training executor for ML pipelines.
///
/// Orchestrates training of multiple model candidates, evaluates on validation set,
/// and selects best model based on specified metric.
///
/// # Node-Centric Design
///
/// - Features: HashMap of node properties (property name → PropertyValues)
/// - Target: Node property to predict (e.g., "label", "class")
/// - Splits: Train/validation/test node indices
/// - Models: Predict node properties from node features
///
/// # Examples
///
/// ```ignore
/// let candidates = vec![
///     ModelCandidate::new("LogisticRegression", params),
/// ];
///
/// let mut executor = TrainingExecutor::new(candidates, ValidationMetric::Accuracy);
/// let model = executor.train(&node_features, "label", &splits)?;
/// ```
pub struct TrainingExecutor {
    model_candidates: Vec<ModelCandidate>,
    metric: ValidationMetric,
    statistics: TrainingStatistics,
}

impl TrainingExecutor {
    /// Create new training executor.
    ///
    /// # Arguments
    ///
    /// * `model_candidates` - List of model configurations to train
    /// * `metric` - Validation metric for model selection
    pub fn new(model_candidates: Vec<ModelCandidate>, metric: ValidationMetric) -> Self {
        Self {
            model_candidates,
            metric,
            statistics: TrainingStatistics::new(),
        }
    }

    /// Train all model candidates and select best.
    ///
    /// # Node-Centric Training Flow
    ///
    /// 1. Extract train/validation node indices from splits
    /// 2. Get target node labels
    /// 3. For each model candidate:
    ///    - Train on training nodes
    ///    - Evaluate on validation nodes
    ///    - Record validation score
    /// 4. Select best candidate based on validation scores
    /// 5. Return best model (placeholder for Phase 2.3)
    ///
    /// # Arguments
    ///
    /// * `features` - Node features (property name → PropertyValues)
    /// * `target` - Target property name to predict
    /// * `splits` - Train/validation/test node splits
    ///
    /// # Phase 2.3 Implementation
    ///
    /// Returns mock validation score. Full model training in Phase 2.5.
    pub fn train(
        &mut self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &str,
        splits: &DatasetSplits,
    ) -> Result<TrainingStatistics, TrainingError> {
        if self.model_candidates.is_empty() {
            return Err(TrainingError::NoCandidates);
        }

        if features.is_empty() {
            return Err(TrainingError::NoFeatures);
        }

        // Verify target exists
        if !features.contains_key(target) {
            return Err(TrainingError::TargetNotFound(target.to_string()));
        }

        // TODO Phase 2.5: Implement actual model training
        // For Phase 2.3: Just simulate training and record mock scores
        for (idx, candidate) in self.model_candidates.iter().enumerate() {
            // Simulate training time
            let training_time_ms = 10 * (idx as u64 + 1);

            // Mock validation score (Phase 2.3)
            // In Phase 2.5: Actually train model and evaluate
            let validation_score = 0.8 + (idx as f64 * 0.02); // Increasing scores

            self.statistics
                .record(candidate.clone(), validation_score, training_time_ms);
        }

        Ok(self.statistics.clone())
    }

    /// Get training statistics.
    pub fn statistics(&self) -> &TrainingStatistics {
        &self.statistics
    }

    /// Get validation metric being used.
    pub fn metric(&self) -> &ValidationMetric {
        &self.metric
    }

    /// Get model candidates.
    pub fn candidates(&self) -> &[ModelCandidate] {
        &self.model_candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::MockDoublePropertyValues;

    fn create_test_features() -> HashMap<String, Arc<dyn PropertyValues>> {
        let mut features = HashMap::new();
        features.insert(
            "pagerank".to_string(),
            Arc::new(MockDoublePropertyValues::from_vec(vec![
                0.1, 0.2, 0.3, 0.4, 0.5,
            ])) as Arc<dyn PropertyValues>,
        );
        features.insert(
            "label".to_string(),
            Arc::new(MockDoublePropertyValues::from_vec(vec![
                0.0, 1.0, 0.0, 1.0, 1.0,
            ])) as Arc<dyn PropertyValues>,
        );
        features
    }

    fn create_test_splits() -> DatasetSplits {
        DatasetSplits::from_fractions(&(0..5).collect::<Vec<_>>(), 0.6, 0.2, 0.2, 42)
    }

    fn create_test_candidate(
        model_type: crate::projection::codegen::descriptors::ml::ModelType,
    ) -> ModelCandidate {
        ModelCandidate {
            model_type,
            params: HashMap::new(),
        }
    }

    #[test]
    fn test_training_executor_creation() {
        use crate::projection::codegen::descriptors::ml::ModelType;
        let candidates = vec![create_test_candidate(ModelType::LogisticRegression)];
        let executor = TrainingExecutor::new(candidates, ValidationMetric::Accuracy);

        assert_eq!(executor.candidates().len(), 1);
        assert_eq!(executor.metric(), &ValidationMetric::Accuracy);
    }

    #[test]
    fn test_train_single_candidate() {
        use crate::projection::codegen::descriptors::ml::ModelType;
        let features = create_test_features();
        let splits = create_test_splits();
        let candidates = vec![create_test_candidate(ModelType::LogisticRegression)];

        let mut executor = TrainingExecutor::new(candidates, ValidationMetric::Accuracy);
        let result = executor.train(&features, "label", &splits);

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.candidate_count(), 1);
        assert!(stats.best_score().is_some());
    }

    #[test]
    fn test_train_multiple_candidates() {
        use crate::projection::codegen::descriptors::ml::ModelType;
        let features = create_test_features();
        let splits = create_test_splits();
        let candidates = vec![
            create_test_candidate(ModelType::LogisticRegression),
            create_test_candidate(ModelType::RandomForest),
            create_test_candidate(ModelType::DecisionTreeClassifier),
        ];

        let mut executor = TrainingExecutor::new(candidates, ValidationMetric::F1);
        let result = executor.train(&features, "label", &splits);

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.candidate_count(), 3);

        // Best candidate should be last (highest mock score)
        let best = stats.best_candidate().unwrap();
        assert_eq!(best.model_type, ModelType::DecisionTreeClassifier);
    }

    #[test]
    fn test_train_no_candidates() {
        let features = create_test_features();
        let splits = create_test_splits();

        let mut executor = TrainingExecutor::new(vec![], ValidationMetric::Accuracy);
        let result = executor.train(&features, "label", &splits);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TrainingError::NoCandidates));
    }

    #[test]
    fn test_train_no_features() {
        use crate::projection::codegen::descriptors::ml::ModelType;
        let splits = create_test_splits();
        let candidates = vec![create_test_candidate(ModelType::LogisticRegression)];

        let mut executor = TrainingExecutor::new(candidates, ValidationMetric::Accuracy);
        let result = executor.train(&HashMap::new(), "label", &splits);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TrainingError::NoFeatures));
    }

    #[test]
    fn test_train_target_not_found() {
        use crate::projection::codegen::descriptors::ml::ModelType;
        let features = create_test_features();
        let splits = create_test_splits();
        let candidates = vec![create_test_candidate(ModelType::LogisticRegression)];

        let mut executor = TrainingExecutor::new(candidates, ValidationMetric::Accuracy);
        let result = executor.train(&features, "missing_target", &splits);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TrainingError::TargetNotFound(_)
        ));
    }

    #[test]
    fn test_training_statistics() {
        use crate::projection::codegen::descriptors::ml::ModelType;
        let mut stats = TrainingStatistics::new();

        let candidate1 = create_test_candidate(ModelType::LogisticRegression);
        let candidate2 = create_test_candidate(ModelType::RandomForest);

        stats.record(candidate1, 0.75, 100);
        stats.record(candidate2, 0.85, 150);

        assert_eq!(stats.candidate_count(), 2);
        assert_eq!(stats.best_score(), Some(0.85));

        let best = stats.best_candidate().unwrap();
        assert_eq!(best.model_type, ModelType::RandomForest);
    }
}

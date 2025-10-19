// Phase 1.6: LinkPredictionModelInfo - Model metadata for link prediction

use std::collections::HashMap;
use std::marker::PhantomData;

// TODO: Replace with real types when available
pub type TrainerConfig = PhantomData<()>;
pub type LinkPredictionPredictPipeline = PhantomData<()>;
pub type Metric = PhantomData<()>;
pub type ModelCandidateStats = PhantomData<()>;
pub type TrainingMethod = PhantomData<()>;

/// Custom metadata for Link Prediction models.
///
/// Implements `CustomInfo` trait for `Model<ClassifierData, LinkPredictionTrainConfig, LinkPredictionModelInfo>`.
///
/// Contains:
/// - **Best parameters**: Winning hyperparameters from model selection
/// - **Metrics**: Test and validation metrics (AUCPR, ROC_AUC, etc.)
/// - **Pipeline**: The predict pipeline (node property steps + link feature steps)
///
/// # Model Selection Process
///
/// ```text
/// 1. RandomSearch over TrainerConfig candidates
/// 2. Cross-validation for each candidate → ModelCandidateStats
/// 3. Select best by metric (e.g., AUCPR) → bestParameters
/// 4. Train final model on full train set
/// 5. Evaluate on test set → testMetrics
/// 6. Bundle → LinkPredictionModelInfo
/// ```
///
/// # Usage in Model
///
/// ```text
/// Model::of(
///     gds_version: "2.5.0",
///     model_type: "LinkPrediction",
///     graph_schema: training_graph_schema,
///     data: classifier_data,              // Trained Classifier
///     train_config: link_pred_config,     // LinkPredictionTrainConfig
///     custom_info: model_info,            // LinkPredictionModelInfo (THIS)
/// )
/// ```
#[derive(Debug, Clone)]
pub struct LinkPredictionModelInfo {
    /// Best hyperparameters selected during training
    best_parameters: TrainerConfig,

    /// Rendered metrics (test + outer train) as string map
    metrics: HashMap<String, serde_json::Value>,

    /// Predict pipeline (subset of training pipeline)
    pipeline: LinkPredictionPredictPipeline,
}

impl LinkPredictionModelInfo {
    /// Creates a new LinkPredictionModelInfo.
    ///
    /// Typically called via `of()` static factory which renders metrics from raw values.
    pub fn new(
        best_parameters: TrainerConfig,
        metrics: HashMap<String, serde_json::Value>,
        pipeline: LinkPredictionPredictPipeline,
    ) -> Self {
        Self {
            best_parameters,
            metrics,
            pipeline,
        }
    }

    /// Creates LinkPredictionModelInfo from test/train metrics and best candidate.
    ///
    /// # Arguments
    ///
    /// * `test_metrics` - Metrics on held-out test set
    /// * `outer_train_metrics` - Metrics on outer training set
    /// * `best_candidate` - Best model candidate from hyperparameter search
    /// * `pipeline` - Predict pipeline for this model
    ///
    /// # Returns
    ///
    /// LinkPredictionModelInfo with rendered metrics
    pub fn of(
        _test_metrics: HashMap<Metric, f64>,
        _outer_train_metrics: HashMap<Metric, f64>,
        _best_candidate: ModelCandidateStats,
        pipeline: LinkPredictionPredictPipeline,
    ) -> Self {
        // TODO: Implement renderMetrics when ModelCandidateStats available
        let metrics = HashMap::new();
        let best_parameters = PhantomData;

        Self::new(best_parameters, metrics, pipeline)
    }

    /// Returns the best hyperparameters.
    pub fn best_parameters(&self) -> &TrainerConfig {
        &self.best_parameters
    }

    /// Returns the metrics map.
    pub fn metrics(&self) -> &HashMap<String, serde_json::Value> {
        &self.metrics
    }

    /// Returns the predict pipeline.
    pub fn pipeline(&self) -> &LinkPredictionPredictPipeline {
        &self.pipeline
    }

    /// Converts to map representation for serialization.
    ///
    /// Includes:
    /// - `bestParameters`: Trainer config with method
    /// - `metrics`: Rendered test/train metrics
    /// - `pipeline`: Full pipeline map
    /// - `nodePropertySteps`: Node property steps (legacy)
    /// - `featureSteps`: Link feature steps (legacy)
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        // TODO: Implement proper serialization when types available
        map.insert("bestParameters".to_string(), serde_json::json!({}));
        map.insert("metrics".to_string(), serde_json::json!(self.metrics));
        map.insert("pipeline".to_string(), serde_json::json!({}));
        map.insert("nodePropertySteps".to_string(), serde_json::json!([]));
        map.insert("featureSteps".to_string(), serde_json::json!([]));

        map
    }

    /// Returns the optional training method.
    ///
    /// Extracted from `best_parameters().method()`.
    pub fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        // TODO: Implement when TrainerConfig available
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_creation() {
        let params = PhantomData;
        let metrics = HashMap::new();
        let pipeline = PhantomData;

        let info = LinkPredictionModelInfo::new(params, metrics, pipeline);

        assert!(info.metrics().is_empty());
    }

    #[test]
    fn test_of_factory() {
        let test_metrics = HashMap::new();
        let train_metrics = HashMap::new();
        let best_candidate = PhantomData;
        let pipeline = PhantomData;

        let info =
            LinkPredictionModelInfo::of(test_metrics, train_metrics, best_candidate, pipeline);

        assert!(info.metrics().is_empty()); // Placeholder implementation
    }

    #[test]
    fn test_accessors() {
        let info = LinkPredictionModelInfo::new(PhantomData, HashMap::new(), PhantomData);

        let _params = info.best_parameters();
        let _metrics = info.metrics();
        let _pipeline = info.pipeline();
    }

    #[test]
    fn test_to_map() {
        let info = LinkPredictionModelInfo::new(PhantomData, HashMap::new(), PhantomData);

        let map = info.to_map();

        assert!(map.contains_key("bestParameters"));
        assert!(map.contains_key("metrics"));
        assert!(map.contains_key("pipeline"));
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureSteps"));
    }

    #[test]
    fn test_optional_trainer_method() {
        let info = LinkPredictionModelInfo::new(PhantomData, HashMap::new(), PhantomData);

        assert!(info.optional_trainer_method().is_none()); // Placeholder
    }

    #[test]
    fn test_clone() {
        let info1 = LinkPredictionModelInfo::new(PhantomData, HashMap::new(), PhantomData);
        let info2 = info1.clone();

        assert_eq!(info1.metrics().len(), info2.metrics().len());
    }
}

/// Model trait system for ML pipelines.
///
/// Defines the contract for trained ML models.
/// Node-centric: models operate on node features to predict node properties.
use crate::projection::codegen::ml::ModelType;
use crate::types::properties::PropertyValues;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Errors during model operations.
#[derive(Debug)]
pub enum ModelError {
    NotTrained,
    PredictionFailed(String),
    EvaluationFailed(String),
    InvalidInput(String),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::NotTrained => write!(f, "Model not trained"),
            ModelError::PredictionFailed(s) => write!(f, "Prediction failed: {}", s),
            ModelError::EvaluationFailed(s) => write!(f, "Evaluation failed: {}", s),
            ModelError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
        }
    }
}

impl std::error::Error for ModelError {}

/// Trained ML model.
///
/// # Node-Centric Design
///
/// Models operate on node features to predict node properties:
/// - **fit()**: Train on node features + target labels
/// - **predict()**: Predict target for nodes given features
/// - **predict_proba()**: Predict class probabilities for nodes (classification)
/// - **evaluate()**: Score predictions against actual node labels
///
/// # Examples
///
/// ```ignore
/// // Train classifier on node features
/// let mut model = LogisticRegressionModel::new();
/// model.fit(&node_features, &node_labels, &train_node_ids)?;
///
/// // Predict for test nodes
/// let predictions = model.predict(&node_features, &test_node_ids)?;
///
/// // Evaluate accuracy
/// let score = model.evaluate(&node_features, &node_labels, &test_node_ids)?;
/// ```
pub trait Model: Send + Sync {
    /// Train model on node features.
    ///
    /// # Arguments
    ///
    /// * `features` - Node feature vectors (property name → PropertyValues)
    /// * `target` - Target node labels to predict
    /// * `node_ids` - Training node indices
    ///
    /// # Node-Centric
    ///
    /// Trains model to predict `target` property for nodes based on their `features`.
    fn fit(
        &mut self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &Arc<dyn PropertyValues>,
        node_ids: &[usize],
    ) -> Result<(), ModelError>;

    /// Predict target values for nodes.
    ///
    /// # Arguments
    ///
    /// * `features` - Node features
    /// * `node_ids` - Nodes to predict for
    ///
    /// # Returns
    ///
    /// Predicted values for each node in `node_ids`.
    ///
    /// # Node-Centric
    ///
    /// Given node features, predicts target property value for specified nodes.
    fn predict(
        &self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        node_ids: &[usize],
    ) -> Result<Vec<f64>, ModelError>;

    /// Predict class probabilities for nodes (classification only).
    ///
    /// # Arguments
    ///
    /// * `features` - Node features
    /// * `node_ids` - Nodes to predict for
    ///
    /// # Returns
    ///
    /// Probability matrix: `[node_count x num_classes]`
    ///
    /// # Node-Centric
    ///
    /// For each node, returns probability distribution over target classes.
    fn predict_proba(
        &self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        node_ids: &[usize],
    ) -> Result<Vec<Vec<f64>>, ModelError> {
        // Default: not supported for regression
        Err(ModelError::PredictionFailed(
            "predict_proba not supported for this model type".into(),
        ))
    }

    /// Evaluate model on nodes.
    ///
    /// # Arguments
    ///
    /// * `features` - Node features
    /// * `target` - Actual node labels
    /// * `node_ids` - Evaluation node indices
    ///
    /// # Returns
    ///
    /// Evaluation score (e.g., accuracy, R², etc.)
    ///
    /// # Node-Centric
    ///
    /// Computes metric comparing predicted vs. actual values for nodes.
    fn evaluate(
        &self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &Arc<dyn PropertyValues>,
        node_ids: &[usize],
    ) -> Result<f64, ModelError>;

    /// Check if model is trained.
    fn is_trained(&self) -> bool;

    /// Get model type.
    fn model_type(&self) -> ModelType;

    /// Get model metadata (hyperparameters, metrics, etc.).
    fn metadata(&self) -> ModelMetadata;
}

/// Model metadata.
///
/// Stores information about trained models.
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub model_type: ModelType,
    pub is_trained: bool,
    pub feature_count: usize,
    pub training_nodes: usize,
    pub hyperparameters: HashMap<String, String>,
}

impl ModelMetadata {
    pub fn new(model_type: ModelType) -> Self {
        Self {
            model_type,
            is_trained: false,
            feature_count: 0,
            training_nodes: 0,
            hyperparameters: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::native::form::MockDoublePropertyValues;

    /// Mock model for testing Model trait.
    struct MockModel {
        model_type: ModelType,
        is_trained: bool,
        training_node_count: usize,
    }

    impl MockModel {
        fn new() -> Self {
            Self {
                model_type: ModelType::LogisticRegression,
                is_trained: false,
                training_node_count: 0,
            }
        }
    }

    impl Model for MockModel {
        fn fit(
            &mut self,
            _features: &HashMap<String, Arc<dyn PropertyValues>>,
            _target: &Arc<dyn PropertyValues>,
            node_ids: &[usize],
        ) -> Result<(), ModelError> {
            self.is_trained = true;
            self.training_node_count = node_ids.len();
            Ok(())
        }

        fn predict(
            &self,
            _features: &HashMap<String, Arc<dyn PropertyValues>>,
            node_ids: &[usize],
        ) -> Result<Vec<f64>, ModelError> {
            if !self.is_trained {
                return Err(ModelError::NotTrained);
            }
            // Mock predictions: all 1.0
            Ok(vec![1.0; node_ids.len()])
        }

        fn evaluate(
            &self,
            features: &HashMap<String, Arc<dyn PropertyValues>>,
            _target: &Arc<dyn PropertyValues>,
            node_ids: &[usize],
        ) -> Result<f64, ModelError> {
            if !self.is_trained {
                return Err(ModelError::NotTrained);
            }
            let _predictions = self.predict(features, node_ids)?;
            // Mock accuracy: 0.85
            Ok(0.85)
        }

        fn is_trained(&self) -> bool {
            self.is_trained
        }

        fn model_type(&self) -> ModelType {
            self.model_type.clone()
        }

        fn metadata(&self) -> ModelMetadata {
            ModelMetadata {
                model_type: self.model_type.clone(),
                is_trained: self.is_trained,
                feature_count: 0,
                training_nodes: self.training_node_count,
                hyperparameters: HashMap::new(),
            }
        }
    }

    fn create_test_features() -> HashMap<String, Arc<dyn PropertyValues>> {
        let mut features = HashMap::new();
        features.insert(
            "pagerank".to_string(),
            Arc::new(MockDoublePropertyValues::from_vec(vec![
                0.1, 0.2, 0.3, 0.4, 0.5,
            ])) as Arc<dyn PropertyValues>,
        );
        features
    }

    fn create_test_target() -> Arc<dyn PropertyValues> {
        Arc::new(MockDoublePropertyValues::from_vec(vec![
            0.0, 1.0, 0.0, 1.0, 1.0,
        ])) as Arc<dyn PropertyValues>
    }

    #[test]
    fn test_model_trait_fit() {
        let features = create_test_features();
        let target = create_test_target();
        let node_ids = vec![0, 1, 2];

        let mut model = MockModel::new();
        assert!(!model.is_trained());

        let result = model.fit(&features, &target, &node_ids);
        assert!(result.is_ok());
        assert!(model.is_trained());
    }

    #[test]
    fn test_model_trait_predict() {
        let features = create_test_features();
        let target = create_test_target();
        let node_ids = vec![0, 1, 2];

        let mut model = MockModel::new();

        // Should fail before training
        let result = model.predict(&features, &node_ids);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ModelError::NotTrained));

        // Train and predict
        model.fit(&features, &target, &node_ids).unwrap();
        let predictions = model.predict(&features, &node_ids).unwrap();
        assert_eq!(predictions.len(), 3);
    }

    #[test]
    fn test_model_trait_evaluate() {
        let features = create_test_features();
        let target = create_test_target();
        let node_ids = vec![0, 1, 2];

        let mut model = MockModel::new();
        model.fit(&features, &target, &node_ids).unwrap();

        let score = model.evaluate(&features, &target, &node_ids).unwrap();
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_model_metadata() {
        let metadata = ModelMetadata::new(ModelType::RandomForest);
        assert_eq!(metadata.model_type, ModelType::RandomForest);
        assert!(!metadata.is_trained);
        assert_eq!(metadata.feature_count, 0);
    }

    #[test]
    fn test_model_type() {
        let model = MockModel::new();
        assert_eq!(model.model_type(), ModelType::LogisticRegression);
    }
}

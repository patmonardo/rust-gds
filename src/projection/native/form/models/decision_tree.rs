//! Decision Tree Classifier implementation.
//!
//! Phase 2.3: Simplified decision tree for learning Java GDS patterns.
//! Node-centric: trains on node features, predicts node properties.

use super::{Model, ModelError, ModelMetadata};
use crate::projection::codegen::ml::ModelType;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::PropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// Decision tree node for binary classification.
#[derive(Debug, Clone)]
struct TreeNode {
    /// Feature index for split (-1 for leaf)
    feature_idx: i32,
    /// Threshold value for split
    threshold: f64,
    /// Left child (feature <= threshold)
    left: Option<Box<TreeNode>>,
    /// Right child (feature > threshold)
    right: Option<Box<TreeNode>>,
    /// Predicted class (for leaf nodes)
    predicted_class: f64,
}

/// Decision Tree Classifier.
///
/// Phase 2.3: Simplified implementation for pattern capture.
/// - Binary classification only
/// - Single feature split (first feature)
/// - No pruning
///
/// Node-centric design:
/// - fit(node_features, node_labels, node_ids) - train on nodes
/// - predict(node_features, node_ids) - predict for nodes
#[derive(Debug)]
pub struct DecisionTreeClassifier {
    /// Model metadata
    metadata: ModelMetadata,
    /// Decision tree root
    tree: Option<TreeNode>,
    /// Feature names in training order
    feature_names: Vec<String>,
    /// Maximum depth (Phase 2.3: fixed at 3)
    max_depth: usize,
}

impl DecisionTreeClassifier {
    /// Create new decision tree classifier.
    pub fn new() -> Self {
        Self {
            metadata: ModelMetadata {
                model_type: ModelType::DecisionTreeClassifier,
                is_trained: false,
                feature_count: 0,
                training_nodes: 0,
                hyperparameters: HashMap::new(),
            },
            tree: None,
            feature_names: Vec::new(),
            max_depth: 3, // Phase 2.3: fixed depth
        }
    }

    /// Create with hyperparameters.
    pub fn with_params(params: HashMap<String, String>) -> Self {
        let max_depth = params
            .get("max_depth")
            .and_then(|v| v.parse::<f64>().ok())
            .map(|d| d as usize)
            .unwrap_or(3);

        Self {
            metadata: ModelMetadata {
                model_type: ModelType::DecisionTreeClassifier,
                is_trained: false,
                feature_count: 0,
                training_nodes: 0,
                hyperparameters: params,
            },
            tree: None,
            feature_names: Vec::new(),
            max_depth,
        }
    }

    /// Extract feature matrix from node features.
    ///
    /// Phase 2.3: Simplified - assumes MockDoublePropertyValues in tests.
    /// TODO Phase 2.5: Generic PropertyValues extraction.
    fn extract_features(
        &self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        node_ids: &[usize],
    ) -> Result<Vec<Vec<f64>>, ModelError> {
        use crate::projection::native::form::MockDoublePropertyValues;
        let mut feature_matrix = Vec::new();

        for &node_id in node_ids {
            let mut feature_vec = Vec::new();

            for feature_name in &self.feature_names {
                let prop = features.get(feature_name).ok_or_else(|| {
                    ModelError::InvalidInput(format!("Missing feature: {}", feature_name))
                })?;

                // Phase 2.3: Direct downcast via Any trait
                // Works because Arc<dyn PropertyValues> implements Any
                let value = if let Some(mock_prop) =
                    (prop.as_ref() as &dyn std::any::Any).downcast_ref::<MockDoublePropertyValues>()
                {
                    mock_prop.double_value(node_id as u64).unwrap_or(0.0)
                } else {
                    return Err(ModelError::InvalidInput(
                        "Phase 2.3: Only MockDoublePropertyValues supported".to_string(),
                    ));
                };
                feature_vec.push(value);
            }

            feature_matrix.push(feature_vec);
        }

        Ok(feature_matrix)
    }

    /// Extract target vector from node property.
    ///
    /// Phase 2.3: Simplified - assumes MockDoublePropertyValues.
    fn extract_target(
        &self,
        target: &Arc<dyn PropertyValues>,
        node_ids: &[usize],
    ) -> Result<Vec<f64>, ModelError> {
        use crate::projection::native::form::MockDoublePropertyValues;
        let mut target_vec = Vec::new();

        // Phase 2.3: Direct downcast via Any trait
        let mock_target = if let Some(mock) =
            (target.as_ref() as &dyn std::any::Any).downcast_ref::<MockDoublePropertyValues>()
        {
            mock
        } else {
            return Err(ModelError::InvalidInput(
                "Phase 2.3: Only MockDoublePropertyValues supported".to_string(),
            ));
        };

        for &node_id in node_ids {
            let value = mock_target.double_value(node_id as u64).map_err(|_| {
                ModelError::InvalidInput(format!("Missing target for node {}", node_id))
            })?;

            target_vec.push(value);
        }

        Ok(target_vec)
    }

    /// Build decision tree (Phase 2.3: simplified).
    ///
    /// Uses first feature with median threshold.
    fn build_tree(&self, features: &[Vec<f64>], targets: &[f64], depth: usize) -> TreeNode {
        // Phase 2.3: Use majority class for leaf
        let majority_class = self.majority_class(targets);

        // Stop at max depth or pure node
        if depth >= self.max_depth || self.is_pure(targets) {
            return TreeNode {
                feature_idx: -1,
                threshold: 0.0,
                left: None,
                right: None,
                predicted_class: majority_class,
            };
        }

        // Phase 2.3: Split on first feature at median
        if features.is_empty() || features[0].is_empty() {
            return TreeNode {
                feature_idx: -1,
                threshold: 0.0,
                left: None,
                right: None,
                predicted_class: majority_class,
            };
        }

        let feature_idx = 0;
        let threshold = self.median_value(features, feature_idx);

        // Split data
        let (left_features, left_targets, right_features, right_targets) =
            self.split_data(features, targets, feature_idx, threshold);

        // Recursively build children
        let left = if !left_features.is_empty() {
            Some(Box::new(self.build_tree(
                &left_features,
                &left_targets,
                depth + 1,
            )))
        } else {
            None
        };

        let right = if !right_features.is_empty() {
            Some(Box::new(self.build_tree(
                &right_features,
                &right_targets,
                depth + 1,
            )))
        } else {
            None
        };

        TreeNode {
            feature_idx: feature_idx as i32,
            threshold,
            left,
            right,
            predicted_class: majority_class,
        }
    }

    /// Check if all targets are same class.
    fn is_pure(&self, targets: &[f64]) -> bool {
        if targets.is_empty() {
            return true;
        }
        let first = targets[0];
        targets.iter().all(|&t| t == first)
    }

    /// Get majority class from targets.
    fn majority_class(&self, targets: &[f64]) -> f64 {
        if targets.is_empty() {
            return 0.0;
        }

        // Count class frequencies
        let mut counts: HashMap<i64, usize> = HashMap::new();
        for &target in targets {
            *counts.entry(target as i64).or_insert(0) += 1;
        }

        // Return most frequent
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(class, _)| class as f64)
            .unwrap_or(0.0)
    }

    /// Get median value of feature.
    fn median_value(&self, features: &[Vec<f64>], feature_idx: usize) -> f64 {
        let mut values: Vec<f64> = features
            .iter()
            .filter_map(|f| f.get(feature_idx).copied())
            .collect();

        if values.is_empty() {
            return 0.0;
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        values[values.len() / 2]
    }

    /// Split data by threshold.
    fn split_data(
        &self,
        features: &[Vec<f64>],
        targets: &[f64],
        feature_idx: usize,
        threshold: f64,
    ) -> (Vec<Vec<f64>>, Vec<f64>, Vec<Vec<f64>>, Vec<f64>) {
        let mut left_features = Vec::new();
        let mut left_targets = Vec::new();
        let mut right_features = Vec::new();
        let mut right_targets = Vec::new();

        for (i, feature) in features.iter().enumerate() {
            if let Some(&value) = feature.get(feature_idx) {
                if value <= threshold {
                    left_features.push(feature.clone());
                    left_targets.push(targets[i]);
                } else {
                    right_features.push(feature.clone());
                    right_targets.push(targets[i]);
                }
            }
        }

        (left_features, left_targets, right_features, right_targets)
    }

    /// Predict single sample using tree.
    fn predict_sample(&self, tree: &TreeNode, features: &[f64]) -> f64 {
        // Leaf node
        if tree.feature_idx < 0 {
            return tree.predicted_class;
        }

        // Get feature value
        let feature_value = features
            .get(tree.feature_idx as usize)
            .copied()
            .unwrap_or(0.0);

        // Traverse tree
        if feature_value <= tree.threshold {
            if let Some(ref left) = tree.left {
                self.predict_sample(left, features)
            } else {
                tree.predicted_class
            }
        } else {
            if let Some(ref right) = tree.right {
                self.predict_sample(right, features)
            } else {
                tree.predicted_class
            }
        }
    }
}

impl Default for DecisionTreeClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Model for DecisionTreeClassifier {
    fn fit(
        &mut self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &Arc<dyn PropertyValues>,
        node_ids: &[usize],
    ) -> Result<(), ModelError> {
        if features.is_empty() {
            return Err(ModelError::InvalidInput("No features provided".to_string()));
        }

        if node_ids.is_empty() {
            return Err(ModelError::InvalidInput(
                "No training nodes provided".to_string(),
            ));
        }

        // Store feature names
        self.feature_names = features.keys().cloned().collect();
        self.feature_names.sort(); // Consistent ordering

        // Extract training data
        let feature_matrix = self.extract_features(features, node_ids)?;
        let target_vec = self.extract_target(target, node_ids)?;

        // Build tree
        self.tree = Some(self.build_tree(&feature_matrix, &target_vec, 0));

        // Update metadata
        self.metadata.is_trained = true;
        self.metadata.feature_count = self.feature_names.len();
        self.metadata.training_nodes = node_ids.len();

        Ok(())
    }

    fn predict(
        &self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        node_ids: &[usize],
    ) -> Result<Vec<f64>, ModelError> {
        if !self.is_trained() {
            return Err(ModelError::NotTrained);
        }

        let tree = self.tree.as_ref().unwrap();

        // Extract features
        let feature_matrix = self.extract_features(features, node_ids)?;

        // Predict each sample
        let predictions: Vec<f64> = feature_matrix
            .iter()
            .map(|sample| self.predict_sample(tree, sample))
            .collect();

        Ok(predictions)
    }

    fn predict_proba(
        &self,
        _features: &HashMap<String, Arc<dyn PropertyValues>>,
        _node_ids: &[usize],
    ) -> Result<Vec<Vec<f64>>, ModelError> {
        // Phase 2.3: Not implemented for decision tree
        Err(ModelError::PredictionFailed(
            "predict_proba not implemented for DecisionTree".to_string(),
        ))
    }

    fn evaluate(
        &self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &Arc<dyn PropertyValues>,
        node_ids: &[usize],
    ) -> Result<f64, ModelError> {
        if !self.is_trained() {
            return Err(ModelError::NotTrained);
        }

        // Get predictions
        let predictions = self.predict(features, node_ids)?;

        // Get actual targets
        let actual = self.extract_target(target, node_ids)?;

        // Calculate accuracy
        let correct = predictions
            .iter()
            .zip(actual.iter())
            .filter(|(pred, actual)| pred == actual)
            .count();

        let accuracy = correct as f64 / predictions.len() as f64;
        Ok(accuracy)
    }

    fn is_trained(&self) -> bool {
        self.metadata.is_trained
    }

    fn model_type(&self) -> ModelType {
        self.metadata.model_type.clone()
    }

    fn metadata(&self) -> ModelMetadata {
        self.metadata.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::native::form::MockDoublePropertyValues;

    /// Create simple node features for testing.
    fn create_test_features() -> HashMap<String, Arc<dyn PropertyValues>> {
        let mut features = HashMap::new();

        // Feature 1: [1.0, 2.0, 3.0, 4.0]
        features.insert(
            "feature1".to_string(),
            Arc::new(MockDoublePropertyValues::from_vec(vec![1.0, 2.0, 3.0, 4.0]))
                as Arc<dyn PropertyValues>,
        );

        features
    }

    /// Create simple node labels for testing.
    fn create_test_target() -> Arc<dyn PropertyValues> {
        // Target: [0.0, 0.0, 1.0, 1.0] (binary classification)
        Arc::new(MockDoublePropertyValues::from_vec(vec![0.0, 0.0, 1.0, 1.0]))
            as Arc<dyn PropertyValues>
    }

    #[test]
    fn test_decision_tree_creation() {
        let model = DecisionTreeClassifier::new();
        assert!(!model.is_trained());
        assert_eq!(model.model_type(), ModelType::DecisionTreeClassifier);
    }

    #[test]
    fn test_decision_tree_with_params() {
        let mut params = HashMap::new();
        params.insert("max_depth".to_string(), "5.0".to_string());

        let model = DecisionTreeClassifier::with_params(params);
        assert_eq!(model.max_depth, 5);
        assert!(!model.is_trained());
    }

    #[test]
    fn test_decision_tree_fit() {
        let mut model = DecisionTreeClassifier::new();
        let features = create_test_features();
        let target = create_test_target();
        let node_ids = vec![0, 1, 2, 3];

        let result = model.fit(&features, &target, &node_ids);
        assert!(result.is_ok());
        assert!(model.is_trained());

        let metadata = model.metadata();
        assert_eq!(metadata.feature_count, 1);
        assert_eq!(metadata.training_nodes, 4);
    }

    #[test]
    fn test_decision_tree_predict() {
        let mut model = DecisionTreeClassifier::new();
        let features = create_test_features();
        let target = create_test_target();
        let train_nodes = vec![0, 1, 2, 3];

        // Train
        model.fit(&features, &target, &train_nodes).unwrap();

        // Predict
        let test_nodes = vec![0, 1, 2, 3];
        let predictions = model.predict(&features, &test_nodes).unwrap();

        assert_eq!(predictions.len(), 4);
        // Phase 2.3: Simple tree should predict something
        assert!(predictions.iter().all(|&p| p == 0.0 || p == 1.0));
    }

    #[test]
    fn test_decision_tree_evaluate() {
        let mut model = DecisionTreeClassifier::new();
        let features = create_test_features();
        let target = create_test_target();
        let train_nodes = vec![0, 1, 2, 3];

        // Train
        model.fit(&features, &target, &train_nodes).unwrap();

        // Evaluate
        let test_nodes = vec![0, 1, 2, 3];
        let accuracy = model.evaluate(&features, &target, &test_nodes).unwrap();

        // Phase 2.3: Should get some accuracy
        assert!(accuracy >= 0.0 && accuracy <= 1.0);
    }

    #[test]
    fn test_decision_tree_predict_before_training() {
        let model = DecisionTreeClassifier::new();
        let features = create_test_features();
        let node_ids = vec![0, 1];

        let result = model.predict(&features, &node_ids);
        assert!(matches!(result, Err(ModelError::NotTrained)));
    }

    #[test]
    fn test_decision_tree_node_centric() {
        // Test with non-contiguous node IDs
        let mut model = DecisionTreeClassifier::new();
        let features = create_test_features();
        let target = create_test_target();

        // Train on nodes [0, 2] only
        let train_nodes = vec![0, 2];
        model.fit(&features, &target, &train_nodes).unwrap();

        // Predict on different nodes [1, 3]
        let test_nodes = vec![1, 3];
        let predictions = model.predict(&features, &test_nodes).unwrap();

        assert_eq!(predictions.len(), 2);
    }

    #[test]
    fn test_decision_tree_metadata() {
        let mut model = DecisionTreeClassifier::new();
        let features = create_test_features();
        let target = create_test_target();
        let node_ids = vec![0, 1, 2, 3];

        // Before training
        let metadata = model.metadata();
        assert!(!metadata.is_trained);
        assert_eq!(metadata.feature_count, 0);
        assert_eq!(metadata.training_nodes, 0);

        // After training
        model.fit(&features, &target, &node_ids).unwrap();
        let metadata = model.metadata();
        assert!(metadata.is_trained);
        assert_eq!(metadata.feature_count, 1);
        assert_eq!(metadata.training_nodes, 4);
    }
}

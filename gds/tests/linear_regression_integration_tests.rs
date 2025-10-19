//! Comprehensive integration tests for Linear Regression.
//!
//! These tests verify the complete Linear Regression pipeline:
//! - Data preparation and feature extraction
//! - Model training with gradient descent
//! - Prediction accuracy and model serialization
//! - Configuration and hyperparameter handling
//!
//! Tests cover:
//! - Simple linear relationships (y = mx + b)
//! - Multi-dimensional features
//! - L2 regularization (ridge regression)
//! - Model persistence and loading
//! - Error handling and edge cases

use parking_lot::RwLock;
use gds::collections::HugeDoubleArray;
use gds::ml::gradient_descent::GradientDescentConfig;
use gds::ml::models::linear::*;
use gds::ml::models::{Features, ModelData, Regressor, RegressorTrainer, TrainerConfig};
use std::sync::Arc;

/// Simple feature store for testing
#[derive(Debug)]
struct TestFeatures {
    features: Vec<Vec<f64>>,
}

impl TestFeatures {
    fn new(features: Vec<Vec<f64>>) -> Self {
        Self { features }
    }
}

impl Features for TestFeatures {
    fn get(&self, index: usize) -> &[f64] {
        &self.features[index]
    }

    fn feature_dimension(&self) -> usize {
        if self.features.is_empty() {
            0
        } else {
            self.features[0].len()
        }
    }

    fn size(&self) -> usize {
        self.features.len()
    }
}

#[cfg(test)]
mod basic_training {
    use super::*;

    #[test]
    fn test_simple_linear_relationship() {
        // Test case: y = 2x + 1
        // Features: [1.0], [2.0], [3.0], [4.0]
        // Targets: [3.0], [5.0], [7.0], [9.0]

        let features = TestFeatures::new(vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]]);

        let targets = HugeDoubleArray::from_vec(vec![3.0, 5.0, 7.0, 9.0]);
        let train_set = Arc::new(vec![0, 1, 2, 3]);

        // Create training configuration with more aggressive learning
        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.1) // Increased learning rate
            .batch_size(4)
            .max_epochs(1000)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        // Train the model
        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&features, &targets, &train_set);

        // Test predictions
        let prediction_1 = regressor.predict(&[1.0]);
        let prediction_2 = regressor.predict(&[2.0]);
        let prediction_3 = regressor.predict(&[3.0]);
        let prediction_4 = regressor.predict(&[4.0]);

        // Check that predictions are close to expected values
        assert!(
            (prediction_1 - 3.0).abs() < 0.1,
            "Prediction 1: expected ~3.0, got {}",
            prediction_1
        );
        assert!(
            (prediction_2 - 5.0).abs() < 0.1,
            "Prediction 2: expected ~5.0, got {}",
            prediction_2
        );
        assert!(
            (prediction_3 - 7.0).abs() < 0.1,
            "Prediction 3: expected ~7.0, got {}",
            prediction_3
        );
        assert!(
            (prediction_4 - 9.0).abs() < 0.1,
            "Prediction 4: expected ~9.0, got {}",
            prediction_4
        );
    }

    #[test]
    fn test_multi_dimensional_features() {
        // Test case: y = 2x1 + 3x2 + 1
        // Features: [1.0, 1.0], [2.0, 1.0], [1.0, 2.0], [2.0, 2.0]
        // Targets: [6.0], [8.0], [9.0], [11.0]

        let features = TestFeatures::new(vec![
            vec![1.0, 1.0],
            vec![2.0, 1.0],
            vec![1.0, 2.0],
            vec![2.0, 2.0],
        ]);

        let targets = HugeDoubleArray::from_vec(vec![6.0, 8.0, 9.0, 11.0]);
        let train_set = Arc::new(vec![0, 1, 2, 3]);

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(4)
            .max_epochs(1000)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&features, &targets, &train_set);

        // Test predictions
        let prediction_1 = regressor.predict(&[1.0, 1.0]);
        let prediction_2 = regressor.predict(&[2.0, 1.0]);
        let prediction_3 = regressor.predict(&[1.0, 2.0]);
        let prediction_4 = regressor.predict(&[2.0, 2.0]);

        assert!(
            (prediction_1 - 6.0).abs() < 0.2,
            "Prediction 1: expected ~6.0, got {}",
            prediction_1
        );
        assert!(
            (prediction_2 - 8.0).abs() < 0.2,
            "Prediction 2: expected ~8.0, got {}",
            prediction_2
        );
        assert!(
            (prediction_3 - 9.0).abs() < 0.2,
            "Prediction 3: expected ~9.0, got {}",
            prediction_3
        );
        assert!(
            (prediction_4 - 11.0).abs() < 0.2,
            "Prediction 4: expected ~11.0, got {}",
            prediction_4
        );
    }
}

#[cfg(test)]
mod regularization_tests {
    use super::*;

    #[test]
    fn test_l2_regularization_effect() {
        // Test that L2 regularization reduces overfitting
        // Use a simple case where regularization should help

        let features = TestFeatures::new(vec![vec![1.0], vec![2.0], vec![3.0]]);

        let targets = HugeDoubleArray::from_vec(vec![2.0, 4.0, 6.0]);
        let train_set = Arc::new(vec![0, 1, 2]);

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(3)
            .max_epochs(500)
            .tolerance(1e-6)
            .build()
            .unwrap();

        // Train without regularization
        let train_config_no_reg = LinearRegressionTrainConfig::new(gradient_config.clone(), 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer_no_reg =
            LinearRegressionTrainer::new(1, train_config_no_reg, termination_flag.clone());
        let regressor_no_reg = trainer_no_reg.train(&features, &targets, &train_set);

        // Train with regularization
        let train_config_reg = LinearRegressionTrainConfig::new(gradient_config, 0.1);
        let trainer_reg = LinearRegressionTrainer::new(1, train_config_reg, termination_flag);
        let regressor_reg = trainer_reg.train(&features, &targets, &train_set);

        // Both should make reasonable predictions
        let pred_no_reg = regressor_no_reg.predict(&[1.0]);
        let pred_reg = regressor_reg.predict(&[1.0]);

        assert!(
            (pred_no_reg - 2.0).abs() < 0.5,
            "No regularization prediction: expected ~2.0, got {}",
            pred_no_reg
        );
        assert!(
            (pred_reg - 2.0).abs() < 0.5,
            "Regularized prediction: expected ~2.0, got {}",
            pred_reg
        );

        // The regularized model should have different (usually smaller) weights
        // This is a basic check that regularization is being applied
        assert_ne!(
            pred_no_reg, pred_reg,
            "Regularization should affect predictions"
        );
    }
}

#[cfg(test)]
mod model_persistence {
    use super::*;

    #[test]
    fn test_model_serialization_and_deserialization() {
        // Train a simple model
        let features = TestFeatures::new(vec![vec![1.0], vec![2.0], vec![3.0]]);

        let targets = HugeDoubleArray::from_vec(vec![2.0, 4.0, 6.0]);
        let train_set = Arc::new(vec![0, 1, 2]);

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(3)
            .max_epochs(100)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&features, &targets, &train_set);

        // Get the underlying data for serialization
        let linear_regressor = regressor
            .as_any()
            .downcast_ref::<LinearRegressor>()
            .unwrap();
        let model_data = linear_regressor.data();

        // Serialize the model
        let serialized = model_data.to_bytes().expect("Serialization should succeed");
        assert!(
            !serialized.is_empty(),
            "Serialized data should not be empty"
        );

        // Deserialize the model
        let deserialized_data =
            LinearRegressionData::from_bytes(&serialized).expect("Deserialization should succeed");

        // Create a new regressor from deserialized data
        let deserialized_regressor = LinearRegressor::new(deserialized_data);

        // Test that predictions match
        let original_prediction = regressor.predict(&[2.0]);
        let deserialized_prediction = deserialized_regressor.predict(&[2.0]);

        assert!(
            (original_prediction - deserialized_prediction).abs() < 1e-10,
            "Predictions should match after serialization/deserialization: {} vs {}",
            original_prediction,
            deserialized_prediction
        );
    }
}

#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_training_configuration() {
        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.05)
            .batch_size(8)
            .max_epochs(200)
            .tolerance(1e-8)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.5);

        // Test configuration accessors
        assert_eq!(train_config.penalty(), 0.5);
        assert_eq!(train_config.learning_rate(), 0.05);
        assert_eq!(train_config.batch_size(), 8);
        assert_eq!(train_config.max_epochs(), 200);
        assert_eq!(train_config.tolerance(), 1e-8);
        assert_eq!(
            train_config.method(),
            gds::ml::models::TrainingMethod::LinearRegression
        );

        // Test configuration modification
        let mut config = train_config.clone();
        config.set_penalty(1.0);
        assert_eq!(config.penalty(), 1.0);

        // Test default configuration
        let default_config = LinearRegressionTrainConfig::default();
        assert_eq!(default_config.penalty(), 0.0);
        assert_eq!(
            default_config.method(),
            gds::ml::models::TrainingMethod::LinearRegression
        );
    }

    #[test]
    fn test_configuration_to_map() {
        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(4)
            .max_epochs(100)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.1);
        let config_map = train_config.to_map();

        assert_eq!(
            config_map.get("method").unwrap().as_str().unwrap(),
            "LinearRegression"
        );
        assert_eq!(config_map.get("penalty").unwrap().as_f64().unwrap(), 0.1);
        assert_eq!(config_map.get("batch_size").unwrap().as_u64().unwrap(), 4);
        assert_eq!(
            config_map.get("learning_rate").unwrap().as_f64().unwrap(),
            0.01
        );
        assert_eq!(config_map.get("max_epochs").unwrap().as_u64().unwrap(), 100);
        assert_eq!(config_map.get("tolerance").unwrap().as_f64().unwrap(), 1e-6);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_single_sample_training() {
        // Test training with just one sample
        let features = TestFeatures::new(vec![vec![1.0]]);
        let targets = HugeDoubleArray::from_vec(vec![2.0]);
        let train_set = Arc::new(vec![0]);

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(1)
            .max_epochs(100)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&features, &targets, &train_set);

        // Should be able to make a prediction
        let prediction = regressor.predict(&[1.0]);
        assert!(
            (prediction - 2.0).abs() < 1.0,
            "Single sample prediction should be reasonable: {}",
            prediction
        );
    }

    #[test]
    fn test_zero_features() {
        // Test with zero-dimensional features (edge case)
        let features = TestFeatures::new(vec![vec![]]);
        let targets = HugeDoubleArray::from_vec(vec![1.0]);
        let train_set = Arc::new(vec![0]);

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(1)
            .max_epochs(10)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&features, &targets, &train_set);

        // Should predict just the bias term
        let prediction = regressor.predict(&[]);
        assert!(
            (prediction - 1.0).abs() < 0.5,
            "Zero-feature prediction should be close to target: {}",
            prediction
        );
    }

    #[test]
    fn test_identical_features() {
        // Test with identical features (should still work)
        let features = TestFeatures::new(vec![vec![1.0], vec![1.0], vec![1.0]]);

        let targets = HugeDoubleArray::from_vec(vec![2.0, 3.0, 4.0]);
        let train_set = Arc::new(vec![0, 1, 2]);

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.01)
            .batch_size(3)
            .max_epochs(100)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&features, &targets, &train_set);

        // Should make a reasonable prediction
        let prediction = regressor.predict(&[1.0]);
        assert!(
            prediction > 0.0,
            "Prediction should be positive: {}",
            prediction
        );
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_dataset_training() {
        // Test with a larger dataset to ensure performance
        let mut features = Vec::new();
        let mut targets = Vec::new();

        // Generate 100 samples: y = 2x + 1 + noise
        for i in 0..100 {
            let x = (i as f64) * 0.1;
            let y = 2.0 * x + 1.0 + (i as f64) * 0.01; // Small noise
            features.push(vec![x]);
            targets.push(y);
        }

        let test_features = TestFeatures::new(features);
        let test_targets = HugeDoubleArray::from_vec(targets);
        let train_set = Arc::new((0..100).collect());

        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.001)
            .batch_size(10)
            .max_epochs(100)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.01);
        let termination_flag = Arc::new(RwLock::new(false));

        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        let regressor = trainer.train(&test_features, &test_targets, &train_set);

        // Test a few predictions
        let prediction_0 = regressor.predict(&[0.0]);
        let prediction_5 = regressor.predict(&[0.5]);
        let prediction_10 = regressor.predict(&[1.0]);

        assert!(
            (prediction_0 - 1.0).abs() < 0.5,
            "Prediction at x=0 should be close to 1.0: {}",
            prediction_0
        );
        assert!(
            (prediction_5 - 2.0).abs() < 0.5,
            "Prediction at x=0.5 should be close to 2.0: {}",
            prediction_5
        );
        assert!(
            (prediction_10 - 3.0).abs() < 0.5,
            "Prediction at x=1.0 should be close to 3.0: {}",
            prediction_10
        );
    }
}

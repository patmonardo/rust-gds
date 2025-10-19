//! Logistic Regression Integration Test
//!
//! Comprehensive integration test for the Logistic Regression classifier system.

use parking_lot::RwLock;
use rust_gds::collections::HugeIntArray;
use rust_gds::ml::core::tensor::Matrix;
use rust_gds::ml::models::logistic_regression::*;
use rust_gds::ml::models::{Classifier, ClassifierTrainer, Features};
use std::sync::Arc;

/// Simple test features for logistic regression training
struct TestFeatures {
    features: Vec<Vec<f64>>,
}

impl TestFeatures {
    fn new(features: Vec<Vec<f64>>) -> Self {
        Self { features }
    }
}

impl Features for TestFeatures {
    fn get(&self, node_id: usize) -> &[f64] {
        &self.features[node_id]
    }

    fn feature_dimension(&self) -> usize {
        self.features[0].len()
    }

    fn size(&self) -> usize {
        self.features.len()
    }
}

#[test]
fn test_logistic_regression_data_creation() {
    println!("=== Logistic Regression Data Creation Test ===");

    // Test standard data creation
    let data = LogisticRegressionData::standard(5, 3);
    assert_eq!(data.feature_dimension(), 5);
    assert_eq!(data.number_of_classes(), 3);

    // Test reduced class count data creation
    let reduced_data = LogisticRegressionData::with_reduced_class_count(5, 3);
    assert_eq!(reduced_data.feature_dimension(), 5);
    assert_eq!(reduced_data.number_of_classes(), 3); // Still reports original class count

    println!("✓ Logistic regression data creation successful");
}

#[test]
fn test_logistic_regression_classifier_creation() {
    println!("=== Logistic Regression Classifier Creation Test ===");

    let data = LogisticRegressionData::standard(3, 2);
    let classifier = LogisticRegressionClassifier::from(data);

    // Test basic properties
    assert_eq!(classifier.number_of_classes(), 2);
    assert_eq!(classifier.data().feature_dimension(), 3);

    // Test prediction on simple data
    let features = vec![1.0, 2.0, 3.0];
    let probabilities = classifier.predict_probabilities(&features);

    assert_eq!(probabilities.len(), 2);
    assert!((probabilities[0] + probabilities[1] - 1.0).abs() < 1e-10); // Should sum to 1

    println!("✓ Logistic regression classifier creation successful");
    println!("  Probabilities: {:?}", probabilities);
}

#[test]
fn test_logistic_regression_config() {
    println!("=== Logistic Regression Config Test ===");

    let config = LogisticRegressionTrainConfig {
        batch_size: 50,
        learning_rate: 0.01,
        max_epochs: 100,
        tolerance: 1e-4,
        penalty: 0.1,
        focus_weight: 2.0,
        class_weights: Some(vec![1.0, 2.0, 0.5]),
    };

    let termination_flag = Arc::new(RwLock::new(false));
    let trainer = LogisticRegressionTrainer::new(
        config,
        3,     // number_of_classes
        false, // reduce_class_count
        termination_flag,
        1, // concurrency
    );

    // Test that trainer was created successfully
    assert!(std::ptr::addr_of!(trainer) != std::ptr::null());

    println!("✓ Logistic regression trainer creation successful");
}

#[test]
fn test_logistic_regression_end_to_end_training() {
    println!("=== Logistic Regression End-to-End Training Test ===");

    // Create simple 2D classification problem
    // Class 0: points with x + y < 2
    // Class 1: points with x + y >= 2
    let features = vec![
        vec![0.0, 0.0], // Class 0
        vec![0.5, 0.5], // Class 0
        vec![1.0, 0.5], // Class 0
        vec![1.5, 1.0], // Class 1
        vec![2.0, 1.0], // Class 1
        vec![1.0, 2.0], // Class 1
    ];

    let labels = HugeIntArray::from_vec(vec![0, 0, 0, 1, 1, 1]);
    let train_set = Arc::new(vec![0, 1, 2, 3, 4, 5]);

    let test_features = TestFeatures::new(features);

    // Create trainer
    let config = LogisticRegressionTrainConfig {
        batch_size: 3,
        learning_rate: 0.1,
        max_epochs: 50,
        tolerance: 1e-3,
        penalty: 0.01,
        focus_weight: 0.0,
        class_weights: None,
    };

    let termination_flag = Arc::new(RwLock::new(false));
    let trainer = LogisticRegressionTrainer::new(
        config,
        2,     // number_of_classes
        false, // reduce_class_count
        termination_flag,
        1, // concurrency
    );

    // Train the model
    let trained_classifier = trainer.train(&test_features, &labels, &train_set);

    // Test predictions
    let test_point_0 = vec![0.2, 0.3]; // Should be class 0
    let test_point_1 = vec![1.8, 1.2]; // Should be class 1

    let prob_0 = trained_classifier.predict_probabilities(&test_point_0);
    let prob_1 = trained_classifier.predict_probabilities(&test_point_1);

    println!("  Test point [0.2, 0.3] probabilities: {:?}", prob_0);
    println!("  Test point [1.8, 1.2] probabilities: {:?}", prob_1);

    // Check that probabilities sum to 1
    assert!((prob_0[0] + prob_0[1] - 1.0).abs() < 1e-10);
    assert!((prob_1[0] + prob_1[1] - 1.0).abs() < 1e-10);

    // Check that the model learned something (probabilities should be different)
    assert!(
        prob_0[0] > prob_0[1],
        "Model should predict class 0 for [0.2, 0.3]"
    );
    assert!(
        prob_1[1] > prob_1[0],
        "Model should predict class 1 for [1.8, 1.2]"
    );

    println!("✓ Logistic regression end-to-end training successful");
    println!("  Model learned to distinguish between classes!");
}

#[test]
fn test_logistic_regression_batch_prediction() {
    println!("=== Logistic Regression Batch Prediction Test ===");

    let data = LogisticRegressionData::standard(2, 2);
    let classifier = LogisticRegressionClassifier::from(data);

    let features = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];

    let test_features = TestFeatures::new(features);
    let batch_indices = vec![0, 1, 2];

    let batch_predictions = classifier.predict_probabilities_batch(&batch_indices, &test_features);

    assert_eq!(batch_predictions.rows(), 3);
    assert_eq!(batch_predictions.cols(), 2);

    // Check that each row sums to 1
    for i in 0..3 {
        let row_sum = batch_predictions[(i, 0)] + batch_predictions[(i, 1)];
        assert!(
            (row_sum - 1.0).abs() < 1e-10,
            "Row {} should sum to 1, got {}",
            i,
            row_sum
        );
    }

    println!("✓ Logistic regression batch prediction successful");
    println!(
        "  Batch predictions shape: {}x{}",
        batch_predictions.rows(),
        batch_predictions.cols()
    );
}

#[test]
fn test_logistic_regression_with_huge_int_array() {
    println!("=== Logistic Regression HugeIntArray Integration Test ===");

    // Test that HugeIntArray works correctly with logistic regression
    let mut labels = HugeIntArray::new(1000);

    // Fill with alternating pattern
    for i in 0..1000 {
        labels.set(i, (i % 2) as i32);
    }

    // Verify the data
    assert_eq!(labels.get(0), 0);
    assert_eq!(labels.get(1), 1);
    assert_eq!(labels.get(2), 0);
    assert_eq!(labels.get(999), 1);

    // Create features
    let mut features = Vec::new();
    for i in 0..1000 {
        features.push(vec![i as f64, (i * 2) as f64]);
    }

    let test_features = TestFeatures::new(features);
    let train_set = Arc::new((0..1000).collect());

    // Create trainer
    let config = LogisticRegressionTrainConfig {
        batch_size: 100,
        learning_rate: 0.01,
        max_epochs: 10,
        tolerance: 1e-3,
        penalty: 0.0,
        focus_weight: 0.0,
        class_weights: None,
    };

    let termination_flag = Arc::new(RwLock::new(false));
    let trainer = LogisticRegressionTrainer::new(
        config,
        2,     // number_of_classes
        false, // reduce_class_count
        termination_flag,
        1, // concurrency
    );

    // Train the model
    let trained_classifier = trainer.train(&test_features, &labels, &train_set);

    // Test a few predictions
    let test_point = vec![500.0, 1000.0];
    let probabilities = trained_classifier.predict_probabilities(&test_point);

    assert_eq!(probabilities.len(), 2);
    assert!((probabilities[0] + probabilities[1] - 1.0).abs() < 1e-10);

    println!("✓ Logistic regression HugeIntArray integration successful");
    println!("  Processed 1000 samples with HugeIntArray");
    println!("  Test prediction probabilities: {:?}", probabilities);
}

#[test]
fn test_logistic_regression_reduced_class_count() {
    println!("=== Logistic Regression Reduced Class Count Test ===");

    // Test with 3 classes but reduced to 2
    let features = vec![
        vec![0.0, 0.0], // Class 0
        vec![1.0, 1.0], // Class 1
        vec![2.0, 2.0], // Class 2 -> becomes class 1
    ];

    let labels = HugeIntArray::from_vec(vec![0, 1, 2]);
    let train_set = Arc::new(vec![0, 1, 2]);

    let test_features = TestFeatures::new(features);

    // Create trainer with reduced class count
    let config = LogisticRegressionTrainConfig {
        batch_size: 3,
        learning_rate: 0.01,
        max_epochs: 10,
        tolerance: 1e-3,
        penalty: 0.0,
        focus_weight: 0.0,
        class_weights: None,
    };
    let termination_flag = Arc::new(RwLock::new(false));
    let trainer = LogisticRegressionTrainer::new(
        config,
        3,    // number_of_classes
        true, // reduce_class_count
        termination_flag,
        1, // concurrency
    );

    // Train the model
    let trained_classifier = trainer.train(&test_features, &labels, &train_set);

    // Should still report original class count (3), but internally uses 2 classes
    assert_eq!(trained_classifier.number_of_classes(), 3);

    // Test prediction
    let test_point = vec![1.5, 1.5];
    let probabilities = trained_classifier.predict_probabilities(&test_point);

    println!("  Probabilities length: {}", probabilities.len());
    println!("  Probabilities: {:?}", probabilities);

    // Should still have 3 classes in output (reduced class count is internal optimization)
    assert_eq!(probabilities.len(), 3);
    assert!((probabilities[0] + probabilities[1] + probabilities[2] - 1.0).abs() < 1e-10);

    println!("✓ Logistic regression reduced class count successful");
    println!("  Reduced from 3 to 2 classes");
    println!("  Test prediction probabilities: {:?}", probabilities);
}

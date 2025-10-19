//! MLP Integration Test
//!
//! Comprehensive integration test for the MLP classifier system.

use rust_gds::collections::HugeIntArray;
// use rust_gds::ml::models::neural::*;
use rust_gds::ml::models::Features;

/// Simple test features for MLP training
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
fn test_mlp_data_creation() {
    println!("=== MLP Data Creation Test ===");

    let data = MLPClassifierData::create(3, 5, &[10, 5], 42);

    assert_eq!(data.number_of_classes(), 3);
    assert_eq!(data.feature_dimension(), 5);
    assert_eq!(data.depth(), 3); // 2 hidden + 1 output
    assert_eq!(data.weights().len(), 3);
    assert_eq!(data.biases().len(), 3);

    // Check dimensions
    assert_eq!(data.weights()[0].dimensions(), vec![10, 5]); // 10x5
    assert_eq!(data.weights()[1].dimensions(), vec![5, 10]); // 5x10
    assert_eq!(data.weights()[2].dimensions(), vec![3, 5]); // 3x5

    assert_eq!(data.biases()[0].dimensions(), vec![10]); // 10
    assert_eq!(data.biases()[1].dimensions(), vec![5]); // 5
    assert_eq!(data.biases()[2].dimensions(), vec![3]); // 3

    println!("✓ MLP data creation successful");
}

#[test]
fn test_mlp_classifier_prediction() {
    println!("=== MLP Classifier Prediction Test ===");

    let data = MLPClassifierData::create(2, 3, &[4], 123);
    let classifier = MLPClassifier::new(data);

    let features = vec![1.0, 2.0, 3.0];
    let probabilities = classifier.predict_probabilities(&features);

    assert_eq!(probabilities.len(), 2);

    // Probabilities should sum to 1.0 (due to softmax)
    let sum: f64 = probabilities.iter().sum();
    assert!((sum - 1.0).abs() < 1e-10);

    // All probabilities should be positive
    for &prob in &probabilities {
        assert!(prob >= 0.0);
    }

    // Test prediction
    let predicted_class = classifier.predict(&features);
    assert!(predicted_class < 2);

    println!("✓ MLP classifier prediction successful");
    println!("  Probabilities: {:?}", probabilities);
    println!("  Predicted class: {}", predicted_class);
}

#[test]
fn test_mlp_objective_loss() {
    println!("=== MLP Objective Loss Test ===");

    let data = MLPClassifierData::create(2, 3, &[4], 456);
    let classifier = MLPClassifier::new(data);

    let features = TestFeatures::new(vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0]]);
    let labels = HugeIntArray::from_vec(vec![0, 1]);

    let objective =
        MLPClassifierObjective::new(classifier, &features, &labels, 0.01, 0.0, vec![1.0, 1.0]);

    // Test weights
    let weights = objective.weights();
    assert_eq!(weights.len(), 4); // 2 weights + 2 biases

    // Test loss computation
    use rust_gds::ml::core::batch::RangeBatch;
    let batch = RangeBatch::new(0, 2, 2);
    let loss_variable = objective.loss(&batch, 2);

    // Should be able to forward pass
    let ctx = rust_gds::ml::core::computation_context::ComputationContext::new();
    let loss_value = ctx.forward(loss_variable.as_ref());

    assert!(loss_value.aggregate_sum() > 0.0);

    println!("✓ MLP objective loss computation successful");
    println!("  Loss value: {}", loss_value.aggregate_sum());
}

#[test]
fn test_mlp_trainer_config() {
    println!("=== MLP Trainer Config Test ===");

    let config = MLPClassifierTrainConfig::builder()
        .batch_size(50)
        .max_epochs(10)
        .learning_rate(0.01)
        .penalty(0.1)
        .focus_weight(2.0)
        .hidden_layer_sizes(vec![64, 32])
        .build()
        .unwrap();

    assert_eq!(config.batch_size, 50);
    assert_eq!(config.max_epochs, 10);
    assert_eq!(config.learning_rate, 0.01);
    assert_eq!(config.penalty, 0.1);
    assert_eq!(config.focus_weight, 2.0);
    assert_eq!(config.hidden_layer_sizes, &vec![64, 32]);

    println!("✓ MLP trainer config successful");
}

#[test]
fn test_mlp_trainer_creation() {
    println!("=== MLP Trainer Creation Test ===");

    let config = MLPClassifierTrainConfig::default();
    let trainer = MLPClassifierTrainer::new(3, config, Some(42), 1);

    assert_eq!(trainer.number_of_classes(), 3);

    println!("✓ MLP trainer creation successful");
}

#[test]
fn test_mlp_end_to_end_training() {
    println!("=== MLP End-to-End Training Test ===");

    // Create simple training data
    let features = TestFeatures::new(vec![
        vec![1.0, 0.0], // Class 0
        vec![0.0, 1.0], // Class 1
        vec![1.0, 1.0], // Class 0
        vec![0.0, 0.0], // Class 1
    ]);
    let labels = HugeIntArray::from_vec(vec![0, 1, 0, 1]);
    let train_set = vec![0, 1, 2, 3];

    // Create trainer with minimal config for fast testing
    let config = MLPClassifierTrainConfig::builder()
        .max_epochs(1) // Just one epoch for testing
        .batch_size(2)
        .learning_rate(0.1)
        .hidden_layer_sizes(vec![4]) // Simple architecture
        .build()
        .unwrap();

    let mut trainer = MLPClassifierTrainer::new(2, config, Some(789), 1);

    // Train the model
    let classifier = trainer.train(&features, &labels, &train_set);

    // Verify the trained model
    assert_eq!(classifier.data().number_of_classes(), 2);
    assert_eq!(classifier.data().feature_dimension(), 2);

    // Test predictions
    let test_features = vec![1.0, 0.0];
    let probabilities = classifier.predict_probabilities(&test_features);

    assert_eq!(probabilities.len(), 2);
    let sum: f64 = probabilities.iter().sum();
    assert!((sum - 1.0).abs() < 1e-10);

    let predicted_class = classifier.predict(&test_features);
    assert!(predicted_class < 2);

    println!("✓ MLP end-to-end training successful");
    println!("  Test probabilities: {:?}", probabilities);
    println!("  Predicted class: {}", predicted_class);
}

#[test]
fn test_mlp_batch_predictions() {
    println!("=== MLP Batch Predictions Test ===");

    let data = MLPClassifierData::create(2, 3, &[4], 999);
    let classifier = MLPClassifier::new(data);

    let features = TestFeatures::new(vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0]]);

    use rust_gds::ml::core::batch::RangeBatch;
    let batch = RangeBatch::new(0, 2, 2);

    let predictions = classifier.predict_probabilities_batch(&batch, &features);

    assert_eq!(predictions.rows(), 2);
    assert_eq!(predictions.cols(), 2);

    // Each row should sum to 1.0 (softmax)
    for row in 0..predictions.rows() {
        let row_sum: f64 = (0..predictions.cols())
            .map(|col| predictions[(row, col)])
            .sum();
        assert!((row_sum - 1.0).abs() < 1e-10);
    }

    println!("✓ MLP batch predictions successful");
}

#[test]
fn test_mlp_computation_graph() {
    println!("=== MLP Computation Graph Test ===");

    let data = MLPClassifierData::create(2, 3, &[4], 111);
    let classifier = MLPClassifier::new(data);

    use rust_gds::ml::core::functions::constant::Constant;
    use rust_gds::ml::core::tensor::Matrix;

    let features = Matrix::new(vec![1.0, 2.0, 3.0], 1, 3);
    let features_var = Constant::new(Box::new(features));

    let predictions_var = classifier.predictions_variable(Box::new(features_var));

    // Should be able to forward pass
    let ctx = rust_gds::ml::core::computation_context::ComputationContext::new();
    let result = ctx.forward(predictions_var.as_ref());

    // Cast to Matrix to access rows/cols
    let result_matrix = result.as_any().downcast_ref::<Matrix>().unwrap();
    assert_eq!(result_matrix.rows(), 1);
    assert_eq!(result_matrix.cols(), 2);

    // Should be able to backward pass
    ctx.backward(predictions_var.as_ref());

    println!("✓ MLP computation graph successful");
}

#[test]
fn test_mlp_kaiming_initialization() {
    println!("=== MLP Kaiming Initialization Test ===");

    let data = MLPClassifierData::create(2, 100, &[50], 222);

    // Check that weights are initialized with proper bounds
    let first_weight = data.weights()[0].snapshot();
    let expected_bound = (2.0_f64 / 100.0_f64).sqrt(); // sqrt(2/100) ≈ 0.141

    let mut max_weight = 0.0_f64;
    for &value in first_weight.data() {
        max_weight = max_weight.max(value.abs());
    }

    assert!(max_weight <= expected_bound + 1e-10); // Allow small floating point errors

    println!("✓ MLP Kaiming initialization successful");
    println!(
        "  Max weight: {}, Expected bound: {}",
        max_weight, expected_bound
    );
}

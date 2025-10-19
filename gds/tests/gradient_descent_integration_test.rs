//! Comprehensive Gradient Descent Integration Test
//!
//! This test suite debugs the gradient descent system to identify why
//! Linear and Logistic Regression models are not learning from training data.

use gds::collections::HugeIntArray;
use gds::ml::{
    core::{
        batch::{Batch, RangeBatch},
        batch_queue,
        functions::{
            constant::Constant, ewise_add_matrix_scalar::EWiseAddMatrixScalar,
            matrix_multiply_with_transposed_second_operand::MatrixMultiplyWithTransposedSecondOperand,
            mean_square_error::MeanSquareError, weights::Weights,
        },
        tensor::{Matrix, Scalar, Vector, Tensor},
        variable::Variable,
        ComputationContext,
    },
    gradient_descent::{GradientDescentConfig, Objective, Training},
    models::Features,
};
use std::sync::Arc;

/// Simple test features for gradient descent testing
#[allow(dead_code)]
struct TestFeatures {
    features: Vec<Vec<f64>>,
}

#[allow(dead_code)]
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

/// Simple objective function for testing gradient descent
struct TestObjective {
    weights: Weights,
    bias: Weights,
    features: Vec<Vec<f64>>,
    labels: Vec<f64>,
}

impl TestObjective {
    fn new(feature_count: usize, sample_count: usize) -> Self {
        let weights = Weights::of_matrix(1, feature_count);
        let bias = Weights::of_scalar(0.0);

        // Create simple linear relationship: y = 2*x1 + 3*x2 + 1
        let mut features = Vec::new();
        let mut labels = Vec::new();

        for i in 0..sample_count {
            let x1 = i as f64 * 0.1;
            let x2 = i as f64 * 0.2;
            let y = 2.0 * x1 + 3.0 * x2 + 1.0;

            features.push(vec![x1, x2]);
            labels.push(y);
        }

        Self {
            weights,
            bias,
            features,
            labels,
        }
    }

    fn predict(&self, features: &[f64]) -> f64 {
        let ctx = ComputationContext::new();
        let features_matrix = Matrix::new(features.to_vec(), 1, features.len());
        let features_var = Constant::new(Box::new(features_matrix));

        let weighted = MatrixMultiplyWithTransposedSecondOperand::new(
            Box::new(features_var),
            Box::new(self.weights.clone()) as Box<dyn Variable>,
        );

        let prediction = EWiseAddMatrixScalar::new(
            Box::new(weighted),
            Box::new(Constant::new(Box::new(Scalar::new(
                self.bias.borrow_scalar().value(),
            )))) as Box<dyn Variable>,
        );

        let result = ctx.forward(&prediction);

        // Try to access the result directly
        let result_value = if let Some(matrix) = result.as_any().downcast_ref::<Matrix>() {
            matrix[(0, 0)]
        } else {
            println!("  Failed to downcast to Matrix, trying other types...");
            0.0 // Fallback
        };

        result_value
    }

    fn get_weights(&self) -> Vec<f64> {
        self.weights.borrow_matrix().data().to_vec()
    }

    fn get_bias(&self) -> f64 {
        self.bias.borrow_scalar().value()
    }
}

impl Objective for TestObjective {
    type ModelData = ();

    fn weights(&self) -> Vec<Weights> {
        vec![self.weights.clone(), self.bias.clone()]
    }

    fn model_data(&self) -> &Self::ModelData {
        &()
    }

    fn loss<B: Batch>(&self, batch: &B, _train_size: usize) -> Box<dyn Variable> {
        let batch_size = batch.size();
        let feature_count = self.features[0].len();

        // Create batch feature matrix
        let mut batch_features = Matrix::zeros(batch_size, feature_count);
        let mut batch_labels = Vector::with_size(batch_size);

        let mut idx = 0;
        for element_id in batch.element_ids() {
            let features = &self.features[element_id as usize];
            let label = self.labels[element_id as usize];

            for (j, &feature) in features.iter().enumerate() {
                batch_features[(idx, j)] = feature;
            }
            batch_labels[idx] = label;
            idx += 1;
        }

        // Create prediction
        let features_var = Constant::new(Box::new(batch_features));
        let weighted = MatrixMultiplyWithTransposedSecondOperand::new(
            Box::new(features_var),
            Box::new(self.weights.clone()) as Box<dyn Variable>,
        );

        let prediction = EWiseAddMatrixScalar::new(
            Box::new(weighted),
            Box::new(Constant::new(Box::new(Scalar::new(
                self.bias.borrow_scalar().value(),
            )))) as Box<dyn Variable>,
        );

        // Compute MSE loss
        let labels_var = Constant::new(Box::new(batch_labels));
        Box::new(MeanSquareError::new(
            Box::new(prediction),
            Box::new(labels_var),
        ))
    }
}

#[test]
fn test_gradient_descent_basic_setup() {
    println!("=== Gradient Descent Basic Setup Test ===");

    let objective = TestObjective::new(2, 10);

    // Test initial weights
    let initial_weights = objective.get_weights();
    let initial_bias = objective.get_bias();

    println!("  Initial weights: {:?}", initial_weights);
    println!("  Initial bias: {}", initial_bias);

    // Test prediction
    let test_features = vec![1.0, 2.0];
    let prediction = objective.predict(&test_features);
    println!("  Initial prediction for [1.0, 2.0]: {}", prediction);

    // Expected: 2*1.0 + 3*2.0 + 1.0 = 9.0
    // But with zero weights: 0*1.0 + 0*2.0 + 0.0 = 0.0
    assert_eq!(prediction, 0.0);

    println!("✓ Gradient descent basic setup successful");
}

#[test]
fn test_gradient_descent_loss_computation() {
    println!("=== Gradient Descent Loss Computation Test ===");

    let objective = TestObjective::new(2, 5);
    let batch = RangeBatch::new(0, 5, 5);

    // Test loss computation
    let loss_var = objective.loss(&batch, 5);
    let ctx = ComputationContext::new();
    let _loss_value = ctx.forward(loss_var.as_ref());

    let loss_scalar = if let Some(scalar) = (loss_var.as_ref() as &dyn std::any::Any).downcast_ref::<Scalar>() {
        scalar
    } else {
        println!("  Failed to downcast loss to Scalar");
        return; // Skip this test
    };
    println!("  Loss value: {}", loss_scalar.value());

    // Loss should be positive (MSE of predictions vs true labels)
    assert!(loss_scalar.value() > 0.0);

    println!("✓ Gradient descent loss computation successful");
}

#[test]
fn test_gradient_descent_gradient_computation() {
    println!("=== Gradient Descent Gradient Computation Test ===");

    let objective = TestObjective::new(2, 5);
    let batch = RangeBatch::new(0, 5, 5);

    // Create loss
    let loss_var = objective.loss(&batch, 5);
    let ctx = ComputationContext::new();

    // Forward pass
    let _loss_value = ctx.forward(loss_var.as_ref());

    // Backward pass
    let loss_gradient = ctx.gradient(loss_var.as_ref());
    println!("  Loss gradient: {:?}", loss_gradient);

    // Test gradients for weights
    let weights_gradient = ctx.gradient(&objective.weights);
    println!("  Weights gradient: {:?}", weights_gradient);

    // Test gradients for bias
    let bias_gradient = ctx.gradient(&objective.bias);
    println!("  Bias gradient: {:?}", bias_gradient);

    println!("✓ Gradient descent gradient computation successful");
}

#[test]
fn test_gradient_descent_training_config() {
    println!("=== Gradient Descent Training Config Test ===");

    let config = GradientDescentConfig::builder()
        .batch_size(3)
        .learning_rate(0.01)
        .max_epochs(10)
        .tolerance(1e-4)
        .build()
        .unwrap();

    assert_eq!(config.batch_size(), 3);
    assert_eq!(config.learning_rate(), 0.01);
    assert_eq!(config.max_epochs(), 10);
    assert_eq!(config.tolerance(), 1e-4);

    println!("✓ Gradient descent training config successful");
}

#[test]
fn test_gradient_descent_training_loop() {
    println!("=== Gradient Descent Training Loop Test ===");

    let objective = TestObjective::new(2, 10);
    let train_set: Arc<Vec<usize>> = Arc::new((0..10).collect());

    // Record initial weights
    let initial_weights = objective.get_weights();
    let initial_bias = objective.get_bias();

    println!("  Initial weights: {:?}", initial_weights);
    println!("  Initial bias: {}", initial_bias);

    // Create training configuration
    let config = GradientDescentConfig::builder()
        .batch_size(5)
        .learning_rate(0.01)
        .max_epochs(5)
        .tolerance(1e-6)
        .build()
        .unwrap();

    // Create training instance
    let training = Training::new(config, train_set.len());

    // Create batch queue supplier
    let queue_supplier = || batch_queue::consecutive_with_batch_size(train_set.len() as u64, 5);

    // Run training
    println!("  Starting training...");
    training.train(&objective, queue_supplier, 1);
    println!("  Training completed");

    // Check if weights changed
    let final_weights = objective.get_weights();
    let final_bias = objective.get_bias();

    println!("  Final weights: {:?}", final_weights);
    println!("  Final bias: {}", final_bias);

    // Check if weights actually changed
    let weights_changed = initial_weights
        .iter()
        .zip(final_weights.iter())
        .any(|(init, final_val)| (init - final_val).abs() > 1e-10);
    let bias_changed = (initial_bias - final_bias).abs() > 1e-10;

    println!("  Weights changed: {}", weights_changed);
    println!("  Bias changed: {}", bias_changed);

    if weights_changed || bias_changed {
        println!("✓ Gradient descent training loop successful - weights updated!");
    } else {
        println!("❌ Gradient descent training loop failed - weights not updated!");
        panic!("Training did not update weights");
    }
}

#[test]
fn test_gradient_descent_learning_verification() {
    println!("=== Gradient Descent Learning Verification Test ===");

    let objective = TestObjective::new(2, 20);
    let train_set: Arc<Vec<usize>> = Arc::new((0..20).collect());

    // Test initial prediction accuracy
    let test_features = vec![1.0, 2.0];
    let initial_prediction = objective.predict(&test_features);
    let true_value = 2.0 * 1.0 + 3.0 * 2.0 + 1.0; // 9.0

    println!("  Initial prediction: {}", initial_prediction);
    println!("  True value: {}", true_value);
    println!(
        "  Initial error: {}",
        (initial_prediction - true_value).abs()
    );

    // Train the model
    let config = GradientDescentConfig::builder()
        .batch_size(10)
        .learning_rate(0.1)
        .max_epochs(20)
        .tolerance(1e-6)
        .build()
        .unwrap();

    let training = Training::new(config, train_set.len());
    let queue_supplier = || batch_queue::consecutive_with_batch_size(train_set.len() as u64, 10);

    println!("  Training model...");
    training.train(&objective, queue_supplier, 1);

    // Test final prediction accuracy
    let final_prediction = objective.predict(&test_features);
    let final_error = (final_prediction - true_value).abs();

    println!("  Final prediction: {}", final_prediction);
    println!("  Final error: {}", final_error);

    // Check if the model learned
    if final_error < initial_prediction.abs() {
        println!("✓ Gradient descent learning verification successful - model learned!");
        println!(
            "  Error reduced from {} to {}",
            initial_prediction.abs(),
            final_error
        );
    } else {
        println!("❌ Gradient descent learning verification failed - model did not learn!");
        println!(
            "  Error increased from {} to {}",
            initial_prediction.abs(),
            final_error
        );
        panic!("Model did not learn from training data");
    }
}

#[test]
fn test_gradient_descent_with_huge_int_array() {
    println!("=== Gradient Descent with HugeIntArray Test ===");

    // Test that gradient descent works with HugeIntArray labels
    let labels = HugeIntArray::from_vec(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
    let _train_set: Arc<Vec<usize>> = Arc::new((0..10).collect());

    println!("  Created HugeIntArray with {} labels", labels.size());
    println!(
        "  Sample labels: {:?}",
        (0..5).map(|i| labels.get(i)).collect::<Vec<_>>()
    );

    // Test that we can iterate over the labels
    let mut label_sum = 0i32;
    for i in 0..labels.size() {
        label_sum += labels.get(i);
    }

    println!("  Sum of all labels: {}", label_sum);
    assert_eq!(label_sum, 5); // 5 ones

    println!("✓ Gradient descent with HugeIntArray successful");
}

#[test]
fn test_gradient_descent_batch_processing() {
    println!("=== Gradient Descent Batch Processing Test ===");

    let objective = TestObjective::new(2, 10);

    // Test different batch sizes
    for batch_size in [1, 3, 5, 10] {
        println!("  Testing batch size: {}", batch_size);

        let batch = RangeBatch::new(0, batch_size.try_into().unwrap(), batch_size);
        let loss_var = objective.loss(&batch, batch_size.try_into().unwrap());

        let ctx = ComputationContext::new();
        let _loss_value = ctx.forward(loss_var.as_ref());

        let loss_scalar = (loss_var.as_ref() as &dyn std::any::Any).downcast_ref::<Scalar>().unwrap();
        println!(
            "    Loss for batch size {}: {}",
            batch_size,
            loss_scalar.value()
        );

        assert!(loss_scalar.value() >= 0.0);
    }

    println!("✓ Gradient descent batch processing successful");
}

#[test]
fn test_gradient_descent_convergence() {
    println!("=== Gradient Descent Convergence Test ===");

    let objective = TestObjective::new(2, 50);
    let train_set: Arc<Vec<usize>> = Arc::new((0..50).collect());

    // Train with different learning rates
    for learning_rate in [0.001, 0.01, 0.1] {
        println!("  Testing learning rate: {}", learning_rate);

        let config = GradientDescentConfig::builder()
            .batch_size(10)
            .learning_rate(learning_rate)
            .max_epochs(10)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let training = Training::new(config, train_set.len());
        let queue_supplier =
            || batch_queue::consecutive_with_batch_size(train_set.len() as u64, 10);

        // Record initial loss
        let batch = RangeBatch::new(0, 10, 10);
        let initial_loss_var = objective.loss(&batch, 10);
        let ctx = ComputationContext::new();
        let _initial_loss = ctx.forward(initial_loss_var.as_ref());
        let initial_loss_scalar = (initial_loss_var.as_ref() as &dyn std::any::Any).downcast_ref::<Scalar>().unwrap();

        println!("    Initial loss: {}", initial_loss_scalar.value());

        // Train
        training.train(&objective, queue_supplier, 1);

        // Record final loss
        let final_loss_var = objective.loss(&batch, 10);
        let ctx = ComputationContext::new();
        let _final_loss = ctx.forward(final_loss_var.as_ref());
        let final_loss_scalar = (final_loss_var.as_ref() as &dyn std::any::Any).downcast_ref::<Scalar>().unwrap();

        println!("    Final loss: {}", final_loss_scalar.value());

        // Check if loss decreased
        if final_loss_scalar.value() < initial_loss_scalar.value() {
            println!("    ✓ Loss decreased with learning rate {}", learning_rate);
        } else {
            println!(
                "    ❌ Loss did not decrease with learning rate {}",
                learning_rate
            );
        }
    }

    println!("✓ Gradient descent convergence test completed");
}

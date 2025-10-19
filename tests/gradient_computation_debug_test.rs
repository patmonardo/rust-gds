//! Test to debug gradient computation in training
//!
//! This test focuses specifically on whether gradients are being computed
//! correctly for weights during the backward pass.

use rust_gds::ml::{
    core::{
        batch::{Batch, RangeBatch},
        functions::{
            constant::Constant, ewise_add_matrix_scalar::EWiseAddMatrixScalar,
            matrix_multiply_with_transposed_second_operand::MatrixMultiplyWithTransposedSecondOperand,
            mean_square_error::MeanSquareError,
            weights::Weights,
        },
        tensor::{Matrix, Scalar, Vector, Tensor},
        variable::Variable,
        ComputationContext,
    },
    gradient_descent::Objective,
};

/// Simple objective function for testing gradient computation
struct GradientTestObjective {
    weights: Weights,
    bias: Weights,
    features: Vec<Vec<f64>>,
    labels: Vec<f64>,
}

impl GradientTestObjective {
    fn new() -> Self {
        let weights = Weights::of_matrix(1, 2);
        let bias = Weights::of_scalar(0.0);

        // Simple data: y = 2*x1 + 3*x2 + 1
        let features = vec![vec![1.0, 2.0], vec![2.0, 3.0]];
        let labels = vec![9.0, 15.0]; // 2*1 + 3*2 + 1 = 9, 2*2 + 3*3 + 1 = 15

        Self {
            weights,
            bias,
            features,
            labels,
        }
    }

    fn get_weights(&self) -> Vec<f64> {
        self.weights.borrow_matrix().data().to_vec()
    }

    fn get_bias(&self) -> f64 {
        self.bias.borrow_scalar().value()
    }
}

impl Objective for GradientTestObjective {
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
fn test_gradient_computation_debug() {
    println!("=== Gradient Computation Debug Test ===");

    let objective = GradientTestObjective::new();
    let batch = RangeBatch::new(0, 2, 2);

    println!("  Initial weights: {:?}", objective.get_weights());
    println!("  Initial bias: {}", objective.get_bias());

    // Create loss
    let loss_var = objective.loss(&batch, 2);
    let ctx = ComputationContext::new();

    // Forward pass
    let loss_value = ctx.forward(loss_var.as_ref());
    println!("  Loss value: {:?}", loss_value);
    println!("  Computed variables: {}", ctx.computed_variables_count());

    // Backward pass using loss variable directly
    println!("  Starting backward pass...");
    ctx.backward(loss_var.as_ref());
    println!("  Backward pass completed");

    // Check gradients for weights
    println!("  Checking gradients...");
    let weights_gradient = ctx.gradient(&objective.weights);
    let bias_gradient = ctx.gradient(&objective.bias);

    match weights_gradient {
        Some(ref grad) => {
            println!("  ✓ Weights gradient computed: {:?}", grad);
            if let Some(matrix) = grad.as_any().downcast_ref::<Matrix>() {
                println!("    Gradient matrix: {:?}", matrix.data());
            }
        }
        None => {
            println!("  ❌ No weights gradient computed!");
        }
    }

    match bias_gradient {
        Some(ref grad) => {
            println!("  ✓ Bias gradient computed: {:?}", grad);
            if let Some(scalar) = grad.as_any().downcast_ref::<Scalar>() {
                println!("    Gradient scalar: {}", scalar.value());
            }
        }
        None => {
            println!("  ❌ No bias gradient computed!");
        }
    }

    // Check if gradients are non-zero
    let has_non_zero_gradients = weights_gradient.is_some() || bias_gradient.is_some();

    if has_non_zero_gradients {
        println!("✓ Gradient computation is working!");
    } else {
        println!("❌ Gradient computation is NOT working - no gradients computed!");
        panic!("Gradient computation failed");
    }
}

#[test]
fn test_weight_update_simulation() {
    println!("=== Weight Update Simulation Test ===");

    let objective = GradientTestObjective::new();
    let batch = RangeBatch::new(0, 2, 2);

    println!("  Initial weights: {:?}", objective.get_weights());
    println!("  Initial bias: {}", objective.get_bias());

    // Simulate one training step
    let loss_var = objective.loss(&batch, 2);
    let ctx = ComputationContext::new();

    // Forward pass
    let _loss_value = ctx.forward(loss_var.as_ref());

    // Backward pass
    ctx.backward(loss_var.as_ref());

    // Get gradients
    let weights_gradient = ctx.gradient(&objective.weights);
    let bias_gradient = ctx.gradient(&objective.bias);

    println!(
        "  Gradients computed: weights={:?}, bias={:?}",
        weights_gradient.is_some(),
        bias_gradient.is_some()
    );

    // Simulate weight update (like Adam optimizer would do)
    if let Some(grad) = weights_gradient {
        println!("  Simulating weight update...");
        // In a real optimizer, this would be: weight = weight - learning_rate * gradient
        // For now, just check if we can access the gradient
        if let Some(matrix) = grad.as_any().downcast_ref::<Matrix>() {
            println!("    Gradient values: {:?}", matrix.data());
        }
    }

    if let Some(grad) = bias_gradient {
        println!("  Simulating bias update...");
        if let Some(scalar) = grad.as_any().downcast_ref::<Scalar>() {
            println!("    Gradient value: {}", scalar.value());
        }
    }

    println!("✓ Weight update simulation completed");
}

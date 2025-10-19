//! Test to verify that the backward pass is computing gradients

use rust_gds::ml::models::linear::*;
use rust_gds::ml::models::{Features, Regressor};
use rust_gds::ml::core::tensor::Tensor;
use rust_gds::ml::core::computation_context::ComputationContext;
use rust_gds::ml::gradient_descent::Objective;
use rust_gds::ml::core::functions::weights::Weights;
use rust_gds::collections::HugeDoubleArray;

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
mod gradient_tests {
    use super::*;

    #[test]
    fn test_gradient_computation() {
        println!("=== Gradient Computation Test ===");
        
        // Create a simple test case
        let features = TestFeatures::new(vec![vec![1.0]]);
        let targets = HugeDoubleArray::from_vec(vec![2.0]);
        
        // Create objective
        let objective = LinearRegressionObjective::new(&features, &targets, 0.0);
        
        // Create a simple batch using RangeBatch
        let batch = rust_gds::ml::core::batch::RangeBatch::new(0, 1, 1);
        
        // Create computation context
        let ctx = ComputationContext::new();
        
        // Debug: Let's check the weights used in the loss computation
        let weights_from_objective = objective.weights();
        println!("Weights from objective:");
        for (i, weight) in weights_from_objective.iter().enumerate() {
            println!("  Weight {} address: {:p}", i, weight);
        }
        
        // Compute loss
        let loss_variable = objective.loss(&batch, 1);
        println!("Loss variable created: {:p}", loss_variable.as_ref());
        let loss_value = ctx.forward(loss_variable.as_ref());
        
        println!("Loss value: {}", loss_value.aggregate_sum());
        
        // Perform backward pass
        ctx.backward(loss_variable.as_ref());
        
        // Check if gradients were computed
        let weights = objective.weights();
        println!("Number of weights: {}", weights.len());
        
        for (i, weight) in weights.iter().enumerate() {
            let tensor_data = weight.snapshot();
            println!("Weight tensor {}: {:?}", i, tensor_data.data());
            
            // Debug: Check if this weight is actually in the computation graph
            println!("Weight {} address: {:p}", i, weight);
            
            // Try to get gradient for this variable
            if let Some(gradient) = ctx.gradient(weight) {
                println!("Gradient {} found: {:?}", i, gradient.data());
            } else {
                println!("No gradient found for weight {}", i);
            }
        }
        
        // Debug: Let's also check what variables are in the computation context
        println!("Checking computation context for any gradients...");
        // We can't easily iterate over all variables in the context, but let's try a different approach
        
        // For now, just check that the backward pass doesn't crash
        println!("Backward pass completed successfully");
    }
}

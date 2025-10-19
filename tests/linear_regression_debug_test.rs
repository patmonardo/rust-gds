//! Debug test for Linear Regression training issues

use parking_lot::RwLock;
use rust_gds::collections::HugeDoubleArray;
use rust_gds::ml::core::tensor::tensor::AsAny;
use rust_gds::ml::core::tensor::Tensor;
use rust_gds::ml::gradient_descent::GradientDescentConfig;
use rust_gds::ml::models::linear::*;
use rust_gds::ml::models::{BaseModelData, Features, Regressor, RegressorTrainer};
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
mod debug_tests {
    use super::*;

    #[test]
    fn test_linear_regressor_direct() {
        println!("=== Direct LinearRegressor Test ===");

        // Create a LinearRegressor directly with some weights
        let model_data = LinearRegressionData::of(1);

        // Manually set some weights and bias
        {
            let mut weights_matrix = model_data.weights().borrow_matrix_mut();
            weights_matrix.set_data_at(0, 0, 2.0); // weight = 2.0 at (0,0)
        }

        {
            let mut bias_scalar = model_data.bias().borrow_scalar_mut();
            bias_scalar.set_data_at(0, 1.0); // bias = 1.0
        }

        let regressor = LinearRegressor::new(model_data);

        // Test predictions: y = 2x + 1
        let prediction_1 = regressor.predict(&[1.0]);
        let prediction_2 = regressor.predict(&[2.0]);

        println!(
            "Direct prediction for x=1.0: {} (expected 3.0)",
            prediction_1
        );
        println!(
            "Direct prediction for x=2.0: {} (expected 5.0)",
            prediction_2
        );

        assert_eq!(prediction_1, 3.0);
        assert_eq!(prediction_2, 5.0);
    }

    #[test]
    fn test_linear_regression_debug() {
        println!("=== Linear Regression Debug Test ===");

        // Very simple test case: y = 2x + 1
        let features = TestFeatures::new(vec![vec![1.0], vec![2.0]]);

        let targets = HugeDoubleArray::from_vec(vec![3.0, 5.0]);
        let train_set = Arc::new(vec![0, 1]);

        println!("Features: {:?}", features.features);
        println!("Targets: {:?}", vec![3.0, 5.0]);
        println!("Feature dimension: {}", features.feature_dimension());

        // Create training configuration
        let gradient_config = GradientDescentConfig::builder()
            .learning_rate(0.1)
            .batch_size(2)
            .max_epochs(100)
            .tolerance(1e-6)
            .build()
            .unwrap();

        let train_config = LinearRegressionTrainConfig::new(gradient_config, 0.0);
        let termination_flag = Arc::new(RwLock::new(false));

        println!(
            "Training config: penalty={}, learning_rate={}, batch_size={}",
            train_config.penalty(),
            train_config.learning_rate(),
            train_config.batch_size()
        );

        // Train the model
        let trainer = LinearRegressionTrainer::new(1, train_config, termination_flag);
        println!("Created trainer");

        let regressor = trainer.train(&features, &targets, &train_set);
        println!("Training completed");

        // Test predictions
        let prediction_1 = regressor.predict(&[1.0]);
        let prediction_2 = regressor.predict(&[2.0]);

        println!("Prediction for x=1.0: {}", prediction_1);
        println!("Prediction for x=2.0: {}", prediction_2);

        // Check if we can access the underlying model data
        if let Some(linear_regressor) = regressor.as_any().downcast_ref::<LinearRegressor>() {
            let model_data = linear_regressor.data();
            println!(
                "Model data feature dimension: {}",
                model_data.feature_dimension()
            );

            // Try to access weights and bias
            let weights_matrix = model_data.weights().borrow_matrix();
            let bias_scalar = model_data.bias().borrow_scalar();

            println!("Weights matrix: {:?}", weights_matrix.data());
            println!("Bias scalar: {}", bias_scalar.value());

            // Manual prediction calculation
            let manual_prediction_1 = weights_matrix.data()[0] * 1.0 + bias_scalar.value();
            let manual_prediction_2 = weights_matrix.data()[0] * 2.0 + bias_scalar.value();

            println!("Manual prediction for x=1.0: {}", manual_prediction_1);
            println!("Manual prediction for x=2.0: {}", manual_prediction_2);
        } else {
            println!("Could not downcast to LinearRegressor");
            println!(
                "Regressor type: {:?}",
                std::any::type_name_of_val(&*regressor)
            );
        }
    }
}

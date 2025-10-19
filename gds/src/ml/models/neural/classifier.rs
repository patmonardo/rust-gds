//! MLP Classifier
//!
//! Translated from `MLPClassifier.java` from Java GDS.

use crate::ml::core::{
    batch::Batch,
    computation_context::ComputationContext,
    functions::{
        constant::Constant,
        matrix_multiply_with_transposed_second_operand::MatrixMultiplyWithTransposedSecondOperand,
        matrix_vector_sum::MatrixVectorSum,
        relu::Relu,
        softmax::Softmax,
    },
    tensor::Matrix,
    variable::Variable,
};
use crate::ml::gradient_descent::batch_feature_matrix;
use crate::ml::models::{Classifier, ClassifierData, Features};
use super::data::MLPClassifierData;

/// Multi-Layer Perceptron Classifier
/// 
/// This corresponds to MLPClassifier in Java GDS.
/// Uses computation graph for automatic differentiation.
#[derive(Debug)]
pub struct MLPClassifier {
    data: MLPClassifierData,
}

impl MLPClassifier {
    /// Create a new MLP classifier
    /// 
    /// Java: `public MLPClassifier(MLPClassifierData data) {this.data = data;}`
    pub fn new(data: MLPClassifierData) -> Self {
        Self { data }
    }
    
    /// Predict probabilities for a single feature vector
    /// 
    /// Java: `public double[] predictProbabilities(double[] features)`
    pub fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        let ctx = ComputationContext::new();
        let features_matrix = Matrix::new(features.to_vec(), 1, features.len());
        let features_variable = Constant::new(Box::new(features_matrix));
        let predictions_variable = self.predictions_variable(Box::new(features_variable));
        let result = ctx.forward(predictions_variable.as_ref());
        
        // Cast to Matrix to access data
        let result_matrix = result.as_any().downcast_ref::<Matrix>().unwrap();
        result_matrix.data().to_vec()
    }
    
    /// Predict probabilities for a batch
    /// 
    /// Java: `public Matrix predictProbabilities(Batch batch, Features features)`
    pub fn predict_probabilities_batch<B: Batch>(&self, batch: &B, features: &dyn Features) -> Matrix {
        let ctx = ComputationContext::new();
        let batch_features = batch_feature_matrix(batch, features);
        let predictions_variable = self.predictions_variable(Box::new(batch_features));
        let result = ctx.forward(predictions_variable.as_ref());
        
        // Cast to Matrix to access data
        let result_matrix = result.as_any().downcast_ref::<Matrix>().unwrap();
        result_matrix.clone()
    }
    
    /// Build the computation graph for predictions
    /// 
    /// Java: `Variable<Matrix> predictionsVariable(Constant<Matrix> batchFeatures)`
    pub fn predictions_variable(&self, batch_features: Box<dyn Variable>) -> Box<dyn Variable> {
        let mut input_to_next_layer = batch_features;
        
        // Hidden layers with ReLU activation
        for i in 0..self.data.depth() - 1 {
            let output_from_prev_layer = input_to_next_layer;
            
            // Matrix multiplication: input * weights^T
            let weighted_features = MatrixMultiplyWithTransposedSecondOperand::new(
                output_from_prev_layer,
                Box::new(self.data.weights()[i].clone()),
            );
            
            // Add bias: weighted_features + bias
            let biased_features = MatrixVectorSum::new(
                Box::new(weighted_features),
                Box::new(self.data.biases()[i].clone()),
            );
            
            // Apply ReLU activation
            input_to_next_layer = Box::new(Relu::new(Box::new(biased_features), 0.0));
        }
        
        // Output layer with Softmax activation
        Box::new(Softmax::new(input_to_next_layer))
    }
    
    /// Get the classifier data
    /// 
    /// Java: `public MLPClassifierData data() {return data;}`
    pub fn data(&self) -> &MLPClassifierData {
        &self.data
    }
}

impl Classifier for MLPClassifier {
    fn data(&self) -> &dyn ClassifierData {
        &self.data
    }
    
    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        self.predict_probabilities(features)
    }
    
    fn predict_probabilities_batch(&self, batch: &[usize], features: &dyn Features) -> Matrix {
        use crate::ml::core::batch::RangeBatch;
        let range_batch = RangeBatch::new(0, batch.len() as u64, batch.len() as u64);
        self.predict_probabilities_batch(&range_batch, features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::batch::RangeBatch;
    
    #[test]
    fn test_mlp_classifier_creation() {
        let data = MLPClassifierData::create(3, 5, &[10], 42);
        let classifier = MLPClassifier::new(data);
        
        assert_eq!(classifier.data().number_of_classes(), 3);
        assert_eq!(classifier.data().feature_dimension(), 5);
    }
    
    #[test]
    fn test_predict_probabilities() {
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
    }
    
    #[test]
    fn test_predictions_variable() {
        let data = MLPClassifierData::create(2, 3, &[4], 456);
        let classifier = MLPClassifier::new(data);
        
        let features = Matrix::from_row(&[1.0, 2.0, 3.0]);
        let features_var = Constant::new(Box::new(features));
        
        let predictions_var = classifier.predictions_variable(Box::new(features_var));
        
        // Should be able to forward pass
        let ctx = ComputationContext::new();
        let result = ctx.forward(predictions_var.as_ref());
        
        assert_eq!(result.rows(), 1);
        assert_eq!(result.cols(), 2);
    }
    
    #[test]
    fn test_batch_predictions() {
        let data = MLPClassifierData::create(2, 3, &[4], 789);
        let classifier = MLPClassifier::new(data);
        
        // Simple test features
        struct TestFeatures;
        impl Features for TestFeatures {
            fn get(&self, _node_id: usize) -> Vec<f64> {
                vec![1.0, 2.0, 3.0]
            }
            
            fn feature_dimension(&self) -> usize {
                3
            }
        }
        
        let features = TestFeatures;
        let batch = RangeBatch::new(0, 2, 2);
        
        let predictions = classifier.predict_probabilities_batch(&batch, &features);
        
        assert_eq!(predictions.rows(), 2);
        assert_eq!(predictions.cols(), 2);
        
        // Each row should sum to 1.0 (softmax)
        for row in 0..predictions.rows() {
            let row_sum: f64 = (0..predictions.cols()).map(|col| predictions[(row, col)]).sum();
            assert!((row_sum - 1.0).abs() < 1e-10);
        }
    }
}

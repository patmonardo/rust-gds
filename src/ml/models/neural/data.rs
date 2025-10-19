//! MLP Classifier Data
//!
//! Translated from `MLPClassifierData.java` from Java GDS.

use crate::ml::core::functions::weights::Weights;
use crate::ml::core::tensor::{Matrix, Vector};
use crate::ml::core::variable::Variable;
use crate::ml::models::{BaseModelData, ClassifierData, TrainingMethod};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Data structure for MLP Classifier
/// 
/// This corresponds to MLPClassifierData in Java GDS.
/// Uses Weights objects for automatic differentiation support.
#[derive(Clone)]
pub struct MLPClassifierData {
    /// Number of classes
    number_of_classes: usize,
    /// Weight matrices for each layer
    weights: Vec<Weights>,
    /// Bias vectors for each layer  
    biases: Vec<Weights>,
}

impl MLPClassifierData {
    /// Create MLP classifier data with Kaiming initialization
    /// 
    /// This matches the Java `create` method with proper weight initialization.
    pub fn create(
        class_count: usize,
        feature_count: usize,
        hidden_layer_sizes: &[usize],
        random_seed: u64,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(random_seed);
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        
        let hidden_depth = hidden_layer_sizes.len();
        
        // First hidden layer: feature_count -> hidden_layer_sizes[0]
        weights.push(Self::generate_weights(hidden_layer_sizes[0], feature_count, &mut rng));
        biases.push(Self::generate_bias(hidden_layer_sizes[0], &mut rng));
        
        // Hidden layers: hidden_layer_sizes[i] -> hidden_layer_sizes[i+1]
        for i in 0..hidden_depth - 1 {
            weights.push(Self::generate_weights(hidden_layer_sizes[i + 1], hidden_layer_sizes[i], &mut rng));
            biases.push(Self::generate_bias(hidden_layer_sizes[i + 1], &mut rng));
        }
        
        // Output layer: hidden_layer_sizes[last] -> class_count
        weights.push(Self::generate_weights(class_count, hidden_layer_sizes[hidden_depth - 1], &mut rng));
        biases.push(Self::generate_bias(class_count, &mut rng));
        
        Self {
            number_of_classes: class_count,
            weights,
            biases,
        }
    }
    
    /// Generate weights matrix with Kaiming initialization for ReLU
    /// 
    /// Bounds are from Kaiming initialization: https://arxiv.org/abs/1502.01852
    /// Weight bound = sqrt(2.0 / input_dimension)
    fn generate_weights(rows: usize, cols: usize, rng: &mut impl Rng) -> Weights {
        let weight_bound = (2.0 / cols as f64).sqrt();
        let mut data = Vec::with_capacity(rows * cols);
        
        for _ in 0..rows * cols {
            data.push(rng.gen_range(-weight_bound..weight_bound));
        }
        
        let matrix = Matrix::new(data, rows, cols);
        Weights::new(Box::new(matrix))
    }
    
    /// Generate bias vector with Kaiming initialization
    fn generate_bias(dim: usize, rng: &mut impl Rng) -> Weights {
        let weight_bound = (2.0 / dim as f64).sqrt();
        let mut data = Vec::with_capacity(dim);
        
        for _ in 0..dim {
            data.push(rng.gen_range(-weight_bound..weight_bound));
        }
        
        let vector = Vector::new(data);
        Weights::new(Box::new(vector))
    }
    
    /// Get the depth of the network (number of layers)
    /// 
    /// Java: `default int depth() {return biases().size() + 1;}`
    pub fn depth(&self) -> usize {
        self.biases.len() + 1
    }
    
    /// Get the number of classes (output dimension)
    /// 
    /// Java: `default int numberOfClasses() {return biases().get(biases().size()-1).dimension(0);}`
    pub fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }
    
    /// Get the feature dimension (input dimension)
    /// 
    /// Java: `default int featureDimension() {return weights().get(0).dimension(Dimensions.COLUMNS_INDEX);}`
    pub fn feature_dimension(&self) -> usize {
        // The first weight matrix has feature_count columns
        if let Some(first_weight) = self.weights.first() {
            first_weight.dimensions()[1] // COLUMNS_INDEX = 1
        } else {
            0
        }
    }
    
    /// Get weight matrices
    /// 
    /// Java: `List<Weights<Matrix>> weights()`
    pub fn weights(&self) -> &Vec<Weights> {
        &self.weights
    }
    
    /// Get bias vectors
    /// 
    /// Java: `List<Weights<Vector>> biases()`
    pub fn biases(&self) -> &Vec<Weights> {
        &self.biases
    }
}

impl BaseModelData for MLPClassifierData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::MLPClassification
    }
    
    fn feature_dimension(&self) -> usize {
        self.feature_dimension()
    }
}

impl ClassifierData for MLPClassifierData {
    fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mlp_data_creation() {
        let data = MLPClassifierData::create(3, 10, &[50, 25], 42);
        
        assert_eq!(data.number_of_classes(), 3);
        assert_eq!(data.feature_dimension(), 10);
        assert_eq!(data.depth(), 3); // 2 hidden + 1 output
        assert_eq!(data.weights().len(), 3); // 2 hidden + 1 output
        assert_eq!(data.biases().len(), 3); // 2 hidden + 1 output
        
        // Check dimensions
        assert_eq!(data.weights()[0].dimensions(), vec![50, 10]); // 50x10
        assert_eq!(data.weights()[1].dimensions(), vec![25, 50]); // 25x50  
        assert_eq!(data.weights()[2].dimensions(), vec![3, 25]);  // 3x25
        
        assert_eq!(data.biases()[0].dimensions(), vec![50]); // 50
        assert_eq!(data.biases()[1].dimensions(), vec![25]); // 25
        assert_eq!(data.biases()[2].dimensions(), vec![3]);  // 3
    }
    
    #[test]
    fn test_kaiming_initialization() {
        let data = MLPClassifierData::create(2, 100, &[50], 123);
        
        // Check that weights are initialized with proper bounds
        let first_weight = data.weights()[0].snapshot();
        let expected_bound = (2.0 / 100.0).sqrt(); // sqrt(2/100) ≈ 0.141
        
        for &value in first_weight.data() {
            assert!(value.abs() <= expected_bound + 1e-10); // Allow small floating point errors
        }
    }
}

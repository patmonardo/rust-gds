//! MLP Classifier Training Configuration
//!
//! Translated from `MLPClassifierTrainConfig.java` from Java GDS.

use crate::ml::gradient_descent::GradientDescentConfig;
use crate::ml::models::{ClassAwareTrainerConfig, PenaltyConfig, TrainingMethod};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Configuration for MLP Classifier training
/// 
/// This corresponds to MLPClassifierTrainConfig in Java GDS.
/// Combines gradient descent, penalty, and class-aware configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct MLPClassifierTrainConfig {
    // Gradient Descent Configuration
    #[builder(default = "100")]
    pub batch_size: usize,
    
    #[builder(default = "1")]
    pub min_epochs: usize,
    
    #[builder(default = "1")]
    pub patience: usize,
    
    #[builder(default = "100")]
    pub max_epochs: usize,
    
    #[builder(default = "1e-3")]
    pub tolerance: f64,
    
    #[builder(default = "0.001")]
    pub learning_rate: f64,
    
    // Penalty Configuration
    #[builder(default = "0.0")]
    pub penalty: f64,
    
    // Class-Aware Configuration
    #[builder(default = "0.0")]
    pub focus_weight: f64,
    
    // MLP-Specific Configuration
    #[builder(default = "vec![100]")]
    pub hidden_layer_sizes: Vec<usize>,
}

impl MLPClassifierTrainConfig {
    /// Create a new MLP classifier training configuration
    pub fn builder() -> MLPClassifierTrainConfigBuilder {
        MLPClassifierTrainConfigBuilder::default()
    }
    
    /// Create default configuration
    /// 
    /// Java: `MLPClassifierTrainConfig DEFAULT = of(Map.of());`
    pub fn default() -> Self {
        Self {
            batch_size: 100,
            min_epochs: 1,
            patience: 1,
            max_epochs: 100,
            tolerance: 1e-3,
            learning_rate: 0.001,
            penalty: 0.0,
            focus_weight: 0.0,
            hidden_layer_sizes: vec![100],
        }
    }
    
    /// Get hidden layer sizes
    /// 
    /// Java: `default List<Integer> hiddenLayerSizes() {return List.of(100);}`
    pub fn hidden_layer_sizes(&self) -> &Vec<usize> {
        &self.hidden_layer_sizes
    }
}

// Remove trait implementations - these are structs, not traits

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = MLPClassifierTrainConfig::default();
        
        assert_eq!(config.batch_size(), 100);
        assert_eq!(config.min_epochs(), 1);
        assert_eq!(config.patience(), 1);
        assert_eq!(config.max_epochs(), 100);
        assert_eq!(config.tolerance(), 1e-3);
        assert_eq!(config.learning_rate(), 0.001);
        assert_eq!(config.penalty(), 0.0);
        assert_eq!(config.focus_weight(), 0.0);
        assert_eq!(config.hidden_layer_sizes(), &vec![100]);
        assert_eq!(config.method(), TrainingMethod::MLPClassification);
    }
    
    #[test]
    fn test_builder_config() {
        let config = MLPClassifierTrainConfig::builder()
            .batch_size(50)
            .max_epochs(200)
            .learning_rate(0.01)
            .penalty(0.1)
            .focus_weight(2.0)
            .hidden_layer_sizes(vec![64, 32])
            .build()
            .unwrap();
        
        assert_eq!(config.batch_size(), 50);
        assert_eq!(config.max_epochs(), 200);
        assert_eq!(config.learning_rate(), 0.01);
        assert_eq!(config.penalty(), 0.1);
        assert_eq!(config.focus_weight(), 2.0);
        assert_eq!(config.hidden_layer_sizes(), &vec![64, 32]);
    }
    
    #[test]
    fn test_class_weights_initialization() {
        let config = MLPClassifierTrainConfig::default();
        let class_weights = config.initialize_class_weights(3);
        
        assert_eq!(class_weights, vec![1.0, 1.0, 1.0]);
    }
}

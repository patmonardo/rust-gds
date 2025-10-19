//! MLP Classifier Trainer
//!
//! Translated from `MLPClassifierTrainer.java` from Java GDS.

use crate::collections::HugeIntArray;
use crate::ml::core::batch::{BatchQueue, ConsecutiveBatchQueue};
use crate::ml::gradient_descent::Training;
use crate::ml::models::{Classifier, ClassifierTrainer, Features};
use rand::{SeedableRng, RngCore};
use rand::rngs::StdRng;
use std::sync::Arc;
use parking_lot::RwLock;

use super::{
    classifier::MLPClassifier,
    config::MLPClassifierTrainConfig,
    data::MLPClassifierData,
    objective::MLPClassifierObjective,
};

/// Trainer for MLP Classifier
/// 
/// This corresponds to MLPClassifierTrainer in Java GDS.
/// Uses the gradient descent system for training.
pub struct MLPClassifierTrainer {
    number_of_classes: usize,
    train_config: MLPClassifierTrainConfig,
    random: StdRng,
    termination_flag: Arc<RwLock<bool>>,
    concurrency: usize,
}

impl MLPClassifierTrainer {
    /// Create a new MLP classifier trainer
    /// 
    /// Java: `public MLPClassifierTrainer(int numberOfClasses, MLPClassifierTrainConfig trainConfig, Optional<Long> randomSeed, ...)`
    pub fn new(
        number_of_classes: usize,
        train_config: MLPClassifierTrainConfig,
        random_seed: Option<u64>,
        concurrency: usize,
    ) -> Self {
        let random = if let Some(seed) = random_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        Self {
            number_of_classes,
            train_config,
            random,
            termination_flag: Arc::new(RwLock::new(false)),
            concurrency,
        }
    }
    
    /// Train the MLP classifier
    /// 
    /// Java: `public MLPClassifier train(Features features, HugeIntArray labels, ReadOnlyHugeLongArray trainSet)`
    pub fn train(
        &mut self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &[u64],
    ) -> MLPClassifier {
        // Create MLP classifier data
        let data = MLPClassifierData::create(
            self.number_of_classes,
            features.feature_dimension(),
            self.train_config.hidden_layer_sizes,
            self.random.next_u64(),
        );
        
        let classifier = MLPClassifier::new(data);
        
        // Create objective function
        let objective = MLPClassifierObjective::new(
            classifier,
            features,
            labels,
            self.train_config.penalty,
            self.train_config.focus_weight,
            vec![1.0; self.number_of_classes],
        );
        
        // Create training instance
        // Create training instance
        let gradient_config = crate::ml::gradient_descent::GradientDescentConfig::builder()
            .batch_size(self.train_config.batch_size)
            .min_epochs(self.train_config.min_epochs)
            .patience(self.train_config.patience)
            .max_epochs(self.train_config.max_epochs)
            .tolerance(self.train_config.tolerance)
            .learning_rate(self.train_config.learning_rate)
            .build()
            .unwrap();
        
        let training = Training::new(gradient_config, train_set.len());
        
        // Create batch queue supplier
        let queue_supplier = || {
            Box::new(ConsecutiveBatchQueue::new(train_set.len() as u64, self.train_config.batch_size)) as Box<dyn BatchQueue>
        };
        
        // Train the model
        training.train(&objective, queue_supplier, self.concurrency);
        
        // Return the trained classifier
        objective.classifier
    }
    
    /// Get the training configuration
    pub fn train_config(&self) -> &MLPClassifierTrainConfig {
        &self.train_config
    }
    
    /// Get the number of classes
    pub fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }
}

impl ClassifierTrainer for MLPClassifierTrainer {
    fn train(
        &self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Classifier> {
        // Create MLP classifier data
        let data = MLPClassifierData::create(
            self.number_of_classes,
            features.feature_dimension(),
            self.train_config.hidden_layer_sizes,
            self.random.next_u64(),
        );
        
        let classifier = MLPClassifier::new(data);
        
        // Create objective function
        let objective = MLPClassifierObjective::new(
            classifier,
            features,
            labels,
            self.train_config.penalty,
            self.train_config.focus_weight,
            vec![1.0; self.number_of_classes],
        );
        
        // Create training instance
        // Create training instance
        let gradient_config = crate::ml::gradient_descent::GradientDescentConfig::builder()
            .batch_size(self.train_config.batch_size)
            .min_epochs(self.train_config.min_epochs)
            .patience(self.train_config.patience)
            .max_epochs(self.train_config.max_epochs)
            .tolerance(self.train_config.tolerance)
            .learning_rate(self.train_config.learning_rate)
            .build()
            .unwrap();
        
        let training = Training::new(gradient_config, train_set.len());
        
        // Create batch queue supplier
        let queue_supplier = || {
            Box::new(ConsecutiveBatchQueue::new(train_set.len() as u64, self.train_config.batch_size)) as Box<dyn BatchQueue>
        };
        
        // Train the model
        training.train(&objective, queue_supplier, self.concurrency);
        
        // Return the trained classifier
        Box::new(objective.classifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trainer_creation() {
        let config = MLPClassifierTrainConfig::default();
        let trainer = MLPClassifierTrainer::new(3, config, Some(42), 1);
        
        assert_eq!(trainer.number_of_classes(), 3);
        assert_eq!(trainer.train_config().number_of_classes(), 3);
    }
    
    #[test]
    fn test_trainer_with_custom_config() {
        let config = MLPClassifierTrainConfig::builder()
            .batch_size(50)
            .max_epochs(10)
            .learning_rate(0.01)
            .hidden_layer_sizes(vec![64, 32])
            .build()
            .unwrap();
        
        let trainer = MLPClassifierTrainer::new(2, config, Some(123), 2);
        
        assert_eq!(trainer.number_of_classes(), 2);
        assert_eq!(trainer.train_config().batch_size(), 50);
        assert_eq!(trainer.train_config().max_epochs(), 10);
        assert_eq!(trainer.train_config().learning_rate(), 0.01);
        assert_eq!(trainer.train_config().hidden_layer_sizes(), &vec![64, 32]);
    }
    
    #[test]
    fn test_trainer_without_seed() {
        let config = MLPClassifierTrainConfig::default();
        let trainer = MLPClassifierTrainer::new(3, config, None, 1);
        
        assert_eq!(trainer.number_of_classes(), 3);
    }
    
    #[test]
    fn test_training_integration() {
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
        
        let config = MLPClassifierTrainConfig::builder()
            .max_epochs(1) // Just one epoch for testing
            .batch_size(2)
            .build()
            .unwrap();
        
        let mut trainer = MLPClassifierTrainer::new(2, config, Some(456), 1);
        let features = TestFeatures;
        let labels = HugeIntArray::from_vec(vec![0, 1, 0, 1]);
        let train_set = vec![0, 1, 2, 3];
        
        let classifier = trainer.train(&features, &labels, &train_set);
        
        assert_eq!(classifier.data().number_of_classes(), 2);
        assert_eq!(classifier.data().feature_dimension(), 3);
        
        // Test prediction
        let test_features = vec![1.0, 2.0, 3.0];
        let probabilities = classifier.predict_probabilities(&test_features);
        
        assert_eq!(probabilities.len(), 2);
        let sum: f64 = probabilities.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10); // Should sum to 1.0 due to softmax
    }
}

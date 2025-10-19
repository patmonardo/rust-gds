use serde::{Deserialize, Serialize};

// Re-export TrainerConfig trait from base module
pub use super::base::TrainerConfigTrait as TrainerConfig;

/// Concrete trainer config structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseTrainerConfig {
    /// Maximum number of iterations
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Convergence tolerance
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    /// Learning rate
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f64,

    /// Random seed for reproducibility
    #[serde(default = "default_seed")]
    pub seed: u64,
}

impl Default for BaseTrainerConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            learning_rate: default_learning_rate(),
            seed: default_seed(),
        }
    }
}

fn default_max_iterations() -> usize {
    100
}
fn default_tolerance() -> f64 {
    1e-4
}
fn default_learning_rate() -> f64 {
    0.01
}
fn default_seed() -> u64 {
    42
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PenaltyConfig {
    /// L1 regularization strength
    #[serde(default)]
    pub l1: f64,

    /// L2 regularization strength  
    #[serde(default)]
    pub l2: f64,
}

impl Default for PenaltyConfig {
    fn default() -> Self {
        Self { l1: 0.0, l2: 0.0 }
    }
}

/// Additional config for trainers that require class info
/// 1:1 with ClassAwareTrainerConfig.java
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassAwareTrainerConfig {
    #[serde(flatten)]
    pub base: BaseTrainerConfig,

    /// Focus weight for imbalanced classes
    #[serde(default)]
    pub focus_weight: f64,

    /// Class weights for imbalanced problems
    #[serde(default)]
    pub class_weights: Vec<f64>,
}

impl ClassAwareTrainerConfig {
    /// Initialize class weights based on number of classes
    /// 1:1 with initializeClassWeights() in Java
    pub fn initialize_class_weights(&self, number_of_classes: usize) -> Vec<f64> {
        if self.class_weights.is_empty() {
            vec![1.0; number_of_classes]
        } else {
            if self.class_weights.len() != number_of_classes {
                panic!(
                    "The classWeights list {:?} has {} entries, but it should have {} entries instead, which is the number of classes.",
                    self.class_weights,
                    self.class_weights.len(),
                    number_of_classes
                );
            }
            self.class_weights.clone()
        }
    }
}

use crate::ml::core::tensor::Matrix;
use crate::ml::models::training_method::TrainingMethod;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

/// Trainer config trait - defined here to avoid circular dependency
pub trait TrainerConfigTrait: Send + Sync {
    /// Get the training method
    fn method(&self) -> TrainingMethod;

    /// Convert to map for serialization (matches Java's ToMapConvertible)
    fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value>;
}

/// Base model data trait - 1:1 with BaseModelData.java
/// All model data must implement this to specify training method and feature dimension
pub trait BaseModelData: Send + Sync + Debug {
    /// Get the training method used to create this model
    fn trainer_method(&self) -> TrainingMethod;

    /// Get the number of features (feature dimension)
    fn feature_dimension(&self) -> usize;
}

/// Classifier data trait - extends BaseModelData
/// 1:1 with Classifier.ClassifierData in Java
pub trait ClassifierData: BaseModelData {
    /// Get the number of classes in the classification problem
    fn number_of_classes(&self) -> usize;
}

/// Regressor data trait - extends BaseModelData
/// 1:1 with Regressor.RegressorData in Java
pub trait RegressorData: BaseModelData {}

/// Core classifier trait - 1:1 with Classifier.java
pub trait Classifier: Send + Sync + Debug {
    /// Get classifier data
    fn data(&self) -> &dyn ClassifierData;

    /// Number of classes (convenience method)
    fn number_of_classes(&self) -> usize {
        self.data().number_of_classes()
    }

    /// Predict class probabilities for a single feature vector
    /// Returns probability distribution over classes
    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64>;

    /// Predict class probabilities for a batch of features
    /// batch: indices into the feature store
    /// features: the feature store
    fn predict_probabilities_batch(&self, batch: &[usize], features: &dyn Features) -> Matrix;
}

/// Core regressor trait - 1:1 with Regressor.java
pub trait Regressor: Send + Sync + Debug {
    /// Get regressor data
    fn data(&self) -> &dyn RegressorData;

    /// Predict a single value for given features
    fn predict(&self, features: &[f64]) -> f64;

    /// Get self as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Features trait - 1:1 with Features.java
pub trait Features: Send + Sync {
    /// Total number of feature vectors
    fn size(&self) -> usize;

    /// Get feature vector for a given id
    fn get(&self, id: usize) -> &[f64];

    /// Get feature dimension (number of features per vector)
    fn feature_dimension(&self) -> usize {
        if self.size() > 0 {
            self.get(0).len()
        } else {
            0
        }
    }
}

/// Model data storage for serialization - kept for backward compatibility
pub trait ModelData: Send + Sync + Debug {
    /// Convert model data to bytes for serialization
    fn to_bytes(&self) -> Result<Vec<u8>>;

    /// Reconstruct model data from bytes
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;

    /// Get number of features the model uses
    fn num_features(&self) -> usize;
}

/// Legacy base model data structure - being phased out
/// Note: Serialization not supported due to Matrix type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyBaseModelData {
    /// Number of features the model expects
    pub num_features: usize,

    /// Model-specific parameters stored as a tensor
    pub weights: Matrix,
}

impl ModelData for LegacyBaseModelData {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(bytes)?)
    }

    fn num_features(&self) -> usize {
        self.num_features
    }
}

/// Classifier trainer trait - 1:1 with ClassifierTrainer.java
pub trait ClassifierTrainer: Send + Sync {
    /// Train a classifier
    /// features: feature store
    /// labels: class labels (HugeIntArray in Java, HugeIntArray in Rust)
    /// train_set: indices of training samples
    fn train(
        &self,
        features: &dyn Features,
        labels: &crate::collections::HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Classifier>;
}

/// Regressor trainer trait - 1:1 with RegressorTrainer.java
pub trait RegressorTrainer: Send + Sync {
    /// Train a regressor
    /// features: feature store
    /// targets: target values
    /// train_set: indices of training samples
    fn train(
        &self,
        features: &dyn Features,
        targets: &crate::collections::HugeDoubleArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Regressor>;
}

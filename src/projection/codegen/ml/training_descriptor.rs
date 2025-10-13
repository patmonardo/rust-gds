//! Training configuration descriptor.
//!
//! Maps to Java training configuration in ml-algo package.

use serde::{Deserialize, Serialize};

/// Training configuration.
///
/// Maps to Java training parameters in PipelineTrainer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDescriptor {
    /// Number of training epochs
    pub epochs: usize,

    /// Learning rate
    pub learning_rate: f64,

    /// Batch size
    pub batch_size: usize,
}

impl TrainingDescriptor {
    /// Create a new training descriptor with default values.
    pub fn new() -> Self {
        Self {
            epochs: 100,
            learning_rate: 0.001,
            batch_size: 32,
        }
    }

    /// Set number of epochs.
    pub fn with_epochs(mut self, epochs: usize) -> Self {
        self.epochs = epochs;
        self
    }

    /// Set learning rate.
    pub fn with_learning_rate(mut self, learning_rate: f64) -> Self {
        self.learning_rate = learning_rate;
        self
    }

    /// Set batch size.
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
}

impl Default for TrainingDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

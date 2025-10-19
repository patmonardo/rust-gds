//! Linear Regression trainer implementation.
//!
//! Direct translation of `LinearRegressionTrainer.java` from Java GDS.

use crate::collections::HugeDoubleArray;
use crate::ml::{
    core::batch::consecutive_with_batch_size,
    gradient_descent::{Objective, Training},
    models::{Features, RegressorTrainer},
};
use parking_lot::RwLock;
use std::sync::Arc;

use super::{
    config::LinearRegressionTrainConfig, objective::LinearRegressionObjective,
    regressor::LinearRegressor,
};

/// Trainer for linear regression models.
///
/// This struct orchestrates the training process by:
/// 1. Creating a LinearRegressionObjective with the provided features, targets, and penalty
/// 2. Setting up batch queues for gradient descent
/// 3. Running the Training process
/// 4. Returning a trained LinearRegressor
pub struct LinearRegressionTrainer {
    concurrency: usize,
    _termination_flag: Arc<RwLock<bool>>, // Reserved for future early termination support
    train_config: LinearRegressionTrainConfig,
}

impl LinearRegressionTrainer {
    /// Create a new LinearRegressionTrainer with the given configuration and resources.
    ///
    /// # Arguments
    /// * `concurrency` - Number of concurrent threads for training
    /// * `config` - Training configuration including gradient descent and penalty parameters
    /// * `termination_flag` - Flag to signal early termination
    pub fn new(
        concurrency: usize,
        config: LinearRegressionTrainConfig,
        termination_flag: Arc<RwLock<bool>>,
    ) -> Self {
        Self {
            concurrency,
            train_config: config,
            _termination_flag: termination_flag,
        }
    }
}

impl RegressorTrainer for LinearRegressionTrainer {
    /// Train a linear regression model using gradient descent.
    ///
    /// This method mirrors the Java `LinearRegressionTrainer.train()` method:
    /// 1. Creates a LinearRegressionObjective with features, targets, and penalty
    /// 2. Sets up batch queues from the training set
    /// 3. Runs gradient descent training
    /// 4. Returns the trained LinearRegressor
    fn train(
        &self,
        features: &dyn Features,
        targets: &HugeDoubleArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn crate::ml::models::Regressor> {
        // Create objective with features, targets, and penalty from config
        let objective =
            LinearRegressionObjective::new(features, targets, self.train_config.penalty());

        // Create batch queue supplier - matches Java's BatchQueue.fromArray(trainSet, batchSize)
        let queue_supplier =
            || consecutive_with_batch_size(train_set.len() as u64, self.train_config.batch_size());

        // Create training instance with config and progress tracking
        let training = Training::new(self.train_config.gradient().clone(), train_set.len());

        // Run gradient descent training
        training.train(&objective, queue_supplier, self.concurrency);

        // Return trained regressor with the optimized model data
        Box::new(LinearRegressor::new(objective.model_data().clone()))
    }
}

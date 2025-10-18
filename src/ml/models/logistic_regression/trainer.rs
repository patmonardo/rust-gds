use super::{
    classifier::LogisticRegressionClassifier, config::LogisticRegressionTrainConfig,
    data::LogisticRegressionData, objective::LogisticRegressionObjective,
};
use crate::ml::{
    core::{
        batch::{Batch, BatchQueue},
        progress::ProgressTracker,
    },
    gradient_descent::Training,
    models::{ClassifierTrainer, Features},
};
use parking_lot::RwLock;
use std::sync::Arc;

/// Trainer for logistic regression models
pub struct LogisticRegressionTrainer {
    train_config: LogisticRegressionTrainConfig,
    number_of_classes: usize,
    progress_tracker: Arc<ProgressTracker>,
    termination_flag: Arc<RwLock<bool>>,
    reduce_class_count: bool,
    concurrency: usize,
}

impl LogisticRegressionTrainer {
    /// Creates a new LogisticRegressionTrainer
    pub fn new(
        train_config: LogisticRegressionTrainConfig,
        number_of_classes: usize,
        reduce_class_count: bool,
        progress_tracker: Arc<ProgressTracker>,
        termination_flag: Arc<RwLock<bool>>,
        concurrency: usize,
    ) -> Self {
        Self {
            train_config,
            number_of_classes,
            progress_tracker,
            termination_flag,
            reduce_class_count,
            concurrency,
        }
    }
}

impl ClassifierTrainer for LogisticRegressionTrainer {
    type Config = LogisticRegressionTrainConfig;
    type Model = LogisticRegressionClassifier;

    fn train(
        &self,
        features: Arc<Features>,
        labels: Arc<RwLock<Vec<i32>>>,
        train_set: Arc<Vec<usize>>,
    ) -> Self::Model {
        let data = if self.reduce_class_count {
            LogisticRegressionData::with_reduced_class_count(
                features.feature_dimension(),
                self.number_of_classes,
            )
        } else {
            LogisticRegressionData::standard(features.feature_dimension(), self.number_of_classes)
        };

        let classifier = LogisticRegressionClassifier::from(data);

        let objective = LogisticRegressionObjective::new(
            classifier,
            self.train_config.penalty,
            features,
            labels,
            self.train_config.focus_weight,
            self.train_config
                .class_weights
                .clone()
                .unwrap_or_else(|| vec![1.0; self.number_of_classes]),
        );

        let training = Training::new(
            self.train_config.clone(),
            self.progress_tracker.clone(),
            self.termination_flag.clone(),
            train_set.len(),
        );

        let queue_supplier =
            move || BatchQueue::from_array(train_set.clone(), self.train_config.batch_size());

        training.train(objective, queue_supplier, self.concurrency)
    }
}

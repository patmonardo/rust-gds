use super::{
    classifier::LogisticRegressionClassifier, config::LogisticRegressionTrainConfig,
    data::LogisticRegressionData, objective::LogisticRegressionObjective,
};
use crate::ml::{
    core::batch::consecutive_with_batch_size,
    gradient_descent::{GradientDescentConfig, Training},
    models::{ClassifierTrainer, Features},
};
use parking_lot::RwLock;
use std::sync::Arc;

/// Trainer for logistic regression models
#[derive(Debug)]
pub struct LogisticRegressionTrainer {
    train_config: LogisticRegressionTrainConfig,
    number_of_classes: usize,
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
        termination_flag: Arc<RwLock<bool>>,
        concurrency: usize,
    ) -> Self {
        Self {
            train_config,
            number_of_classes,
            termination_flag,
            reduce_class_count,
            concurrency,
        }
    }
}

impl ClassifierTrainer for LogisticRegressionTrainer {
    fn train(
        &self,
        features: &dyn Features,
        labels: &crate::collections::HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn crate::ml::models::Classifier> {
        let data = if self.reduce_class_count {
            LogisticRegressionData::with_reduced_class_count(
                features.feature_dimension(),
                self.number_of_classes,
            )
        } else {
            LogisticRegressionData::standard(features.feature_dimension(), self.number_of_classes)
        };

        let classifier = LogisticRegressionClassifier::from(data);

        // Convert HugeIntArray to Vec<i32> for labels
        let labels_vec: Vec<i32> = (0..labels.size()).map(|i| labels.get(i)).collect();
        let labels_arc = Arc::new(RwLock::new(labels_vec));

        let objective = LogisticRegressionObjective::new(
            classifier.clone(),
            self.train_config.penalty,
            features,
            labels_arc,
            self.train_config.focus_weight,
            self.train_config
                .class_weights
                .clone()
                .unwrap_or_else(|| vec![1.0; self.number_of_classes]),
        );

        let training = Training::new(
            GradientDescentConfig::builder()
                .batch_size(self.train_config.batch_size)
                .learning_rate(self.train_config.learning_rate)
                .max_epochs(self.train_config.max_epochs)
                .tolerance(self.train_config.tolerance)
                .build()
                .unwrap(),
            train_set.len(),
        );

        let queue_supplier =
            || consecutive_with_batch_size(train_set.len() as u64, self.train_config.batch_size);

        training.train(&objective, queue_supplier, self.concurrency);

        Box::new(classifier)
    }
}

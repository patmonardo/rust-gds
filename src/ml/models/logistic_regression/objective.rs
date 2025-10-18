use crate::ml::{
    core::{
        batch::Batch,
        functions::{
            constant::Constant,
            constant_scale::ConstantScale,
            element_sum::ElementSum,
            l2_norm_squared::L2NormSquared,
            reduced_cross_entropy_loss::ReducedCrossEntropyLoss,
            reduced_focal_loss::ReducedFocalLoss,
            Variable,
        },
        tensor::{Scalar, Tensor, Vector},
    },
    gradient_descent::Objective,
    models::Features,
};
use super::{classifier::LogisticRegressionClassifier, data::LogisticRegressionData};
use std::sync::Arc;
use parking_lot::RwLock;

/// Objective function for logistic regression training
pub struct LogisticRegressionObjective {
    classifier: LogisticRegressionClassifier,
    penalty: f64,
    features: Arc<Features>,
    labels: Arc<RwLock<Vec<i32>>>,
    focus_weight: f64,
    class_weights: Vec<f64>,
}

impl LogisticRegressionObjective {
    /// Creates a new LogisticRegressionObjective
    pub fn new(
        classifier: LogisticRegressionClassifier,
        penalty: f64,
        features: Arc<Features>,
        labels: Arc<RwLock<Vec<i32>>>,
        focus_weight: f64,
        class_weights: Vec<f64>,
    ) -> Self {
        assert!(!features.is_empty(), "Features cannot be empty");
        
        Self {
            classifier,
            penalty,
            features,
            labels,
            focus_weight,
            class_weights,
        }
    }

    /// Computes the penalty term for the batch
    fn penalty_for_batch(&self, batch: &Batch, train_size: usize) -> ConstantScale<Scalar> {
        ConstantScale::new(
            L2NormSquared::new(self.model_data().weights().clone()),
            (batch.size() as f64) * self.penalty / (train_size as f64),
        )
    }

    /// Computes the cross-entropy loss for the batch
    fn cross_entropy_loss(&self, batch: &Batch) -> Variable<Scalar> {
        let batch_labels = self.batch_label_vector(batch);
        let batch_features = self.batch_feature_matrix(batch, &self.features);
        let predictions = self.classifier.predictions_variable(batch_features);

        if self.focus_weight == 0.0 {
            ReducedCrossEntropyLoss::new(
                predictions,
                self.classifier.data().weights().clone(),
                self.classifier.data().bias().clone(),
                batch_features,
                batch_labels,
                self.class_weights.clone(),
            )
        } else {
            ReducedFocalLoss::new(
                predictions,
                self.classifier.data().weights().clone(),
                self.classifier.data().bias().clone(),
                batch_features,
                batch_labels,
                self.focus_weight,
                self.class_weights.clone(),
            )
        }
    }

    /// Creates a vector of labels for the batch
    fn batch_label_vector(&self, batch: &Batch) -> Constant<Vector> {
        let labels = self.labels.read();
        let mut batched_targets = Vector::zeros(batch.size());
        
        for (i, &id) in batch.element_ids().iter().enumerate() {
            batched_targets.set(i, labels[id] as f64);
        }

        Constant::vector(batched_targets)
    }
}

impl Objective for LogisticRegressionObjective {
    type ModelData = LogisticRegressionData;

    fn loss(&self, batch: &Batch, train_size: usize) -> Variable<Scalar> {
        let unpenalized_loss = self.cross_entropy_loss(batch);
        let penalty_variable = self.penalty_for_batch(batch, train_size);
        
        ElementSum::new(vec![unpenalized_loss, penalty_variable])
    }

    fn model_data(&self) -> &Self::ModelData {
        self.classifier.data()
    }
}
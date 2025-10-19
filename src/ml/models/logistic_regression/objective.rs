use crate::ml::{
    core::{
        batch::Batch,
        computation_context::ComputationContext,
        functions::{
            constant::Constant,
            constant_scale::ConstantScale,
            element_sum::ElementSum,
            l2_norm_squared::L2NormSquared,
            reduced_cross_entropy_loss::ReducedCrossEntropyLoss,
            reduced_focal_loss::ReducedFocalLoss,
        },
        tensor::Tensor,
        variable::Variable,
    },
    gradient_descent::{batch_feature_matrix, Objective},
    models::Features,
};
use super::{classifier::LogisticRegressionClassifier, data::LogisticRegressionData};
use std::sync::Arc;
use parking_lot::RwLock;

/// Objective function for logistic regression training
pub struct LogisticRegressionObjective<'a> {
    classifier: LogisticRegressionClassifier,
    penalty: f64,
    features: &'a dyn Features,
    labels: Arc<RwLock<Vec<i32>>>,
    focus_weight: f64,
    class_weights: Vec<f64>,
}

impl<'a> LogisticRegressionObjective<'a> {
    /// Creates a new LogisticRegressionObjective
    pub fn new(
        classifier: LogisticRegressionClassifier,
        penalty: f64,
        features: &'a dyn Features,
        labels: Arc<RwLock<Vec<i32>>>,
        focus_weight: f64,
        class_weights: Vec<f64>,
    ) -> Self {
        assert!(features.size() > 0, "Features cannot be empty");
        
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
    fn penalty_for_batch<B: Batch>(&self, batch: &B, train_size: usize) -> ConstantScale {
        let penalty_variable = L2NormSquared::new(Box::new(self.classifier.data().weights().clone()));
        let scale = (batch.size() as f64) * self.penalty / (train_size as f64);
        ConstantScale::new(Box::new(penalty_variable), scale)
    }

    /// Computes the cross-entropy loss for the batch
    fn cross_entropy_loss<B: Batch>(&self, batch: &B) -> Box<dyn crate::ml::core::variable::Variable> {
        let batch_labels = self.batch_label_vector(batch);
        let batch_features = batch_feature_matrix(batch, self.features);
        let ctx = ComputationContext::new();
        let tensor_data = batch_features.apply(&ctx);
        let batch_features_clone = Constant::new(tensor_data);
        let predictions = self.classifier.predictions_variable(batch_features);

        if self.focus_weight == 0.0 {
            Box::new(ReducedCrossEntropyLoss::new(
                predictions,
                Box::new(self.classifier.data().weights().clone()),
                Box::new(self.classifier.data().bias().clone()),
                Box::new(batch_features_clone),
                Box::new(batch_labels),
                self.class_weights.clone(),
            ))
        } else {
            Box::new(ReducedFocalLoss::new(
                predictions,
                Box::new(self.classifier.data().weights().clone()),
                Box::new(self.classifier.data().bias().clone()),
                Box::new(batch_features_clone),
                Box::new(batch_labels),
                self.focus_weight,
                self.class_weights.clone(),
            ))
        }
    }

    /// Creates a vector of labels for the batch
    fn batch_label_vector<B: Batch>(&self, batch: &B) -> Constant {
        let labels = self.labels.read();
        let mut batched_targets = Vec::with_capacity(batch.size());
        
        for element_id in batch.element_ids() {
            batched_targets.push(labels[element_id as usize] as f64);
        }

        Constant::vector(batched_targets)
    }
}

impl<'a> Objective for LogisticRegressionObjective<'a> {
    type ModelData = LogisticRegressionData;

    fn weight_handles(&self) -> Vec<Arc<RwLock<Box<dyn Tensor>>>> {
        vec![
            self.classifier.data().weights().handle(),
            self.classifier.data().bias().handle(),
        ]
    }

    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> Box<dyn crate::ml::core::variable::Variable> {
        let unpenalized_loss = self.cross_entropy_loss(batch);
        let penalty_variable = self.penalty_for_batch(batch, train_size);
        
        Box::new(ElementSum::new(vec![unpenalized_loss, Box::new(penalty_variable)]))
    }

    fn model_data(&self) -> &Self::ModelData {
        self.classifier.data()
    }
}
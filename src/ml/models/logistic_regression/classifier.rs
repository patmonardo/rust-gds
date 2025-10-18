use super::data::LogisticRegressionData;
use crate::ml::{
    core::{
        batch::Batch,
        functions::{
            constant::Constant, matrix_multiply::MatrixMultiplyWithTransposedSecondOperand,
            matrix_vector_sum::MatrixVectorSum, reduced_softmax::ReducedSoftmax, sigmoid::Sigmoid,
            softmax::Softmax, Variable,
        },
        tensor::Matrix,
        ComputationContext,
    },
    models::{Classifier, Features},
};

/// Logistic Regression classifier implementation
pub struct LogisticRegressionClassifier {
    data: LogisticRegressionData,
    prediction_strategy: PredictionStrategy,
}

enum PredictionStrategy {
    Binary,
    MultiClass,
}

impl LogisticRegressionClassifier {
    /// Creates a new LogisticRegressionClassifier from the given data
    pub fn from(data: LogisticRegressionData) -> Self {
        let prediction_strategy =
            if data.number_of_classes() == 2 && data.weights().data().rows() == 1 {
                // Binary classification case (e.g. link prediction)
                PredictionStrategy::Binary
            } else {
                // Multi-class case
                PredictionStrategy::MultiClass
            };

        Self {
            data,
            prediction_strategy,
        }
    }

    /// Creates the predictions variable for the computation graph
    pub(crate) fn predictions_variable(
        &self,
        batch_features: Constant<Matrix>,
    ) -> Variable<Matrix> {
        let weights = self.data.weights();
        let weighted_features =
            MatrixMultiplyWithTransposedSecondOperand::of(batch_features, weights.clone());
        let softmax_input = MatrixVectorSum::new(weighted_features, self.data.bias().clone());

        if weights.data().rows() == self.data.number_of_classes() {
            Softmax::new(softmax_input)
        } else {
            ReducedSoftmax::new(softmax_input)
        }
    }
}

impl Classifier for LogisticRegressionClassifier {
    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        match self.prediction_strategy {
            PredictionStrategy::Binary => {
                let mut affinity = 0.0;
                let weights = self.data.weights().data().data();

                for (i, &feature) in features.iter().enumerate() {
                    affinity += weights[i] * feature;
                }

                let sigmoid_val = Sigmoid::sigmoid(affinity + self.data.bias().data().data()[0]);
                vec![sigmoid_val, 1.0 - sigmoid_val]
            }
            PredictionStrategy::MultiClass => {
                let ctx = ComputationContext::new();
                let features_variable = Constant::matrix(features.to_vec(), 1, features.len());
                let predictions_variable = self.predictions_variable(features_variable);
                ctx.forward(predictions_variable).data().to_vec()
            }
        }
    }

    fn predict_probabilities_batch(&self, batch: &Batch, features: &Features) -> Matrix {
        let ctx = ComputationContext::new();
        let batch_features = self.batch_feature_matrix(batch, features);
        ctx.forward(self.predictions_variable(batch_features))
    }

    fn data(&self) -> &LogisticRegressionData {
        &self.data
    }
}

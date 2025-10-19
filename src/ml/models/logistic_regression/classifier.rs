use super::data::LogisticRegressionData;
use crate::ml::{
    core::{
        batch::Batch,
        functions::{
            constant::Constant, ewise_add_matrix_scalar::EWiseAddMatrixScalar,
            matrix_multiply_with_transposed_second_operand::MatrixMultiplyWithTransposedSecondOperand,
            sigmoid::Sigmoid, softmax::Softmax, reduced_softmax::ReducedSoftmax,
        },
        tensor::Matrix,
        ComputationContext,
        variable::Variable,
    },
    models::{Classifier, Features},
};

/// Logistic Regression classifier implementation
#[derive(Debug, Clone)]
pub struct LogisticRegressionClassifier {
    data: LogisticRegressionData,
    prediction_strategy: PredictionStrategy,
}

#[derive(Debug, Clone)]
enum PredictionStrategy {
    Binary,
    MultiClass,
}

impl LogisticRegressionClassifier {
    /// Creates a new LogisticRegressionClassifier from the given data
    pub fn from(data: LogisticRegressionData) -> Self {
        let prediction_strategy =
            if data.number_of_classes() == 2 && data.weights().borrow_matrix().rows() == 1 {
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

    /// Accessor for the underlying model data
    pub fn data(&self) -> &LogisticRegressionData {
        &self.data
    }

    /// Creates the predictions variable for the computation graph
    pub(crate) fn predictions_variable(
        &self,
        batch_features: Constant,
    ) -> Box<dyn Variable> {
        let weights = self.data.weights();
        let weighted_features = MatrixMultiplyWithTransposedSecondOperand::new(
            Box::new(batch_features),
            Box::new(weights.clone()),
        );
        let softmax_input = EWiseAddMatrixScalar::new(
            Box::new(weighted_features),
            Box::new(self.data.bias().clone()),
        );

        if weights.borrow_matrix().rows() == self.data.number_of_classes() {
            Box::new(Softmax::new(Box::new(softmax_input)))
        } else {
            Box::new(ReducedSoftmax::new(Box::new(softmax_input)))
        }
    }

    /// Creates batch feature matrix from batch and features
    fn batch_feature_matrix_from_indices(&self, batch: &[usize], features: &dyn Features) -> Constant {
        let rows = batch.len();
        let cols = features.feature_dimension();
        let mut batch_features = Matrix::zeros(rows, cols);

        for (row, &element_id) in batch.iter().enumerate() {
            let feature_vec = features.get(element_id);
            for col in 0..cols {
                batch_features[(row, col)] = feature_vec[col];
            }
        }

        Constant::new(Box::new(batch_features))
    }

    /// Creates batch feature matrix from batch and features
    fn batch_feature_matrix<B: Batch>(&self, batch: &B, features: &dyn Features) -> Constant {
        let rows = batch.size();
        let cols = features.feature_dimension();
        let mut batch_features = Matrix::zeros(rows, cols);
        let batch_iterator = batch.element_ids();
        let mut current_row = 0;

        for element_id in batch_iterator {
            let feature_vec = features.get(element_id as usize);
            for col in 0..cols {
                batch_features[(current_row, col)] = feature_vec[col];
            }
            current_row += 1;
        }

        Constant::new(Box::new(batch_features))
    }
}

impl Classifier for LogisticRegressionClassifier {
    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        match self.prediction_strategy {
            PredictionStrategy::Binary => {
                let mut affinity = 0.0;
                let weights_matrix = self.data.weights().borrow_matrix();

                for (i, &feature) in features.iter().enumerate() {
                    affinity += weights_matrix[(0, i)] * feature;
                }

                let bias_value = self.data.bias().borrow_vector()[0];
                let sigmoid_val = Sigmoid::sigmoid(affinity + bias_value);
                vec![sigmoid_val, 1.0 - sigmoid_val]
            }
            PredictionStrategy::MultiClass => {
                let ctx = ComputationContext::new();
                let features_variable = Constant::matrix(features.to_vec(), 1, features.len());
                let predictions_variable = self.predictions_variable(features_variable);
                let result = ctx.forward(predictions_variable.as_ref());
                result.data().to_vec()
            }
        }
    }

    fn predict_probabilities_batch(&self, batch: &[usize], features: &dyn Features) -> Matrix {
        let ctx = ComputationContext::new();
        let batch_features = self.batch_feature_matrix_from_indices(batch, features);
        let predictions = self.predictions_variable(batch_features);
        let result = ctx.forward(predictions.as_ref());
        let dimensions = result.dimensions();
        let rows = dimensions[0];
        let cols = dimensions[1];
        Matrix::new(result.data().to_vec(), rows, cols)
    }

    fn data(&self) -> &dyn crate::ml::models::ClassifierData {
        &self.data
    }
}

use super::functions::{Function, ReLU, Softmax};
use crate::ml::core::tensor::{Matrix, Tensor, Vector};
use crate::ml::models::{BaseModelData, Classifier, Model, ModelData};
use crate::types::prelude::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Multi-Layer Perceptron classifier
#[derive(Debug)]
pub struct MLPClassifier {
    data: MLPClassifierData,
}

impl MLPClassifier {
    pub fn new(data: MLPClassifierData) -> Self {
        Self { data }
    }

    /// Number of layers including input and output
    pub fn depth(&self) -> usize {
        self.data.biases.len() + 1
    }

    /// Forward pass through the network
    fn forward(&self, features: &Vector) -> Vector {
        let batch_features = Matrix::from_row(features);
        let mut current = batch_features;

        // Hidden layers with ReLU
        for i in 0..self.depth() - 1 {
            let weights = &self.data.weights[i];
            let biases = &self.data.biases[i];

            // Linear transformation
            current = current.matmul_transposed(weights);
            for row in 0..current.rows() {
                for col in 0..current.cols() {
                    current[(row, col)] += biases[col];
                }
            }

            // ReLU activation
            current = ReLU.forward(&current);
        }

        // Output layer with Softmax
        let probs = Softmax.forward(&current);
        probs.row(0).to_owned()
    }
}

impl Model for MLPClassifier {
    type Data = MLPClassifierData;
    type Prediction = usize; // Class index

    fn predict(&self, features: &Vector) -> usize {
        let probs = self.predict_proba(features);
        probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap()
    }
}

impl Classifier for MLPClassifier {
    fn num_classes(&self) -> usize {
        self.data.biases.last().unwrap().len()
    }

    fn predict_proba(&self, features: &Vector) -> Vector {
        self.forward(features)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPClassifierData {
    #[serde(flatten)]
    pub base: BaseModelData,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Vector>,
}

impl MLPClassifierData {
    pub fn create(
        num_classes: usize,
        num_features: usize,
        hidden_layer_sizes: &[usize],
        rng: &mut impl rand::Rng,
    ) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let hidden_depth = hidden_layer_sizes.len();

        // Input layer to first hidden layer
        let w_bound = (2.0 / num_features as f64).sqrt();
        weights.push(Matrix::random_uniform(
            hidden_layer_sizes[0],
            num_features,
            -w_bound,
            w_bound,
            rng,
        ));
        biases.push(Vector::random_uniform(
            hidden_layer_sizes[0],
            -w_bound,
            w_bound,
            rng,
        ));

        // Hidden layers
        for i in 0..hidden_depth - 1 {
            let w_bound = (2.0 / hidden_layer_sizes[i] as f64).sqrt();
            weights.push(Matrix::random_uniform(
                hidden_layer_sizes[i + 1],
                hidden_layer_sizes[i],
                -w_bound,
                w_bound,
                rng,
            ));
            biases.push(Vector::random_uniform(
                hidden_layer_sizes[i + 1],
                -w_bound,
                w_bound,
                rng,
            ));
        }

        // Output layer
        let w_bound = (2.0 / hidden_layer_sizes[hidden_depth - 1] as f64).sqrt();
        weights.push(Matrix::random_uniform(
            num_classes,
            hidden_layer_sizes[hidden_depth - 1],
            -w_bound,
            w_bound,
            rng,
        ));
        biases.push(Vector::random_uniform(num_classes, -w_bound, w_bound, rng));

        Self {
            base: BaseModelData {
                num_features,
                weights: weights[0].clone(),
            },
            weights,
            biases,
        }
    }
}

impl ModelData for MLPClassifierData {
    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Failed to serialize MLPClassifierData")
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization failed: {}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPClassifierConfig {
    /// Sizes of hidden layers
    #[serde(default = "default_hidden_layers")]
    pub hidden_layer_sizes: Vec<usize>,

    /// L2 regularization strength
    #[serde(default)]
    pub penalty: f64,

    /// Class weights for imbalanced problems
    #[serde(default)]
    pub class_weights: Option<Vec<f64>>,

    /// Focus weight for focal loss
    #[serde(default)]
    pub focus_weight: f64,
}

impl Default for MLPClassifierConfig {
    fn default() -> Self {
        Self {
            hidden_layer_sizes: default_hidden_layers(),
            penalty: 0.0,
            class_weights: None,
            focus_weight: 0.0,
        }
    }
}

fn default_hidden_layers() -> Vec<usize> {
    vec![100]
}

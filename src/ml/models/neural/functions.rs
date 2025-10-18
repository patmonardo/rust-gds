use crate::ml::core::tensor::{Matrix, Tensor, Vector};
use std::ops::{Add, Mul};

/// Neural network function trait
pub trait Function {
    type Input;
    type Output;
    type Gradient;

    fn forward(&self, input: &Self::Input) -> Self::Output;
    fn backward(&self, gradient: &Self::Gradient) -> Self::Gradient;
}

/// ReLU activation function
pub struct ReLU;

impl Function for ReLU {
    type Input = Matrix;
    type Output = Matrix;
    type Gradient = Matrix;

    fn forward(&self, input: &Matrix) -> Matrix {
        let mut output = input.clone();
        output.iter_mut().for_each(|x| *x = x.max(0.0));
        output
    }

    fn backward(&self, gradient: &Matrix) -> Matrix {
        let mut output = gradient.clone();
        output
            .iter_mut()
            .for_each(|x| *x = if *x > 0.0 { 1.0 } else { 0.0 });
        output
    }
}

/// Softmax activation function
pub struct Softmax;

impl Function for Softmax {
    type Input = Matrix;
    type Output = Matrix;
    type Gradient = Matrix;

    fn forward(&self, input: &Matrix) -> Matrix {
        let mut output = Matrix::zeros(input.rows(), input.cols());

        for row in 0..input.rows() {
            // Find max for numerical stability
            let max = input
                .row(row)
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            // Compute exp(x - max) for each element
            let mut exp_sum = 0.0;
            for col in 0..input.cols() {
                let exp_val = (input[(row, col)] - max).exp();
                output[(row, col)] = exp_val;
                exp_sum += exp_val;
            }

            // Normalize
            for col in 0..input.cols() {
                output[(row, col)] /= exp_sum;
            }
        }

        output
    }

    fn backward(&self, gradient: &Matrix) -> Matrix {
        // Gradient of softmax is more complex due to cross-dependencies
        // This is typically handled in the loss function for efficiency
        unimplemented!("Softmax backward pass is handled in CrossEntropyLoss")
    }
}

/// Cross-entropy loss with softmax
pub struct CrossEntropyLoss {
    class_weights: Option<Vector>,
}

impl CrossEntropyLoss {
    pub fn new(class_weights: Option<Vector>) -> Self {
        Self { class_weights }
    }

    pub fn compute_loss(&self, predictions: &Matrix, targets: &Vector) -> f64 {
        let batch_size = predictions.rows() as f64;
        let mut loss = 0.0;

        for i in 0..predictions.rows() {
            let true_class = targets[i] as usize;
            let mut sample_loss = -(predictions[(i, true_class)].ln());

            if let Some(ref weights) = self.class_weights {
                sample_loss *= weights[true_class];
            }

            loss += sample_loss;
        }

        loss / batch_size
    }

    pub fn compute_gradient(&self, predictions: &Matrix, targets: &Vector) -> Matrix {
        let mut gradient = predictions.clone();
        let batch_size = predictions.rows() as f64;

        for i in 0..predictions.rows() {
            let true_class = targets[i] as usize;

            for j in 0..predictions.cols() {
                let mut grad = predictions[(i, j)];
                if j == true_class {
                    grad -= 1.0;
                }

                if let Some(ref weights) = self.class_weights {
                    grad *= weights[j];
                }

                gradient[(i, j)] = grad / batch_size;
            }
        }

        gradient
    }
}

/// L2 regularization
pub struct L2Penalty {
    strength: f64,
}

impl L2Penalty {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }

    pub fn compute_penalty(&self, weights: &[&Matrix]) -> f64 {
        weights
            .iter()
            .map(|w| w.iter().map(|x| x.powi(2)).sum::<f64>())
            .sum::<f64>()
            * (self.strength / 2.0)
    }

    pub fn compute_gradient(&self, weight: &Matrix) -> Matrix {
        let mut gradient = weight.clone();
        gradient.iter_mut().for_each(|x| *x *= self.strength);
        gradient
    }
}

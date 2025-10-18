use crate::ml::core::tensor::Tensor;

/// Functional interface for parameter updates
pub trait Updater {
    /// Update weights using computed gradients
    fn update(&mut self, gradients: &[Box<dyn Tensor>]);
}

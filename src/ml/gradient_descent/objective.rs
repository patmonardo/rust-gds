use crate::ml::core::{
    batch::Batch,
    functions::Constant,
    tensor::{Matrix, Tensor},
    variable::Variable,
};
use crate::ml::models::Features;
use parking_lot::RwLock;
use std::sync::Arc;

/// A training objective that computes a loss over a batch of nodes
pub trait Objective {
    type ModelData;

    /// Handles to trainable weight tensors used by the objective.
    /// Each handle should point to shared tensor storage so optimizers can update in-place.
    fn weight_handles(&self) -> Vec<Arc<RwLock<Box<dyn Tensor>>>>;

    /// Computes the loss for a batch
    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> Box<dyn Variable>;

    /// Returns the model data needed for storage/loading
    fn model_data(&self) -> &Self::ModelData;
}

pub fn batch_feature_matrix<B: Batch>(batch: &B, features: &dyn Features) -> Constant {
    let rows = batch.size();
    let cols = features.feature_dimension();
    let mut batch_features = Matrix::zeros(rows, cols);
    let mut batch_iterator = batch.element_ids();
    let mut current_row = 0;

    while let Some(element_id) = batch_iterator.next() {
        let feature_vec = features.get(element_id as usize);
        for col in 0..cols {
            batch_features[(current_row, col)] = feature_vec[col];
        }
        current_row += 1;
    }

    Constant::new(Box::new(batch_features))
}

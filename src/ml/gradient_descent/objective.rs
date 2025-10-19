use crate::ml::core::{
    batch::Batch,
    functions::{Constant, weights::Weights},
    tensor::Matrix,
    variable::Variable,
};
use crate::ml::models::Features;

/// A training objective that computes a loss over a batch of nodes
pub trait Objective {
    type ModelData;

    /// Returns the Weights variables used in the computation graph.
    /// These are the actual Variable objects that can be used with ctx.gradient().
    fn weights(&self) -> Vec<Weights>;

    /// Computes the loss for a batch
    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> Box<dyn Variable>;

    /// Returns the model data needed for storage/loading
    fn model_data(&self) -> &Self::ModelData;
}

pub fn batch_feature_matrix<B: Batch>(batch: &B, features: &dyn Features) -> Constant {
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

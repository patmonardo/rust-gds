use super::functions::{CrossEntropyLoss, L2Penalty};
use super::mlp::{MLPClassifier, MLPClassifierConfig, MLPClassifierData};
use crate::ml::core::tensor::{Matrix, Vector};
use crate::ml::models::{Features, TrainerConfig};
use crate::types::prelude::*;
use anyhow::Result;
use rand::prelude::*;

pub struct MLPClassifierTrainer {
    num_classes: usize,
    config: MLPClassifierConfig,
    train_config: TrainerConfig,
    rng: Box<dyn RngCore>,
}

impl MLPClassifierTrainer {
    pub fn new(
        num_classes: usize,
        config: MLPClassifierConfig,
        train_config: TrainerConfig,
        seed: Option<u64>,
    ) -> Self {
        let rng = match seed {
            Some(s) => Box::new(StdRng::seed_from_u64(s)),
            None => Box::new(StdRng::from_entropy()),
        };

        Self {
            num_classes,
            config,
            train_config,
            rng,
        }
    }

    pub fn fit(
        &mut self,
        features: &dyn Features,
        target_property: &str,
        node_ids: &[NodeId],
        graph: &GraphStore,
    ) -> Result<MLPClassifier> {
        // Initialize model
        let data = MLPClassifierData::create(
            self.num_classes,
            features.dimension(),
            &self.config.hidden_layer_sizes,
            &mut self.rng,
        );
        let mut model = MLPClassifier::new(data);

        // Loss functions
        let cross_entropy = CrossEntropyLoss::new(
            self.config
                .class_weights
                .as_ref()
                .map(|w| Vector::from(w.clone())),
        );
        let l2_penalty = L2Penalty::new(self.config.penalty);

        // Extract targets
        let targets = self.extract_targets(target_property, node_ids, graph)?;

        // Training loop
        let mut prev_loss = f64::INFINITY;
        for epoch in 0..self.train_config.max_iterations {
            let mut total_loss = 0.0;
            let mut total_batches = 0;

            // Mini-batch gradient descent
            for batch in node_ids.chunks(64) {
                // TODO: Make batch size configurable
                let x = features.features_for_nodes(batch);
                let y: Vector = batch
                    .iter()
                    .map(|&id| targets[id as usize] as f64)
                    .collect();

                // Forward pass
                let mut activations = vec![Matrix::from_rows(&x)];
                let mut layer_inputs = Vec::new();

                for i in 0..model.depth() {
                    let weights = &model.data.weights[i];
                    let biases = &model.data.biases[i];

                    // Linear transformation
                    let mut z = activations.last().unwrap().matmul_transposed(weights);
                    for row in 0..z.rows() {
                        for col in 0..z.cols() {
                            z[(row, col)] += biases[col];
                        }
                    }
                    layer_inputs.push(z.clone());

                    // Activation function
                    let a = if i == model.depth() - 1 {
                        super::functions::Softmax.forward(&z)
                    } else {
                        super::functions::ReLU.forward(&z)
                    };
                    activations.push(a);
                }

                // Compute loss
                let predictions = activations.last().unwrap();
                let ce_loss = cross_entropy.compute_loss(predictions, &y);
                let l2_loss =
                    l2_penalty.compute_penalty(&model.data.weights.iter().collect::<Vec<_>>());
                total_loss += ce_loss + l2_loss;
                total_batches += 1;

                // Backward pass
                let mut gradient = cross_entropy.compute_gradient(predictions, &y);

                for i in (0..model.depth()).rev() {
                    // Layer gradients
                    let d_weights = activations[i].matmul_transposed(&gradient);
                    let d_biases = gradient.col_sum();

                    // Update gradients with L2 penalty
                    if self.config.penalty > 0.0 {
                        let l2_grad = l2_penalty.compute_gradient(&model.data.weights[i]);
                        d_weights.add_assign(&l2_grad);
                    }

                    // Update parameters
                    let lr = self.train_config.learning_rate;
                    model.data.weights[i].sub_assign(&d_weights.scale(lr));
                    model.data.biases[i].sub_assign(&d_biases.scale(lr));

                    // Propagate gradient if not input layer
                    if i > 0 {
                        let w_grad = gradient.matmul(&model.data.weights[i]);
                        if i < model.depth() - 1 {
                            // ReLU gradient
                            w_grad.iter_mut().for_each(|g| {
                                *g *= if layer_inputs[i - 1][0] > 0.0 {
                                    1.0
                                } else {
                                    0.0
                                }
                            });
                        }
                        gradient = w_grad;
                    }
                }
            }

            // Check convergence
            let avg_loss = total_loss / total_batches as f64;
            if (prev_loss - avg_loss).abs() < self.train_config.tolerance {
                break;
            }
            prev_loss = avg_loss;
        }

        Ok(model)
    }

    fn extract_targets(
        &self,
        target_property: &str,
        node_ids: &[NodeId],
        graph: &GraphStore,
    ) -> Result<Vec<usize>> {
        // TODO: Implement target extraction from graph store
        unimplemented!("Target extraction not yet implemented")
    }
}

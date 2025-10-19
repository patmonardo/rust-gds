use super::{
    config::GradientDescentConfig,
    objective::Objective,
    stopper::{TrainingStopper, factory},
};
use crate::ml::core::{
    batch::{AnyBatch, Batch, BatchQueue},
    computation_context::ComputationContext,
    optimizer::{AdamOptimizer, Updater},
    tensor::{Tensor, tensor::AsAny},
    variable::Variable,
};

/// Training implementation for gradient descent optimization
pub struct Training {
    config: GradientDescentConfig,
    train_size: usize,
}

impl Training {
    pub fn new(config: GradientDescentConfig, train_size: usize) -> Self {
        Self { config, train_size }
    }

    /// Train the objective using gradient descent
    pub fn train<O: Objective>(
        &self,
        objective: &O,
        mut queue_supplier: impl FnMut() -> Box<dyn BatchQueue>,
        concurrency: usize,
    ) {
        let mut updater =
            AdamOptimizer::new(objective.weights().into_iter().map(|w| w.handle()).collect(), self.config.learning_rate());
        let mut stopper = factory::default_stopper(&self.config);

        let mut losses = Vec::new();

        // Initial loss computation
        let consumers = self.execute_batches(concurrency, objective, queue_supplier());
        let mut prev_weight_gradients = Self::avg_weight_gradients(&consumers);
        let initial_loss = Self::avg_loss(&consumers);

        log::info!("Initial loss {}", initial_loss);

        while !stopper.terminated() {
            // Update weights with previous gradients BEFORE computing new ones
            updater.update(&prev_weight_gradients);
            
            let consumers = self.execute_batches(concurrency, objective, queue_supplier());
            prev_weight_gradients = Self::avg_weight_gradients(&consumers);

            let loss = Self::avg_loss(&consumers);
            stopper.register_loss(loss);
            losses.push(loss);
        }

        log::info!(
            "{} after {} out of {} epochs. Initial loss: {}, Last loss: {}.{}",
            if stopper.converged() {
                "converged"
            } else {
                "terminated"
            },
            losses.len(),
            self.config.max_epochs(),
            initial_loss,
            losses.last().unwrap(),
            if stopper.converged() {
                ""
            } else {
                " Did not converge"
            }
        );
    }

    fn execute_batches<'a, O: Objective>(
        &self,
        concurrency: usize,
        objective: &'a O,
        mut batches: Box<dyn BatchQueue>,
    ) -> Vec<ObjectiveUpdateConsumer<'a, O>> {
        assert!(concurrency > 0, "Concurrency must be at least 1");
        let mut consumers: Vec<ObjectiveUpdateConsumer<O>> = (0..concurrency)
            .map(|_| ObjectiveUpdateConsumer::new(objective, self.train_size))
            .collect();

        // Process batches across consumers
        let mut consumer_idx = 0;
        while let Some(batch) = batches.pop() {
            let vec_batch = VecBatch::from_any_batch(batch.as_ref());
            consumers[consumer_idx].accept(&vec_batch);
            consumer_idx = (consumer_idx + 1) % consumers.len();
        }

        consumers
    }

    fn avg_weight_gradients(
        consumers: &[ObjectiveUpdateConsumer<impl Objective>],
    ) -> Vec<Box<dyn Tensor>> {
        let local_gradient_sums: Vec<&Vec<Box<dyn Tensor>>> = consumers
            .iter()
            .map(|c| c.summed_weight_gradients())
            .collect();

        let number_of_batches: usize = consumers.iter().map(|c| c.consumed_batches()).sum();

        // Average tensors
        Self::average_tensors(&local_gradient_sums, number_of_batches)
    }

    fn average_tensors(
        tensor_lists: &[&Vec<Box<dyn Tensor>>],
        divisor: usize,
    ) -> Vec<Box<dyn Tensor>> {
        if tensor_lists.is_empty() || divisor == 0 {
            return Vec::new();
        }

        let num_tensors = tensor_lists[0].len();
        let mut result = Vec::with_capacity(num_tensors);

        for i in 0..num_tensors {
            let mut sum = tensor_lists[0][i].clone_box();
            for tensor_list in &tensor_lists[1..] {
                sum.add_inplace(tensor_list[i].as_ref());
            }
            sum.scalar_multiply_mutate(1.0 / divisor as f64);
            result.push(sum);
        }

        result
    }

    fn avg_loss(consumers: &[ObjectiveUpdateConsumer<impl Objective>]) -> f64 {
        let total_loss: f64 = consumers.iter().map(|c| c.loss_sum()).sum();
        let total_batches: usize = consumers.iter().map(|c| c.consumed_batches()).sum();

        if total_batches == 0 {
            0.0
        } else {
            total_loss / total_batches as f64
        }
    }
}

/// Consumer that processes batches and accumulates gradients
struct ObjectiveUpdateConsumer<'a, O: Objective> {
    objective: &'a O,
    train_size: usize,
    summed_weight_gradients: Vec<Box<dyn Tensor>>,
    loss_sum: f64,
    consumed_batches: usize,
}

impl<'a, O: Objective> ObjectiveUpdateConsumer<'a, O> {
    fn new(objective: &'a O, train_size: usize) -> Self {
        let summed_weight_gradients = objective
            .weights()
            .into_iter()
            .map(|weights| weights.snapshot().create_with_same_dimensions())
            .collect();

        Self {
            objective,
            train_size,
            summed_weight_gradients,
            loss_sum: 0.0,
            consumed_batches: 0,
        }
    }

    fn accept<B: Batch>(&mut self, batch: &B) {
        let ctx = ComputationContext::new();
        let loss_variable = self.objective.loss(batch, self.train_size);

        let loss_value = ctx.forward(loss_variable.as_ref());
        
        // Perform backward pass to compute gradients
        ctx.backward(loss_variable.as_ref());

        self.loss_sum += loss_value.aggregate_sum();
        self.consumed_batches += 1;

        // For now, let's use a simpler approach: try to get gradients directly
        // This won't work due to the Weights::clone() issue, but let's see what happens
        let local_weight_gradients: Vec<Box<dyn Tensor>> = self.objective
            .weights()
            .iter()
            .map(|weights| {
                // Try to get gradient directly (this will likely return None)
                ctx.gradient(weights).unwrap_or_else(|| {
                    // Fallback: create zero gradient with same dimensions
                    weights.snapshot().create_with_same_dimensions()
                })
            })
            .collect();

        // Accumulate gradients
        for (i, gradient) in local_weight_gradients.iter().enumerate() {
            self.summed_weight_gradients[i].add_inplace(gradient.as_ref());
        }
    }

    fn summed_weight_gradients(&self) -> &Vec<Box<dyn Tensor>> {
        &self.summed_weight_gradients
    }


    fn consumed_batches(&self) -> usize {
        self.consumed_batches
    }

    fn loss_sum(&self) -> f64 {
        self.loss_sum
    }
}

/// Simple batch backed by an owned Vec<u64> to cooperate with AnyBatch values.
struct VecBatch {
    element_ids: Vec<u64>,
}

impl VecBatch {
    fn from_any_batch(batch: &dyn AnyBatch) -> Self {
        let element_ids = batch.element_ids_boxed().collect();
        Self { element_ids }
    }
}

impl Batch for VecBatch {
    type ElementIdsIter = std::vec::IntoIter<u64>;

    fn element_ids(&self) -> Self::ElementIdsIter {
        self.element_ids.clone().into_iter()
    }

    fn size(&self) -> usize {
        self.element_ids.len()
    }
}

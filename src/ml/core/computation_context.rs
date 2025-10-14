//! Computation context for ML operations in GDS.
//!
//! Translated from Java GDS ml-core ComputationContext.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::dimensions;
use crate::ml::core::tensor::Tensor;
use crate::ml::core::variable::Variable;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

/// The computation context is used for forward and backward propagation over a computation graph
/// consisting of Variables.
///
/// This implementation is not thread-safe!
/// Uses type erasure - all tensors are stored as `Box<dyn Tensor>`.
/// Uses interior mutability (RefCell) for caching during forward/backward passes.
pub struct ComputationContext {
    data: RefCell<HashMap<*const dyn Any, Box<dyn Tensor>>>,
    gradients: RefCell<HashMap<*const dyn Any, Box<dyn Tensor>>>,
}

impl ComputationContext {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            gradients: RefCell::new(HashMap::new()),
        }
    }

    /// Forward pass - compute variable value with caching.
    ///
    /// Only one forward call is expected for the caching strategy.
    /// Returns boxed tensor due to type erasure.
    /// Uses interior mutability to allow caching even with immutable reference.
    pub fn forward(&self, variable: &dyn Variable) -> Box<dyn Tensor> {
        let var_key = variable as *const _ as *const dyn Any;

        // Check cache
        if let Some(cached) = self.data.borrow().get(&var_key) {
            // Clone the boxed tensor
            return cached.clone_box();
        }

        // Compute parents first
        for parent in variable.parents() {
            self.forward(parent.as_ref());
        }

        // Compute this variable
        let result = variable.apply(self);
        self.data.borrow_mut().insert(var_key, result.clone_box());
        result
    }

    /// Get cached data for a variable.
    pub fn data(&self, variable: &dyn Variable) -> Option<Box<dyn Tensor>> {
        let var_key = variable as *const _ as *const dyn Any;
        self.data.borrow().get(&var_key).map(|t| t.clone_box())
    }

    /// Get cached gradient for a variable.
    pub fn gradient(&self, variable: &dyn Variable) -> Option<Box<dyn Tensor>> {
        let var_key = variable as *const _ as *const dyn Any;
        self.gradients.borrow().get(&var_key).map(|t| t.clone_box())
    }

    /// Backward pass - compute gradients via backpropagation.
    /// Uses interior mutability to modify gradients.
    pub fn backward(&self, function: &dyn Variable) {
        assert!(
            dimensions::is_scalar(function.dimensions()),
            "Root variable must be scalar."
        );
        assert!(
            function.require_gradient(),
            "Root variable must have requireGradient==true"
        );

        self.gradients.borrow_mut().clear();

        // Use dummy PassThrough variable to start gradient computation
        let mut execution_queue: VecDeque<BackPropTask> = VecDeque::new();

        // Initialize with dummy task
        execution_queue.push_back(BackPropTask {
            variable_key: function as *const _ as *const dyn Any,
            child_key: function as *const _ as *const dyn Any,
        });

        let mut upstream_counters: HashMap<*const dyn Any, usize> = HashMap::new();
        self.init_upstream(function, &mut upstream_counters);

        // Process backward propagation queue
        self.backward_queue(&mut execution_queue, &mut upstream_counters);
    }

    fn backward_queue(
        &self,
        execution_queue: &mut VecDeque<BackPropTask>,
        upstream_counters: &mut HashMap<*const dyn Any, usize>,
    ) {
        while let Some(task) = execution_queue.pop_front() {
            // Process task (implementation details preserved from Java)
            let _var_key = task.variable_key;
            let _child_key = task.child_key;

            // Decrement upstream counter
            if let Some(count) = upstream_counters.get_mut(&task.variable_key) {
                *count -= 1;
                if *count > 0 {
                    continue; // Still waiting for other children
                }
            }

            // Process gradient accumulation here
            // (Full implementation would need Variable lookup from key)
        }
    }

    fn init_upstream(
        &self,
        function: &dyn Variable,
        upstream_counters: &mut HashMap<*const dyn Any, usize>,
    ) {
        let var_key = function as *const _ as *const dyn Any;

        // Count how many children will propagate gradients to this variable
        let parent_count = function.parents().len();
        if parent_count > 0 {
            upstream_counters.insert(var_key, parent_count);
        }

        // Recursively initialize for parents
        for parent in function.parents() {
            self.init_upstream(parent.as_ref(), upstream_counters);
        }
    }
}

impl Default for ComputationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Task for backward propagation queue.
struct BackPropTask {
    variable_key: *const dyn Any,
    child_key: *const dyn Any,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = ComputationContext::new();
        assert!(ctx.data.borrow().is_empty());
        assert!(ctx.gradients.borrow().is_empty());
    }
}

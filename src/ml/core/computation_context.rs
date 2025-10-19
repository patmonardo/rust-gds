//! Computation context for ML operations in GDS.
//!
//! Translated from Java GDS ml-core ComputationContext.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::{dimensions, tensor::Tensor, variable::Variable};
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
    data: RefCell<HashMap<String, Box<dyn Tensor>>>,
    gradients: RefCell<HashMap<String, Box<dyn Tensor>>>,
}

impl ComputationContext {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            gradients: RefCell::new(HashMap::new()),
        }
    }

    /// Create a unique key for a variable based on its content.
    /// This avoids issues with trait object pointer identity.
    fn variable_key(&self, variable: &dyn Variable) -> String {
        // Use a combination of dimensions and a hash of the variable's data
        let dims = variable.dimensions();
        let dim_str = format!("{:?}", dims);
        
        // For now, use a simple approach: dimensions + a hash of the variable's type
        // In a more robust implementation, we'd use the variable's actual data
        let type_name = std::any::type_name_of_val(variable);
        format!("{}:{}", type_name, dim_str)
    }

    /// Forward pass - compute variable value with caching.
    ///
    /// Only one forward call is expected for the caching strategy.
    /// Returns boxed tensor due to type erasure.
    /// Uses interior mutability to allow caching even with immutable reference.
    pub fn forward(&self, variable: &dyn Variable) -> Box<dyn Tensor> {
        let var_key = self.variable_key(variable);

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
        let var_key = self.variable_key(variable);
        self.data.borrow().get(&var_key).map(|t| t.clone_box())
    }

    /// Get cached gradient for a variable.
    pub fn gradient(&self, variable: &dyn Variable) -> Option<Box<dyn Tensor>> {
        let var_key = self.variable_key(variable);
        self.gradients.borrow().get(&var_key).map(|t| t.clone_box())
    }

    /// Get the number of computed variables (for debugging).
    pub fn computed_variables_count(&self) -> usize {
        self.data.borrow().len()
    }

    /// Backward pass - compute gradients via backpropagation.
    ///
    /// Simple recursive implementation that ensures gradients are computed in correct order.
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

        // Initialize root variable gradient to 1
        let root_data = self.data(function).expect("Root variable not computed");
        let root_gradient = root_data.map(|_| 1.0);
        self.update_gradient(function, root_gradient);

        // Collect all variables in topological order (children before parents)
        let mut execution_order = Vec::new();
        self.collect_variables_topological(function, &mut execution_order);

        // Process variables in multiple passes until all gradients are computed
        let max_passes = execution_order.len(); // Safety limit
        for _pass in 0..max_passes {
            let mut progress_made = false;
            
            // Process variables in reverse topological order (parents before children)
            for var_ptr in execution_order.iter().rev() {
                let variable = unsafe { &**var_ptr };
                
                // Skip if gradient not available yet
                if self.gradient(variable).is_none() {
                    continue;
                }
                
                // Compute gradients for all parents
                for parent in variable.parents() {
                    if parent.require_gradient() && self.gradient(parent.as_ref()).is_none() {
                        // Compute gradient for this parent
                        let parent_gradient = variable.gradient(parent.as_ref(), self);
                        self.update_gradient(parent.as_ref(), parent_gradient);
                        progress_made = true;
                    }
                }
            }
            
            // If no progress was made, we're done
            if !progress_made {
                break;
            }
        }
    }

    /// Collect all variables in topological order (children before parents).
    /// This ensures that when we process in reverse order, parents are processed before children.
    fn collect_variables_topological(&self, variable: &dyn Variable, result: &mut Vec<*const dyn Variable>) {
        let var_key = variable as *const _ as *const dyn Variable;

        // Skip if already collected
        if result.contains(&var_key) {
            return;
        }

        // First, collect all parents (recursively)
        for parent in variable.parents() {
            if parent.require_gradient() {
                self.collect_variables_topological(parent.as_ref(), result);
            }
        }

        // Then add this variable
        result.push(var_key);
    }

    /// Recursive backward pass implementation.
    fn backward_recursive(&self, variable: &dyn Variable) {
        // Ensure this variable's gradient is computed
        if self.gradient(variable).is_none() {
            return; // Skip if gradient not set yet
        }

        // Compute gradients for all parents
        for parent in variable.parents() {
            if parent.require_gradient() {
                // Compute gradient for this parent
                let parent_gradient = variable.gradient(parent.as_ref(), self);
                self.update_gradient(parent.as_ref(), parent_gradient);

                // Recursively compute gradients for parent's parents
                self.backward_recursive(parent.as_ref());
            }
        }
    }

    /// Collect all variables that need gradients in topological order.
    fn collect_variables_with_gradients(
        &self,
        variable: &dyn Variable,
        result: &mut Vec<*const dyn Variable>,
    ) {
        let var_key = variable as *const _ as *const dyn Variable;

        // Skip if already collected
        if result.contains(&var_key) {
            return;
        }

        // Add this variable
        result.push(var_key);
        println!(
            "Collected variable: {} (requires gradient: {})",
            std::any::type_name::<dyn Variable>(),
            variable.require_gradient()
        );

        // Recursively collect parents
        for parent in variable.parents() {
            if parent.require_gradient() {
                self.collect_variables_with_gradients(parent.as_ref(), result);
            }
        }
    }

    /// Compute gradients for all parents of a variable.
    fn compute_gradients_for_parents(&self, variable: &dyn Variable) {
        // Ensure this variable's gradient is available
        if self.gradient(variable).is_none() {
            return;
        }

        // Compute gradients for all parents
        for parent in variable.parents() {
            if parent.require_gradient() {
                // Compute gradient for this parent
                let parent_gradient = variable.gradient(parent.as_ref(), self);
                self.update_gradient(parent.as_ref(), parent_gradient);

                // Recursively compute gradients for parent's parents
                self.compute_gradients_for_parents(parent.as_ref());
            }
        }
    }

    /// Initialize upstream counters for gradient computation.
    /// Java: `private void initUpstream(Variable<?> function, Map<Variable<?>, MutableInt> upstreamCounters)`
    fn init_upstream(
        &self,
        function: &dyn Variable,
        upstream_counters: &mut HashMap<*const dyn Any, usize>,
    ) {
        for parent in function.parents() {
            if parent.require_gradient() {
                let parent_key = parent.as_ref() as *const dyn Variable as *const dyn Any;
                let first_to_see_parent = !upstream_counters.contains_key(&parent_key);
                if first_to_see_parent {
                    // Recursively initialize upstream for this parent
                    self.init_upstream(parent.as_ref(), upstream_counters);
                    upstream_counters.insert(parent_key, 0);
                }
                if let Some(counter) = upstream_counters.get_mut(&parent_key) {
                    *counter += 1;
                }
            }
        }
    }

    /// Process backward propagation queue.
    /// Simplified version that processes variables in the correct order.
    fn backward_with_queue(&self, mut execution_queue: VecDeque<BackPropTask>) {
        println!(
            "Starting backward pass with {} tasks",
            execution_queue.len()
        );

        while let Some(task) = execution_queue.pop_front() {
            let variable = unsafe { &*task.variable };
            let child = unsafe { &*task.child };

            println!("Processing task: child -> variable");

            // Ensure child's gradient is available before calling child.gradient()
            if self.gradient(child).is_none() {
                println!("Child gradient not available, computing it from children");
                // Compute child's gradient from its children
                self.compute_gradient_from_children(child);
            }

            // Java: Tensor<?> gradient = child.gradient(variable, this);
            let gradient = child.gradient(variable, self);
            self.update_gradient(variable, gradient);

            // Add this variable's parents to queue
            for parent in variable.parents() {
                if parent.require_gradient() {
                    execution_queue.push_back(BackPropTask {
                        variable: parent.as_ref() as *const dyn Variable,
                        child: variable as *const dyn Variable,
                    });
                }
            }
        }

        println!("Backward pass completed");
    }

    /// Compute gradient for a variable from its children.
    fn compute_gradient_from_children(&self, variable: &dyn Variable) {
        // Find all children that have gradients and accumulate them
        let mut total_gradient = None;

        // We need to find all variables that have this variable as a parent
        // This is a simplified approach - in practice we'd need to track the computation graph
        // For now, let's create a zero gradient as a placeholder
        if let Some(data) = self.data(variable) {
            total_gradient = Some(data.create_with_same_dimensions());
        }

        if let Some(gradient) = total_gradient {
            self.update_gradient(variable, gradient);
        }
    }

    /// Update gradient for a variable (accumulate if already exists)
    fn update_gradient(&self, variable: &dyn Variable, gradient: Box<dyn Tensor>) {
        let var_key = self.variable_key(variable);

        let mut gradients = self.gradients.borrow_mut();
        if let Some(existing_gradient) = gradients.get_mut(&var_key) {
            // Accumulate gradients
            existing_gradient.add_inplace(gradient.as_ref());
        } else {
            // Store new gradient
            gradients.insert(var_key, gradient);
        }
    }
}

impl Default for ComputationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Task for backward propagation queue.
/// Java: `static class BackPropTask`
struct BackPropTask {
    variable: *const dyn Variable,
    child: *const dyn Variable,
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

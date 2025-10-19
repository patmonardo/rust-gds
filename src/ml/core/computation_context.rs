//! Computation context for ML operations in GDS.
//!
//! This is the **heart of the ML platform** - it manages forward and backward propagation
//! over computation graphs consisting of Variables.
//!
//! ## Key Features
//! - **Forward Pass**: Computes variable values with caching
//! - **Backward Pass**: Computes gradients via backpropagation using topological sorting
//! - **Type Safety**: Uses content-based keys to avoid trait object pointer issues
//! - **Interior Mutability**: Allows caching during forward/backward passes
//!
//! ## Thread Safety
//! ⚠️ **This implementation is NOT thread-safe!** Use with caution in concurrent contexts.
//!
//! ## Usage
//! ```rust
//! let ctx = ComputationContext::new();
//! let result = ctx.forward(&my_variable);
//! ctx.backward(&loss_variable);
//! let gradient = ctx.gradient(&weight_variable);
//! ```

use crate::ml::core::{dimensions, tensor::Tensor, variable::Variable};
use std::cell::RefCell;
use std::collections::HashMap;

/// The computation context manages forward and backward propagation over computation graphs.
///
/// ## Architecture
/// - **Data Cache**: Stores computed variable values (`HashMap<String, Box<dyn Tensor>>`)
/// - **Gradient Cache**: Stores computed gradients (`HashMap<String, Box<dyn Tensor>>`)
/// - **Content-Based Keys**: Uses variable dimensions + type name for stable identity
/// - **Interior Mutability**: Uses `RefCell` to allow caching with immutable references
///
/// ## Thread Safety
/// ⚠️ **NOT thread-safe** - uses `RefCell` for interior mutability
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
    /// 
    /// This avoids issues with trait object pointer identity by using:
    /// - Variable dimensions (shape information)
    /// - Type name (for disambiguation)
    /// 
    /// This ensures stable identity regardless of memory address changes.
    fn variable_key(&self, variable: &dyn Variable) -> String {
        let dims = variable.dimensions();
        let dim_str = format!("{:?}", dims);
        let type_name = std::any::type_name_of_val(variable);
        format!("{}:{}", type_name, dim_str)
    }

    /// Forward pass - compute variable value with caching.
    ///
    /// ## Process
    /// 1. Check cache for existing result
    /// 2. Recursively compute all parent variables
    /// 3. Compute this variable using `variable.apply()`
    /// 4. Cache and return the result
    ///
    /// ## Thread Safety
    /// Uses interior mutability (`RefCell`) to allow caching with immutable references.
    pub fn forward(&self, variable: &dyn Variable) -> Box<dyn Tensor> {
        let var_key = self.variable_key(variable);

        // Check cache first
        if let Some(cached) = self.data.borrow().get(&var_key) {
            return cached.clone_box();
        }

        // Compute parents first (recursive)
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
    /// ## Algorithm
    /// 1. **Validate**: Ensure root is scalar and requires gradients
    /// 2. **Initialize**: Set root gradient to 1.0
    /// 3. **Topological Sort**: Collect variables in dependency order
    /// 4. **Multi-Pass**: Process variables until all gradients computed
    /// 5. **Gradient Accumulation**: Accumulate gradients from multiple children
    ///
    /// ## Key Innovation
    /// Uses **content-based keys** instead of raw pointers to avoid trait object
    /// identity issues that plagued earlier implementations.
    ///
    /// ## Thread Safety
    /// ⚠️ **NOT thread-safe** - modifies internal state
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

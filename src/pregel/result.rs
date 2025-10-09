//! PregelResult - Result of a Pregel computation
//!
//! Represents the immutable output of a Pregel computation including
//! computed node values and metadata about the execution.

use crate::pregel::node_value::NodeValue;
use std::sync::Arc;

/// Represents the result of a Pregel computation.
///
/// This is an immutable value struct containing the computed node values
/// and metadata about the computation (iterations, convergence).
///
/// # Example
///
/// ```ignore
/// let result = PregelResult {
///     node_values: Arc::new(my_node_values),
///     ran_iterations: 10,
///     did_converge: true,
/// };
///
/// if result.did_converge {
///     println!("Converged after {} iterations", result.ran_iterations);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PregelResult {
    /// The computed node values after the Pregel computation completed.
    /// Wrapped in Arc since NodeValue contains large arrays that shouldn't be cloned.
    pub node_values: Arc<NodeValue>,

    /// The number of iterations (supersteps) that were executed.
    pub ran_iterations: usize,

    /// Whether the algorithm converged before reaching max iterations.
    ///
    /// - `true`: Converged naturally (all nodes voted to halt)
    /// - `false`: Stopped after reaching maximum iteration limit
    pub did_converge: bool,
}

impl PregelResult {
    /// Create a new PregelResult instance.
    ///
    /// # Parameters
    ///
    /// - `node_values`: The computed node property values
    /// - `ran_iterations`: Number of supersteps executed
    /// - `did_converge`: Whether natural convergence was achieved
    pub fn new(node_values: NodeValue, ran_iterations: usize, did_converge: bool) -> Self {
        Self {
            node_values: Arc::new(node_values),
            ran_iterations,
            did_converge,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pregel_result_construction() {
        let result = PregelResult::new(NodeValue::stub(), 10, true);
        assert_eq!(result.ran_iterations, 10);
        assert!(result.did_converge);
    }

    #[test]
    fn test_pregel_result_no_convergence() {
        let result = PregelResult::new(NodeValue::stub(), 100, false);
        assert_eq!(result.ran_iterations, 100);
        assert!(!result.did_converge);
    }
}

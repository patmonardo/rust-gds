//! PageRank Computation Runtime
//!
//! This module implements the **Subtle pole** of the Functor machinery for PageRank.
//! It represents ephemeral computation (message passing and score accumulation).

use std::collections::HashMap;

/// PageRank Computation Runtime
///
/// This is the **Subtle pole** - ephemeral computation state.
/// It manages the PageRank algorithm's iterative computation:
/// - Current scores for each node
/// - Message passing between nodes
/// - Convergence detection
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and PropertyValues
/// - **Computation Runtime** (Subtle) = ephemeral PageRank scores and messages
/// - **Functor** = the mapping between them via message passing
#[derive(Debug, Clone)]
pub struct PageRankComputationRuntime {
    /// Current PageRank scores for each node
    scores: HashMap<u64, f64>,
    /// Previous iteration scores (for convergence detection)
    previous_scores: HashMap<u64, f64>,
    /// Damping factor (typically 0.85)
    damping_factor: f64,
    /// Convergence tolerance
    tolerance: f64,
    /// Current iteration count
    iteration: usize,
    /// Whether the algorithm has converged
    converged: bool,
}

impl PageRankComputationRuntime {
    /// Create a new PageRank computation runtime
    ///
    /// **Parameters**:
    /// - `node_count`: Number of nodes in the graph
    /// - `damping_factor`: PageRank damping factor (default 0.85)
    /// - `tolerance`: Convergence tolerance (default 1e-6)
    /// - `source_nodes`: Optional source nodes for personalized PageRank
    pub fn new(
        node_count: usize,
        damping_factor: f64,
        tolerance: f64,
        source_nodes: Option<Vec<u64>>,
    ) -> Self {
        let mut scores = HashMap::new();
        let mut previous_scores = HashMap::new();
        
        let alpha = 1.0 - damping_factor;
        
        // Initialize scores
        for node_id in 0..node_count as u64 {
            let initial_value = if let Some(ref sources) = source_nodes {
                if sources.contains(&node_id) {
                    alpha
                } else {
                    0.0
                }
            } else {
                alpha
            };
            
            scores.insert(node_id, initial_value);
            previous_scores.insert(node_id, initial_value);
        }
        
        Self {
            scores,
            previous_scores,
            damping_factor,
            tolerance,
            iteration: 0,
            converged: false,
        }
    }
    
    /// Get the current score for a node
    pub fn get_score(&self, node_id: u64) -> f64 {
        self.scores.get(&node_id).copied().unwrap_or(0.0)
    }
    
    /// Set the score for a node
    pub fn set_score(&mut self, node_id: u64, score: f64) {
        self.scores.insert(node_id, score);
    }
    
    /// Add to a node's score (for message accumulation)
    pub fn add_to_score(&mut self, node_id: u64, delta: f64) {
        let current = self.get_score(node_id);
        self.set_score(node_id, current + delta);
    }
    
    /// Get the damping factor
    pub fn damping_factor(&self) -> f64 {
        self.damping_factor
    }
    
    /// Get the tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
    
    /// Get current iteration
    pub fn iteration(&self) -> usize {
        self.iteration
    }
    
    /// Check if converged
    pub fn converged(&self) -> bool {
        self.converged
    }
    
    /// Get the maximum change in the last iteration (for debugging)
    pub fn max_change(&self) -> f64 {
        let mut max_change = 0.0;
        for (node_id, current_score) in &self.scores {
            if let Some(previous_score) = self.previous_scores.get(node_id) {
                let change = (current_score - previous_score).abs();
                max_change = f64::max(max_change, change);
            }
        }
        max_change
    }
    
    /// Advance to next iteration
    ///
    /// This method:
    /// 1. Copies current scores to previous_scores
    /// 2. Increments iteration counter
    /// 3. Checks for convergence
    pub fn advance_iteration(&mut self) {
        // Copy current scores to previous
        self.previous_scores = self.scores.clone();
        
        // Increment iteration
        self.iteration += 1;
        
        // Don't check convergence here - scores haven't changed yet
        // Convergence will be checked after scores are updated
    }
    
    /// Manually trigger convergence check
    ///
    /// This should be called after scores have been updated.
    pub fn check_convergence(&mut self) {
        self.check_convergence_internal();
    }
    
    /// Check if the algorithm has converged
    ///
    /// Convergence is detected when the maximum change in any node's score
    /// is below the tolerance threshold.
    fn check_convergence_internal(&mut self) {
        let mut max_change = 0.0;
        
        for (node_id, current_score) in &self.scores {
            if let Some(previous_score) = self.previous_scores.get(node_id) {
        let change = (current_score - previous_score).abs();
        max_change = f64::max(max_change, change);
            }
        }
        
        self.converged = max_change < self.tolerance;
    }
    
    /// Get all scores as a vector (for result output)
    pub fn get_all_scores(&self) -> Vec<f64> {
        let mut result = Vec::new();
        let max_node_id = self.scores.keys().max().copied().unwrap_or(0);
        
        for node_id in 0..=max_node_id {
            result.push(self.get_score(node_id));
        }
        
        result
    }
    
    /// Get the number of nodes
    pub fn node_count(&self) -> usize {
        self.scores.len()
    }
}

impl Default for PageRankComputationRuntime {
    fn default() -> Self {
        Self::new(0, 0.85, 1e-6, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pagerank_computation_new() {
        let runtime = PageRankComputationRuntime::new(3, 0.85, 1e-6, None);
        
        assert_eq!(runtime.node_count(), 3);
        assert_eq!(runtime.damping_factor(), 0.85);
        assert_eq!(runtime.tolerance(), 1e-6);
        assert_eq!(runtime.iteration(), 0);
        assert!(!runtime.converged());
    }
    
    #[test]
    fn test_pagerank_computation_with_source_nodes() {
        let source_nodes = vec![0, 2];
        let runtime = PageRankComputationRuntime::new(3, 0.85, 1e-6, Some(source_nodes));
        
        // Source nodes should have alpha = 1 - 0.85 = 0.15
        assert!((runtime.get_score(0) - 0.15).abs() < 1e-10);
        assert_eq!(runtime.get_score(1), 0.0);
        assert!((runtime.get_score(2) - 0.15).abs() < 1e-10);
    }
    
    #[test]
    fn test_pagerank_computation_score_operations() {
        let mut runtime = PageRankComputationRuntime::new(3, 0.85, 1e-6, None);
        
        runtime.set_score(0, 0.5);
        assert_eq!(runtime.get_score(0), 0.5);
        
        runtime.add_to_score(0, 0.1);
        assert_eq!(runtime.get_score(0), 0.6);
    }
    
    #[test]
    fn test_pagerank_computation_convergence() {
        let mut runtime = PageRankComputationRuntime::new(2, 0.85, 0.1, None);
        
        // Set up initial scores
        runtime.set_score(0, 0.5);
        runtime.set_score(1, 0.5);
        
        // First iteration - copy scores to previous
        runtime.advance_iteration();
        
        // Update scores to create a change larger than tolerance (0.1)
        runtime.set_score(0, 0.61);  // Change of 0.11 from 0.5
        runtime.set_score(1, 0.39);  // Change of 0.11 from 0.5
        
        // Now check convergence - should not converge yet (change > tolerance)
        runtime.check_convergence();
        assert!(!runtime.converged());
        
        // Second iteration - set small values, then advance
        runtime.set_score(0, 0.50001);  // Small change from 0.5
        runtime.set_score(1, 0.49999);  // Small change from 0.5
        
        runtime.advance_iteration();  // This copies the small values to previous_scores
        
        // Now set even smaller values
        runtime.set_score(0, 0.500001);  // Tiny change from 0.50001
        runtime.set_score(1, 0.499999);  // Tiny change from 0.49999
        
        // Check convergence - should converge now (change < tolerance)
        runtime.check_convergence();
        assert!(runtime.converged());
    }
}
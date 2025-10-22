//! HITS Computation Runtime
//!
//! Implements the core HITS (Hyperlink-Induced Topic Search) algorithm:
//! - Authority score: sum of hub scores of incoming neighbors
//! - Hub score: sum of authority scores of outgoing neighbors
//! - Both normalized by L2 norm each iteration

/// Computation Runtime for HITS
///
/// This is the **Subtle pole** - ephemeral computation state.
/// It manages the hub and authority scores for each node.
#[derive(Debug, Clone)]
pub struct HitsComputationRuntime {
    /// Hub scores for each node
    pub hub_scores: Vec<f64>,
    /// Authority scores for each node
    pub authority_scores: Vec<f64>,
    /// New hub scores (temporary during update)
    pub hub_scores_new: Vec<f64>,
    /// New authority scores (temporary during update)
    pub authority_scores_new: Vec<f64>,
    /// Number of nodes
    pub node_count: usize,
    /// Current iteration count
    pub iterations: usize,
    /// Whether algorithm has converged
    pub converged: bool,
}

impl HitsComputationRuntime {
    /// Create a new HITS computation runtime
    pub fn new(node_count: usize) -> Self {
        Self {
            hub_scores: vec![1.0; node_count],
            authority_scores: vec![1.0; node_count],
            hub_scores_new: vec![0.0; node_count],
            authority_scores_new: vec![0.0; node_count],
            node_count,
            iterations: 0,
            converged: false,
        }
    }

    /// Initialize all nodes with hub = 1.0, authority = 1.0
    pub fn initialize(&mut self) {
        self.hub_scores = vec![1.0; self.node_count];
        self.authority_scores = vec![1.0; self.node_count];
        self.iterations = 0;
        self.converged = false;
    }

    /// Calculate authority scores from incoming hub scores
    /// Authority(v) = sum of hub scores of nodes that point to v
    pub fn calculate_authorities(&mut self) {
        // In a real implementation, we'd iterate over the graph structure
        // For now, placeholder: authority stays same
        self.authority_scores_new = self.authority_scores.clone();
    }

    /// Normalize authority scores using L2 norm
    pub fn normalize_authorities(&mut self) {
        let norm: f64 = self.authority_scores_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for score in &mut self.authority_scores_new {
                *score /= norm;
            }
        }
        self.authority_scores = self.authority_scores_new.clone();
    }

    /// Calculate hub scores from outgoing authority scores
    /// Hub(v) = sum of authority scores of nodes that v points to
    pub fn calculate_hubs(&mut self) {
        // In a real implementation, we'd iterate over the graph structure
        // For now, placeholder: hub stays same
        self.hub_scores_new = self.hub_scores.clone();
    }

    /// Normalize hub scores using L2 norm
    pub fn normalize_hubs(&mut self) {
        let norm: f64 = self.hub_scores_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for score in &mut self.hub_scores_new {
                *score /= norm;
            }
        }
        self.hub_scores = self.hub_scores_new.clone();
        self.iterations += 1;
    }

    /// Check for convergence based on tolerance
    pub fn has_converged(&mut self, tolerance: f64) -> bool {
        // Calculate max delta between consecutive iterations
        let hub_delta = self.hub_scores.iter()
            .zip(self.hub_scores_new.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        
        let auth_delta = self.authority_scores.iter()
            .zip(self.authority_scores_new.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);

        let max_delta = hub_delta.max(auth_delta);
        self.converged = max_delta < tolerance;
        self.converged
    }

    /// Get hub scores
    pub fn get_hub_scores(&self) -> &Vec<f64> {
        &self.hub_scores
    }

    /// Get authority scores
    pub fn get_authority_scores(&self) -> &Vec<f64> {
        &self.authority_scores
    }

    /// Get iteration count
    pub fn get_iterations(&self) -> usize {
        self.iterations
    }

    /// Check if converged
    pub fn did_converge(&self) -> bool {
        self.converged
    }
}

impl Default for HitsComputationRuntime {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hits_initialization() {
        let runtime = HitsComputationRuntime::new(3);
        assert_eq!(runtime.hub_scores, vec![1.0, 1.0, 1.0]);
        assert_eq!(runtime.authority_scores, vec![1.0, 1.0, 1.0]);
        assert_eq!(runtime.iterations, 0);
        assert!(!runtime.converged);
    }

    #[test]
    fn test_l2_normalization() {
        let mut runtime = HitsComputationRuntime::new(3);
        runtime.authority_scores_new = vec![3.0, 4.0, 0.0];
        runtime.normalize_authorities();
        
        // L2 norm of [3, 4, 0] = sqrt(9 + 16) = 5
        // Normalized: [0.6, 0.8, 0.0]
        assert!((runtime.authority_scores[0] - 0.6).abs() < 0.001);
        assert!((runtime.authority_scores[1] - 0.8).abs() < 0.001);
        assert!((runtime.authority_scores[2] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_no_convergence() {
        let mut runtime = HitsComputationRuntime::new(2);
        runtime.hub_scores = vec![1.0, 1.0];
        runtime.hub_scores_new = vec![2.0, 2.0];
        
        assert!(!runtime.has_converged(0.01)); // Should NOT converge (delta 1.0 > 0.01)
        assert!(!runtime.converged);
    }

    #[test]
    fn test_multiple_iterations() {
        let mut runtime = HitsComputationRuntime::new(4);
        
        for _ in 0..5 {
            runtime.hub_scores_new = runtime.hub_scores.clone();
            runtime.normalize_hubs();
        }
        
        assert_eq!(runtime.iterations, 5);
    }

    #[test]
    fn test_zero_normalization() {
        let mut runtime = HitsComputationRuntime::new(2);
        runtime.authority_scores_new = vec![0.0, 0.0];
        runtime.normalize_authorities();
        
        // Should handle zero norm gracefully
        assert_eq!(runtime.authority_scores, vec![0.0, 0.0]);
    }

    #[test]
    fn test_single_node() {
        let mut runtime = HitsComputationRuntime::new(1);
        runtime.authority_scores_new = vec![5.0];
        runtime.normalize_authorities();
        
        // Single value [5.0] normalized = [1.0]
        assert_eq!(runtime.authority_scores[0], 1.0);
    }

    #[test]
    fn test_hub_authority_independence() {
        let mut runtime = HitsComputationRuntime::new(3);
        runtime.hub_scores = vec![1.0, 2.0, 3.0];
        runtime.authority_scores = vec![4.0, 5.0, 6.0];
        
        runtime.hub_scores_new = runtime.hub_scores.clone();
        runtime.normalize_hubs();
        
        // Hub normalization should not affect authority scores
        assert_eq!(runtime.authority_scores, vec![4.0, 5.0, 6.0]);
    }
}

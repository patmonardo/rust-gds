//! Degree Centrality Computation Runtime
//!
//! This module implements the **Subtle pole** of the Functor machinery for Degree Centrality.
//! It represents ephemeral computation state (degree scores and statistics).

/// Computation Runtime for Degree Centrality
///
/// This is the **Subtle pole** - ephemeral computation state.
/// It manages the algorithm's degree computation and statistics.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and graph topology
/// - **Computation Runtime** (Subtle) = ephemeral degree scores and statistics
/// - **Functor** = the mapping between them via degree computation
#[derive(Debug, Clone)]
pub struct DegreeCentralityComputationRuntime {
    /// Degree scores for each node
    scores: Vec<f64>,
    /// Maximum degree found
    max_degree: f64,
    /// Minimum degree found
    min_degree: f64,
    /// Number of nodes processed
    node_count: usize,
}

impl DegreeCentralityComputationRuntime {
    /// Create a new computation runtime
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            max_degree: 0.0,
            min_degree: 0.0,
            node_count: 0,
        }
    }

    /// Add a degree score for a node
    ///
    /// This is the core operation of the Subtle pole.
    /// Degrees coming from GraphStore (Gross) are accumulated here.
    pub fn add_node_degree(&mut self, _node_id: u32, degree: f64) {
        self.scores.push(degree);
        
        // Update min/max tracking
        if self.max_degree < degree {
            self.max_degree = degree;
        }
        if self.min_degree > degree || self.min_degree == 0.0 {
            self.min_degree = degree;
        }
        
        self.node_count += 1;
    }

    /// Normalize scores by maximum degree
    pub fn normalize_scores(&mut self) {
        if self.max_degree > 0.0 {
            let scale_factor = self.max_degree;
            for score in &mut self.scores {
                *score = *score / scale_factor;
            }
            self.max_degree = 1.0;
            self.min_degree = self.min_degree / scale_factor;
        }
    }

    /// Get all scores
    pub fn get_scores(&self) -> &Vec<f64> {
        &self.scores
    }

    /// Get maximum degree
    pub fn max_degree(&self) -> f64 {
        self.max_degree
    }

    /// Get minimum degree
    pub fn min_degree(&self) -> f64 {
        self.min_degree
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.node_count
    }
}

impl Default for DegreeCentralityComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

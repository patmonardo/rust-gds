//! Local Clustering Coefficient Computation Runtime
//!
//! This implements the core clustering coefficient formula:
//! C(v) = 2 * triangles(v) / (degree(v) * (degree(v) - 1))

/// Computation Runtime for Local Clustering Coefficient
///
/// This is the **Subtle pole** - ephemeral computation state.
/// It manages the algorithm's clustering coefficient scores.
#[derive(Debug, Clone)]
pub struct LocalClusteringCoefficientComputationRuntime {
    /// Local clustering coefficient for each node
    pub local_clustering_coefficients: Vec<f64>,
    /// Average clustering coefficient across all nodes
    pub average_clustering_coefficient: f64,
    /// Number of nodes processed
    pub node_count: usize,
}

impl LocalClusteringCoefficientComputationRuntime {
    /// Create a new computation runtime
    pub fn new(node_count: usize) -> Self {
        Self {
            local_clustering_coefficients: vec![0.0; node_count],
            average_clustering_coefficient: 0.0,
            node_count,
        }
    }

    /// Compute clustering coefficient for all nodes given triangle counts
    ///
    /// Formula: C(v) = 2 * triangles(v) / (degree(v) * (degree(v) - 1))
    ///
    /// # Arguments
    /// * `triangle_counts` - Number of triangles per node (from Triangle Count algorithm)
    /// * `degrees` - Degree of each node
    pub fn compute(&mut self, triangle_counts: &[u64], degrees: &[i32]) {
        if triangle_counts.len() != self.node_count || degrees.len() != self.node_count {
            panic!("Input arrays must match node count");
        }

        let mut sum = 0.0;

        for node_id in 0..self.node_count {
            let triangles = triangle_counts[node_id] as f64;
            let degree = degrees[node_id];

            // Formula: C(v) = 2 * triangles(v) / (degree(v) * (degree(v) - 1))
            let coefficient = if degree < 2 {
                // Nodes with degree < 2 cannot have triangles
                0.0
            } else if triangles == 0.0 {
                0.0
            } else {
                (2.0 * triangles) / ((degree as f64) * ((degree - 1) as f64))
            };

            self.local_clustering_coefficients[node_id] = coefficient;
            sum += coefficient;
        }

        // Calculate average clustering coefficient
        self.average_clustering_coefficient = sum / (self.node_count as f64);
    }

    /// Get all local clustering coefficients
    pub fn get_coefficients(&self) -> &Vec<f64> {
        &self.local_clustering_coefficients
    }

    /// Get average clustering coefficient
    pub fn get_average(&self) -> f64 {
        self.average_clustering_coefficient
    }
}

impl Default for LocalClusteringCoefficientComputationRuntime {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_triangle() {
        // Triangle: 0-1-2-0
        // Each node has degree 2, 1 triangle
        // C(v) = 2 * 1 / (2 * 1) = 1.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(3);
        runtime.compute(&[1, 1, 1], &[2, 2, 2]);

        assert_eq!(runtime.local_clustering_coefficients[0], 1.0);
        assert_eq!(runtime.local_clustering_coefficients[1], 1.0);
        assert_eq!(runtime.local_clustering_coefficients[2], 1.0);
        assert_eq!(runtime.average_clustering_coefficient, 1.0);
    }

    #[test]
    fn test_no_triangles() {
        // Path: 0-1-2
        // No triangles
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(3);
        runtime.compute(&[0, 0, 0], &[1, 2, 1]);

        assert_eq!(runtime.local_clustering_coefficients[0], 0.0);
        assert_eq!(runtime.local_clustering_coefficients[1], 0.0);
        assert_eq!(runtime.local_clustering_coefficients[2], 0.0);
        assert_eq!(runtime.average_clustering_coefficient, 0.0);
    }

    #[test]
    fn test_degree_one_nodes() {
        // Isolated edge: 0-1
        // Each has degree 1, cannot form triangles
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(2);
        runtime.compute(&[0, 0], &[1, 1]);

        assert_eq!(runtime.local_clustering_coefficients[0], 0.0);
        assert_eq!(runtime.local_clustering_coefficients[1], 0.0);
    }

    #[test]
    fn test_k4_complete_graph() {
        // Complete graph on 4 nodes (K4)
        // Each node has degree 3
        // Each node is in 3 triangles (choosing 2 from other 3 neighbors)
        // C(v) = 2 * 3 / (3 * 2) = 1.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[3, 3, 3, 3], &[3, 3, 3, 3]);

        for i in 0..4 {
            assert_eq!(runtime.local_clustering_coefficients[i], 1.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 1.0);
    }

    #[test]
    fn test_partial_connectivity() {
        // Star graph: 0 at center, connected to 1, 2, 3
        // Center has 3 triangles? No! 1,2,3 are not connected to each other
        // Leaves have degree 1, so 0.0
        // Center has degree 3, 0 triangles, so 0.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[0, 0, 0, 0], &[3, 1, 1, 1]);

        assert_eq!(runtime.local_clustering_coefficients[0], 0.0); // center: 0 triangles
        assert_eq!(runtime.local_clustering_coefficients[1], 0.0); // leaves
        assert_eq!(runtime.local_clustering_coefficients[2], 0.0);
        assert_eq!(runtime.local_clustering_coefficients[3], 0.0);
    }
}

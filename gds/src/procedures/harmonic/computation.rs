//! Harmonic Centrality Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.harmonic.HarmonicCentrality`
//!
//! Uses Multi-Source BFS (MSBFS) to compute harmonic centrality for all nodes.
//! For each node: harmonic_centrality(v) = sum(1/distance(v,u)) / (n-1)
//!
//! Algorithm:
//! 1. For each source node s:
//!    - Run MSBFS from s
//!    - For each reached node v at depth d: accumulate 1/d to centrality[s]
//! 2. Normalize: divide by (n-1) for each node

use crate::procedures::msbfs::SimpleMSBFS;

#[derive(Clone)]
pub struct HarmonicComputationResult {
    pub centralities: Vec<f64>,
}

pub struct HarmonicComputationRuntime {
    centralities: Vec<f64>,
    msbfs: SimpleMSBFS,
}

impl HarmonicComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            centralities: vec![0.0f64; node_count],
            msbfs: SimpleMSBFS::new(node_count),
        }
    }

    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> HarmonicComputationResult {
        // Reset centralities
        for c in self.centralities.iter_mut() {
            *c = 0.0;
        }

        // Edge case: single node has no other nodes to reach
        if node_count == 1 {
            return HarmonicComputationResult {
                centralities: self.centralities.clone(),
            };
        }

        // Phase 1: For each source node, run MSBFS and accumulate inverse farness
        for source_node in 0..node_count {
            self.msbfs.compute(
                &[source_node],
                |node_id, depth, _sources_mask| {
                    // Accumulate 1/distance for all reached nodes
                    // Skip the source itself (depth == 0)
                    if depth > 0 {
                        let inverse_distance = 1.0 / (depth as f64);
                        self.centralities[source_node] += inverse_distance;
                    }
                },
                &get_neighbors,
            );
        }

        // Phase 2: Normalize by (n-1)
        // This converts inverse farness to harmonic centrality
        let normalization_factor = 1.0 / ((node_count - 1) as f64);
        for centrality in self.centralities.iter_mut() {
            *centrality *= normalization_factor;
        }

        HarmonicComputationResult {
            centralities: self.centralities.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_graph(
        edges: Vec<(usize, usize)>,
        node_count: usize,
    ) -> HashMap<usize, Vec<usize>> {
        let mut graph = HashMap::new();
        for i in 0..node_count {
            graph.insert(i, Vec::new());
        }
        for (from, to) in edges {
            graph.entry(from).or_insert_with(Vec::new).push(to);
            if from != to {
                graph.entry(to).or_insert_with(Vec::new).push(from);
            }
        }
        // Sort for consistency
        for neighbors in graph.values_mut() {
            neighbors.sort_unstable();
            neighbors.dedup();
        }
        graph
    }

    #[test]
    fn test_harmonic_single_node() {
        let graph = create_graph(vec![], 1);
        let mut runtime = HarmonicComputationRuntime::new(1);
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());

        // Single node has no other nodes to reach, so centrality = 0
        assert_eq!(result.centralities[0], 0.0);
    }

    #[test]
    fn test_harmonic_two_nodes_connected() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = HarmonicComputationRuntime::new(2);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());

        // Each node can reach one other node at distance 1
        // harmonic = 1/1 / (2-1) = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_harmonic_linear_path() {
        // Linear: 0-1-2-3
        let graph = create_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = HarmonicComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        // Node 0: can reach 1 at d=1, 2 at d=2, 3 at d=3
        //         = (1/1 + 1/2 + 1/3) / 3 = (6/6 + 3/6 + 2/6) / 3 = 11/18
        let expected_0 = (1.0 + 0.5 + 1.0/3.0) / 3.0;
        assert!((result.centralities[0] - expected_0).abs() < 1e-10, 
                "Node 0: expected {}, got {}", expected_0, result.centralities[0]);

        // Node 1: can reach 0 at d=1, 2 at d=1, 3 at d=2
        //         = (1/1 + 1/1 + 1/2) / 3 = 2.5 / 3
        let expected_1 = (1.0 + 1.0 + 0.5) / 3.0;
        assert!((result.centralities[1] - expected_1).abs() < 1e-10,
                "Node 1: expected {}, got {}", expected_1, result.centralities[1]);

        // Node 3: can reach 2 at d=1, 1 at d=2, 0 at d=3
        //         = (1/1 + 1/2 + 1/3) / 3 = 11/18
        let expected_3 = (1.0 + 0.5 + 1.0/3.0) / 3.0;
        assert!((result.centralities[3] - expected_3).abs() < 1e-10,
                "Node 3: expected {}, got {}", expected_3, result.centralities[3]);
    }

    #[test]
    fn test_harmonic_star_graph() {
        // Star: center=0, leaves=[1,2,3,4]
        let graph = create_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = HarmonicComputationRuntime::new(5);
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());

        // Center (0): can reach all 4 leaves at d=1
        //             = (1 + 1 + 1 + 1) / 4 = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10,
                "Center: expected 1.0, got {}", result.centralities[0]);

        // Leaf (1): can reach center at d=1, others at d=2
        //           = (1/1 + 1/2 + 1/2 + 1/2) / 4 = 2.5 / 4 = 0.625
        let expected_leaf = (1.0 + 0.5 + 0.5 + 0.5) / 4.0;
        assert!((result.centralities[1] - expected_leaf).abs() < 1e-10,
                "Leaf: expected {}, got {}", expected_leaf, result.centralities[1]);
    }

    #[test]
    fn test_harmonic_complete_graph() {
        // Complete: all nodes connected to all others
        let graph = create_graph(vec![(0, 1), (0, 2), (1, 2)], 3);
        let mut runtime = HarmonicComputationRuntime::new(3);
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());

        // Each node can reach 2 others at d=1
        // harmonic = (1/1 + 1/1) / 2 = 1.0
        for i in 0..3 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10,
                    "Node {}: expected 1.0, got {}", i, result.centralities[i]);
        }
    }

    #[test]
    fn test_harmonic_disconnected() {
        // Two components: [0-1] and [2-3]
        let graph = create_graph(vec![(0, 1), (2, 3)], 4);
        let mut runtime = HarmonicComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        // Node 0: can reach 1 at d=1, cannot reach 2,3
        //         = 1/1 / 3 = 1/3
        let expected = 1.0 / 3.0;
        assert!((result.centralities[0] - expected).abs() < 1e-10,
                "Node 0: expected {}, got {}", expected, result.centralities[0]);

        // Same for node 1
        assert!((result.centralities[1] - expected).abs() < 1e-10,
                "Node 1: expected {}, got {}", expected, result.centralities[1]);

        // Node 2 and 3 same as 0 and 1
        assert!((result.centralities[2] - expected).abs() < 1e-10);
        assert!((result.centralities[3] - expected).abs() < 1e-10);
    }

    #[test]
    fn test_harmonic_triangle() {
        // Triangle: 0-1-2-0
        let graph = create_graph(vec![(0, 1), (1, 2), (0, 2)], 3);
        let mut runtime = HarmonicComputationRuntime::new(3);
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());

        // Each node can reach 2 others at d=1
        // harmonic = (1/1 + 1/1) / 2 = 1.0
        for i in 0..3 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10);
        }
    }
}

//! Closeness Centrality Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.closeness.ClosenessCentrality`
//!
//! Uses Multi-Source BFS (MSBFS) to compute closeness centrality for all nodes.
//! For each node: closeness_centrality(v) = component_size / sum(distance(v,u))
//!
//! Two variants supported:
//! - Default: component_size / farness
//! - Wasserman-Faust: (component_size / farness) * (component_size / (n-1))

use crate::procedures::msbfs::SimpleMSBFS;

#[derive(Clone)]
pub struct ClosenessCentralityComputationResult {
    pub centralities: Vec<f64>,
}

pub struct ClosenessCentralityComputationRuntime {
    farness: Vec<u64>,          // Sum of distances from each node
    component_size: Vec<u64>,   // Count of reachable nodes from each node
    msbfs: SimpleMSBFS,
}

impl ClosenessCentralityComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            farness: vec![0u64; node_count],
            component_size: vec![0u64; node_count],
            msbfs: SimpleMSBFS::new(node_count),
        }
    }

    pub fn compute(
        &mut self,
        node_count: usize,
        wasserman_faust: bool,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> ClosenessCentralityComputationResult {
        // Reset arrays
        for i in 0..node_count {
            self.farness[i] = 0;
            self.component_size[i] = 0;
        }

        // Phase 1: For each source node, run MSBFS and accumulate distances
        for source_node in 0..node_count {
            self.msbfs.compute(
                &[source_node],
                |node_id, depth, _sources_mask| {
                    // Accumulate distance for all reached nodes
                    // Skip the source itself (depth == 0)
                    if depth > 0 {
                        self.farness[source_node] += depth as u64;
                        self.component_size[source_node] += 1;
                    }
                },
                &get_neighbors,
            );
        }

        // Phase 2: Compute closeness centrality
        let mut centralities = vec![0.0f64; node_count];
        for node_id in 0..node_count {
            centralities[node_id] = self.compute_centrality(
                self.farness[node_id],
                self.component_size[node_id],
                node_count as u64,
                wasserman_faust,
            );
        }

        ClosenessCentralityComputationResult { centralities }
    }

    fn compute_centrality(
        &self,
        farness: u64,
        component_size: u64,
        node_count: u64,
        wasserman_faust: bool,
    ) -> f64 {
        if farness == 0 {
            return 0.0;
        }

        let base_centrality = component_size as f64 / farness as f64;

        if wasserman_faust {
            // Wasserman-Faust normalization: multiply by (component_size / (n-1))
            base_centrality * (component_size as f64 / (node_count - 1) as f64)
        } else {
            // Default: just the base centrality
            base_centrality
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
        let mut relationships: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..node_count {
            relationships.insert(i, Vec::new());
        }

        for (from, to) in edges {
            relationships.entry(from).or_insert_with(Vec::new).push(to);
            if from != to {
                relationships.entry(to).or_insert_with(Vec::new).push(from);
            }
        }

        // Sort for consistency
        for neighbors in relationships.values_mut() {
            neighbors.sort_unstable();
            neighbors.dedup();
        }

        relationships
    }

    #[test]
    fn test_closeness_single_node() {
        let graph = create_graph(vec![], 1);
        let mut runtime = ClosenessCentralityComputationRuntime::new(1);
        let result = runtime.compute(1, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Single node has no other nodes to reach, so centrality = 0
        assert_eq!(result.centralities[0], 0.0);
    }

    #[test]
    fn test_closeness_two_nodes_connected() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = ClosenessCentralityComputationRuntime::new(2);
        let result = runtime.compute(2, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Each node can reach 1 other at distance 1
        // closeness = 1 / 1 = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_closeness_linear_path() {
        let graph = create_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = ClosenessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Node 0: can reach 1@d1, 2@d2, 3@d3 = sum = 6, component = 3
        //         closeness = 3 / 6 = 0.5
        let expected_0 = 3.0 / 6.0;
        assert!((result.centralities[0] - expected_0).abs() < 1e-10,
                "Node 0: expected {}, got {}", expected_0, result.centralities[0]);

        // Node 1: can reach 0@d1, 2@d1, 3@d2 = sum = 4, component = 3
        //         closeness = 3 / 4 = 0.75
        let expected_1 = 3.0 / 4.0;
        assert!((result.centralities[1] - expected_1).abs() < 1e-10,
                "Node 1: expected {}, got {}", expected_1, result.centralities[1]);

        // Node 2: by symmetry like node 1
        let expected_2 = 3.0 / 4.0;
        assert!((result.centralities[2] - expected_2).abs() < 1e-10,
                "Node 2: expected {}, got {}", expected_2, result.centralities[2]);

        // Node 3: by symmetry like node 0
        let expected_3 = 3.0 / 6.0;
        assert!((result.centralities[3] - expected_3).abs() < 1e-10);
    }

    #[test]
    fn test_closeness_star_graph() {
        // Center=0, leaves=[1,2,3,4]
        let graph = create_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = ClosenessCentralityComputationRuntime::new(5);
        let result = runtime.compute(5, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Center: 4 neighbors at d=1 = sum = 4, component = 4
        //         closeness = 4 / 4 = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10,
                "Center: expected 1.0, got {}", result.centralities[0]);

        // Leaf (e.g., node 1): 1@d1 + 3@d2 = sum = 7, component = 4
        //                      closeness = 4 / 7 ≈ 0.571
        let expected_leaf = 4.0 / 7.0;
        for i in 1..5 {
            assert!((result.centralities[i] - expected_leaf).abs() < 1e-10,
                    "Leaf {}: expected {}, got {}", i, expected_leaf, result.centralities[i]);
        }
    }

    #[test]
    fn test_closeness_complete_graph() {
        // All nodes connected to all others
        let graph = create_graph(vec![(0, 1), (0, 2), (1, 2)], 3);
        let mut runtime = ClosenessCentralityComputationRuntime::new(3);
        let result = runtime.compute(3, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Each node reaches 2 others at d=1 = sum = 2, component = 2
        // closeness = 2 / 2 = 1.0
        for i in 0..3 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10,
                    "Node {}: expected 1.0, got {}", i, result.centralities[i]);
        }
    }

    #[test]
    fn test_closeness_wasserman_faust() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = ClosenessCentralityComputationRuntime::new(2);
        let result = runtime.compute(2, true, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Base: 1 / 1 = 1.0
        // Wasserman-Faust: 1.0 * (1 / (2-1)) = 1.0 * 1.0 = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_closeness_disconnected() {
        // Two components: [0-1] and [2-3]
        let graph = create_graph(vec![(0, 1), (2, 3)], 4);
        let mut runtime = ClosenessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Component [0-1]: node 0 reaches 1@d1 = sum = 1, component = 1
        //                  closeness = 1 / 1 = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
        assert!((result.centralities[2] - 1.0).abs() < 1e-10);
        assert!((result.centralities[3] - 1.0).abs() < 1e-10);
    }
}

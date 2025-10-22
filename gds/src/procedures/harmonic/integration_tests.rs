//! Harmonic Centrality Integration Tests
//!
//! Tests using simple graph representations for verification

#[cfg(test)]
mod tests {
    use super::super::computation::HarmonicComputationRuntime;
    use std::collections::HashMap;

    fn build_graph(edges: Vec<(usize, usize)>, node_count: usize) -> HashMap<usize, Vec<usize>> {
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
    fn test_harmonic_single_node() {
        let graph = build_graph(vec![], 1);
        let mut runtime = HarmonicComputationRuntime::new(1);

        let result = runtime.compute(1, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        assert_eq!(result.centralities[0], 0.0);
    }

    #[test]
    fn test_harmonic_two_nodes_connected() {
        let graph = build_graph(vec![(0, 1)], 2);
        let mut runtime = HarmonicComputationRuntime::new(2);

        let result = runtime.compute(2, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0] - 1.0).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_harmonic_linear_path() {
        let graph = build_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = HarmonicComputationRuntime::new(4);

        let result = runtime.compute(4, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Node 0: 1@d1 + 0.5@d2 + 0.333@d3 = 1.833, normalized by 1/3 = 0.611
        let expected_0 = (1.0 + 0.5 + 1.0/3.0) / 3.0;
        assert!((result.centralities[0] - expected_0).abs() < 1e-6,
                "Node 0: expected {}, got {}", expected_0, result.centralities[0]);

        // Node 1: 1@d1 + 1@d1 + 0.5@d2 = 2.5, normalized by 1/3 = 0.833
        let expected_1 = (1.0 + 1.0 + 0.5) / 3.0;
        assert!((result.centralities[1] - expected_1).abs() < 1e-6,
                "Node 1: expected {}, got {}", expected_1, result.centralities[1]);

        // Node 2: 1@d1 + 1@d1 + 0.5@d2 = 2.5, normalized by 1/3 = 0.833
        let expected_2 = (1.0 + 1.0 + 0.5) / 3.0;
        assert!((result.centralities[2] - expected_2).abs() < 1e-6,
                "Node 2: expected {}, got {}", expected_2, result.centralities[2]);

        // Node 3: 1@d1 + 0.5@d2 + 0.333@d3 = 1.833, normalized by 1/3 = 0.611
        let expected_3 = (1.0 + 0.5 + 1.0/3.0) / 3.0;
        assert!((result.centralities[3] - expected_3).abs() < 1e-6,
                "Node 3: expected {}, got {}", expected_3, result.centralities[3]);
    }

    #[test]
    fn test_harmonic_star_graph() {
        // Center=0, leaves=[1,2,3,4]
        let graph = build_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = HarmonicComputationRuntime::new(5);

        let result = runtime.compute(5, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Center: 4@d1 = 4, normalized by 1/4 = 1.0
        assert!((result.centralities[0] - 1.0).abs() < 1e-10,
                "Center: expected 1.0, got {}", result.centralities[0]);

        // Leaf (e.g., node 1): 1@d1 + 3*0.5@d2 = 1 + 1.5 = 2.5, normalized by 1/4 = 0.625
        let expected_leaf = (1.0 + 0.5 + 0.5 + 0.5) / 4.0;
        for i in 1..5 {
            assert!((result.centralities[i] - expected_leaf).abs() < 1e-10,
                    "Leaf {}: expected {}, got {}", i, expected_leaf, result.centralities[i]);
        }
    }

    #[test]
    fn test_harmonic_complete_graph() {
        // All nodes connected to all others
        let graph = build_graph(vec![(0, 1), (0, 2), (1, 2)], 3);
        let mut runtime = HarmonicComputationRuntime::new(3);

        let result = runtime.compute(3, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Each node reaches 2 others at d=1: 2/1 = 2, normalized by 1/2 = 1.0
        for i in 0..3 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10,
                    "Node {}: expected 1.0, got {}", i, result.centralities[i]);
        }
    }

    #[test]
    fn test_harmonic_disconnected() {
        // Two components: [0-1] and [2-3]
        let graph = build_graph(vec![(0, 1), (2, 3)], 4);
        let mut runtime = HarmonicComputationRuntime::new(4);

        let result = runtime.compute(4, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Component [0-1]: each node reaches 1 other at d=1, cannot reach 2,3
        // centrality = 1/1 / 3 = 1/3
        let expected = 1.0 / 3.0;
        for i in 0..4 {
            assert!((result.centralities[i] - expected).abs() < 1e-10,
                    "Node {}: expected {}, got {}", i, expected, result.centralities[i]);
        }
    }

    #[test]
    fn test_harmonic_triangle() {
        // Triangle: 0-1-2-0
        let graph = build_graph(vec![(0, 1), (1, 2), (0, 2)], 3);
        let mut runtime = HarmonicComputationRuntime::new(3);

        let result = runtime.compute(3, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Each node reaches 2 others at d=1: 2/1 = 2, normalized by 1/2 = 1.0
        for i in 0..3 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_harmonic_square_grid() {
        // Square grid:
        // 0 - 1
        // |   |
        // 2 - 3
        let graph = build_graph(vec![(0, 1), (1, 3), (3, 2), (2, 0)], 4);
        let mut runtime = HarmonicComputationRuntime::new(4);

        let result = runtime.compute(4, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Each corner (0,1,2,3): reaches 2 adjacent at d=1, 1 diagonal at d=2
        // centrality = (1 + 1 + 0.5) / 3 = 0.833
        let expected = (1.0 + 1.0 + 0.5) / 3.0;
        for i in 0..4 {
            assert!((result.centralities[i] - expected).abs() < 1e-6,
                    "Node {}: expected {}, got {}", i, expected, result.centralities[i]);
        }
    }

    #[test]
    fn test_harmonic_large_complete_graph() {
        // Complete graph with 10 nodes: each node reaches 9 others at d=1
        let edges: Vec<_> = (0..10)
            .flat_map(|i| (i + 1..10).map(move |j| (i, j)))
            .collect();
        let graph = build_graph(edges, 10);
        let mut runtime = HarmonicComputationRuntime::new(10);

        let result = runtime.compute(10, |node_id| {
            graph.get(&node_id).cloned().unwrap_or_default()
        });

        // Each node reaches 9 others at d=1: 9/1 = 9, normalized by 1/9 = 1.0
        for i in 0..10 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10,
                    "Node {}: expected 1.0, got {}", i, result.centralities[i]);
        }
    }
}

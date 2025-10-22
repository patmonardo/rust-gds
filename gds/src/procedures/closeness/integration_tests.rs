//! Closeness Centrality Integration Tests
//!
//! Tests using simple graph representations for verification

#[cfg(test)]
mod tests {
    use super::super::computation::ClosenessCentralityComputationRuntime;
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
    fn test_closeness_single_node() {
        let graph = build_graph(vec![], 1);
        let mut runtime = ClosenessCentralityComputationRuntime::new(1);
        let result = runtime.compute(1, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert_eq!(result.centralities[0], 0.0);
    }

    #[test]
    fn test_closeness_two_nodes_connected() {
        let graph = build_graph(vec![(0, 1)], 2);
        let mut runtime = ClosenessCentralityComputationRuntime::new(2);
        let result = runtime.compute(2, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0] - 1.0).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_closeness_linear_path() {
        let graph = build_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = ClosenessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        let expected_0 = 3.0 / 6.0;
        assert!((result.centralities[0] - expected_0).abs() < 1e-10);

        let expected_1 = 3.0 / 4.0;
        assert!((result.centralities[1] - expected_1).abs() < 1e-10);

        let expected_2 = 3.0 / 4.0;
        assert!((result.centralities[2] - expected_2).abs() < 1e-10);

        let expected_3 = 3.0 / 6.0;
        assert!((result.centralities[3] - expected_3).abs() < 1e-10);
    }

    #[test]
    fn test_closeness_star_graph() {
        let graph = build_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = ClosenessCentralityComputationRuntime::new(5);
        let result = runtime.compute(5, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0] - 1.0).abs() < 1e-10);

        let expected_leaf = 4.0 / 7.0;
        for i in 1..5 {
            assert!((result.centralities[i] - expected_leaf).abs() < 1e-10);
        }
    }

    #[test]
    fn test_closeness_complete_graph() {
        let graph = build_graph(vec![(0, 1), (0, 2), (1, 2)], 3);
        let mut runtime = ClosenessCentralityComputationRuntime::new(3);
        let result = runtime.compute(3, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        for i in 0..3 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_closeness_square_grid() {
        // Square grid:
        // 0 - 1
        // |   |
        // 2 - 3
        let graph = build_graph(vec![(0, 1), (1, 3), (3, 2), (2, 0)], 4);
        let mut runtime = ClosenessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Each corner: 2@d1 + 1@d2 = sum = 4, component = 3
        // closeness = 3 / 4 = 0.75
        let expected = 3.0 / 4.0;
        for i in 0..4 {
            assert!((result.centralities[i] - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn test_closeness_large_complete_graph() {
        // Complete graph with 10 nodes: each at d=1 to 9 others
        let edges: Vec<_> = (0..10)
            .flat_map(|i| (i + 1..10).map(move |j| (i, j)))
            .collect();
        let graph = build_graph(edges, 10);
        let mut runtime = ClosenessCentralityComputationRuntime::new(10);
        let result = runtime.compute(10, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Each node reaches 9 others at d=1 = sum = 9, component = 9
        // closeness = 9 / 9 = 1.0
        for i in 0..10 {
            assert!((result.centralities[i] - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_closeness_wasserman_faust_normalization() {
        let graph = build_graph(vec![(0, 1), (1, 2)], 3);
        let mut runtime = ClosenessCentralityComputationRuntime::new(3);
        
        // Wasserman-Faust normalized
        let result_wf = runtime.compute(3, true, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Without normalization
        let mut runtime2 = ClosenessCentralityComputationRuntime::new(3);
        let result_default = runtime2.compute(3, false, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Wasserman-Faust should apply additional normalization factor
        // For node 0: base = 2/3, WF = (2/3) * (2/2) = 2/3
        // For node 1: base = 2/2 = 1.0, WF = 1.0 * (2/2) = 1.0
        assert!((result_wf.centralities[0] - (2.0 / 3.0)).abs() < 1e-10);
        assert!((result_wf.centralities[1] - 1.0).abs() < 1e-10);
    }
}

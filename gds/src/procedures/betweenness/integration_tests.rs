//! Betweenness Centrality Integration Tests

#[cfg(test)]
mod tests {
    use super::super::computation::BetweennessCentralityComputationRuntime;
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

        for neighbors in relationships.values_mut() {
            neighbors.sort_unstable();
            neighbors.dedup();
        }

        relationships
    }

    #[test]
    fn test_betweenness_single_edge() {
        let graph = build_graph(vec![(0, 1)], 2);
        let mut runtime = BetweennessCentralityComputationRuntime::new(2);
        let result = runtime.compute(2, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0]).abs() < 1e-10);
        assert!((result.centralities[1]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_path_three_nodes() {
        let graph = build_graph(vec![(0, 1), (1, 2)], 3);
        let mut runtime = BetweennessCentralityComputationRuntime::new(3);
        let result = runtime.compute(3, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0]).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
        assert!((result.centralities[2]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_path_four_nodes() {
        let graph = build_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0]).abs() < 1e-10);
        assert!(result.centralities[1] > 0.0);
        assert!(result.centralities[2] > 0.0);
        assert!((result.centralities[3]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_star_graph() {
        let graph = build_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = BetweennessCentralityComputationRuntime::new(5);
        let result = runtime.compute(5, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        assert!((result.centralities[0] - 6.0).abs() < 1e-10);
        for i in 1..5 {
            assert!((result.centralities[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_betweenness_complete_graph() {
        let graph = build_graph(
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
            4,
        );
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        for i in 0..4 {
            assert!((result.centralities[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_betweenness_diamond_graph() {
        let graph = build_graph(vec![(0, 1), (0, 2), (1, 3), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Nodes 1 and 2 should have equal centrality (symmetric structure)
        assert!((result.centralities[1] - result.centralities[2]).abs() < 1e-10);
        
        // All centralities should be non-negative
        for i in 0..4 {
            assert!(result.centralities[i] >= 0.0);
        }
    }

    #[test]
    fn test_betweenness_cycle_four_nodes() {
        let graph = build_graph(vec![(0, 1), (1, 2), (2, 3), (3, 0)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // In a 4-cycle, all nodes have equal centrality (symmetric)
        let expected_centrality = result.centralities[0];
        for i in 1..4 {
            assert!((result.centralities[i] - expected_centrality).abs() < 1e-10);
        }
    }

    #[test]
    fn test_betweenness_two_triangles() {
        // Two triangles sharing a single node: (0,1,2) connected to (0,3,4)
        let graph = build_graph(vec![(0, 1), (1, 2), (0, 2), (0, 3), (3, 4), (0, 4)], 5);
        let mut runtime = BetweennessCentralityComputationRuntime::new(5);
        let result = runtime.compute(5, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Node 0 connects the two triangles
        assert!(result.centralities[0] > 0.0,
                "Node 0 should have high centrality (bridge)");
        
        // All nodes should have non-negative centrality
        for i in 0..5 {
            assert!(result.centralities[i] >= 0.0,
                    "Node {}: centrality should be non-negative", i);
        }
    }
}

//! KCore Decomposition Integration Tests

#[cfg(test)]
mod tests {
    use crate::procedures::kcore::computation::KCoreDecompositionRuntime;
    use std::collections::HashMap;

    fn create_graph(edges: Vec<(usize, usize)>, node_count: usize) -> HashMap<usize, Vec<usize>> {
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

    fn verify_kcore(
        core_values: &[i32],
        graph: &HashMap<usize, Vec<usize>>,
        degeneracy: i32,
    ) -> bool {
        // Verify: max core value <= degeneracy
        let max_core = *core_values.iter().max().unwrap_or(&0);
        if max_core > degeneracy {
            return false;
        }

        // Verify: for each node with core k, induced subgraph has min degree k
        for k in 1..=degeneracy {
            let core_k_nodes: Vec<usize> = core_values
                .iter()
                .enumerate()
                .filter(|(_, &v)| v >= k)
                .map(|(i, _)| i)
                .collect();

            // Check minimum degree in core-k subgraph
            for &node in &core_k_nodes {
                let neighbors_in_core_k = graph
                    .get(&node)
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|&&n| core_values[n] >= k)
                    .count();

                if neighbors_in_core_k < k as usize {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_single_node() {
        let graph = create_graph(vec![], 1);
        let mut runtime = KCoreDecompositionRuntime::new(1);
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.core_values[0], 0);
        assert_eq!(result.degeneracy, 0);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_two_nodes_no_edge() {
        let graph = create_graph(vec![], 2);
        let mut runtime = KCoreDecompositionRuntime::new(2);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.core_values[0], 0);
        assert_eq!(result.core_values[1], 0);
        assert_eq!(result.degeneracy, 0);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_two_nodes_with_edge() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = KCoreDecompositionRuntime::new(2);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());

        // Both nodes have degree 1, so both are in 1-core
        assert!(result.core_values.iter().all(|&v| v >= 1));
        assert_eq!(result.degeneracy, 1);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_triangle() {
        // 0-1-2-0 (complete graph K3)
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let graph = create_graph(edges, 3);
        let mut runtime = KCoreDecompositionRuntime::new(3);
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());

        // All nodes have degree 2, so all are in 2-core
        assert!(result.core_values.iter().all(|&v| v >= 2));
        assert_eq!(result.degeneracy, 2);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_square() {
        // 0-1-2-3-0 (cycle of length 4)
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let graph = create_graph(edges, 4);
        let mut runtime = KCoreDecompositionRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        // All nodes have degree 2, so all are in 2-core
        assert!(result.core_values.iter().all(|&v| v >= 2));
        assert_eq!(result.degeneracy, 2);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_star_graph() {
        // Center (0) connected to leaves (1,2,3,4)
        let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let graph = create_graph(edges, 5);
        let mut runtime = KCoreDecompositionRuntime::new(5);
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());

        // In a star graph:
        // - Leaves have degree 1
        // - Center has degree 4
        // - Leaves removed first (when scanning_degree=1, leaves removed)
        // - Center left with degree 0 (assigned to 0-core)
        // - Degeneracy is 1 (max core value of any removed node)
        
        // All leaves should be in 1-core (not 0)
        for i in 1..5 {
            assert!(result.core_values[i] >= 1, "Leaf {} should be >= 1-core", i);
        }
        assert_eq!(result.degeneracy, 1);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_complete_graph_k4() {
        // All nodes connected to all others
        let edges = vec![
            (0, 1), (0, 2), (0, 3),
            (1, 2), (1, 3),
            (2, 3),
        ];
        let graph = create_graph(edges, 4);
        let mut runtime = KCoreDecompositionRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        // All nodes have degree 3, so all are in 3-core
        assert!(result.core_values.iter().all(|&v| v >= 3));
        assert_eq!(result.degeneracy, 3);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }

    #[test]
    fn test_mixed_degrees() {
        // Complex graph with mixed degrees
        let edges = vec![
            (0, 1), (0, 2), (0, 3),
            (1, 2),
            (3, 4),
            (4, 5),
        ];
        let graph = create_graph(edges, 6);
        let mut runtime = KCoreDecompositionRuntime::new(6);
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());

        // Verify structure is valid
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
        // Verify degeneracy is correct (should be 2 for the triangle 0-1-2)
        assert_eq!(result.degeneracy, 2);
    }

    #[test]
    fn test_disconnected_components() {
        // Two disconnected components
        let edges = vec![
            (0, 1), (1, 2), (2, 0), // Triangle
            (3, 4), (4, 5), (5, 3), // Another triangle
        ];
        let graph = create_graph(edges, 6);
        let mut runtime = KCoreDecompositionRuntime::new(6);
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());

        // Both components are 2-cores
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
        assert_eq!(result.degeneracy, 2);
    }

    #[test]
    fn test_path_graph() {
        // Linear path: 0-1-2-3-4
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let graph = create_graph(edges, 5);
        let mut runtime = KCoreDecompositionRuntime::new(5);
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());

        // Path graph has degeneracy 1 (all nodes removed when degree < 2)
        assert_eq!(result.degeneracy, 1);
        assert!(verify_kcore(&result.core_values, &graph, result.degeneracy));
    }
}

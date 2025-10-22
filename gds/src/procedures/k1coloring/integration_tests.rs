//! K1Coloring Integration Tests

#[cfg(test)]
mod tests {
    use crate::procedures::k1coloring::computation::K1ColoringComputationRuntime;
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

    fn is_valid_coloring(
        colors: &[u64],
        graph: &HashMap<usize, Vec<usize>>,
    ) -> bool {
        // Check that no adjacent nodes have the same color
        for (node_id, neighbors) in graph {
            let node_color = colors[*node_id];
            for &neighbor in neighbors {
                if *node_id != neighbor && colors[neighbor] == node_color {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_single_node() {
        let graph = create_graph(vec![], 1);
        let mut runtime = K1ColoringComputationRuntime::new(1, 10);
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        assert!(result.did_converge);
    }

    #[test]
    fn test_two_nodes_no_edge() {
        let graph = create_graph(vec![], 2);
        let mut runtime = K1ColoringComputationRuntime::new(2, 10);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        assert!(result.did_converge);
    }

    #[test]
    fn test_two_nodes_with_edge() {
        // Two nodes connected: need different colors
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = K1ColoringComputationRuntime::new(2, 10);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        assert_ne!(result.colors[0], result.colors[1]);
        assert!(result.did_converge);
    }

    #[test]
    fn test_path_graph() {
        // Linear path: 0-1-2-3
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = create_graph(edges, 4);
        let mut runtime = K1ColoringComputationRuntime::new(4, 10);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        // Path graph needs at most 2 colors
        let unique_colors: std::collections::HashSet<_> = result.colors.iter().collect();
        assert!(unique_colors.len() <= 3); // Allow some margin for greedy algorithm
    }

    #[test]
    fn test_triangle() {
        // Triangle: 0-1-2-0
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let graph = create_graph(edges, 3);
        let mut runtime = K1ColoringComputationRuntime::new(3, 10);
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        // Triangle needs exactly 3 colors
        let unique_colors: std::collections::HashSet<_> = result.colors.iter().collect();
        assert!(unique_colors.len() >= 3);
    }

    #[test]
    fn test_square() {
        // Square: 0-1-2-3-0
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let graph = create_graph(edges, 4);
        let mut runtime = K1ColoringComputationRuntime::new(4, 10);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        // Square (bipartite) needs exactly 2 colors
        let unique_colors: std::collections::HashSet<_> = result.colors.iter().collect();
        assert!(unique_colors.len() <= 3); // Allow margin for greedy
    }

    #[test]
    fn test_star_graph() {
        // Star: center node 0 connected to 1,2,3,4
        let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let graph = create_graph(edges, 5);
        let mut runtime = K1ColoringComputationRuntime::new(5, 10);
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        // Star needs exactly 2 colors (center + one for leaves)
        let unique_colors: std::collections::HashSet<_> = result.colors.iter().collect();
        assert!(unique_colors.len() >= 2);
    }

    #[test]
    fn test_complete_graph_k4() {
        // Complete graph: all nodes connected to all others
        let edges = vec![
            (0, 1), (0, 2), (0, 3),
            (1, 2), (1, 3),
            (2, 3),
        ];
        let graph = create_graph(edges, 4);
        let mut runtime = K1ColoringComputationRuntime::new(4, 20);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        // K4 needs exactly 4 colors
        let unique_colors: std::collections::HashSet<_> = result.colors.iter().collect();
        assert!(unique_colors.len() >= 4);
    }

    #[test]
    fn test_disconnected_components() {
        // Two disconnected triangles
        let edges = vec![
            (0, 1), (1, 2), (2, 0),
            (3, 4), (4, 5), (5, 3),
        ];
        let graph = create_graph(edges, 6);
        let mut runtime = K1ColoringComputationRuntime::new(6, 10);
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
    }

    #[test]
    fn test_bipartite_graph() {
        // Bipartite graph: two groups with all edges between groups
        let edges = vec![
            (0, 2), (0, 3),
            (1, 2), (1, 3),
        ];
        let graph = create_graph(edges, 4);
        let mut runtime = K1ColoringComputationRuntime::new(4, 10);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert!(is_valid_coloring(&result.colors, &graph));
        // Bipartite graph needs exactly 2 colors
        let unique_colors: std::collections::HashSet<_> = result.colors.iter().collect();
        assert!(unique_colors.len() <= 2);
    }
}

//! Triangle Count Integration Tests

#[cfg(test)]
mod tests {
    use crate::procedures::triangle_count::computation::TriangleCountComputationRuntime;
    use std::collections::HashMap;

    fn create_graph(edges: Vec<(usize, usize)>, node_count: usize) -> HashMap<usize, Vec<usize>> {
        let mut graph = HashMap::new();
        for i in 0..node_count {
            graph.insert(i, Vec::new());
        }
        for (from, to) in edges {
            graph.entry(from).or_insert_with(Vec::new).push(to);
            graph.entry(to).or_insert_with(Vec::new).push(from);
        }
        // Sort for consistent ordering
        for neighbors in graph.values_mut() {
            neighbors.sort_unstable();
        }
        graph
    }

    #[test]
    fn test_no_triangles() {
        // Path: 0-1-2-3 (no triangles)
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = create_graph(edges, 4);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 0);
        for count in &result.local_triangles {
            assert_eq!(*count, 0);
        }
    }

    #[test]
    fn test_single_triangle() {
        // Single triangle: 0-1-2
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let graph = create_graph(edges, 3);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 1);
        // Each node in triangle should have count 1
        assert_eq!(result.local_triangles[0], 1);
        assert_eq!(result.local_triangles[1], 1);
        assert_eq!(result.local_triangles[2], 1);
    }

    #[test]
    fn test_two_triangles_sharing_edge() {
        // Two triangles sharing edge 0-1: (0-1-2) and (0-1-3)
        let edges = vec![(0, 1), (1, 2), (2, 0), (0, 3), (1, 3)];
        let graph = create_graph(edges, 4);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 2);
        // Nodes 0 and 1 are in both triangles
        assert_eq!(result.local_triangles[0], 2);
        assert_eq!(result.local_triangles[1], 2);
        // Nodes 2 and 3 are each in one triangle
        assert_eq!(result.local_triangles[2], 1);
        assert_eq!(result.local_triangles[3], 1);
    }

    #[test]
    fn test_tetrahedron() {
        // Complete graph K4 (tetrahedron): 4 triangles
        let edges = vec![
            (0, 1), (0, 2), (0, 3),
            (1, 2), (1, 3),
            (2, 3),
        ];
        let graph = create_graph(edges, 4);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 4);
        // Each node is in 3 triangles
        for count in &result.local_triangles {
            assert_eq!(*count, 3);
        }
    }

    #[test]
    fn test_square_with_diagonal() {
        // Square with one diagonal: 0-1-2-3-0 + diagonal 0-2
        // Triangles: (0-1-2) and (0-2-3)
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)];
        let graph = create_graph(edges, 4);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 2);
        // Nodes 0 and 2 are in both triangles
        assert_eq!(result.local_triangles[0], 2);
        assert_eq!(result.local_triangles[2], 2);
        // Nodes 1 and 3 are each in one triangle
        assert_eq!(result.local_triangles[1], 1);
        assert_eq!(result.local_triangles[3], 1);
    }

    #[test]
    fn test_empty_graph() {
        // No edges
        let graph = create_graph(vec![], 3);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 0);
        for count in &result.local_triangles {
            assert_eq!(*count, 0);
        }
    }

    #[test]
    fn test_single_node() {
        // Single node, no edges
        let graph = create_graph(vec![], 1);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 0);
        assert_eq!(result.local_triangles[0], 0);
    }

    #[test]
    fn test_k5_complete_graph() {
        // Complete graph K5: 10 triangles
        let edges = vec![
            (0, 1), (0, 2), (0, 3), (0, 4),
            (1, 2), (1, 3), (1, 4),
            (2, 3), (2, 4),
            (3, 4),
        ];
        let graph = create_graph(edges, 5);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());

        // K5 has C(5,3) = 10 triangles
        assert_eq!(result.global_triangles, 10);
        // Each node is in C(4,2) = 6 triangles
        for count in &result.local_triangles {
            assert_eq!(*count, 6);
        }
    }

    #[test]
    fn test_two_disconnected_triangles() {
        // Two separate triangles: (0-1-2) and (3-4-5)
        let edges = vec![
            (0, 1), (1, 2), (2, 0),
            (3, 4), (4, 5), (5, 3),
        ];
        let graph = create_graph(edges, 6);

        let mut runtime = TriangleCountComputationRuntime::new();
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());

        assert_eq!(result.global_triangles, 2);
        // Each node is in one triangle
        for i in 0..6 {
            assert_eq!(result.local_triangles[i], 1);
        }
    }
}

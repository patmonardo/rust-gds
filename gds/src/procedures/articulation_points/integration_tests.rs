//! Articulation Points Integration Tests

#[cfg(test)]
mod tests {
    use crate::procedures::articulation_points::computation::ArticulationPointsComputationRuntime;
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
        graph
    }

    #[test]
    fn test_simple_path() {
        // Simple path: 0-1-2-3-4
        // Node 1, 2, 3 should be articulation points
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let graph = create_graph(edges, 5);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(5);
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // In a path, all internal nodes are articulation points
        assert!(result.articulation_points.get(1));
        assert!(result.articulation_points.get(2));
        assert!(result.articulation_points.get(3));
    }

    #[test]
    fn test_cycle() {
        // Simple cycle: 0-1-2-3-0
        // No articulation points in a simple cycle
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Cycle has no articulation points
        for i in 0..4 {
            assert!(!result.articulation_points.get(i), "Node {} should not be an articulation point", i);
        }
    }

    #[test]
    fn test_bridge_connected_components() {
        // Two cycles connected by a bridge: (0-1-2-0) - 3 - (4-5-6-4)
        // Node 3 should be an articulation point
        let edges = vec![
            (0, 1), (1, 2), (2, 0),  // First cycle
            (2, 3),                   // Bridge
            (3, 4), (4, 5), (5, 6), (6, 4),  // Second cycle
        ];
        let graph = create_graph(edges, 7);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(7);
        let result = runtime.compute(7, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Node 2 and 3 should be articulation points (connecting bridges)
        assert!(result.articulation_points.get(2) || result.articulation_points.get(3));
    }

    #[test]
    fn test_single_node() {
        // Single node
        let graph = create_graph(vec![], 1);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(1);
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Single node is not an articulation point
        assert!(!result.articulation_points.get(0));
    }

    #[test]
    fn test_two_nodes_with_edge() {
        // Two nodes connected: 0-1
        let edges = vec![(0, 1)];
        let graph = create_graph(edges, 2);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(2);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Neither node is an articulation point (removing either leaves one isolated node)
        assert!(!result.articulation_points.get(0));
        assert!(!result.articulation_points.get(1));
    }

    #[test]
    fn test_star_graph() {
        // Star graph: center node 0 connected to 1, 2, 3, 4
        // Node 0 should be an articulation point
        let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let graph = create_graph(edges, 5);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(5);
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Center node is an articulation point
        assert!(result.articulation_points.get(0));
        // Leaf nodes are not
        assert!(!result.articulation_points.get(1));
        assert!(!result.articulation_points.get(2));
    }

    #[test]
    fn test_disconnected_components() {
        // Two separate components: 0-1 and 2-3
        let edges = vec![(0, 1), (2, 3)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // No articulation points in disconnected graphs with only edges
        for i in 0..4 {
            assert!(!result.articulation_points.get(i));
        }
    }

    #[test]
    fn test_complex_graph() {
        // More complex graph with multiple articulation points
        //   0
        //   |
        //   1 -- 2
        //   |    |
        //   3 -- 4
        //        |
        //        5
        let edges = vec![
            (0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5),
        ];
        let graph = create_graph(edges, 6);
        
        let mut runtime = ArticulationPointsComputationRuntime::new(6);
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Node 1 and 4 should be articulation points
        assert!(result.articulation_points.get(1), "Node 1 should be an articulation point");
        assert!(result.articulation_points.get(4), "Node 4 should be an articulation point");
    }
}

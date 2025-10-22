//! WCC Integration Tests

#[cfg(test)]
mod tests {
    use crate::procedures::wcc::computation::WccComputationRuntime;
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
    fn test_single_component() {
        // All nodes in one component: 0-1-2-3
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 1 component
        assert_eq!(result.component_count, 1);
        // All nodes should have same component id
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[1], result.components[2]);
        assert_eq!(result.components[2], result.components[3]);
    }

    #[test]
    fn test_two_components() {
        // Two components: 0-1 and 2-3
        let edges = vec![(0, 1), (2, 3)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 2 components
        assert_eq!(result.component_count, 2);
        // Nodes in different components should have different ids
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[2], result.components[3]);
        assert_ne!(result.components[0], result.components[2]);
    }

    #[test]
    fn test_isolated_nodes() {
        // All nodes isolated
        let graph = create_graph(vec![], 5);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 5 components (one per node)
        assert_eq!(result.component_count, 5);
    }

    #[test]
    fn test_single_node() {
        // Single node
        let graph = create_graph(vec![], 1);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 1 component
        assert_eq!(result.component_count, 1);
    }

    #[test]
    fn test_cycle() {
        // Single cycle: 0-1-2-3-0
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 1 component
        assert_eq!(result.component_count, 1);
        // All nodes should have same component
        for i in 1..4 {
            assert_eq!(result.components[i], result.components[0]);
        }
    }

    #[test]
    fn test_complex_graph() {
        //   0
        //   |
        //   1 - 2    5
        //   |   |    |
        //   3 - 4    6
        let edges = vec![
            (0, 1), (1, 2), (1, 3), (2, 4), (3, 4),  // First component
            (5, 6),  // Second component
        ];
        let graph = create_graph(edges, 7);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(7, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 3 components: {0-4}, {5-6}, and implicitly any isolated nodes
        // Actually: {0,1,2,3,4}, {5,6}
        assert_eq!(result.component_count, 2);
        // 0,1,2,3,4 in one component
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[1], result.components[4]);
        // 5,6 in another component
        assert_eq!(result.components[5], result.components[6]);
        // Different from first component
        assert_ne!(result.components[0], result.components[5]);
    }

    #[test]
    fn test_star_graph() {
        // Star: 0 connected to 1,2,3,4
        let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let graph = create_graph(edges, 5);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(5, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 1 component
        assert_eq!(result.component_count, 1);
        // All nodes should have same component
        for i in 1..5 {
            assert_eq!(result.components[i], result.components[0]);
        }
    }

    #[test]
    fn test_three_components() {
        // Three separate components
        let edges = vec![(0, 1), (2, 3), (4, 5)];
        let graph = create_graph(edges, 6);
        
        let mut runtime = WccComputationRuntime::new();
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Should have 3 components
        assert_eq!(result.component_count, 3);
    }
}

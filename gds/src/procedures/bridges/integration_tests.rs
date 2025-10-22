//! Bridges Integration Tests

#[cfg(test)]
mod tests {
    use crate::procedures::bridges::computation::BridgesComputationRuntime;
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
    fn test_simple_bridge() {
        // Simple path: 0-1-2 (edge 0-1 and 1-2 are both bridges)
        let edges = vec![(0, 1), (1, 2)];
        let graph = create_graph(edges, 3);
        
        let mut runtime = BridgesComputationRuntime::new(3);
        let result = runtime.compute(3, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Both edges should be bridges
        assert_eq!(result.bridges.len(), 2);
    }

    #[test]
    fn test_cycle_no_bridges() {
        // Simple cycle: 0-1-2-3-0 (no bridges)
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = BridgesComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // No bridges in a simple cycle
        assert_eq!(result.bridges.len(), 0);
    }

    #[test]
    fn test_two_cycles_with_bridge() {
        // Two cycles connected by a bridge: (0-1-2-0) - (3-4-5-3)
        let edges = vec![
            (0, 1), (1, 2), (2, 0),  // First cycle
            (2, 3),                   // Bridge
            (3, 4), (4, 5), (5, 3),  // Second cycle
        ];
        let graph = create_graph(edges, 6);
        
        let mut runtime = BridgesComputationRuntime::new(6);
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Only the edge 2-3 is a bridge
        assert_eq!(result.bridges.len(), 1);
    }

    #[test]
    fn test_single_node() {
        // Single node - no edges, no bridges
        let graph = create_graph(vec![], 1);
        
        let mut runtime = BridgesComputationRuntime::new(1);
        let result = runtime.compute(1, |node| graph.get(&node).cloned().unwrap_or_default());
        
        assert_eq!(result.bridges.len(), 0);
    }

    #[test]
    fn test_two_nodes() {
        // Two nodes with one edge: 0-1 (this is a bridge)
        let edges = vec![(0, 1)];
        let graph = create_graph(edges, 2);
        
        let mut runtime = BridgesComputationRuntime::new(2);
        let result = runtime.compute(2, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Single edge is a bridge
        assert_eq!(result.bridges.len(), 1);
    }

    #[test]
    fn test_star_graph_no_bridges() {
        // Star: center 0 connected to 1,2,3 with triangles
        // 0-1-2-0 (cycle), 0-2-3-0 (cycle) - no bridges
        let edges = vec![(0, 1), (1, 2), (2, 0), (0, 2), (2, 3), (3, 0)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = BridgesComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // With multiple cycles, no bridges
        assert_eq!(result.bridges.len(), 0);
    }

    #[test]
    fn test_complex_graph() {
        //   0
        //   |
        //   1 - 2
        //   |   |
        //   3 - 4
        //       |
        //       5
        let edges = vec![
            (0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5),
        ];
        let graph = create_graph(edges, 6);
        
        let mut runtime = BridgesComputationRuntime::new(6);
        let result = runtime.compute(6, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Bridges: 0-1 and 4-5
        assert_eq!(result.bridges.len(), 2);
    }

    #[test]
    fn test_disconnected_components() {
        // Two separate components: 0-1 and 2-3
        let edges = vec![(0, 1), (2, 3)];
        let graph = create_graph(edges, 4);
        
        let mut runtime = BridgesComputationRuntime::new(4);
        let result = runtime.compute(4, |node| graph.get(&node).cloned().unwrap_or_default());
        
        // Both edges are bridges
        assert_eq!(result.bridges.len(), 2);
    }
}

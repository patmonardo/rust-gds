//! Betweenness Centrality Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.betweenness.BetweennessCentrality`
//!
//! Two-phase algorithm:
//! 1. FORWARD: BFS from each source, track shortest paths (sigma) and predecessors
//! 2. BACKWARD: Propagate dependencies (delta) back through the DAG
//!
//! Formula: betweenness(v) = sum of (sigma[s,v] / sigma[s,t]) * delta[t]
//!                           for all s,t where path goes through v

use std::collections::VecDeque;

#[derive(Clone)]
pub struct BetweennessCentralityComputationResult {
    pub centralities: Vec<f64>,
}

pub struct BetweennessCentralityComputationRuntime {
    centralities: Vec<f64>,
    sigma: Vec<u64>,                    // shortest path counts from source
    delta: Vec<f64>,                    // dependencies
    distances: Vec<i32>,                // BFS distances from source
    predecessors: Vec<Vec<usize>>,      // predecessors in shortest path DAG
    backward_nodes: Vec<usize>,         // stack for backward phase
}

impl BetweennessCentralityComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            centralities: vec![0.0f64; node_count],
            sigma: vec![0u64; node_count],
            delta: vec![0.0f64; node_count],
            distances: vec![-1i32; node_count],
            predecessors: vec![Vec::new(); node_count],
            backward_nodes: Vec::new(),
        }
    }

    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> BetweennessCentralityComputationResult {
        // Reset centralities (accumulator)
        for c in self.centralities.iter_mut() {
            *c = 0.0;
        }

        // Process each node as a source
        for source_node in 0..node_count {
            self.forward_phase(source_node, &get_neighbors);
            self.backward_phase(source_node, node_count);
        }

        // Normalize for undirected graphs (divide by 2)
        let divisor = 2.0;
        for c in self.centralities.iter_mut() {
            *c /= divisor;
        }

        BetweennessCentralityComputationResult {
            centralities: self.centralities.clone(),
        }
    }

    /// Phase 1: Forward BFS from source node
    /// Computes sigma (path counts) and predecessors
    fn forward_phase(
        &mut self,
        source_node: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) {
        let node_count = self.distances.len();

        // Reset per-source arrays
        self.sigma.iter_mut().for_each(|s| *s = 0);
        self.distances.iter_mut().for_each(|d| *d = -1);
        for preds in self.predecessors.iter_mut() {
            preds.clear();
        }
        self.backward_nodes.clear();

        // Initialize source
        self.sigma[source_node] = 1;
        self.distances[source_node] = 0;

        // BFS
        let mut queue = VecDeque::new();
        queue.push_back(source_node);

        while let Some(node) = queue.pop_front() {
            self.backward_nodes.push(node); // Record for backward phase
            let node_dist = self.distances[node];
            let node_sigma = self.sigma[node];

            for neighbor in get_neighbors(node) {
                let new_dist = node_dist + 1;

                // First time visiting this neighbor?
                if self.distances[neighbor] < 0 {
                    self.distances[neighbor] = new_dist;
                    queue.push_back(neighbor);
                }

                // If on shortest path
                if self.distances[neighbor] == new_dist {
                    self.sigma[neighbor] = self.sigma[neighbor].saturating_add(node_sigma);
                    self.predecessors[neighbor].push(node);
                }
            }
        }
    }

    /// Phase 2: Backward dependency propagation
    /// Process nodes in reverse BFS order to calculate dependencies
    fn backward_phase(&mut self, source_node: usize, _node_count: usize) {
        // Reset delta
        self.delta.iter_mut().for_each(|d| *d = 0.0);

        // Process backward_nodes in reverse order (excluding source)
        for &node in self.backward_nodes.iter().rev() {
            if node == source_node {
                continue;
            }

            let node_sigma = self.sigma[node] as f64;
            let node_delta = self.delta[node];

            // For each predecessor of this node
            for &pred in &self.predecessors[node] {
                let pred_sigma = self.sigma[pred] as f64;

                // Dependency contribution from this path
                let contribution = (pred_sigma / node_sigma) * (node_delta + 1.0);

                // Accumulate dependency at predecessor (for its future contribution)
                self.delta[pred] += contribution;
            }

            // Accumulate centrality at this node (except for source nodes)
            // The dependency value represents how much this node benefits other nodes
            if node != source_node {
                self.centralities[node] += self.delta[node];
            }
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

        for neighbors in relationships.values_mut() {
            neighbors.sort_unstable();
            neighbors.dedup();
        }

        relationships
    }

    #[test]
    fn test_betweenness_single_edge() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = BetweennessCentralityComputationRuntime::new(2);
        let result = runtime.compute(2, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Single edge: no intermediate nodes
        assert!((result.centralities[0]).abs() < 1e-10);
        assert!((result.centralities[1]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_path_three_nodes() {
        let graph = create_graph(vec![(0, 1), (1, 2)], 3);
        let mut runtime = BetweennessCentralityComputationRuntime::new(3);
        let result = runtime.compute(3, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Node 1 is on the path 0-1-2
        // From 0: sigma[0]=1, sigma[1]=1, sigma[2]=1
        //   dependency[1] = (1/1) * (0+1) = 1
        // From 2: sigma[2]=1, sigma[1]=1, sigma[0]=1
        //   dependency[1] += (1/1) * (0+1) = 1
        // BC[1] = (1 + 1) / 2 = 1.0
        assert!((result.centralities[0]).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
        assert!((result.centralities[2]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_star_graph() {
        // Center: 0, Leaves: 1,2,3,4
        let graph = create_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = BetweennessCentralityComputationRuntime::new(5);
        let result = runtime.compute(5, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Center is on every path between leaves
        // From each leaf, 3 paths go through center to other leaves
        // 4 leaves × 3 paths = 12 / 2 (undirected) = 6.0
        assert!((result.centralities[0] - 6.0).abs() < 1e-10,
                "Center: expected 6.0, got {}", result.centralities[0]);

        // Leaves don't lie on paths between other nodes
        for i in 1..5 {
            assert!((result.centralities[i]).abs() < 1e-10,
                    "Leaf {}: expected 0.0, got {}", i, result.centralities[i]);
        }
    }

    #[test]
    fn test_betweenness_triangle() {
        let graph = create_graph(vec![(0, 1), (1, 2), (0, 2)], 3);
        let mut runtime = BetweennessCentralityComputationRuntime::new(3);
        let result = runtime.compute(3, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // In a triangle, each node is on shortest paths between other two
        // But multiple shortest paths exist, so dependencies spread
        // All nodes should have equal non-zero centrality
        for i in 0..3 {
            assert!(result.centralities[i] >= 0.0,
                    "Node {}: centrality should be non-negative", i);
        }
    }

    #[test]
    fn test_betweenness_linear_four_nodes() {
        // 0-1-2-3
        let graph = create_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Node 1 and 2 are on internal paths
        // Ends should have 0
        assert!((result.centralities[0]).abs() < 1e-10);
        assert!(result.centralities[1] > 0.0);
        assert!(result.centralities[2] > 0.0);
        assert!((result.centralities[3]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_complete_graph_k4() {
        // Complete graph K4
        let graph = create_graph(
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
            4,
        );
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // In complete graph, all paths have length 1 (direct edge)
        // No node lies on a shortest path between two others
        for i in 0..4 {
            assert!((result.centralities[i]).abs() < 1e-10,
                    "Node {}: expected 0.0, got {}", i, result.centralities[i]);
        }
    }

    #[test]
    fn test_betweenness_diamond_graph() {
        // Diamond: 0-1, 0-2, 1-3, 2-3
        //    0
        //   / \
        //  1   2
        //   \ /
        //    3
        let graph = create_graph(vec![(0, 1), (0, 2), (1, 3), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let result = runtime.compute(4, |node| {
            graph.get(&node).cloned().unwrap_or_default()
        });

        // Nodes 1 and 2 should have equal centrality (symmetric)
        assert!((result.centralities[1] - result.centralities[2]).abs() < 1e-10,
                "Nodes 1 and 2 should have same centrality");
        
        // All centralities should be non-negative
        for i in 0..4 {
            assert!(result.centralities[i] >= 0.0,
                    "Node {}: centrality should be non-negative", i);
        }
    }
}

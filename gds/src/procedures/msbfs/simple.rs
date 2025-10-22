//! Simple Multi-Source BFS Implementation
//!
//! Sequential, bit-packed implementation of MSBFS using u64 bitmasks.
//! Each bit represents whether a source node has reached a particular target node
//! at the current BFS depth.

/// Simplified Multi-Source BFS using bit-packed u64 masks
///
/// **OMEGA = 64**: Can simultaneously process up to 64 source nodes
/// by using 64 bits of a u64 to track which sources have reached each node.
///
/// # Example
///
/// ```ignore
/// let mut msbfs = SimpleMSBFS::new(node_count);
/// msbfs.compute(
///     &source_nodes,
///     |node_id, depth, sources_mask| {
///         // Called for each (node, depth) pair reached by sources
///         // sources_mask indicates which source nodes are at this node
///         println!("Node {} at depth {} reached by {} sources", 
///                  node_id, depth, sources_mask.count_ones());
///     },
///     |node_id| graph.neighbors(node_id),
/// );
/// let result = msbfs.result();
/// ```
pub struct SimpleMSBFS {
    /// Current frontier: visit_set[node] = bitmask of sources at this node
    visit_set: Vec<u64>,
    /// Next frontier: will become visit_set in next iteration
    visit_next_set: Vec<u64>,
    /// Already visited: seen_set[node] = bitmask of sources that visited this node
    seen_set: Vec<u64>,
    /// Number of nodes in graph
    node_count: usize,
}

impl SimpleMSBFS {
    /// Create a new MSBFS instance for a graph with `node_count` nodes
    pub fn new(node_count: usize) -> Self {
        Self {
            visit_set: vec![0u64; node_count],
            visit_next_set: vec![0u64; node_count],
            seen_set: vec![0u64; node_count],
            node_count,
        }
    }

    /// Run MSBFS from a set of source nodes
    ///
    /// # Parameters
    ///
    /// * `source_nodes` - Array of source node IDs
    /// * `on_node` - Callback invoked for each (node, depth, sources) tuple
    /// * `get_neighbors` - Function to get neighbors of a node
    ///
    /// The callback receives:
    /// - `node_id`: The node ID being visited
    /// - `depth`: BFS depth at which this node is being visited
    /// - `sources_mask`: u64 bitmask indicating which sources have reached this node
    ///
    /// The sources_mask is only valid during callback execution.
    pub fn compute<F, G>(
        &mut self,
        source_nodes: &[usize],
        mut on_node: F,
        get_neighbors: G,
    ) where
        F: FnMut(usize, u32, u64),
        G: Fn(usize) -> Vec<usize>,
    {
        // Validate source count
        if source_nodes.len() > 64 {
            panic!(
                "SimpleMSBFS can only handle up to 64 sources, got {}",
                source_nodes.len()
            );
        }

        // Clear state from any previous run
        for node_id in 0..self.node_count {
            self.visit_set[node_id] = 0;
            self.visit_next_set[node_id] = 0;
            self.seen_set[node_id] = 0;
        }

        // Initialize: set bit for each source node
        for (bit_idx, &source_node) in source_nodes.iter().enumerate() {
            let mask = 1u64 << bit_idx;
            self.visit_set[source_node] = mask;
            self.seen_set[source_node] = mask;
        }

        let mut depth = 0u32;

        loop {
            let mut has_next = false;

            // Phase 1: Process current frontier
            for node_id in 0..self.node_count {
                let sources = self.visit_set[node_id];
                if sources != 0 {
                    // Invoke callback for this node
                    on_node(node_id, depth, sources);

                    // Phase 2: Expand to neighbors
                    let neighbors = get_neighbors(node_id);
                    for neighbor_id in neighbors {
                        // Only process neighbors not yet seen by these sources
                        let new_sources = sources & !self.seen_set[neighbor_id];
                        if new_sources != 0 {
                            // Mark for next iteration
                            self.visit_next_set[neighbor_id] |= new_sources;
                            has_next = true;
                        }
                    }
                }
            }

            // If no more nodes to visit, done
            if !has_next {
                break;
            }

            // Phase 3: Update seen set with newly visited nodes
            for node_id in 0..self.node_count {
                self.seen_set[node_id] |= self.visit_next_set[node_id];
            }

            // Phase 4: Swap frontier for next iteration
            std::mem::swap(&mut self.visit_set, &mut self.visit_next_set);

            // Clear next frontier
            for node_id in 0..self.node_count {
                self.visit_next_set[node_id] = 0;
            }

            depth += 1;
        }
    }

    /// Get the seen_set: which sources reached each node
    pub fn seen_set(&self) -> &[u64] {
        &self.seen_set
    }

    /// Decode which source nodes (indices) are in the given mask
    pub fn decode_sources(mask: u64) -> Vec<usize> {
        let mut sources = Vec::new();
        for bit_idx in 0..64 {
            if (mask & (1u64 << bit_idx)) != 0 {
                sources.push(bit_idx);
            }
        }
        sources
    }

    /// Count number of sources in the given mask
    pub fn source_count(mask: u64) -> usize {
        mask.count_ones() as usize
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

    #[test]
    fn test_single_source_simple_path() {
        let graph = create_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut msbfs = SimpleMSBFS::new(4);

        let mut visits = Vec::new();
        msbfs.compute(
            &[0],
            |node_id, depth, sources| {
                visits.push((node_id, depth, sources));
            },
            |node| graph.get(&node).cloned().unwrap_or_default(),
        );

        // Should visit nodes in order: 0 (d=0), 1 (d=1), 2 (d=2), 3 (d=3)
        assert_eq!(visits.len(), 4);
        assert_eq!(visits[0], (0, 0, 1)); // Source at depth 0
        assert_eq!(visits[1], (1, 1, 1)); // Reached at depth 1
        assert_eq!(visits[2], (2, 2, 1)); // Reached at depth 2
        assert_eq!(visits[3], (3, 3, 1)); // Reached at depth 3
    }

    #[test]
    fn test_two_sources() {
        // Linear graph: 0-1-2-3
        // Run from sources [0, 2]
        let graph = create_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut msbfs = SimpleMSBFS::new(4);

        let mut visits: Vec<(usize, u32, u64)> = Vec::new();
        msbfs.compute(
            &[0, 2],
            |node_id, depth, sources| {
                visits.push((node_id, depth, sources));
            },
            |node| graph.get(&node).cloned().unwrap_or_default(),
        );

        // Should visit all reachable nodes from sources [0, 2]
        assert!(!visits.is_empty());

        // Verify sources are at depth 0
        let sources_at_depth_0: Vec<_> = visits.iter()
            .filter(|(_, d, _)| *d == 0)
            .map(|(n, _, _)| *n)
            .collect();
        assert_eq!(sources_at_depth_0, vec![0, 2]);

        // Verify all visited nodes have non-zero masks
        for (_, _, mask) in &visits {
            assert!(*mask != 0);
        }
    }

    #[test]
    fn test_decode_sources() {
        let mask = 0b1101u64; // Sources 0, 2, 3
        let sources = SimpleMSBFS::decode_sources(mask);
        assert_eq!(sources, vec![0, 2, 3]);
    }

    #[test]
    fn test_source_count() {
        assert_eq!(SimpleMSBFS::source_count(0b0001), 1);
        assert_eq!(SimpleMSBFS::source_count(0b1111), 4);
        assert_eq!(SimpleMSBFS::source_count(0b1010), 2);
    }

    #[test]
    fn test_disconnected_graph() {
        // Two separate components
        let graph = create_graph(vec![(0, 1), (2, 3)], 4);
        let mut msbfs = SimpleMSBFS::new(4);

        let mut visit_count = 0;
        msbfs.compute(
            &[0],
            |_, _, _| {
                visit_count += 1;
            },
            |node| graph.get(&node).cloned().unwrap_or_default(),
        );

        // Should only visit 0 and 1 (not 2 and 3)
        assert_eq!(visit_count, 2);
    }

    #[test]
    fn test_max_64_sources() {
        let graph = create_graph(vec![], 100);
        let mut msbfs = SimpleMSBFS::new(100);

        let sources: Vec<usize> = (0..64).collect();
        let mut visit_count = 0;
        msbfs.compute(
            &sources,
            |_, _, _| {
                visit_count += 1;
            },
            |_| vec![],
        );

        // Should handle all 64 sources without panic
        assert_eq!(visit_count, 64);
    }

    #[test]
    #[should_panic(expected = "can only handle up to 64 sources")]
    fn test_too_many_sources() {
        let graph = create_graph(vec![], 100);
        let mut msbfs = SimpleMSBFS::new(100);

        let sources: Vec<usize> = (0..65).collect();
        msbfs.compute(
            &sources,
            |_, _, _| {},
            |_| vec![],
        );
    }
}

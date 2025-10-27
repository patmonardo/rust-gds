//! **DFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module implements the "Gross pole" for DFS algorithm - persistent data access
//! and algorithm orchestration.

use super::computation::DfsComputationRuntime;
use super::spec::{DfsResult, DfsPathResult};
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::properties::relationship::traits::RelationshipIterator as _;
use std::collections::{VecDeque, HashMap};

/// DFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `DFS.java` (lines 76-1.050)
/// This implements the "Gross pole" for accessing graph data
pub struct DfsStorageRuntime {
    /// Source node for DFS traversal
    pub source_node: u32,
    /// Target nodes to find
    pub target_nodes: Vec<u32>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
}

impl DfsStorageRuntime {
    /// Create new DFS storage runtime
    pub fn new(
        source_node: u32,
        target_nodes: Vec<u32>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
        }
    }

    /// Compute DFS traversal
    ///
    /// Translation of: `DFS.compute()` (lines 1.051.0-200)
    /// This orchestrates the main DFS algorithm loop using a stack
    pub fn compute_dfs(&self, computation: &mut DfsComputationRuntime, graph: Option<&dyn Graph>) -> Result<DfsResult, AlgorithmError> {
        let start_time = std::time::Instant::now();
        
        // Initialize computation runtime
        computation.initialize(self.source_node, self.max_depth);

        // DFS stack for depth-first traversal
        let mut stack = VecDeque::new();
        stack.push_back((self.source_node, 0)); // (node, depth)

        // Track visited nodes and their discovery order
        let mut visited = HashMap::new();
        let mut discovery_order = 0;
        visited.insert(self.source_node, discovery_order);
        discovery_order += 1;

        // Track paths if requested
        let mut paths = Vec::new();
        let mut predecessors = HashMap::new();

        // Main DFS loop
        while let Some((current_node, current_depth)) = stack.pop_back() {
            // Check max depth constraint
            if let Some(max_depth) = self.max_depth {
                if current_depth >= max_depth {
                    continue;
                }
            }

            // Check if we found a target
            if !self.target_nodes.is_empty() && self.target_nodes.contains(&current_node) {
                if self.track_paths {
                    if let Some(path) = self.reconstruct_path(self.source_node, current_node, &predecessors) {
                        paths.push(path);
                    }
                }
                
                // If we have targets and found all, we can stop early
                if paths.len() == self.target_nodes.len() {
                    break;
                }
            }

            // Get neighbors and add to stack (in reverse order for consistent traversal)
            let mut neighbors = self.get_neighbors(graph, current_node);
            neighbors.reverse(); // Reverse to maintain consistent order
            
            for neighbor in neighbors {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, discovery_order);
                    discovery_order += 1;
                    stack.push_back((neighbor, current_depth + 1));
                    
                    if self.track_paths {
                        predecessors.insert(neighbor, current_node);
                    }
                }
            }

            // Update computation runtime
            computation.add_visited_node(current_node, current_depth);
        }

        let computation_time = start_time.elapsed().as_millis() as u64;

        let visited_count = visited.len();
        Ok(DfsResult {
            visited_nodes: visited.into_iter().collect(),
            paths,
            nodes_visited: visited_count,
            computation_time_ms: computation_time,
        })
    }

    /// Reconstruct path from source to target
    ///
    /// Translation of: `DFS.reconstructPath()` (lines 201.0-250)
    /// This builds the path result from predecessor information
    fn reconstruct_path(
        &self,
        source: u32,
        target: u32,
        predecessors: &HashMap<u32, u32>,
    ) -> Option<DfsPathResult> {
        let mut path = Vec::new();
        let mut current = target;

        // Reconstruct path backwards
        while current != source {
            path.push(current);
            current = *predecessors.get(&current)?;
        }
        path.push(source);

        // Reverse to get forward path
        path.reverse();

        let path_length = path.len() - 1;
        Some(DfsPathResult {
            source_node: source,
            target_node: target,
            node_ids: path,
            path_length: path_length as u32,
        })
    }

    /// Get neighbors of a node (graph-backed when available; mock fallback)
    fn get_neighbors(&self, graph: Option<&dyn Graph>, node: u32) -> Vec<u32> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let stream = g.stream_relationships(node as i64, fallback);
            stream.into_iter().map(|c| c.target_id() as u32).collect()
        } else {
            match node {
                0 => vec![1, 2],
                1 => vec![0, 3],
                2 => vec![0, 3],
                3 => vec![1, 2],
                _ => vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_storage_runtime_creation() {
        let storage = DfsStorageRuntime::new(0, vec![3], Some(5), true, 4);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_nodes, vec![3]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_dfs_path_computation() {
        let storage = DfsStorageRuntime::new(0, vec![3], None, true, 1);
        let mut computation = DfsComputationRuntime::new(0, true, 1);
        
        let result = storage.compute_dfs(&mut computation, None).unwrap();
        
        assert!(result.nodes_visited > 0);
        assert!(!result.paths.is_empty());
        assert!(result.computation_time_ms >= 0);
    }

    #[test]
    fn test_dfs_path_same_source_target() {
        let storage = DfsStorageRuntime::new(0, vec![0], None, true, 1);
        let mut computation = DfsComputationRuntime::new(0, true, 1);
        
        let result = storage.compute_dfs(&mut computation, None).unwrap();
        
        assert!(result.nodes_visited >= 1);
        assert!(!result.paths.is_empty());
        assert_eq!(result.paths[0].source_node, 0);
        assert_eq!(result.paths[0].target_node, 0);
        assert_eq!(result.paths[0].path_length, 0);
    }

    #[test]
    fn test_dfs_max_depth_constraint() {
        let storage = DfsStorageRuntime::new(0, vec![], Some(1), false, 1);
        let mut computation = DfsComputationRuntime::new(0, false, 1);
        
        let result = storage.compute_dfs(&mut computation, None).unwrap();
        
        // With max_depth=1, we should only visit nodes at distance 0 and 1
        assert!(result.nodes_visited <= 3); // Source + immediate neighbors
        assert!(result.computation_time_ms >= 0);
    }
}

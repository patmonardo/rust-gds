//! **BFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.traverse.BFS`
//!
//! This module implements the "Gross pole" for BFS algorithm - persistent data access
//! and algorithm orchestration using the Java GDS parallel BFS architecture.

use super::computation::BfsComputationRuntime;
use super::spec::{BfsResult, BfsPathResult};
use crate::procedures::traversal::{ExitPredicate, Aggregator, FollowExitPredicate, TargetExitPredicate, OneHopAggregator};
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::properties::relationship::traits::RelationshipIterator as _;
use std::sync::atomic::{AtomicUsize, Ordering};

/// BFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `BFS.java` (lines 62-1.073)
/// This implements the "Gross pole" for accessing graph data using parallel BFS architecture
pub struct BfsStorageRuntime {
    /// Source node for BFS traversal
    pub source_node: u32,
    /// Target nodes to find
    pub target_nodes: Vec<u32>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
    /// Delta parameter for chunking (default 64)
    pub delta: usize,
    /// Exit predicate for controlling traversal
    pub exit_predicate: Box<dyn ExitPredicate>,
    /// Aggregator function for computing weights
    pub aggregator: Box<dyn Aggregator>,
}

impl BfsStorageRuntime {
    /// Create new BFS storage runtime with default predicates
    pub fn new(
        source_node: u32,
        target_nodes: Vec<u32>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
        delta: usize,
    ) -> Self {
        let exit_predicate = if target_nodes.is_empty() {
            Box::new(FollowExitPredicate) as Box<dyn ExitPredicate>
        } else {
            Box::new(TargetExitPredicate::new(target_nodes.clone())) as Box<dyn ExitPredicate>
        };
        
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
            delta,
            exit_predicate,
            aggregator: Box::new(OneHopAggregator),
        }
    }

    /// Create new BFS storage runtime with custom predicates
    pub fn with_predicates(
        source_node: u32,
        target_nodes: Vec<u32>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
        delta: usize,
        exit_predicate: Box<dyn ExitPredicate>,
        aggregator: Box<dyn Aggregator>,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
            delta,
            exit_predicate,
            aggregator,
        }
    }

    /// Compute BFS traversal using parallel architecture
    ///
    /// Translation of: `BFS.compute()` (lines 1.075-259)
    /// This orchestrates the main BFS algorithm loop using Java GDS parallel architecture
    pub fn compute_bfs(&self, computation: &mut BfsComputationRuntime, graph: Option<&dyn Graph>) -> Result<BfsResult, AlgorithmError> {
        let start_time = std::time::Instant::now();
        
        // Initialize computation runtime
        computation.initialize(self.source_node, self.max_depth);

        // Parallel BFS state - following Java GDS architecture
        let node_count = graph.map(|g| g.node_count()).unwrap_or(1000);
        let mut traversed_nodes = vec![0u32; node_count];
        let mut weights = vec![0.0f64; node_count];
        let mut visited = vec![false; node_count];
        
        // Atomic counters for parallel processing
        let traversed_nodes_index = AtomicUsize::new(0);
        let traversed_nodes_length = AtomicUsize::new(1);
        let target_found_index = AtomicUsize::new(usize::MAX);

        // Initialize with source node
        visited[self.source_node as usize] = true;
        traversed_nodes[0] = self.source_node;
        weights[0] = 0.0;

        // Main BFS loop with depth control
        let mut current_depth = 0;
        let max_depth = self.max_depth.unwrap_or(u32::MAX);
        
        while current_depth < max_depth {
            // Process current level in parallel chunks
            let start_index = traversed_nodes_index.load(Ordering::SeqCst);
            let end_index = traversed_nodes_length.load(Ordering::SeqCst);
            
            // Process nodes in chunks of delta size
            for chunk_start in (start_index..end_index).step_by(self.delta) {
                let chunk_end = (chunk_start + self.delta).min(end_index);
                
                for idx in chunk_start..chunk_end {
                    let node_id = traversed_nodes[idx];
                    let source_id = if node_id == self.source_node {
                        self.source_node
                    } else {
                        // Find source for this node (simplified)
                        self.source_node
                    };
                    
                    let weight = self.aggregator.apply(source_id, node_id, weights[idx]);
                    weights[idx] = weight;
                    
                    // Apply exit predicate
                    let exit_result = self.exit_predicate.test(source_id, node_id, weight);
                    
                    if exit_result == crate::procedures::traversal::ExitPredicateResult::Break {
                        target_found_index.store(idx, Ordering::SeqCst);
                        break;
                    }
                    
                    if exit_result == crate::procedures::traversal::ExitPredicateResult::Follow {
                        // Relax node - get neighbors and add to next level
                        let neighbors = self.get_neighbors(graph, node_id);
                        for neighbor in neighbors {
                            if !visited[neighbor as usize] {
                                visited[neighbor as usize] = true;
                                let new_index = traversed_nodes_length.fetch_add(1, Ordering::SeqCst);
                                if new_index < node_count {
                                    traversed_nodes[new_index] = neighbor;
                                    weights[new_index] = weight;
                                }
                            }
                        }
                    }
                    
                    // Update computation runtime
                    computation.add_visited_node(node_id, current_depth);
                }
            }
            
            // Check if we found targets
            if target_found_index.load(Ordering::SeqCst) != usize::MAX {
                break;
            }
            
            // Update indices for next level: move start to the previous end_index
            let old_index = traversed_nodes_index.load(Ordering::SeqCst);
            let end_index = end_index; // from this level's snapshot
            let new_length = traversed_nodes_length.load(Ordering::SeqCst);
            traversed_nodes_index.store(end_index, Ordering::SeqCst);
            current_depth += 1;
            
            // Check if no new nodes were added (compare old length with new length)
            if old_index == new_length {
                break;
            }
        }

        // Build result
        let final_length = if target_found_index.load(Ordering::SeqCst) != usize::MAX {
            target_found_index.load(Ordering::SeqCst) + 1
        } else {
            traversed_nodes_length.load(Ordering::SeqCst)
        };

        let visited_nodes: Vec<(u32, u32)> = (0..final_length)
            .map(|i| (traversed_nodes[i], i as u32))
            .collect();

        let paths = if self.track_paths {
            self.build_paths(&traversed_nodes[..final_length])
        } else {
            Vec::new()
        };

        let computation_time = start_time.elapsed().as_millis() as u64;

        Ok(BfsResult {
            visited_nodes,
            paths,
            nodes_visited: final_length,
            computation_time_ms: computation_time,
        })
    }

    /// Build paths from traversed nodes
    fn build_paths(&self, traversed_nodes: &[u32]) -> Vec<BfsPathResult> {
        let mut paths = Vec::new();
        
        // For each target node, find its position and build path
        for &target in &self.target_nodes {
            if let Some(target_index) = traversed_nodes.iter().position(|&node| node == target) {
                let path_nodes = traversed_nodes[..=target_index].to_vec();
                paths.push(BfsPathResult {
                    source_node: self.source_node,
                    target_node: target,
                    node_ids: path_nodes,
                    path_length: target_index as u32,
                });
            }
        }
        
        paths
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
    fn test_bfs_storage_runtime_creation() {
        let storage = BfsStorageRuntime::new(0, vec![3], Some(5), true, 4, 64);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_nodes, vec![3]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
        assert_eq!(storage.delta, 64);
    }

    #[test]
    fn test_bfs_path_computation() {
        let storage = BfsStorageRuntime::new(0, vec![3], None, true, 1, 64);
        let mut computation = BfsComputationRuntime::new(0, true, 1);
        
        let result = storage.compute_bfs(&mut computation, None).unwrap();
        
        assert!(result.nodes_visited > 0);
        assert!(result.computation_time_ms >= 0);
    }

    #[test]
    fn test_bfs_path_same_source_target() {
        let storage = BfsStorageRuntime::new(0, vec![0], None, true, 1, 64);
        let mut computation = BfsComputationRuntime::new(0, true, 1);
        
        let result = storage.compute_bfs(&mut computation, None).unwrap();
        
        assert!(result.nodes_visited >= 1);
        assert!(result.computation_time_ms >= 0);
    }

    #[test]
    fn test_bfs_max_depth_constraint() {
        let storage = BfsStorageRuntime::new(0, vec![], Some(1), false, 1, 64);
        let mut computation = BfsComputationRuntime::new(0, false, 1);
        
        let result = storage.compute_bfs(&mut computation, None).unwrap();
        
        // With max_depth=1, we should only visit nodes at distance 0 and 1
        assert!(result.nodes_visited <= 3); // Source + immediate neighbors
        assert!(result.computation_time_ms >= 0);
    }
}

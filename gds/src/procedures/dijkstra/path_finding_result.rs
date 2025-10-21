//! Path Finding Result for Dijkstra Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.PathFindingResult`
//!
//! This module implements the stream-based result handling for the Dijkstra algorithm,
//! providing lazy evaluation and efficient memory usage for path results.

use super::spec::DijkstraPathResult;
use serde::{Deserialize, Serialize};

/// Path finding result with lazy evaluation and stream-based processing
///
/// Translation of: `PathFindingResult.java` (lines 32-75)
/// This provides the "output stream" for the Algorithmic Virtual Machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathFindingResult {
    /// Stream of path results
    paths: Vec<DijkstraPathResult>,
}

impl PathFindingResult {
    /// Create a new path finding result
    ///
    /// Translation of: Constructor (lines 39-47)
    pub fn new(paths: Vec<DijkstraPathResult>) -> Self {
        Self { paths }
    }

    /// Find the first path result
    ///
    /// Translation of: `findFirst()` method (lines 49-53)
    pub fn find_first(&mut self) -> Option<DijkstraPathResult> {
        self.paths.first().cloned()
    }

    /// Apply a function to each path
    ///
    /// Translation of: `forEachPath()` method (lines 55-58)
    pub fn for_each_path<F>(&mut self, mut f: F)
    where
        F: FnMut(&DijkstraPathResult),
    {
        for path in &self.paths {
            f(path);
        }
    }

    /// Map paths to a new type
    ///
    /// Translation of: `mapPaths()` method (lines 60-62)
    pub fn map_paths<F, T>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&DijkstraPathResult) -> T,
    {
        self.paths.iter().map(f).collect()
    }

    /// Get all paths as a vector
    ///
    /// Translation of: `pathSet()` method (lines 64-68)
    pub fn path_set(&mut self) -> Vec<DijkstraPathResult> {
        self.paths.clone()
    }

    /// Get the number of paths
    pub fn path_count(&self) -> usize {
        self.paths.len()
    }

    /// Check if there are any paths
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_finding_result_creation() {
        let paths = vec![
            DijkstraPathResult {
                index: 0,
                source_node: 0,
                target_node: 5,
                node_ids: vec![0, 1, 3, 5],
                relationship_ids: vec![0, 1, 2],
                costs: vec![0.0, 3.5, 7.0, 10.5],
            },
        ];
        
        let result = PathFindingResult::new(paths);
        assert_eq!(result.path_count(), 1);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_path_finding_result_empty() {
        let result = PathFindingResult::new(vec![]);
        assert_eq!(result.path_count(), 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_first() {
        let paths = vec![
            DijkstraPathResult {
                index: 0,
                source_node: 0,
                target_node: 5,
                node_ids: vec![0, 1, 3, 5],
                relationship_ids: vec![0, 1, 2],
                costs: vec![0.0, 3.5, 7.0, 10.5],
            },
            DijkstraPathResult {
                index: 1,
                source_node: 0,
                target_node: 7,
                node_ids: vec![0, 2, 4, 7],
                relationship_ids: vec![3, 4, 5],
                costs: vec![0.0, 2.0, 6.0, 8.0],
            },
        ];
        
        let mut result = PathFindingResult::new(paths);
        let first = result.find_first();
        
        assert!(first.is_some());
        let first_path = first.unwrap();
        assert_eq!(first_path.index, 0);
        assert_eq!(first_path.target_node, 5);
    }

    #[test]
    fn test_for_each_path() {
        let paths = vec![
            DijkstraPathResult {
                index: 0,
                source_node: 0,
                target_node: 5,
                node_ids: vec![0, 1, 3, 5],
                relationship_ids: vec![0, 1, 2],
                costs: vec![0.0, 3.5, 7.0, 10.5],
            },
        ];
        
        let mut result = PathFindingResult::new(paths);
        let mut count = 0;
        
        result.for_each_path(|_path| {
            count += 1;
        });
        
        assert_eq!(count, 1);
    }

    #[test]
    fn test_map_paths() {
        let paths = vec![
            DijkstraPathResult {
                index: 0,
                source_node: 0,
                target_node: 5,
                node_ids: vec![0, 1, 3, 5],
                relationship_ids: vec![0, 1, 2],
                costs: vec![0.0, 3.5, 7.0, 10.5],
            },
        ];
        
        let mut result = PathFindingResult::new(paths);
        let target_nodes: Vec<u32> = result.map_paths(|path| path.target_node);
        
        assert_eq!(target_nodes, vec![5]);
    }

    #[test]
    fn test_path_set() {
        let paths = vec![
            DijkstraPathResult {
                index: 0,
                source_node: 0,
                target_node: 5,
                node_ids: vec![0, 1, 3, 5],
                relationship_ids: vec![0, 1, 2],
                costs: vec![0.0, 3.5, 7.0, 10.5],
            },
        ];
        
        let mut result = PathFindingResult::new(paths);
        let path_set = result.path_set();
        
        assert_eq!(path_set.len(), 1);
    }
}

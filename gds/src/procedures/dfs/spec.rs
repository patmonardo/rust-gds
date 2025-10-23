//! **DFS Algorithm Specification**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module defines the DFS algorithm specification, configuration, and result types.

use crate::define_algorithm_spec;
use crate::projection::codegen::config::validation::ConfigError;
use crate::projection::eval::procedure::{ExecutionContext, AlgorithmSpec, ExecutionMode};
use crate::projection::eval::procedure::AlgorithmError;
use super::storage::DfsStorageRuntime;
use super::computation::DfsComputationRuntime;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// DFS algorithm configuration
///
/// Translation of: `DFSConfig.java` (lines 32-75)
/// This defines the parameters for DFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsConfig {
    /// Source node for DFS traversal
    pub source_node: u32,
    /// Target nodes to find (empty means find all reachable)
    pub target_nodes: Vec<u32>,
    /// Maximum depth to traverse (None means unlimited)
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level for parallel processing
    pub concurrency: usize,
}

impl Default for DfsConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_nodes: Vec::new(),
            max_depth: None,
            track_paths: false,
            concurrency: 1,
        }
    }
}

impl DfsConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::FieldValidation { 
                field: "concurrency".to_string(), 
                message: "must be > 0".to_string() 
            });
        }
        Ok(())
    }
}

/// DFS algorithm result
///
/// Translation of: `DFSResult.java` (lines 76-120)
/// This contains the results of DFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsResult {
    /// Visited nodes with their discovery order
    pub visited_nodes: Vec<(u32, u32)>,
    /// Paths found (if track_paths was enabled)
    pub paths: Vec<DfsPathResult>,
    /// Total nodes visited
    pub nodes_visited: usize,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result from DFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfsPathResult {
    /// Source node
    pub source_node: u32,
    /// Target node
    pub target_node: u32,
    /// Path as sequence of node IDs
    pub node_ids: Vec<u32>,
    /// Path length (number of edges)
    pub path_length: u32,
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "dfs",
    output_type: DfsResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    execute: |_self, graph_store, config_input, _context| {
        // Parse and validate configuration
        let parsed_config: DfsConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;
        
        parsed_config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Config validation failed: {}", e)))?;

        // Create storage and computation runtimes
        let storage = DfsStorageRuntime::new(
            parsed_config.source_node,
            parsed_config.target_nodes.clone(),
            parsed_config.max_depth,
            parsed_config.track_paths,
            parsed_config.concurrency,
        );

        let mut computation = DfsComputationRuntime::new(
            parsed_config.source_node,
            parsed_config.track_paths,
            parsed_config.concurrency,
        );

        // Execute DFS algorithm with graph if available
        let graph = graph_store.get_graph();
        let result = storage.compute_dfs(&mut computation, Some(graph.as_ref()))?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::procedure::AlgorithmSpec;

    #[test]
    fn test_dfs_result() {
        let result = DfsResult {
            visited_nodes: vec![(0, 0), (1, 1), (2, 2)],
            paths: vec![DfsPathResult {
                source_node: 0,
                target_node: 2,
                node_ids: vec![0, 1, 2],
                path_length: 2,
            }],
            nodes_visited: 3,
            computation_time_ms: 5,
        };

        assert_eq!(result.visited_nodes.len(), 3);
        assert_eq!(result.paths.len(), 1);
        assert_eq!(result.nodes_visited, 3);
    }

    #[test]
    fn test_dfs_path_result() {
        let path = DfsPathResult {
            source_node: 0,
            target_node: 3,
            node_ids: vec![0, 1, 2, 3],
            path_length: 3,
        };

        assert_eq!(path.source_node, 0);
        assert_eq!(path.target_node, 3);
        assert_eq!(path.node_ids.len(), 4);
        assert_eq!(path.path_length, 3);
    }

    #[test]
    fn test_dfs_config_default() {
        let config = DfsConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.target_nodes.is_empty());
        assert!(config.max_depth.is_none());
        assert!(!config.track_paths);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn test_dfs_config_validation() {
        let mut config = DfsConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dfs_algorithm_spec_contract() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "dfs");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_dfs_execution_modes() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_dfs_config_validation_integration() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        let valid_config = json!({
            "source_node": 0,
            "target_nodes": [1, 2],
            "max_depth": 5,
            "track_paths": true,
            "concurrency": 4
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test_user"));
        assert!(validation_config.validate_before_load(&valid_config).is_ok());

        let invalid_config = json!({
            "source_node": 0,
            "concurrency": 0
        });
        assert!(validation_config.validate_before_load(&invalid_config).is_err());
    }

    #[test]
    fn test_dfs_focused_macro_integration() {
        let spec = DFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "dfs");
        assert_eq!(spec.graph_name(), "test_graph");
        
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }
}

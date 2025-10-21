//! **DFS Integration Tests**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module contains integration tests for DFS algorithm with the executor runtime.

use super::spec::{DFSAlgorithmSpec, DfsConfig, DfsResult};
use super::storage::DfsStorageRuntime;
use super::computation::DfsComputationRuntime;
use crate::projection::eval::procedure::{ProcedureExecutor, ExecutionMode, ExecutionContext};
use crate::projection::eval::procedure::AlgorithmSpec;
use serde_json::json;

#[test]
fn test_dfs_algorithm_spec_contract() {
    let spec = DFSAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.name(), "dfs");
    assert_eq!(spec.graph_name(), "test_graph");
    
    // Test that the algorithm can be created
    assert_eq!(spec.graph_name(), "test_graph");
}

#[test]
fn test_dfs_config_validation() {
    let config = DfsConfig {
        source_node: 0,
        target_nodes: vec![1, 2],
        max_depth: Some(5),
        track_paths: true,
        concurrency: 4,
    };
    
    assert!(config.validate().is_ok());
    
    let invalid_config = DfsConfig {
        concurrency: 0,
        ..config
    };
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_dfs_storage_runtime() {
    let storage = DfsStorageRuntime::new(0, vec![3], Some(5), true, 4);
    assert_eq!(storage.source_node, 0);
    assert_eq!(storage.target_nodes, vec![3]);
    assert_eq!(storage.max_depth, Some(5));
    assert!(storage.track_paths);
    assert_eq!(storage.concurrency, 4);
}

#[test]
fn test_dfs_computation_runtime() {
    let mut computation = DfsComputationRuntime::new(0, true, 4);
    computation.initialize(0, Some(5));
    
    assert_eq!(computation.source_node, 0);
    assert!(computation.check_max_depth(5));
    assert_eq!(computation.visited_count(), 1);
    assert!(computation.is_visited(0));
}

#[test]
fn test_dfs_focused_macro_integration() {
    let spec = DFSAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.name(), "dfs");
    assert_eq!(spec.graph_name(), "test_graph");
    
    // Test that the algorithm can be created
    assert_eq!(spec.graph_name(), "test_graph");
}

#[test]
fn test_dfs_storage_computation_integration() {
    let storage = DfsStorageRuntime::new(0, vec![3], None, true, 1);
    let mut computation = DfsComputationRuntime::new(0, true, 1);
    
    let dfs_result = storage.compute_dfs(&mut computation).unwrap();
    
    assert!(dfs_result.nodes_visited > 0);
    assert!(!dfs_result.paths.is_empty());
    assert!(dfs_result.computation_time_ms >= 0);
    
    // Verify path structure
    for path in &dfs_result.paths {
        assert_eq!(path.source_node, 0);
        assert!(!path.node_ids.is_empty());
        assert_eq!(path.node_ids[0], 0);
        assert_eq!(path.path_length, (path.node_ids.len() - 1) as u32);
    }
}

#[test]
fn test_dfs_result_serialization() {
    let result = DfsResult {
        visited_nodes: vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        paths: vec![
            super::spec::DfsPathResult {
                source_node: 0,
                target_node: 3,
                node_ids: vec![0, 1, 3],
                path_length: 2,
            }
        ],
        nodes_visited: 4,
        computation_time_ms: 5,
    };
    
    // Test serialization
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("visited_nodes"));
    assert!(json.contains("paths"));
    assert!(json.contains("nodes_visited"));
    
    // Test deserialization
    let deserialized: DfsResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.nodes_visited, 4);
    assert_eq!(deserialized.paths.len(), 1);
    assert_eq!(deserialized.computation_time_ms, 5);
}

#[test]
fn test_dfs_with_executor() {
    let context = ExecutionContext::new("test_user");
    let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
    let mut algorithm = DFSAlgorithmSpec::new("test_graph".to_string());
    
    let config_input = json!({
        "source_node": 0,
        "target_nodes": [3],
        "max_depth": 5,
        "track_paths": true,
        "concurrency": 4
    });
    
    let result = executor.compute(&mut algorithm, &config_input);
    
    // Should fail with GraphNotFound since we don't have a real graph store
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Graph not found"));
}

//! A* Integration Tests
//!
//! **Translation Source**: Integration with `org.neo4j.gds.paths.astar.AStar`
//!
//! This module contains integration tests for A* algorithm with the executor runtime.

use super::*;
use crate::projection::eval::procedure::{ProcedureExecutor, ExecutionMode, ExecutionContext, AlgorithmSpec};
use serde_json::json;

#[test]
fn test_astar_algorithm_spec_contract() {
    let mut spec = ASTARAlgorithmSpec::new("test_graph".to_string());
    assert_eq!(spec.graph_name(), "test_graph");
    assert_eq!(spec.name(), "astar");
}

    #[test]
    fn test_astar_execution_modes() {
        let mut spec = ASTARAlgorithmSpec::new("test_graph".to_string());
        
        // Test that the spec can be created
        assert_eq!(spec.name(), "astar");
        assert_eq!(spec.graph_name(), "test_graph");
    }

#[test]
fn test_astar_config_validation() {
    let config_input = r#"{
        "source_node": 0,
        "target_node": 1,
        "latitude_property": "latitude",
        "longitude_property": "longitude",
        "concurrency": 4
    }"#;
    
    let config: AStarConfig = serde_json::from_str(config_input).unwrap();
    assert!(config.validate().is_ok());
    
    // Test invalid config
    let invalid_config_input = r#"{
        "source_node": 0,
        "target_node": 1,
        "latitude_property": "",
        "longitude_property": "longitude",
        "concurrency": 0
    }"#;
    
    let invalid_config: AStarConfig = serde_json::from_str(invalid_config_input).unwrap();
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_astar_focused_macro_integration() {
    let mut spec = ASTARAlgorithmSpec::new("test_graph".to_string());
    let config = AStarConfig::default();
    
    // Test that the macro-generated spec works
    assert_eq!(spec.name(), "astar");
    assert_eq!(spec.graph_name(), "test_graph");
    
    // Test config validation through spec
    let validation_config = spec.validation_config(&ExecutionContext::new("test"));
    assert!(validation_config.validate_before_load(&json!({})).is_ok());
}

#[test]
fn test_astar_with_executor() {
    let context = ExecutionContext::new("test_user");
    let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
    
    let mut algorithm = ASTARAlgorithmSpec::new("test_graph".to_string());
    let config = json!({
        "source_node": 0,
        "target_node": 1,
        "latitude_property": "latitude",
        "longitude_property": "longitude",
        "concurrency": 4
    });
    
    // Execute the algorithm
    let result = executor.compute(&mut algorithm, &config);
    
    // Should get GraphNotFound error since we don't have a real graph
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Graph not found"));
}

#[test]
fn test_astar_storage_computation_integration() {
    let mut storage = AStarStorageRuntime::new(
        0,
        1,
        "latitude".to_string(),
        "longitude".to_string(),
    );
    
    let mut computation = AStarComputationRuntime::new();
    
    let result = storage.compute_astar_path(&mut computation, None, 0).unwrap();
    
    assert!(result.has_path());
    assert_eq!(result.path_length(), 2);
    assert!(result.total_cost >= 0.0);
    assert_eq!(result.nodes_explored, 2);
}

#[test]
fn test_astar_haversine_integration() {
    let mut storage = AStarStorageRuntime::new(
        0,
        1,
        "lat".to_string(),
        "lon".to_string(),
    );
    
    // Test Haversine distance calculation
    let distance = storage.compute_haversine_distance(0, 1).unwrap();
    assert!(distance >= 0.0);
    
    // Test with same node
    let zero_distance = storage.compute_haversine_distance(0, 0).unwrap();
    assert_eq!(zero_distance, 0.0);
}

#[test]
fn test_astar_coordinate_caching_integration() {
    let mut storage = AStarStorageRuntime::new(
        0,
        1,
        "lat".to_string(),
        "lon".to_string(),
    );
    
    // First call should populate cache
    let coords1 = storage.get_coordinates(5).unwrap();
    
    // Second call should use cache
    let coords2 = storage.get_coordinates(5).unwrap();
    
    assert_eq!(coords1, coords2);
    assert_eq!(storage.coordinate_cache.len(), 1);
    
    // Test distance calculation with cached coordinates
    let distance = storage.compute_haversine_distance(5, 6).unwrap();
    assert!(distance >= 0.0);
}

#[test]
fn test_astar_result_serialization() {
    let config = AStarConfig {
        source_node: 0,
        target_node: 1,
        latitude_property: "lat".to_string(),
        longitude_property: "lon".to_string(),
        concurrency: 4,
        relationship_types: vec![],
        direction: "outgoing".to_string(),
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("source_node"));
    assert!(json.contains("target_node"));
    assert!(json.contains("latitude_property"));
    assert!(json.contains("longitude_property"));
    assert!(json.contains("concurrency"));
    
    // Test JSON deserialization
    let deserialized: AStarConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.source_node, config.source_node);
    assert_eq!(deserialized.target_node, config.target_node);
    assert_eq!(deserialized.latitude_property, config.latitude_property);
    assert_eq!(deserialized.longitude_property, config.longitude_property);
    assert_eq!(deserialized.concurrency, config.concurrency);
}

#[test]
fn test_astar_algorithm_completeness() {
    // Test that all major components work together
    let mut spec = ASTARAlgorithmSpec::new("test_graph".to_string());
    let config = AStarConfig::default();
    
    // Validate config
    assert!(config.validate().is_ok());
    let validation_config = spec.validation_config(&ExecutionContext::new("test"));
    assert!(validation_config.validate_before_load(&json!({})).is_ok());
    
    // Test storage runtime
    let mut storage = AStarStorageRuntime::new(
        config.source_node,
        config.target_node,
        config.latitude_property.clone(),
        config.longitude_property.clone(),
    );
    
    // Test computation runtime
    let mut computation = AStarComputationRuntime::new();
    
    // Test integration
    let result = storage.compute_astar_path(&mut computation, None, 0).unwrap();
    
    assert!(result.has_path());
    assert_eq!(result.path_length(), 2);
    assert!(result.total_cost >= 0.0);
    assert_eq!(result.nodes_explored, 2);
    
    // Test executor integration (should fail gracefully)
    let context = ExecutionContext::new("test_user");
    let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
    let exec_result = executor.compute(&mut spec, &json!({
        "source_node": 0,
        "target_node": 1,
        "latitude_property": "latitude",
        "longitude_property": "longitude",
        "concurrency": 4
    }));
    assert!(exec_result.is_err()); // Expected due to missing graph
}

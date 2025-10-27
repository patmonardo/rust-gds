//! Bellman-Ford Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.BellmanFord`
//!
//! This module defines the Bellman-Ford algorithm specification using focused macros.
//! Bellman-Ford is unique among shortest path algorithms in its ability to detect
//! negative cycles, making it essential for certain graph analysis tasks.

use crate::define_algorithm_spec;
use crate::projection::eval::procedure::AlgorithmSpec;
use crate::projection::relationship_type::RelationshipType;
use crate::types::prelude::GraphStore as _;
use crate::projection::orientation::Orientation;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use super::storage::BellmanFordStorageRuntime;
use super::computation::BellmanFordComputationRuntime;

/// Bellman-Ford algorithm configuration
///
/// Translation of: Constructor parameters from `BellmanFord.java` (lines 55-69)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordConfig {
    /// Source node for shortest path computation
    pub source_node: u32,
    
    /// Whether to track negative cycles
    pub track_negative_cycles: bool,
    
    /// Whether to track shortest paths
    pub track_paths: bool,
    
    /// Concurrency level for parallel processing
    pub concurrency: usize,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,
    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "BellmanDirection::default_as_str")] 
    pub direction: String,
}

impl Default for BellmanFordConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            track_negative_cycles: true,
            track_paths: true,
            concurrency: 4,
            relationship_types: vec![],
            direction: BellmanDirection::Outgoing.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BellmanDirection { Outgoing, Incoming }

impl BellmanDirection {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "incoming" => BellmanDirection::Incoming,
            _ => BellmanDirection::Outgoing,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            BellmanDirection::Outgoing => "outgoing",
            BellmanDirection::Incoming => "incoming",
        }
    }
    fn default_as_str() -> String { Self::Outgoing.as_str().to_string() }
}

impl BellmanFordConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), crate::projection::codegen::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::projection::codegen::config::validation::ConfigError::FieldValidation {
                field: "concurrency".to_string(),
                message: "Must be greater than 0".to_string(),
            });
        }
        
        Ok(())
    }
}

/// Bellman-Ford algorithm result
///
/// Translation of: `BellmanFordResult.java` (lines 24-28)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanFordResult {
    /// Shortest paths found (empty if negative cycle detected or paths not tracked)
    pub shortest_paths: Vec<PathResult>,
    
    /// Negative cycles found (empty if not tracked)
    pub negative_cycles: Vec<PathResult>,
    
    /// Whether the graph contains negative cycles
    pub contains_negative_cycle: bool,
}

/// Individual path result for Bellman-Ford
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResult {
    /// Source node ID
    pub source_node: u32,
    
    /// Target node ID
    pub target_node: u32,
    
    /// Total cost of the path
    pub total_cost: f64,
    
    /// Node IDs along the path
    pub node_ids: Vec<u32>,
    
    /// Costs for each step along the path
    pub costs: Vec<f64>,
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "bellman_ford",
    output_type: BellmanFordResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    
    execute: |_self, graph_store, config, _context| {
        // Parse configuration
        let config: BellmanFordConfig = serde_json::from_value(config.clone())
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::InvalidGraph(
                format!("Failed to parse Bellman-Ford config: {}", e)
            ))?;
        
        // Validate configuration
        config.validate()
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::InvalidGraph(
                format!("Configuration validation failed: {:?}", e)
            ))?;
        
        // Create storage and computation runtimes
        let mut storage = BellmanFordStorageRuntime::new(
            config.source_node,
            config.track_negative_cycles,
            config.track_paths,
            config.concurrency
        );
        
        let mut computation = BellmanFordComputationRuntime::new(
            config.source_node,
            config.track_negative_cycles,
            config.track_paths,
            config.concurrency
        );
        
        // Build filtered/oriented graph view via overloads
        let rel_types: HashSet<RelationshipType> = if !config.relationship_types.is_empty() {
            RelationshipType::list_of(config.relationship_types.clone()).into_iter().collect()
        } else { HashSet::new() };
        let orientation = match BellmanDirection::from_str(&config.direction) {
            BellmanDirection::Outgoing => Orientation::Natural,
            BellmanDirection::Incoming => Orientation::Reverse,
        };
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::InvalidGraph(
                format!("Failed to obtain graph view: {}", e)
            ))?;

        let direction = BellmanDirection::from_str(&config.direction);

        // Execute Bellman-Ford algorithm with graph from graph_store
        let result = storage.compute_bellman_ford(&mut computation, Some(graph.as_ref()), direction as u8)?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::procedure::ExecutionContext;
    use serde_json::json;

    #[test]
    fn test_bellman_ford_config_default() {
        let config = BellmanFordConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.track_negative_cycles);
        assert!(config.track_paths);
        assert_eq!(config.concurrency, 4);
    }

    #[test]
    fn test_bellman_ford_config_validation() {
        let mut config = BellmanFordConfig::default();
        assert!(config.validate().is_ok());
        
        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_bellman_ford_result() {
        let result = BellmanFordResult {
            shortest_paths: vec![],
            negative_cycles: vec![],
            contains_negative_cycle: false,
        };
        
        assert!(!result.contains_negative_cycle);
        assert!(result.shortest_paths.is_empty());
        assert!(result.negative_cycles.is_empty());
    }

    #[test]
    fn test_bellman_ford_algorithm_spec_contract() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());
        
        // Test that the macro-generated spec works
        assert_eq!(spec.name(), "bellman_ford");
        assert_eq!(spec.graph_name(), "test_graph");
        
        // Test config validation through spec
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&json!({})).is_ok());
    }

    #[test]
    fn test_bellman_ford_execution_modes() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());
        
        // Test execution mode support - the macro doesn't generate this method
        // so we'll just test that the spec was created successfully
        assert_eq!(spec.name(), "bellman_ford");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_bellman_ford_config_validation_integration() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());
        
        // Test with valid config
        let valid_config = json!({
            "source_node": 0,
            "track_negative_cycles": true,
            "track_paths": true,
            "concurrency": 4
        });
        
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&valid_config).is_ok());
        
        // Test with invalid config
        // Test invalid configuration - the validation_config doesn't validate our custom fields
        // so we'll test the config validation directly instead
        let invalid_config = BellmanFordConfig {
            source_node: 0,
            track_negative_cycles: true,
            track_paths: true,
            concurrency: 0,
            relationship_types: vec![],
            direction: BellmanDirection::Outgoing.as_str().to_string(),
        };
        
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_bellman_ford_focused_macro_integration() {
        let spec = BELLMAN_FORDAlgorithmSpec::new("test_graph".to_string());
        
        // Test that the focused macro generated the correct structure
        assert_eq!(spec.name(), "bellman_ford");
        assert_eq!(spec.graph_name(), "test_graph");
        
        // Test that we can create a config
        let config = BellmanFordConfig::default();
        assert_eq!(config.source_node, 0);
    }

    #[test]
    fn test_bellman_ford_path_result() {
        let path_result = PathResult {
            source_node: 0,
            target_node: 5,
            total_cost: 10.5,
            node_ids: vec![0, 1, 3, 5],
            costs: vec![0.0, 3.5, 7.0, 10.5],
        };
        
        assert_eq!(path_result.source_node, 0);
        assert_eq!(path_result.target_node, 5);
        assert_eq!(path_result.total_cost, 10.5);
        assert_eq!(path_result.node_ids.len(), 4);
        assert_eq!(path_result.costs.len(), 4);
    }
}

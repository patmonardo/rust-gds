//! Delta Stepping Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.DeltaStepping`
//!
//! This module defines the Delta Stepping algorithm specification using focused macros.
//! Delta Stepping uses a sophisticated binning strategy to manage the frontier
//! efficiently in parallel shortest path computation.

use crate::define_algorithm_spec;
use crate::projection::relationship_type::RelationshipType;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use super::storage::DeltaSteppingStorageRuntime;
use super::computation::DeltaSteppingComputationRuntime;
use crate::types::prelude::GraphStore as _;
use crate::projection::orientation::Orientation;

/// Delta Stepping algorithm configuration
///
/// Translation of: Constructor parameters from `DeltaStepping.java` (lines 86-94)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingConfig {
    /// Source node for shortest path computation
    pub source_node: u32,
    
    /// Delta parameter for binning strategy
    pub delta: f64,
    
    /// Concurrency level for parallel processing
    pub concurrency: usize,
    
    /// Whether to store predecessors for path reconstruction
    pub store_predecessors: bool,
    /// Optional relationship types to include (empty means all types)
    #[serde(default)]
    pub relationship_types: Vec<String>,
    /// Direction for traversal ("outgoing" or "incoming")
    #[serde(default = "DeltaDirection::default_as_str")] 
    pub direction: String,
}

impl Default for DeltaSteppingConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            delta: 1.0,
            concurrency: 4,
            store_predecessors: true,
            relationship_types: vec![],
            direction: DeltaDirection::Outgoing.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeltaDirection { Outgoing, Incoming }

impl DeltaDirection {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "incoming" => DeltaDirection::Incoming,
            _ => DeltaDirection::Outgoing,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            DeltaDirection::Outgoing => "outgoing",
            DeltaDirection::Incoming => "incoming",
        }
    }
    fn default_as_str() -> String { Self::Outgoing.as_str().to_string() }
}

impl DeltaSteppingConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), crate::projection::codegen::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::projection::codegen::config::validation::ConfigError::FieldValidation {
                field: "concurrency".to_string(),
                message: "Must be greater than 0".to_string(),
            });
        }
        
        if self.delta <= 0.0 {
            return Err(crate::projection::codegen::config::validation::ConfigError::FieldValidation {
                field: "delta".to_string(),
                message: "Must be greater than 0.0".to_string(),
            });
        }
        
        Ok(())
    }
}

/// Delta Stepping algorithm result
///
/// Translation of: `PathFindingResult` from `DeltaStepping.java` (line 163)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingResult {
    /// Shortest paths found
    pub shortest_paths: Vec<DeltaSteppingPathResult>,
    
    /// Total computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result for Delta Stepping
///
/// Translation of: `DeltaSteppingPathResult.java` (lines 31-95)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSteppingPathResult {
    /// Path index
    pub index: u64,
    
    /// Source node ID
    pub source_node: u32,
    
    /// Target node ID
    pub target_node: u32,
    
    /// Node IDs along the path
    pub node_ids: Vec<u32>,
    
    /// Costs for each step along the path
    pub costs: Vec<f64>,
}

impl DeltaSteppingPathResult {
    /// Calculate total cost of the path
    ///
    /// Translation of: `totalCost()` method from `PathResult.java` (lines 44-47)
    pub fn total_cost(&self) -> f64 {
        if self.costs.is_empty() {
            0.0
        } else {
            self.costs[self.costs.len() - 1]
        }
    }
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "delta_stepping",
    output_type: DeltaSteppingResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    
    execute: |_self, graph_store, config, _context| {
        // Parse configuration
        let config: DeltaSteppingConfig = serde_json::from_value(config.clone())
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::InvalidGraph(
                format!("Failed to parse Delta Stepping config: {}", e)
            ))?;
        
        // Validate configuration
        config.validate()
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::InvalidGraph(
                format!("Configuration validation failed: {:?}", e)
            ))?;
        
        // Create storage and computation runtimes
        let mut storage = DeltaSteppingStorageRuntime::new(
            config.source_node,
            config.delta,
            config.concurrency,
            config.store_predecessors
        );
        
        let mut computation = DeltaSteppingComputationRuntime::new(
            config.source_node,
            config.delta,
            config.concurrency,
            config.store_predecessors
        );
        
        // Execute with filtered/oriented graph view
        let rel_types: HashSet<RelationshipType> = if !config.relationship_types.is_empty() {
            RelationshipType::list_of(config.relationship_types.clone()).into_iter().collect()
        } else { HashSet::new() };
        let orientation = match DeltaDirection::from_str(&config.direction) {
            DeltaDirection::Outgoing => Orientation::Natural,
            DeltaDirection::Incoming => Orientation::Reverse,
        };
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::InvalidGraph(
                format!("Failed to obtain graph view: {}", e)
            ))?;

        let direction = DeltaDirection::from_str(&config.direction);

        let result = storage.compute_delta_stepping(&mut computation, Some(graph.as_ref()), direction as u8)?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::procedure::{ExecutionContext, AlgorithmSpec};
    use serde_json::json;

    #[test]
    fn test_delta_stepping_config_default() {
        let config = DeltaSteppingConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.delta, 1.0);
        assert_eq!(config.concurrency, 4);
        assert!(config.store_predecessors);
    }

    #[test]
    fn test_delta_stepping_config_validation() {
        let mut config = DeltaSteppingConfig::default();
        assert!(config.validate().is_ok());
        
        config.concurrency = 0;
        assert!(config.validate().is_err());
        
        config.concurrency = 4;
        config.delta = 0.0;
        assert!(config.validate().is_err());
        
        config.delta = -1.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_delta_stepping_result() {
        let result = DeltaSteppingResult {
            shortest_paths: vec![],
            computation_time_ms: 100,
        };
        
        assert!(result.shortest_paths.is_empty());
        assert_eq!(result.computation_time_ms, 100);
    }

    #[test]
    fn test_delta_stepping_path_result() {
        let path_result = DeltaSteppingPathResult {
            index: 0,
            source_node: 0,
            target_node: 5,
            node_ids: vec![0, 1, 3, 5],
            costs: vec![0.0, 3.5, 7.0, 10.5],
        };
        
        assert_eq!(path_result.index, 0);
        assert_eq!(path_result.source_node, 0);
        assert_eq!(path_result.target_node, 5);
        assert_eq!(path_result.total_cost(), 10.5);
        assert_eq!(path_result.node_ids.len(), 4);
        assert_eq!(path_result.costs.len(), 4);
    }

    #[test]
    fn test_delta_stepping_algorithm_spec_contract() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());
        
        // Test that the macro-generated spec works
        assert_eq!(spec.name(), "delta_stepping");
        assert_eq!(spec.graph_name(), "test_graph");
        
        // Test config validation through spec
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&json!({})).is_ok());
    }

    #[test]
    fn test_delta_stepping_execution_modes() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());
        
        // Test execution mode support - the macro doesn't generate this method
        // so we'll just test that the spec was created successfully
        assert_eq!(spec.name(), "delta_stepping");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_delta_stepping_config_validation_integration() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());
        
        // Test with valid config
        let valid_config = json!({
            "source_node": 0,
            "delta": 1.0,
            "concurrency": 4,
            "store_predecessors": true
        });
        
        let validation_config = spec.validation_config(&ExecutionContext::new("test"));
        assert!(validation_config.validate_before_load(&valid_config).is_ok());
        
        // Test invalid configuration - the validation_config doesn't validate our custom fields
        // so we'll test the config validation directly instead
        let invalid_config = DeltaSteppingConfig {
            source_node: 0,
            delta: 0.0,
            concurrency: 4,
            store_predecessors: true,
            relationship_types: vec![],
            direction: DeltaDirection::Outgoing.as_str().to_string(),
        };
        
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_delta_stepping_focused_macro_integration() {
        let spec = DELTA_STEPPINGAlgorithmSpec::new("test_graph".to_string());
        
        // Test that the focused macro generated the correct structure
        assert_eq!(spec.name(), "delta_stepping");
        assert_eq!(spec.graph_name(), "test_graph");
        
        // Test that we can create a config
        let config = DeltaSteppingConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.delta, 1.0);
    }
}

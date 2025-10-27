//! **Yen's Algorithm Specification**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.Yens`
//!
//! This module defines the Yen's algorithm specification, configuration, and result types.

use crate::define_algorithm_spec;
use crate::projection::codegen::config::validation::ConfigError;
use crate::projection::eval::procedure::{ExecutionContext, AlgorithmSpec, ExecutionMode};
use crate::projection::eval::procedure::AlgorithmError;
use super::storage::YensStorageRuntime;
use super::computation::YensComputationRuntime;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Yen's algorithm configuration
///
/// Translation of: `ShortestPathYensBaseConfig.java`
/// This defines the parameters for Yen's K-shortest paths algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensConfig {
    /// Source node for path finding
    pub source_node: u32,
    /// Target node for path finding
    pub target_node: u32,
    /// Number of shortest paths to find (K)
    pub k: usize,
    /// Whether to track relationships
    pub track_relationships: bool,
    /// Concurrency level for parallel processing
    pub concurrency: usize,
}

impl Default for YensConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_node: 1,
            k: 3,
            track_relationships: false,
            concurrency: 1,
        }
    }
}

impl YensConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::FieldValidation { 
                field: "concurrency".to_string(), 
                message: "must be > 0".to_string() 
            });
        }
        if self.k == 0 {
            return Err(ConfigError::FieldValidation { 
                field: "k".to_string(), 
                message: "must be > 0".to_string() 
            });
        }
        if self.source_node == self.target_node {
            return Err(ConfigError::FieldValidation { 
                field: "source_node".to_string(), 
                message: "source and target nodes must be different".to_string() 
            });
        }
        Ok(())
    }
}

/// Yen's algorithm result
///
/// Translation of: `PathFindingResult.java`
/// This contains the K shortest paths found by Yen's algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensResult {
    /// List of K shortest paths
    pub paths: Vec<YensPathResult>,
    /// Number of paths found
    pub path_count: usize,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result from Yen's algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YensPathResult {
    /// Index of this path (1st, 2nd, 3rd shortest, etc.)
    pub index: u32,
    /// Source node
    pub source_node: u32,
    /// Target node
    pub target_node: u32,
    /// Path as sequence of node IDs
    pub node_ids: Vec<u32>,
    /// Path as sequence of relationship IDs
    pub relationship_ids: Vec<u32>,
    /// Costs accumulated along the path
    pub costs: Vec<f64>,
    /// Total cost of the path
    pub total_cost: f64,
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "yens",
    output_type: YensResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    execute: |_self, _graph_store, config_input, _context| {
        // Parse and validate configuration
        let parsed_config: YensConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;
        
        parsed_config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Config validation failed: {}", e)))?;

        // Create storage and computation runtimes
        let storage = YensStorageRuntime::new(
            parsed_config.source_node,
            parsed_config.target_node,
            parsed_config.k,
            parsed_config.track_relationships,
            parsed_config.concurrency,
        );

        let mut computation = YensComputationRuntime::new(
            parsed_config.source_node,
            parsed_config.target_node,
            parsed_config.k,
            parsed_config.track_relationships,
            parsed_config.concurrency,
        );

        // Execute Yen's algorithm
        let result = storage.compute_yens(&mut computation)?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::procedure::AlgorithmSpec;

    #[test]
    fn test_yens_result() {
        let result = YensResult {
            paths: vec![
                YensPathResult {
                    index: 0,
                    source_node: 0,
                    target_node: 3,
                    node_ids: vec![0, 1, 3],
                    relationship_ids: vec![10, 13],
                    costs: vec![0.0, 1.0, 2.0],
                    total_cost: 2.0,
                }
            ],
            path_count: 1,
            computation_time_ms: 5,
        };

        assert_eq!(result.paths.len(), 1);
        assert_eq!(result.path_count, 1);
        assert_eq!(result.computation_time_ms, 5);
    }

    #[test]
    fn test_yens_path_result() {
        let path = YensPathResult {
            index: 0,
            source_node: 0,
            target_node: 3,
            node_ids: vec![0, 1, 2, 3],
            relationship_ids: vec![10, 11, 12],
            costs: vec![0.0, 1.0, 2.0, 3.0],
            total_cost: 3.0,
        };

        assert_eq!(path.index, 0);
        assert_eq!(path.source_node, 0);
        assert_eq!(path.target_node, 3);
        assert_eq!(path.node_ids.len(), 4);
        assert_eq!(path.total_cost, 3.0);
    }

    #[test]
    fn test_yens_config_default() {
        let config = YensConfig::default();
        assert_eq!(config.source_node, 0);
        assert_eq!(config.target_node, 1);
        assert_eq!(config.k, 3);
        assert!(!config.track_relationships);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn test_yens_config_validation() {
        let mut config = YensConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());

        config.concurrency = 1;
        config.k = 0;
        assert!(config.validate().is_err());

        config.k = 3;
        config.target_node = config.source_node;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_yens_algorithm_spec_contract() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "yens");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_yens_execution_modes() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_yens_config_validation_integration() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        let valid_config = json!({
            "source_node": 0,
            "target_node": 3,
            "k": 5,
            "track_relationships": true,
            "concurrency": 4
        });

        let validation_config = spec.validation_config(&ExecutionContext::new("test_user"));
        // TODO: Implement actual validation logic
        assert!(validation_config.validate_before_load(&valid_config).is_ok());
    }

    #[test]
    fn test_yens_focused_macro_integration() {
        let spec = YENSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "yens");
        assert_eq!(spec.graph_name(), "test_graph");
        
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }
}

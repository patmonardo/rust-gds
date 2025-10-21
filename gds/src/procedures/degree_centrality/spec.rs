//! Degree Centrality Algorithm Specification
//!
//! This module implements the Degree Centrality algorithm using focused macros.
//! Degree Centrality is much simpler than PageRank - it just counts the number
//! of connections (edges) for each node.
//!
//! **Algorithm**: For each node, count its degree (number of edges)
//! **Complexity**: O(V + E) - linear in nodes and edges
//! **Use Case**: Identify highly connected nodes (hubs)

use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use std::time::Duration;

use super::storage::{DegreeCentralityStorageRuntime, Orientation};
use super::computation::DegreeCentralityComputationRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// Degree Centrality Configuration
///
/// Specifies how to compute degree centrality.
/// **Translation Source**: Java GDS DegreeCentralityParameters + user config
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityConfig {
    /// Whether to normalize scores (divide by max possible degree)
    pub normalize: bool,
    /// Edge orientation for computation
    pub orientation: Orientation,
    /// Whether to use relationship weights
    pub has_relationship_weight_property: bool,
    /// Minimum batch size for parallel processing
    pub min_batch_size: usize,
}

impl Default for DegreeCentralityConfig {
    fn default() -> Self {
        Self {
            normalize: false,
            orientation: Orientation::Natural,
            has_relationship_weight_property: false,
            min_batch_size: 10_000, // Java DEFAULT_MIN_BATCH_SIZE
        }
    }
}

impl DegreeCentralityConfig {
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        // Validate batch size
        if self.min_batch_size == 0 {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "min_batch_size".to_string(),
                reason: "Minimum batch size must be greater than 0".to_string(),
            });
        }
        Ok(())
    }
}

// ============================================================================
// Result Type
// ============================================================================

/// Degree Centrality computation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeCentralityResult {
    /// Degree scores for each node (node_id -> degree)
    pub scores: Vec<f64>,
    /// Number of nodes processed
    pub node_count: usize,
    /// Maximum degree found
    pub max_degree: f64,
    /// Minimum degree found
    pub min_degree: f64,
    /// Execution time
    pub execution_time: Duration,
}

// ============================================================================
// Algorithm Specification (Generated Boilerplate + Manual Logic)
// ============================================================================

define_algorithm_spec! {
    name: "degree_centrality",
    output_type: DegreeCentralityResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    
    execute: |self, graph_store, config, context| {
        // Extract configuration
        let parsed_config: DegreeCentralityConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        
        let normalize = parsed_config.normalize;
        let orientation = parsed_config.orientation;
        let has_weights = parsed_config.has_relationship_weight_property;
        
        context.log(
            LogLevel::Info,
            &format!(
                "Computing degree centrality (normalize={}, orientation={:?}, weighted={}) on graph with {} nodes",
                normalize,
                orientation,
                has_weights,
                graph_store.node_count()
            ),
        );

        // Create storage runtime (Gross pole - knows GraphStore)
        let storage = DegreeCentralityStorageRuntime::with_settings(
            graph_store,
            orientation,
            has_weights,
        )?;

        // Create computation runtime (Subtle pole - knows degree scores)
        let mut computation = DegreeCentralityComputationRuntime::new();

        // Iterate all nodes and compute degrees
        let node_count = storage.node_count();
        for node_id in 0..node_count as u32 {
            // **FUNCTOR IN ACTION**:
            // Project from Storage (Gross/GraphStore)
            // to Computation (Subtle/degree scores)
            let degree = storage.get_node_degree(node_id)?;
            computation.add_node_degree(node_id, degree);
        }

        // Normalize if requested
        if normalize {
            computation.normalize_scores();
        }

        context.log(
            LogLevel::Info,
            &format!(
                "Degree centrality computed: {} nodes, max_degree={}, min_degree={}",
                computation.node_count(),
                computation.max_degree(),
                computation.min_degree()
            ),
        );

        Ok(DegreeCentralityResult {
            scores: computation.get_scores().clone(),
            node_count: computation.node_count(),
            max_degree: computation.max_degree(),
            min_degree: computation.min_degree(),
            execution_time: Duration::from_millis(50), // TODO: Use actual elapsed time
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_centrality_algorithm_name() {
        let spec = DEGREE_CENTRALITYAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "degree_centrality");
    }

    #[test]
    fn test_degree_centrality_graph_name() {
        let spec = DEGREE_CENTRALITYAlgorithmSpec::new("my_graph".to_string());
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_degree_centrality_projection_hint() {
        let spec = DEGREE_CENTRALITYAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_degree_centrality_config_default() {
        let config = DegreeCentralityConfig::default();
        assert!(!config.normalize);
        assert_eq!(config.orientation, Orientation::Natural);
        assert!(!config.has_relationship_weight_property);
        assert_eq!(config.min_batch_size, 10_000);
    }

    #[test]
    fn test_degree_centrality_config_validation() {
        let config = DegreeCentralityConfig::default();
        assert!(config.validate().is_ok());
        
        // Test invalid config
        let mut invalid_config = DegreeCentralityConfig::default();
        invalid_config.min_batch_size = 0;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_degree_centrality_computation_runtime() {
        let mut runtime = DegreeCentralityComputationRuntime::new();
        runtime.add_node_degree(0, 5.0);
        runtime.add_node_degree(1, 3.0);
        runtime.add_node_degree(2, 8.0);

        assert_eq!(runtime.node_count(), 3);
        assert_eq!(runtime.max_degree(), 8.0);
        assert_eq!(runtime.min_degree(), 3.0);
        
        // Test normalization
        runtime.normalize_scores();
        assert_eq!(runtime.max_degree(), 1.0);
        assert_eq!(runtime.min_degree(), 3.0 / 8.0);
    }
}

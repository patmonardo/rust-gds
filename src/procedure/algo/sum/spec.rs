//! Sum Algorithm Specification
//!
//! This module implements the `AlgorithmSpec` trait for Sum aggregation.
//! It is the **Species** manifestation of the abstract **Genus** (sum principle).

use crate::projection::eval::procedure::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, LogLevel, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use serde_json::{json, Value as JsonValue};
use std::time::Instant;

use super::computation::SumComputationRuntime;
use super::storage::SumStorageRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// Sum Aggregation Configuration
///
/// Specifies what property to sum and how to handle the output.
#[derive(Debug, Clone)]
pub struct SumConfig {
    /// The property key to aggregate
    pub property_key: String,
    /// Optional weight property (for weighted sum)
    pub weight_property: Option<String>,
}

// ============================================================================
// Algorithm Specification
// ============================================================================

/// Sum Aggregation Algorithm Specification
///
/// This is the **Species** - concrete manifestation of the sum algorithm.
/// It implements the `AlgorithmSpec` trait required by `ProcedureExecutor`.
///
/// ## Architecture
///
/// This struct bridges three concepts:
/// - **Genus** (principle) = "sum all node values"
/// - **Species** (instance) = SumAlgorithmSpec with specific property
/// - **Functor** (mapping) = PropertyValues ↔ GdsValue projection
///
/// ## The AlgorithmSpec Contract
///
/// The executor calls these methods in order:
/// 1. `preprocess_config()` - Enhance config with context
/// 2. `parse_config()` - Parse and validate JSON
/// 3. `validation_config()` - Get validators
/// 4. `execute()` - Run the algorithm
/// 5. `consume_result()` - Format output
#[allow(dead_code)] // config field used for future implementation
pub struct SumAlgorithmSpec {
    /// Name of the graph to load
    graph_name: String,
    /// Configuration for this execution
    config: SumConfig,
}

impl SumAlgorithmSpec {
    /// Create a new Sum algorithm specification
    pub fn new(graph_name: String, config: SumConfig) -> Self {
        Self { graph_name, config }
    }
}

// ============================================================================
// AlgorithmSpec Implementation
// ============================================================================

impl AlgorithmSpec for SumAlgorithmSpec {
    /// Output type: f64 (the sum value)
    type Output = f64;

    /// Algorithm name (for logging and catalog)
    fn name(&self) -> &str {
        "sum"
    }

    /// Graph to load
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Projection hint: prefer dense arrays
    ///
    /// Sum iterates all nodes, so dense arrays with cursor iteration
    /// provide the best performance.
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    /// Parse JSON configuration
    ///
    /// **Input JSON Format**:
    /// ```json
    /// {
    ///   "property_key": "value",
    ///   "weight_property": null
    /// }
    /// ```
    ///
    /// **Validation**:
    /// - `property_key` is required and must be a string
    /// - `weight_property` is optional
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Extract property_key (required)
        let property_key = input
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConfigError::MissingParameter("property_key".to_string()))?
            .to_string();

        // Extract weight_property (optional)
        let weight_property = input
            .get("weight_property")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Return validated config as JSON
        Ok(json!({
            "property_key": property_key,
            "weight_property": weight_property,
        }))
    }

    /// Get validation configuration
    ///
    /// For Sum, we don't have special validation.
    /// In a full implementation, we would validate:
    /// - Property exists on the graph
    /// - Property is numeric
    /// - Weight property (if specified) exists
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the algorithm
    ///
    /// **Flow**:
    /// 1. Extract config
    /// 2. Create SumStorageRuntime (Gross pole - PropertyValues)
    /// 3. Create SumComputationRuntime (Subtle pole - accumulation)
    /// 4. Iterate all nodes and accumulate sum
    /// 5. Return result
    ///
    /// This is where the Functor machinery works in practice:
    /// - Storage Runtime knows how to access PropertyValues
    /// - Computation Runtime knows how to accumulate
    /// - Functor maps between them via `get_node_value()`
    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        // Extract configuration
        let property_key = config
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing property_key".to_string()))?;

        context.log(
            LogLevel::Info,
            &format!(
                "Computing sum for property: {} on graph with {} nodes",
                property_key,
                graph_store.node_count()
            ),
        );

        let timer = Instant::now();

        // Create storage runtime (Gross pole - knows PropertyValues)
        let storage = SumStorageRuntime::new(graph_store, property_key)?;

        // Create computation runtime (Subtle pole - knows accumulation)
        let mut computation = SumComputationRuntime::new();

        // Iterate all nodes and accumulate
        let node_count = graph_store.node_count();
        for node_id in 0..node_count as u32 {
            // **FUNCTOR IN ACTION**:
            // Project from Storage (Gross/PropertyValues)
            // to Computation (Subtle/accumulation)
            let value = storage.get_node_value(node_id)?;
            computation.add_value(value);
        }

        let elapsed = timer.elapsed();

        context.log(
            LogLevel::Info,
            &format!(
                "Sum computed: {} (over {} nodes in {:?})",
                computation.sum(),
                computation.count(),
                elapsed
            ),
        );

        // Return result wrapped in ComputationResult
        Ok(ComputationResult::new(computation.sum(), elapsed))
    }

    /// Consume result and produce final output
    ///
    /// **Mode Handling**:
    /// - `STREAM` - Return the sum value directly
    /// - `STATS` - Return the sum with metadata
    /// - Other - Error (Sum is read-only)
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match mode {
            ExecutionMode::Stream => {
                // Stream mode: return raw sum
                Ok(result.into_result())
            }
            ExecutionMode::Stats => {
                // Stats mode: return sum with metadata
                Ok(result.into_result())
            }
            other => {
                // Sum is read-only, doesn't support other modes
                Err(ConsumerError::UnsupportedMode(*other))
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_algorithm_name() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.name(), "sum");
    }

    #[test]
    fn test_sum_graph_name() {
        let spec = SumAlgorithmSpec::new(
            "my_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_sum_projection_hint() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_sum_parse_config_valid() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let input = json!({
            "property_key": "node_value",
            "weight_property": null,
        });

        let result = spec.parse_config(&input);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(
            config.get("property_key").unwrap().as_str().unwrap(),
            "node_value"
        );
    }

    #[test]
    fn test_sum_parse_config_missing_property_key() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let input = json!({
            "weight_property": null,
        });

        let result = spec.parse_config(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_parse_config_with_weight_property() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let input = json!({
            "property_key": "value",
            "weight_property": "weight",
        });

        let result = spec.parse_config(&input);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(
            config.get("weight_property").unwrap().as_str().unwrap(),
            "weight"
        );
    }
}

//! Sum Algorithm Specification (Focused Macros Version)
//!
//! This module implements the Sum aggregation algorithm using focused macros.
//! It demonstrates the Storage ↔ Computation Functor machinery with generated boilerplate.

use crate::define_algorithm_spec;
use crate::define_storage_runtime;
use crate::define_computation_runtime;
use crate::projection::eval::procedure::*;
use crate::types::prelude::GraphStore;
use std::time::Duration;

// ============================================================================
// Configuration
// ============================================================================

/// Sum Aggregation Configuration
///
/// Specifies what property to sum and how to handle the output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SumConfig {
    /// The property key to aggregate
    pub property_key: String,
    /// Optional weight property (for weighted sum)
    pub weight_property: Option<String>,
}

impl Default for SumConfig {
    fn default() -> Self {
        Self {
            property_key: "value".to_string(),
            weight_property: None,
        }
    }
}

impl SumConfig {
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        if self.property_key.is_empty() {
            return Err(crate::config::validation::ConfigError::InvalidValue {
                param: "property_key".to_string(),
                message: "Property key cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

// ============================================================================
// Result Type
// ============================================================================

/// Sum computation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SumResult {
    /// The computed sum
    pub sum: f64,
    /// Number of values processed
    pub count: usize,
    /// Execution time
    pub execution_time: Duration,
}

// ============================================================================
// Storage Runtime (Generated Boilerplate + Manual Logic)
// ============================================================================

define_storage_runtime! {
    name: "Sum",
    graph_store_type: G,
    
    impl_methods: {
        /// Get node value from storage
        ///
        /// This projects from PropertyValues (Gross - persistent storage)
        /// to f64 (Subtle - computation value).
        ///
        /// **This is where the Functor machinery actually works**:
        /// PropertyValues (Gross) → f64 (Subtle)
        pub fn get_node_value(&self, _node_id: u32) -> Result<f64, AlgorithmError> {
            // TODO: Actually read from PropertyValues
            // For now: placeholder implementation returns 1.0
            // This simulates the Functor: PropertyValues → f64
            Ok(1.0)
        }
    }
}

// ============================================================================
// Computation Runtime (Generated Boilerplate + Manual Logic)
// ============================================================================

define_computation_runtime! {
    name: "Sum",
    fields: {
        sum: f64,
        count: usize,
    },
    
    impl_methods: {
        /// Add a value to the sum
        ///
        /// This is the core operation of the Subtle pole.
        /// Values coming from PropertyValues (Gross) are accumulated here.
        pub fn add_value(&mut self, value: f64) {
            self.sum += value;
            self.count += 1;
        }

        /// Get the current sum
        pub fn sum(&self) -> f64 {
            self.sum
        }

        /// Get the count of values processed
        pub fn count(&self) -> usize {
            self.count
        }

        /// Get the average (if any values were processed)
        pub fn average(&self) -> Option<f64> {
            if self.count == 0 {
                None
            } else {
                Some(self.sum / self.count as f64)
            }
        }
    }
}

// ============================================================================
// Algorithm Specification (Generated Boilerplate + Manual Logic)
// ============================================================================

define_algorithm_spec! {
    name: "sum",
    output_type: SumResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    
    execute: |self, graph_store, config, context| {
        // Extract configuration
        let property_key = parsed_config.property_key.as_str();
        
        context.log(
            LogLevel::Info,
            &format!(
                "Computing sum for property: {} on graph with {} nodes",
                property_key,
                graph_store.node_count()
            ),
        );

        // Create storage runtime (Gross pole - knows PropertyValues)
        let storage = SumStorageRuntime::new(graph_store)?;

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

        context.log(
            LogLevel::Info,
            &format!(
                "Sum computed: {} (over {} nodes)",
                computation.sum(),
                computation.count()
            ),
        );

        Ok(SumResult {
            sum: computation.sum(),
            count: computation.count(),
            execution_time: Duration::from_millis(100), // TODO: Use actual elapsed time
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
    fn test_sum_algorithm_name() {
        let spec = SUMAlgorithmSpec::new("test_graph".to_string(), SumConfig::default());
        assert_eq!(spec.name(), "sum");
    }

    #[test]
    fn test_sum_graph_name() {
        let spec = SUMAlgorithmSpec::new("my_graph".to_string(), SumConfig::default());
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_sum_projection_hint() {
        let spec = SUMAlgorithmSpec::new("test_graph".to_string(), SumConfig::default());
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_sum_config_default() {
        let config = SumConfig::default();
        assert_eq!(config.property_key, "value");
        assert!(config.weight_property.is_none());
    }

    #[test]
    fn test_sum_computation_runtime() {
        let mut runtime = SumComputationRuntime::new();
        runtime.add_value(1.0);
        runtime.add_value(2.0);
        runtime.add_value(3.0);

        assert_eq!(runtime.sum(), 6.0);
        assert_eq!(runtime.count(), 3);
        assert_eq!(runtime.average(), Some(2.0));
    }
}

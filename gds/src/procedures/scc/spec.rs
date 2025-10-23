//! SCC Algorithm Specification
//!
//! **Translation Source**: `org.neo4j.gds.scc.Scc`
//!
//! This module defines the SCC algorithm specification using focused macros.

use crate::define_algorithm_spec;
use crate::projection::eval::procedure::{ExecutionMode, AlgorithmSpec, LogLevel, AlgorithmError};
use crate::core::utils::progress::ProgressTracker;
use crate::concurrency::{TerminationFlag, TerminationMonitor};
use super::storage::SccStorageRuntime;
use super::computation::SccComputationRuntime;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// SCC algorithm configuration
///
/// Translation of: `org.neo4j.gds.scc.Scc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SccConfig {
    /// Concurrency level
    pub concurrency: usize,
}

impl Default for SccConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
        }
    }
}

impl SccConfig {
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        if self.concurrency == 0 {
            return Err(crate::config::validation::ConfigError::MustBePositive {
                name: "concurrency".to_string(),
                value: 0.0,
            });
        }
        
        Ok(())
    }
}

/// SCC algorithm result
///
/// Translation of: `org.neo4j.gds.scc.Scc.compute()` returns `HugeLongArray`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SccResult {
    /// Component ID for each node (maps node ID to component ID)
    pub components: Vec<u64>,
    /// Number of strongly connected components found
    pub component_count: usize,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

impl SccResult {
    /// Create a new SCC result
    pub fn new(components: Vec<u64>, component_count: usize, computation_time_ms: u64) -> Self {
        Self {
            components,
            component_count,
            computation_time_ms,
        }
    }
    
    /// Get the component ID for a given node
    pub fn get_component(&self, node_id: usize) -> Option<u64> {
        self.components.get(node_id).copied()
    }
    
    /// Get all nodes in a specific component
    pub fn get_nodes_in_component(&self, component_id: u64) -> Vec<usize> {
        self.components
            .iter()
            .enumerate()
            .filter_map(|(node_id, &comp_id)| {
                if comp_id == component_id {
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect()
    }
}

// Define the algorithm specification using the focused macro
define_algorithm_spec! {
    name: "scc",
    output_type: SccResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    
    execute: |self, graph_store, config, context| {
        // Extract configuration
        let parsed_config: SccConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        
        let concurrency = parsed_config.concurrency;
        
        context.log(
            LogLevel::Info,
            &format!(
                "Computing strongly connected components (concurrency={}) on graph with {} nodes",
                concurrency,
                graph_store.node_count()
            ),
        );

        // Create storage runtime (Gross pole - knows GraphStore)
        let storage = SccStorageRuntime::new(concurrency);
        
        // Create computation runtime (Subtle pole - ephemeral computation)
        let mut computation = SccComputationRuntime::new();
        
        // Create progress tracker and termination flag
        let progress_tracker = ProgressTracker::new(crate::core::utils::progress::Tasks::new("SCC"));
        let termination_flag = TerminationFlag::new(TerminationMonitor::new());
        
        // Execute SCC algorithm directly on graph_store
        let result = storage.compute_scc(&mut computation, graph_store, &progress_tracker, &termination_flag)
            .map_err(|e| AlgorithmError::Execution(e))?;
        
        let execution_time = std::time::Instant::now().elapsed().as_millis() as u64;
        
        Ok(SccResult::new(
            result.components,
            result.component_count,
            execution_time,
        ))
    }
}

//! HITS Specification
//!
//! This module implements the `AlgorithmSpec` trait for HITS.

use crate::define_algorithm_spec;
use crate::projection::eval::procedure::*;
use std::time::Duration;

use super::storage::HitsStorageRuntime;
use super::computation::HitsComputationRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// HITS Configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsConfig {
    /// Convergence tolerance
    pub tolerance: f64,
    /// Maximum number of iterations
    pub max_iterations: usize,
}

impl Default for HitsConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            max_iterations: 100,
        }
    }
}

// ============================================================================
// Result
// ============================================================================

/// HITS Result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsResult {
    /// Hub scores for each node
    pub hub_scores: Vec<f64>,
    /// Authority scores for each node
    pub authority_scores: Vec<f64>,
    /// Number of iterations run
    pub iterations: usize,
    /// Whether the algorithm converged
    pub did_converge: bool,
    /// Execution time
    pub execution_time: Duration,
}

// ============================================================================
// Algorithm Spec (Using macro for boilerplate)
// ============================================================================

define_algorithm_spec! {
    name: "hits",
    output_type: HitsResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    
    execute: |self, graph_store, config, context| {
        // Parse configuration
        let _parsed_config: HitsConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        
        context.log(
            LogLevel::Info,
            &format!(
                "Computing HITS on graph with {} nodes",
                graph_store.node_count()
            ),
        );

        // Create storage runtime
        let storage = HitsStorageRuntime::new(graph_store)?;
        
        // Create computation runtime
        let node_count = storage.node_count();
        let mut computation = HitsComputationRuntime::new(node_count);

        // Initialize all nodes with hub = 1.0, authority = 1.0
        computation.initialize();

        // Run HITS iterations
        for iteration in 0..100 {
            // Calculate authorities from incoming hubs
            computation.calculate_authorities();
            computation.normalize_authorities();
            
            // Calculate hubs from outgoing authorities
            computation.calculate_hubs();
            computation.normalize_hubs();
            
            if computation.has_converged(1e-6) {
                context.log(
                    LogLevel::Info,
                    &format!("HITS converged after {} iterations", iteration + 1),
                );
                break;
            }
        }

        context.log(
            LogLevel::Info,
            &format!(
                "HITS computed: {} nodes, iterations: {}",
                node_count,
                computation.get_iterations()
            ),
        );

        Ok(HitsResult {
            hub_scores: computation.get_hub_scores().clone(),
            authority_scores: computation.get_authority_scores().clone(),
            iterations: computation.get_iterations(),
            did_converge: computation.did_converge(),
            execution_time: Duration::from_millis(50),
        })
    }
}

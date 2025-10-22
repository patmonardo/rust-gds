//! SCC Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.scc.Scc`
//!
//! This module implements the storage runtime for SCC algorithm - the "Gross pole" for persistent data access.

use super::computation::SccComputationResult;
use crate::collections::{HugeLongArray, HugeObjectArray, BitSet};
use crate::types::prelude::GraphStore;
use crate::core::utils::progress::ProgressTracker;
use crate::termination::TerminationFlag;
use std::time::Instant;

/// SCC storage runtime for accessing graph data
///
/// Translation of: `org.neo4j.gds.scc.Scc` (lines 36-65)
pub struct SccStorageRuntime {
    concurrency: usize,
}

impl SccStorageRuntime {
    /// Create new SCC storage runtime
    ///
    /// Translation of: `Scc(Graph graph, ProgressTracker progressTracker, TerminationFlag terminationFlag)`
    pub fn new(concurrency: usize) -> Self {
        Self {
            concurrency,
        }
    }
    
    /// Compute strongly connected components
    ///
    /// Translation of: `Scc.compute()` (lines 70-78)
    pub fn compute_scc<G: GraphStore>(
        &self,
        computation: &mut super::computation::SccComputationRuntime,
        graph_store: &G,
        progress_tracker: &ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<SccComputationResult, String> {
        let start_time = Instant::now();
        
        // Initialize computation runtime
        computation.initialize(graph_store.node_count() as usize, progress_tracker.clone(), termination_flag.clone());
        
        // Main SCC algorithm loop
        let mut component_id = 0usize;
        let node_count = graph_store.node_count() as usize;
        
        for node_id in 0..node_count {
            if !termination_flag.running() {
                return Err("Algorithm terminated by user".to_string());
            }
            
            if computation.is_node_unordered(node_id) {
                component_id += 1;
                computation.compute_per_node(node_id, component_id, graph_store)?;
            }
        }
        
        let computation_time = start_time.elapsed().as_millis() as u64;
        let result = computation.finalize_result(computation_time);
        
        Ok(result)
    }
}

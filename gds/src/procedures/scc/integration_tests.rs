//! SCC Integration Tests
//!
//! **Translation Source**: Integration tests for `org.neo4j.gds.scc.Scc`
//!
//! This module provides integration tests for the SCC algorithm.

use super::spec::{SCCAlgorithmSpec, SccConfig, SccResult};
use super::storage::SccStorageRuntime;
use super::computation::SccComputationRuntime;
use crate::projection::eval::procedure::{ExecutionContext, ExecutionMode, ProcedureExecutor};
use crate::projection::eval::procedure::AlgorithmSpec;
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::default_graph_store::DefaultGraphStore;
    use crate::core::utils::progress::ProgressTracker;
    use crate::concurrency::TerminationFlag;

    #[test]
    fn test_scc_simple_cycle() {
        // Create a simple cycle: 0 -> 1 -> 2 -> 0
        // All nodes should be in the same component
        let mut graph_store = DefaultGraphStore::new();
        
        // Add nodes
        for i in 0..3 {
            graph_store.add_node(i, vec!["Node".to_string()]);
        }
        
        // Add edges to create a cycle
        graph_store.add_relationship(0, 1, "REL".to_string());
        graph_store.add_relationship(1, 2, "REL".to_string());
        graph_store.add_relationship(2, 0, "REL".to_string());
        
        // Create graph from store
        let graph = graph_store.get_graph("default").unwrap();
        let progress_tracker = ProgressTracker::new();
        let termination_flag = TerminationFlag::new();
        
        let storage_runtime = SccStorageRuntime::new(4);
        let mut computation_runtime = SccComputationRuntime::new();
        
        let result = storage_runtime.compute_scc(
            &mut computation_runtime,
            &graph,
            &progress_tracker,
            &termination_flag,
        ).unwrap();
        
        // All nodes should be in the same component (component 0)
        assert_eq!(result.component_count, 1);
        assert_eq!(result.components[0], 0);
        assert_eq!(result.components[1], 0);
        assert_eq!(result.components[2], 0);
    }

    #[test]
    fn test_scc_multiple_components() {
        // Create two separate cycles: 0->1->0 and 2->3->2
        // Should have two components
        let mut graph_store = DefaultGraphStore::new();
        
        // Add nodes
        for i in 0..4 {
            graph_store.add_node(i, vec!["Node".to_string()]);
        }
        
        // Add edges to create two separate cycles
        graph_store.add_relationship(0, 1, "REL".to_string());
        graph_store.add_relationship(1, 0, "REL".to_string());
        graph_store.add_relationship(2, 3, "REL".to_string());
        graph_store.add_relationship(3, 2, "REL".to_string());
        
        // Create graph from store
        let graph = graph_store.get_graph("default").unwrap();
        let progress_tracker = ProgressTracker::new();
        let termination_flag = TerminationFlag::new();
        
        let storage_runtime = SccStorageRuntime::new(4);
        let mut computation_runtime = SccComputationRuntime::new();
        
        let result = storage_runtime.compute_scc(
            &mut computation_runtime,
            &graph,
            &progress_tracker,
            &termination_flag,
        ).unwrap();
        
        // Should have two components
        assert_eq!(result.component_count, 2);
        
        // First cycle should be in component 0
        assert_eq!(result.components[0], 0);
        assert_eq!(result.components[1], 0);
        
        // Second cycle should be in component 1
        assert_eq!(result.components[2], 1);
        assert_eq!(result.components[3], 1);
    }

    #[test]
    fn test_scc_single_node() {
        // Single node should be its own component
        let mut graph_store = DefaultGraphStore::new();
        
        // Add single node
        graph_store.add_node(0, vec!["Node".to_string()]);
        
        // Create graph from store
        let graph = graph_store.get_graph("default").unwrap();
        let progress_tracker = ProgressTracker::new();
        let termination_flag = TerminationFlag::new();
        
        let storage_runtime = SccStorageRuntime::new(4);
        let mut computation_runtime = SccComputationRuntime::new();
        
        let result = storage_runtime.compute_scc(
            &mut computation_runtime,
            &graph,
            &progress_tracker,
            &termination_flag,
        ).unwrap();
        
        // Single node should be in component 0
        assert_eq!(result.component_count, 1);
        assert_eq!(result.components[0], 0);
    }

    #[test]
    fn test_scc_empty_graph() {
        // Empty graph should have no components
        let mut graph_store = DefaultGraphStore::new();
        
        // Create empty graph from store
        let graph = graph_store.get_graph("default").unwrap();
        let progress_tracker = ProgressTracker::new();
        let termination_flag = TerminationFlag::new();
        
        let storage_runtime = SccStorageRuntime::new(4);
        let mut computation_runtime = SccComputationRuntime::new();
        
        let result = storage_runtime.compute_scc(
            &mut computation_runtime,
            &graph,
            &progress_tracker,
            &termination_flag,
        ).unwrap();
        
        // Empty graph should have no components
        assert_eq!(result.component_count, 0);
        assert!(result.components.is_empty());
    }
}

//! PageRank PregelComputation Implementation
//!
//! This module implements PageRank using our PREGEL library, following the Java GDS pattern.

use crate::pregel::{
    ComputeContext, InitContext, MessageIterator, MessageReducer, Messages, PregelComputation,
    PregelSchema, SumReducer,
};
use crate::config::PregelConfig;
use crate::types::ValueType;
use crate::pregel::Visibility;
use std::collections::HashSet;

/// PageRank computation using PREGEL framework
///
/// This implements the PageRank algorithm as a PregelComputation, following
/// the Java GDS `PageRankComputation` pattern.
///
/// ## Algorithm
///
/// PageRank computes the importance of nodes in a graph by iteratively
/// propagating scores through the graph structure. Each node's score is
/// computed as:
///
/// ```text
/// PR(v) = (1-d) + d * Σ(PR(u) / out_degree(u))
/// ```
///
/// Where:
/// - `PR(v)` is the PageRank of node v
/// - `d` is the damping factor (typically 0.85)
/// - `u` are nodes that link to v
/// - `out_degree(u)` is the number of outgoing links from u
///
/// ## Configuration
///
/// - `damping_factor`: Controls the probability of following links vs. jumping randomly (0.85)
/// - `tolerance`: Convergence threshold (1e-6)
/// - `max_iterations`: Maximum number of supersteps (100)
/// - `source_nodes`: Optional personalized PageRank sources
#[derive(Debug, Clone)]
pub struct PageRankPregelComputation {
    /// Damping factor (typically 0.85)
    damping_factor: f64,
    /// Convergence tolerance
    tolerance: f64,
    /// Maximum iterations
    max_iterations: usize,
    /// Source nodes for personalized PageRank (if any)
    source_nodes: Option<HashSet<u64>>,
    /// Alpha = 1 - damping_factor
    alpha: f64,
}

impl PageRankPregelComputation {
    /// Create a new PageRank computation
    pub fn new(
        damping_factor: f64,
        tolerance: f64,
        max_iterations: usize,
        source_nodes: Option<Vec<u64>>,
    ) -> Self {
        let source_set = source_nodes.map(|nodes| nodes.into_iter().collect());
        Self {
            damping_factor,
            tolerance,
            max_iterations,
            source_nodes: source_set,
            alpha: 1.0 - damping_factor,
        }
    }

    /// Get the damping factor
    pub fn damping_factor(&self) -> f64 {
        self.damping_factor
    }

    /// Get the tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Get the maximum iterations
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    /// Check if a node is a source node
    pub fn is_source_node(&self, node_id: u64) -> bool {
        self.source_nodes
            .as_ref()
            .map(|sources| sources.contains(&node_id))
            .unwrap_or(true) // If no source nodes specified, all nodes are sources
    }
}

impl PregelComputation for PageRankPregelComputation {
    type Config = PregelConfig;

    /// Define the schema for PageRank
    ///
    /// PageRank stores a single double value per node representing the PageRank score.
    fn schema(&self, _config: &Self::Config) -> PregelSchema {
        PregelSchema::builder()
            .add("pagerank", ValueType::Double, Visibility::Public)
            .build()
    }

    /// Initialize node values
    ///
    /// For PageRank, we initialize:
    /// - Source nodes (if specified) to alpha = 1 - damping_factor
    /// - Non-source nodes to 0.0
    /// - If no source nodes specified, all nodes get alpha
    fn init(&mut self, context: &mut InitContext<Self::Config>) {
        let initial_value = if self.is_source_node(context.node_id()) {
            self.alpha
        } else {
            0.0
        };
        
        context.set_node_value("pagerank", initial_value);
    }

    /// Compute PageRank for a single node
    ///
    /// This implements the core PageRank algorithm:
    /// 1. Sum incoming messages (PageRank from neighbors)
    /// 2. Apply damping factor: new_rank = alpha + damping_factor * sum
    /// 3. Send outgoing messages: rank / out_degree to neighbors
    /// 4. Vote to halt if change is below tolerance
    fn compute<I: MessageIterator>(
        &mut self,
        context: &mut ComputeContext<Self::Config, I>,
        messages: &mut Messages<I>,
    ) {
        let current_rank = context.double_node_value("pagerank");
        let mut delta = current_rank;

        if !context.is_initial_superstep() {
            // Sum incoming messages (PageRank from neighbors)
            let mut sum = 0.0;
            for message in messages {
                sum += message;
            }
            
            // Apply damping factor: new_rank = alpha + damping_factor * sum
            delta = self.damping_factor * sum;
            let new_rank = self.alpha + delta;
            context.set_node_value("pagerank", new_rank);
        }

        // Send messages to neighbors if we have significant change
        if delta > self.tolerance || context.is_initial_superstep() {
            let out_degree = context.degree();
            if out_degree > 0 {
                // Send rank / out_degree to all neighbors
                let message_value = delta / out_degree as f64;
                context.send_to_neighbors(message_value);
            }
        } else {
            // Change is below tolerance, vote to halt
            context.vote_to_halt();
        }
    }

    /// Message reducer for PageRank
    ///
    /// PageRank uses sum reduction to combine multiple messages
    /// sent to the same target node.
    fn reducer(&self) -> Option<Box<dyn MessageReducer<f64>>> {
        Some(Box::new(SumReducer))
    }

    /// Apply relationship weights
    ///
    /// For weighted graphs, multiply the message value by the relationship weight.
    fn apply_relationship_weight(&self, node_value: f64, relationship_weight: f64) -> f64 {
        node_value * relationship_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_computation_creation() {
        let computation = PageRankPregelComputation::new(0.85, 1e-6, 100, None);
        
        assert_eq!(computation.damping_factor(), 0.85);
        assert_eq!(computation.tolerance(), 1e-6);
        assert_eq!(computation.max_iterations(), 100);
        assert!((computation.alpha - 0.15).abs() < 1e-10);
    }

    #[test]
    fn test_pagerank_computation_with_source_nodes() {
        let source_nodes = vec![0, 2, 4];
        let computation = PageRankPregelComputation::new(0.85, 1e-6, 100, Some(source_nodes));
        
        assert!(computation.is_source_node(0));
        assert!(!computation.is_source_node(1));
        assert!(computation.is_source_node(2));
        assert!(!computation.is_source_node(3));
        assert!(computation.is_source_node(4));
    }

    #[test]
    fn test_pagerank_computation_no_source_nodes() {
        let computation = PageRankPregelComputation::new(0.85, 1e-6, 100, None);
        
        // All nodes should be considered source nodes when none specified
        assert!(computation.is_source_node(0));
        assert!(computation.is_source_node(1));
        assert!(computation.is_source_node(999));
    }

    #[test]
    fn test_pagerank_schema() {
        let computation = PageRankPregelComputation::new(0.85, 1e-6, 100, None);
        let config = PregelConfig::default();
        let schema = computation.schema(&config);
        
        // Schema should have one property: "pagerank" of type Double
        assert_eq!(schema.elements().len(), 1);
        assert!(schema.elements().iter().any(|e| e.property_key == "pagerank"));
    }
}

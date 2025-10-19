//! PageRank Memory Estimation
//!
//! This module provides memory estimation for PageRank algorithm execution.

use crate::mem::{MemoryEstimation, MemoryRange, MemoryTree};
use crate::core::graph_dimensions::GraphDimensions;
use crate::pregel::{PregelSchema, Visibility};
use crate::types::ValueType;

/// Memory estimation for PageRank algorithm
///
/// This provides memory estimates for PageRank computation based on:
/// - Node count
/// - Graph structure (dense vs sparse)
/// - Message passing overhead
/// - Convergence tracking
///
/// ## Memory Components
///
/// 1. **Node Values**: Storage for PageRank scores (8 bytes per node)
/// 2. **Message Queues**: Temporary storage for messages between supersteps
/// 3. **Graph Structure**: Adjacency lists and relationship data
/// 4. **Convergence Tracking**: Previous iteration values for comparison
///
/// ## Estimation Formula
///
/// ```text
/// Memory = NodeValues + MessageQueues + ConvergenceTracking
/// NodeValues = node_count * 8 bytes
/// MessageQueues = node_count * avg_degree * 8 bytes * 2 (double buffering)
/// ConvergenceTracking = node_count * 8 bytes
/// ```
#[derive(Debug, Clone)]
pub struct PageRankMemoryEstimation {
    /// Number of nodes in the graph
    node_count: usize,
    /// Average degree (for message queue estimation)
    avg_degree: f64,
}

impl PageRankMemoryEstimation {
    /// Create a new PageRank memory estimation
    pub fn new(node_count: usize, avg_degree: f64) -> Self {
        Self {
            node_count,
            avg_degree,
        }
    }

    /// Estimate memory for node values storage
    ///
    /// PageRank stores one double (8 bytes) per node for the current score.
    pub fn estimate_node_values(&self) -> MemoryRange {
        let bytes_per_node = 8; // f64
        let total_bytes = self.node_count * bytes_per_node;
        MemoryRange::of(total_bytes)
    }

    /// Estimate memory for message queues
    ///
    /// PageRank uses double buffering for messages:
    /// - Current iteration messages
    /// - Next iteration messages
    /// - Each message is 8 bytes (f64)
    pub fn estimate_message_queues(&self) -> MemoryRange {
        let bytes_per_message = 8; // f64
        let messages_per_node = self.avg_degree.ceil() as usize;
        let total_messages = self.node_count * messages_per_node;
        let total_bytes = total_messages * bytes_per_message * 2; // Double buffering
        MemoryRange::of(total_bytes)
    }

    /// Estimate memory for convergence tracking
    ///
    /// PageRank tracks previous iteration values to detect convergence.
    pub fn estimate_convergence_tracking(&self) -> MemoryRange {
        let bytes_per_node = 8; // f64 for previous value
        let total_bytes = self.node_count * bytes_per_node;
        MemoryRange::of(total_bytes)
    }

    /// Estimate memory for graph structure overhead
    ///
    /// Additional memory for adjacency lists and relationship data.
    pub fn estimate_graph_overhead(&self) -> MemoryRange {
        // Rough estimate: 16 bytes per relationship (source + target + weight + overhead)
        let bytes_per_relationship = 16;
        let total_relationships = (self.node_count as f64 * self.avg_degree) as usize;
        let total_bytes = total_relationships * bytes_per_relationship;
        MemoryRange::of(total_bytes)
    }
}

impl MemoryEstimation for PageRankMemoryEstimation {
    /// Get a description of this memory estimation
    fn description(&self) -> String {
        format!("PageRank computation for {} nodes", self.node_count)
    }

    /// Estimate memory usage based on graph dimensions and concurrency
    fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        let node_values = self.estimate_node_values();
        let message_queues = self.estimate_message_queues();
        let convergence_tracking = self.estimate_convergence_tracking();
        let graph_overhead = self.estimate_graph_overhead();

        // Create a hierarchical memory tree
        MemoryTree::new(
            self.description(),
            node_values.add(&message_queues).add(&convergence_tracking).add(&graph_overhead),
            vec![
                MemoryTree::leaf("Node Values".to_string(), node_values),
                MemoryTree::leaf("Message Queues".to_string(), message_queues),
                MemoryTree::leaf("Convergence Tracking".to_string(), convergence_tracking),
                MemoryTree::leaf("Graph Overhead".to_string(), graph_overhead),
            ],
        )
    }
}

/// Create memory estimation from Pregel schema
///
/// This function creates a PageRank memory estimation based on the
/// Pregel schema and graph properties.
pub fn create_pagerank_memory_estimation(
    schema: &PregelSchema,
    node_count: usize,
    avg_degree: f64,
) -> PageRankMemoryEstimation {
    // Validate that this is a PageRank schema
    if let Some(pagerank_prop) = schema.elements().iter().find(|e| e.property_key == "pagerank") {
        assert_eq!(pagerank_prop.property_type, ValueType::Double);
        assert_eq!(pagerank_prop.visibility, Visibility::Public);
    } else {
        panic!("Schema must contain 'pagerank' property");
    }

    PageRankMemoryEstimation::new(node_count, avg_degree)
}

/// Estimate memory for PageRank with default parameters
///
/// This is a convenience function that creates a memory estimation
/// with reasonable defaults for typical PageRank usage.
pub fn estimate_pagerank_memory(node_count: usize) -> MemoryRange {
    // Assume average degree of 10 (typical for many real-world graphs)
    let avg_degree = 10.0;
    let estimation = PageRankMemoryEstimation::new(node_count, avg_degree);
    let dims = crate::core::graph_dimensions::ConcreteGraphDimensions::of(node_count, (node_count as f64 * avg_degree) as usize);
    estimation.estimate(&dims, 1).memory_usage().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregel::PregelSchemaBuilder;

    #[test]
    fn test_pagerank_memory_estimation_creation() {
        let estimation = PageRankMemoryEstimation::new(1000, 5.0);
        assert_eq!(estimation.node_count, 1000);
        assert_eq!(estimation.avg_degree, 5.0);
    }

    #[test]
    fn test_node_values_estimation() {
        let estimation = PageRankMemoryEstimation::new(1000, 5.0);
        let node_values = estimation.estimate_node_values();
        
        // 1000 nodes * 8 bytes = 8000 bytes
        assert_eq!(node_values.min(), 8000);
        assert_eq!(node_values.max(), 8000);
    }

    #[test]
    fn test_message_queues_estimation() {
        let estimation = PageRankMemoryEstimation::new(100, 3.0);
        let message_queues = estimation.estimate_message_queues();
        
        // 100 nodes * 3 messages * 8 bytes * 2 (double buffering) = 4800 bytes
        assert_eq!(message_queues.min(), 4800);
        assert_eq!(message_queues.max(), 4800);
    }

    #[test]
    fn test_convergence_tracking_estimation() {
        let estimation = PageRankMemoryEstimation::new(500, 2.0);
        let convergence = estimation.estimate_convergence_tracking();
        
        // 500 nodes * 8 bytes = 4000 bytes
        assert_eq!(convergence.min(), 4000);
        assert_eq!(convergence.max(), 4000);
    }

    #[test]
    fn test_total_memory_estimation() {
        let estimation = PageRankMemoryEstimation::new(100, 2.0);
        let dims = crate::core::graph_dimensions::ConcreteGraphDimensions::of(100, 200);
        let tree = estimation.estimate(&dims, 1);
        
        // Should be sum of all components
        assert!(tree.memory_usage().min() > 0);
        assert!(tree.memory_usage().max() > 0);
    }

    #[test]
    fn test_create_from_schema() {
        let schema = PregelSchemaBuilder::new()
            .add("pagerank", ValueType::Double, Visibility::Public)
            .build();
        
        let estimation = create_pagerank_memory_estimation(&schema, 1000, 5.0);
        assert_eq!(estimation.node_count, 1000);
        assert_eq!(estimation.avg_degree, 5.0);
    }

    #[test]
    fn test_estimate_with_defaults() {
        let memory = estimate_pagerank_memory(1000);
        assert!(memory.min() > 0);
        assert!(memory.max() > 0);
    }

    #[test]
    #[should_panic(expected = "Schema must contain 'pagerank' property")]
    fn test_invalid_schema_panics() {
        let schema = PregelSchemaBuilder::new()
            .add("other_property", ValueType::Double, Visibility::Public)
            .build();
        
        create_pagerank_memory_estimation(&schema, 1000, 5.0);
    }
}

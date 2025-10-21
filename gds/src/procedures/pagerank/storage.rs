//! Storage Runtime for PageRank
//!
//! This module implements the **Gross pole** of the Functor machinery for PageRank.
//! It represents persistent data structures (GraphStore and PropertyValues).

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::GraphStore;
use crate::types::graph::Graph;
use std::collections::HashSet;
use std::sync::Arc;

/// Storage Runtime for PageRank
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure and node properties:
/// - Node degrees (for message distribution)
/// - Relationship weights (if any)
/// - Graph topology (for neighbor iteration)
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and PropertyValues
/// - **Computation Runtime** (Subtle) = ephemeral PageRank scores and messages
/// - **Functor** = the mapping between them via message passing
pub struct PageRankStorageRuntime<'a, G: GraphStore> {
    /// Reference to the graph store
    graph_store: &'a G,
    /// Graph view for efficient access
    graph: Arc<dyn Graph>,
    /// Source nodes for personalized PageRank (if any)
    source_nodes: Option<HashSet<u64>>,
    /// Whether the graph has relationship weights
    has_relationship_weights: bool,
    /// Relationship weight property key (if any)
    weight_property: Option<String>,
}

impl<'a, G: GraphStore> PageRankStorageRuntime<'a, G> {
    /// Create a new PageRank storage runtime
    ///
    /// **Parameters**:
    /// - `graph_store`: The graph to access
    /// - `source_nodes`: Optional source nodes for personalized PageRank
    /// - `weight_property`: Optional relationship weight property
    pub fn new(
        graph_store: &'a G,
        source_nodes: Option<Vec<u64>>,
        weight_property: Option<String>,
    ) -> Result<Self, AlgorithmError> {
        let source_set = source_nodes.map(|nodes| nodes.into_iter().collect());
        let has_weights = weight_property.is_some();
        
        // Get graph view using the trait method
        let graph = graph_store.get_graph();
        
        Ok(Self {
            graph_store,
            graph,
            source_nodes: source_set,
            has_relationship_weights: has_weights,
            weight_property,
        })
    }
    
    /// Get the degree of a node
    ///
    /// This projects from GraphStore (Gross - persistent structure)
    /// to f64 (Subtle - computation value).
    ///
    /// **This is where the Functor machinery works**:
    /// GraphStore (Gross) → f64 (Subtle)
    pub fn get_node_degree(&self, node_id: u64) -> Result<f64, AlgorithmError> {
        // Convert external node ID to internal mapped ID
        let mapped_id = self.graph.to_mapped_node_id(node_id as i64);
        if mapped_id.is_none() {
            return Ok(0.0); // Node doesn't exist
        }
        
        // Get the degree from the graph
        let degree = self.graph.degree(mapped_id.unwrap());
        Ok(degree as f64)
    }
    
    /// Get the weight of a relationship
    ///
    /// Returns the weight of the relationship between source and target nodes.
    /// If no weight property is specified, returns 1.0.
    pub fn get_relationship_weight(&self, _source: u64, _target: u64) -> Result<f64, AlgorithmError> {
        if !self.has_relationship_weights {
            return Ok(1.0);
        }
        
        // TODO: Implement actual weight lookup from GraphStore
        // For now: placeholder implementation returns 1.0
        // This simulates the Functor: GraphStore → f64
        
        // In a real implementation, this would:
        // 1. Look up the relationship between source and target
        // 2. Get the weight property value
        // 3. Convert to f64
        // 4. Return the weight
        
        Ok(1.0)
    }
    
    /// Get all neighbors of a node
    ///
    /// Returns a vector of node IDs that are neighbors of the given node.
    pub fn get_neighbors(&self, node_id: u64) -> Result<Vec<u64>, AlgorithmError> {
        // Convert external node ID to internal mapped ID
        let mapped_id = self.graph.to_mapped_node_id(node_id as i64);
        if mapped_id.is_none() {
            return Ok(Vec::new()); // Node doesn't exist
        }
        
        // Get neighbors by streaming relationships
        let mut neighbors = Vec::new();
        let relationships = self.graph.stream_relationships(mapped_id.unwrap(), self.graph.default_property_value());
        
        for cursor in relationships {
            let target_mapped_id = cursor.target_id();
            let target_external_id = self.graph.to_original_node_id(target_mapped_id);
            if let Some(external_id) = target_external_id {
                neighbors.push(external_id as u64);
            }
        }
        
        Ok(neighbors)
    }
    
    /// Check if a node is a source node
    pub fn is_source_node(&self, node_id: u64) -> bool {
        self.source_nodes
            .as_ref()
            .map(|sources| sources.contains(&node_id))
            .unwrap_or(true) // If no source nodes specified, all nodes are sources
    }
    
    /// Get the number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.graph_store.node_count()
    }
    
    /// Get the number of relationships in the graph
    pub fn relationship_count(&self) -> usize {
        // TODO: Implement actual relationship count from GraphStore
        // For now: placeholder implementation
        0
    }
    
    /// Get reference to graph store
    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }
    
    /// Check if the graph has relationship weights
    pub fn has_relationship_weights(&self) -> bool {
        self.has_relationship_weights
    }
    
    /// Get the weight property key
    pub fn weight_property(&self) -> Option<&str> {
        self.weight_property.as_deref()
    }
}

#[cfg(test)]
mod tests {
    // Note: Full tests would require a mock GraphStore
    // For now, we just test creation
    
    #[test]
    fn test_storage_runtime_creation() {
        // This test would need a mock GraphStore
        // Skipping implementation for now
    }
    
    #[test]
    fn test_source_node_detection() {
        // This test would need a mock GraphStore
        // Skipping implementation for now
    }
}
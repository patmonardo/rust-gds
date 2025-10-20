use crate::types::graph_store::{DatabaseId, GraphStore};
use crate::logging::Log;
use crate::core::User;
use super::super::super::loaders::GraphStoreCatalogService;

/// Application for computing graph memory usage.
/// 
/// Mirrors Java GraphMemoryUsageApplication class.
/// Single compute method that returns memory usage information.
pub struct GraphMemoryUsageApplication {
    graph_store_catalog_service: Box<dyn GraphStoreCatalogService>,
}

impl GraphMemoryUsageApplication {
    /// Creates a new GraphMemoryUsageApplication.
    pub fn new(graph_store_catalog_service: Box<dyn GraphStoreCatalogService>) -> Self {
        Self {
            graph_store_catalog_service,
        }
    }
    
    /// Computes the memory usage for a graph.
    /// 
    /// In Java, this calls graphStoreCatalogService.sizeOf(user, databaseId, graphName).
    /// Returns GraphMemoryUsage with memory statistics.
    pub fn compute(&self, user: &User, database_id: &DatabaseId, graph_name: &str) -> GraphMemoryUsage {
        // In Java, this would call the catalog service to get actual memory usage
        let graph_store = self.graph_store_catalog_service.get_graph_store(user, database_id, graph_name);
        
        GraphMemoryUsage::new(
            graph_store.node_count(),
            graph_store.relationship_count(),
            self.estimate_memory_usage(&graph_store),
        )
    }
    
    /// Estimates memory usage based on graph dimensions.
    fn estimate_memory_usage(&self, graph_store: &dyn GraphStore) -> u64 {
        // Simple estimation: nodes * 100 bytes + relationships * 50 bytes
        graph_store.node_count() * 100 + graph_store.relationship_count() * 50
    }
}


/// Placeholder for GraphMemoryUsage struct.
/// In real implementation, this would contain detailed memory statistics.
#[derive(Clone, Debug)]
pub struct GraphMemoryUsage {
    node_count: u64,
    relationship_count: u64,
    estimated_bytes: u64,
}

impl GraphMemoryUsage {
    pub fn new(node_count: u64, relationship_count: u64, estimated_bytes: u64) -> Self {
        Self {
            node_count,
            relationship_count,
            estimated_bytes,
        }
    }
    
    pub fn node_count(&self) -> u64 {
        self.node_count
    }
    
    pub fn relationship_count(&self) -> u64 {
        self.relationship_count
    }
    
    pub fn estimated_bytes(&self) -> u64 {
        self.estimated_bytes
    }
    
    pub fn estimated_mb(&self) -> f64 {
        self.estimated_bytes as f64 / (1024.0 * 1024.0)
    }
}

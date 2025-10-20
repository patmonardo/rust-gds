use super::{GraphStoreCreator, GraphStoreLoader, GraphProjectConfig, GraphStore, ResultStore, GraphDimensions, MemoryEstimation};

/// Implementation of GraphStoreCreator for fictitious/testing scenarios.
/// 
/// Mirrors Java FictitiousGraphStoreLoader class.
/// Implementation for testing that doesn't actually load real data.
pub struct FictitiousGraphStoreLoader {
    graph_project_config: Box<dyn GraphProjectConfig>,
    graph_store: Box<dyn GraphStore>,
    result_store: Box<dyn ResultStore>,
    graph_dimensions: Box<dyn GraphDimensions>,
}

impl FictitiousGraphStoreLoader {
    /// Creates a new FictitiousGraphStoreLoader.
    pub fn new(graph_project_config: Box<dyn GraphProjectConfig>) -> Self {
        // Create fictitious implementations
        let graph_store = Box::new(FictitiousGraphStore::new());
        let result_store = Box::new(FictitiousResultStore::new());
        let graph_dimensions = Box::new(FictitiousGraphDimensions::new());
        
        Self {
            graph_project_config,
            graph_store,
            result_store,
            graph_dimensions,
        }
    }
}

impl GraphStoreLoader for FictitiousGraphStoreLoader {
    fn graph_project_config(&self) -> Box<dyn GraphProjectConfig> {
        // Clone the config - in real implementation would have proper cloning
        Box::new(FictitiousGraphProjectConfig::new(
            self.graph_project_config.graph_name().to_string(),
            self.graph_project_config.username().to_string(),
        ))
    }
    
    fn graph_store(&self) -> Box<dyn GraphStore> {
        Box::new(FictitiousGraphStore::new())
    }
    
    fn result_store(&self) -> Box<dyn ResultStore> {
        Box::new(FictitiousResultStore::new())
    }
    
    fn graph_dimensions(&self) -> Box<dyn GraphDimensions> {
        Box::new(FictitiousGraphDimensions::new())
    }
}

impl GraphStoreCreator for FictitiousGraphStoreLoader {
    fn estimate_memory_usage_during_loading(&self) -> Box<dyn MemoryEstimation> {
        Box::new(FictitiousMemoryEstimation::new())
    }
    
    fn estimate_memory_usage_after_loading(&self) -> Box<dyn MemoryEstimation> {
        Box::new(FictitiousMemoryEstimation::new())
    }
}

// Fictitious implementations for testing

#[derive(Clone, Debug)]
struct FictitiousGraphProjectConfig {
    graph_name: String,
    username: String,
}

impl FictitiousGraphProjectConfig {
    fn new(graph_name: String, username: String) -> Self {
        Self { graph_name, username }
    }
}

impl GraphProjectConfig for FictitiousGraphProjectConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }
    
    fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Clone, Debug)]
struct FictitiousGraphStore {
    node_count: u64,
    relationship_count: u64,
}

impl FictitiousGraphStore {
    fn new() -> Self {
        Self {
            node_count: 1000,
            relationship_count: 5000,
        }
    }
}

impl GraphStore for FictitiousGraphStore {
    fn node_count(&self) -> u64 {
        self.node_count
    }
    
    fn relationship_count(&self) -> u64 {
        self.relationship_count
    }
}

#[derive(Clone, Debug)]
struct FictitiousResultStore {
    is_empty: bool,
}

impl FictitiousResultStore {
    fn new() -> Self {
        Self { is_empty: true }
    }
}

impl ResultStore for FictitiousResultStore {
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

#[derive(Clone, Debug)]
struct FictitiousGraphDimensions {
    node_count: u64,
    relationship_count: u64,
}

impl FictitiousGraphDimensions {
    fn new() -> Self {
        Self {
            node_count: 1000,
            relationship_count: 5000,
        }
    }
}

impl GraphDimensions for FictitiousGraphDimensions {
    fn node_count(&self) -> u64 {
        self.node_count
    }
    
    fn relationship_count(&self) -> u64 {
        self.relationship_count
    }
}

#[derive(Clone, Debug)]
struct FictitiousMemoryEstimation {
    estimated_bytes: u64,
}

impl FictitiousMemoryEstimation {
    fn new() -> Self {
        Self {
            estimated_bytes: 1024 * 1024, // 1MB
        }
    }
}

impl MemoryEstimation for FictitiousMemoryEstimation {
    fn estimate(&self, _dimensions: &dyn GraphDimensions) -> u64 {
        self.estimated_bytes
    }
}

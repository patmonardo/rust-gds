//! Graph procedure stub interface for ML pipelines.
//!
//! This module provides the abstraction layer between ML pipelines and graph algorithms.
//! ML pipelines call graph procedures via this interface, but the actual implementations
//! live in their own executors (GDS Procs have their own runtime).
//!
//! **Key Design Principle:**
//! - ML pipelines don't know about graph algorithm internals
//! - Graph procedures return PropertyValues (opaque to ML)
//! - Registry pattern allows dynamic procedure lookup
//! - Stub pattern decouples ML from graph execution

use std::collections::HashMap;
use std::sync::Arc;

use crate::projection::codegen::ComputeError;
use crate::types::graph::Graph;
use crate::types::properties::PropertyValues;

/// Graph procedure interface - minimal contract for graph algorithms.
///
/// ML pipelines call procedures via this trait. The actual implementation
/// delegates to GDS Procs executor (separate runtime).
///
/// Maps to Java GDS ExecutableNodePropertyStep pattern.
pub trait GraphProcedure: Send + Sync {
    /// Execute the graph procedure and return computed property values.
    ///
    /// # Arguments
    /// * `graph` - Input graph (any type implementing Graph)
    /// * `config` - Algorithm configuration (procedure-specific)
    ///
    /// # Returns
    /// PropertyValues containing one value per node
    fn execute(
        &self,
        graph: &dyn Graph,
        config: &HashMap<String, String>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError>;

    /// Get procedure name (e.g., "pageRank", "fastRP", "louvain").
    fn name(&self) -> &str;

    /// Estimate memory requirements for this procedure.
    ///
    /// Returns estimated bytes needed to run on given graph.
    /// Used for resource planning in FormProcessor.
    fn estimate_memory(&self, graph: &dyn Graph) -> usize;

    /// Get algorithm category (e.g., "centrality", "embedding", "community").
    fn category(&self) -> &str {
        "general"
    }
}

/// Registry for graph procedures.
///
/// Allows ML pipelines to look up and execute procedures by name.
/// Procedures register themselves at startup or plugin load time.
///
/// In production, this would be populated by the GDS Procs system.
/// For Phase 2.3, we use mock implementations for testing.
#[derive(Default)]
pub struct GraphProcedureRegistry {
    procedures: HashMap<String, Arc<dyn GraphProcedure>>,
}

impl GraphProcedureRegistry {
    /// Create empty registry.
    pub fn new() -> Self {
        Self {
            procedures: HashMap::new(),
        }
    }

    /// Register a graph procedure.
    pub fn register(&mut self, procedure: Arc<dyn GraphProcedure>) {
        let name = procedure.name().to_string();
        self.procedures.insert(name, procedure);
    }

    /// Get procedure by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn GraphProcedure>> {
        self.procedures.get(name).cloned()
    }

    /// Check if procedure exists.
    pub fn contains(&self, name: &str) -> bool {
        self.procedures.contains_key(name)
    }

    /// List all registered procedure names.
    pub fn list_procedures(&self) -> Vec<String> {
        self.procedures.keys().cloned().collect()
    }

    /// Get number of registered procedures.
    pub fn len(&self) -> usize {
        self.procedures.len()
    }

    /// Check if registry is empty.
    pub fn is_empty(&self) -> bool {
        self.procedures.is_empty()
    }
}

// ============================================================================
// Mock Implementations (for Phase 2.3 testing)
// ============================================================================

/// Mock PageRank procedure (stub for testing).
///
/// In production, this would delegate to the real GDS PageRank executor.
/// For now, returns mock scores (0.15 for all nodes).
pub struct MockPageRankProcedure;

impl GraphProcedure for MockPageRankProcedure {
    fn execute(
        &self,
        graph: &dyn Graph,
        _config: &HashMap<String, String>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        use crate::projection::native::form::mock_property_values::MockLongPropertyValues;

        let node_count = graph.node_count() as usize;

        // Mock PageRank scores (deterministic generation)
        // In production, this would call GDS PageRank executor
        Ok(Arc::new(MockLongPropertyValues::new(node_count)))
    }
    fn name(&self) -> &str {
        "pageRank"
    }

    fn estimate_memory(&self, graph: &dyn Graph) -> usize {
        // Rough estimate: node_count * (score + adjacency)
        let node_count = graph.node_count() as usize;
        let edge_count = graph.relationship_count() as usize;
        node_count * 8 + edge_count * 16 // 8 bytes per node, 16 per edge
    }

    fn category(&self) -> &str {
        "centrality"
    }
}

/// Mock FastRP procedure (stub for testing).
///
/// In production, this would delegate to the real GDS FastRP executor.
/// For now, returns mock embeddings (zeros).
pub struct MockFastRPProcedure {
    embedding_dimension: usize,
}

impl MockFastRPProcedure {
    pub fn new(embedding_dimension: usize) -> Self {
        Self {
            embedding_dimension,
        }
    }
}

impl GraphProcedure for MockFastRPProcedure {
    fn execute(
        &self,
        graph: &dyn Graph,
        _config: &HashMap<String, String>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        use crate::projection::native::form::mock_property_values::MockEmbeddingPropertyValues;

        let node_count = graph.node_count() as usize;

        // Mock embeddings (deterministic generation)
        // In production, this would call GDS FastRP executor
        Ok(Arc::new(MockEmbeddingPropertyValues::new(
            node_count,
            self.embedding_dimension,
        )))
    }
    fn name(&self) -> &str {
        "fastRP"
    }

    fn estimate_memory(&self, graph: &dyn Graph) -> usize {
        // Embedding memory: node_count * embedding_dim * 8 bytes
        let node_count = graph.node_count() as usize;
        node_count * self.embedding_dimension * 8
    }

    fn category(&self) -> &str {
        "embedding"
    }
}

/// Mock Louvain procedure (stub for testing).
///
/// In production, this would delegate to the real GDS Louvain executor.
/// For now, returns mock community IDs (all nodes in community 0).
pub struct MockLouvainProcedure;

impl GraphProcedure for MockLouvainProcedure {
    fn execute(
        &self,
        graph: &dyn Graph,
        _config: &HashMap<String, String>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        use crate::projection::native::form::mock_property_values::MockLongPropertyValues;

        let node_count = graph.node_count() as usize;

        // Mock community IDs (deterministic generation)
        // In production, this would call GDS Louvain executor
        Ok(Arc::new(MockLongPropertyValues::new(node_count)))
    }
    fn name(&self) -> &str {
        "louvain"
    }

    fn estimate_memory(&self, graph: &dyn Graph) -> usize {
        // Community detection: node_count * 8 + edge_count * 8
        let node_count = graph.node_count() as usize;
        let edge_count = graph.relationship_count() as usize;
        node_count * 8 + edge_count * 8
    }

    fn category(&self) -> &str {
        "community"
    }
}

/// Create a default registry with common mock procedures.
///
/// Useful for testing and Phase 2.3 validation.
/// In production, real procedures would register themselves.
pub fn create_mock_registry() -> GraphProcedureRegistry {
    let mut registry = GraphProcedureRegistry::new();

    registry.register(Arc::new(MockPageRankProcedure));
    registry.register(Arc::new(MockFastRPProcedure::new(128)));
    registry.register(Arc::new(MockLouvainProcedure));

    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::random_graph::RandomGraphConfig;

    #[test]
    fn test_registry_creation() {
        let registry = GraphProcedureRegistry::new();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = GraphProcedureRegistry::new();

        let procedure = Arc::new(MockPageRankProcedure);
        registry.register(procedure.clone());

        assert_eq!(registry.len(), 1);
        assert!(registry.contains("pageRank"));

        let retrieved = registry.get("pageRank");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "pageRank");
    }

    #[test]
    fn test_registry_list_procedures() {
        let registry = create_mock_registry();

        let procedures = registry.list_procedures();
        assert_eq!(procedures.len(), 3);
        assert!(procedures.contains(&"pageRank".to_string()));
        assert!(procedures.contains(&"fastRP".to_string()));
        assert!(procedures.contains(&"louvain".to_string()));
    }

    #[test]
    fn test_mock_pagerank_execution() {
        let config = RandomGraphConfig::default().with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("Failed to create graph");
        let graph = store.graph();
        let procedure = MockPageRankProcedure;

        let config = HashMap::new();
        let result = procedure.execute(graph.as_ref(), &config);
        assert!(result.is_ok());
        // PropertyValues don't have len(), just verify it succeeded
    }
    #[test]
    fn test_mock_fastrp_execution() {
        let config = RandomGraphConfig::default().with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("Failed to create graph");
        let graph = store.graph();
        let procedure = MockFastRPProcedure::new(128);

        let config = HashMap::new();
        let result = procedure.execute(graph.as_ref(), &config);
        assert!(result.is_ok());
        // PropertyValues don't have len(), just verify it succeeded
    }
    #[test]
    fn test_mock_louvain_execution() {
        let config = RandomGraphConfig::default().with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("Failed to create graph");
        let graph = store.graph();
        let procedure = MockLouvainProcedure;

        let config = HashMap::new();
        let result = procedure.execute(graph.as_ref(), &config);
        assert!(result.is_ok());
        // PropertyValues don't have len(), just verify it succeeded
    }
    #[test]
    fn test_procedure_categories() {
        let pagerank = MockPageRankProcedure;
        let fastrp = MockFastRPProcedure::new(64);
        let louvain = MockLouvainProcedure;

        assert_eq!(pagerank.category(), "centrality");
        assert_eq!(fastrp.category(), "embedding");
        assert_eq!(louvain.category(), "community");
    }

    #[test]
    fn test_memory_estimation() {
        let config = RandomGraphConfig::default().with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("Failed to create graph");
        let graph = store.graph();

        let pagerank = MockPageRankProcedure;
        let memory = pagerank.estimate_memory(graph.as_ref()); // Should be reasonable estimate (> 0, < 1GB for small graph)
        assert!(memory > 0);
        assert!(memory < 1_000_000_000);
    }
    #[test]
    fn test_registry_get_missing_procedure() {
        let registry = create_mock_registry();

        let result = registry.get("nonexistent");
        assert!(result.is_none());
        assert!(!registry.contains("nonexistent"));
    }

    #[test]
    fn test_create_mock_registry() {
        let registry = create_mock_registry();

        assert_eq!(registry.len(), 3);
        assert!(registry.contains("pageRank"));
        assert!(registry.contains("fastRP"));
        assert!(registry.contains("louvain"));
    }
}

use crate::types::graph_store::{DatabaseId, GraphStore};
use crate::logging::Log;
use crate::core::User;
use super::super::super::loaders::GraphStoreCatalogService;
use super::super::super::facade::GraphStoreCatalogEntry;

/// Application for dropping graphs from the catalog.
/// 
/// Mirrors Java DropGraphApplication class.
/// Contains graph dropping logic with validation and error handling.
pub struct DropGraphApplication {
    graph_store_catalog_service: Box<dyn GraphStoreCatalogService>,
}

impl DropGraphApplication {
    /// Creates a new DropGraphApplication.
    pub fn new(graph_store_catalog_service: Box<dyn GraphStoreCatalogService>) -> Self {
        Self {
            graph_store_catalog_service,
        }
    }
    
    /// Computes the drop operation for multiple graphs.
    /// 
    /// In Java, this handles both single graphs and lists of graphs.
    /// Returns metadata for the graphs that were removed.
    pub fn compute(
        &self,
        graph_names: &[String],
        should_fail_if_missing: bool,
        database_id: &DatabaseId,
        operator: &User,
        username_override: Option<&str>,
    ) -> Result<Vec<GraphStoreCatalogEntry>, String> {
        let request = CatalogRequest::new(
            database_id.database_name().to_string(),
            operator.username().to_string(),
            username_override.map(|s| s.to_string()),
            operator.is_admin(),
        );
        
        if should_fail_if_missing {
            self.validate_graphs_exist(&request, graph_names)?;
        }
        
        self.drop_graphs(&request, graph_names, should_fail_if_missing)
    }
    
    /// Validates that all graphs exist before attempting to drop them.
    fn validate_graphs_exist(&self, request: &CatalogRequest, graph_names: &[String]) -> Result<(), String> {
        let mut missing_graphs = Vec::new();
        
        for graph_name in graph_names {
            if !self.graph_store_catalog_service.graph_exists(request, graph_name) {
                missing_graphs.push(graph_name.clone());
            }
        }
        
        if !missing_graphs.is_empty() {
            let message = if missing_graphs.len() == 1 {
                format!("Graph '{}' does not exist", missing_graphs[0])
            } else {
                format!("Graphs {} do not exist", missing_graphs.join(", "))
            };
            return Err(message);
        }
        
        Ok(())
    }
    
    /// Drops the specified graphs from the catalog.
    fn drop_graphs(
        &self,
        request: &CatalogRequest,
        graph_names: &[String],
        should_fail_if_missing: bool,
    ) -> Result<Vec<GraphStoreCatalogEntry>, String> {
        let mut results = Vec::new();
        
        for graph_name in graph_names {
            if let Some(entry) = self.graph_store_catalog_service.remove_graph(request, graph_name, should_fail_if_missing) {
                results.push(entry);
            }
        }
        
        Ok(results)
    }
}

/// Placeholder for CatalogRequest struct.
/// In real implementation, this would contain catalog request information.
#[derive(Clone, Debug)]
pub struct CatalogRequest {
    database_name: String,
    username: String,
    username_override: Option<String>,
    is_admin: bool,
}

impl CatalogRequest {
    pub fn new(database_name: String, username: String, username_override: Option<String>, is_admin: bool) -> Self {
        Self {
            database_name,
            username,
            username_override,
            is_admin,
        }
    }
    
    pub fn database_name(&self) -> &str {
        &self.database_name
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn username_override(&self) -> Option<&str> {
        self.username_override.as_deref()
    }
    
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}

/// Extended GraphStoreCatalogService with additional methods.
pub trait ExtendedGraphStoreCatalogService: GraphStoreCatalogService {
    fn graph_exists(&self, request: &CatalogRequest, graph_name: &str) -> bool;
    fn remove_graph(&self, request: &CatalogRequest, graph_name: &str, fail_if_missing: bool) -> Option<GraphStoreCatalogEntry>;
}

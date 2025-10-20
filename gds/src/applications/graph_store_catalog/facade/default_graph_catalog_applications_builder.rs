use super::*;
use crate::logging::Log;
use crate::types::graph_store::{DatabaseId, GraphStore};

/// Builder for DefaultGraphCatalogApplications.
/// 
/// Mirrors Java DefaultGraphCatalogApplicationsBuilder class.
/// Implements the builder pattern for constructing the facade with all dependencies.
pub struct DefaultGraphCatalogApplicationsBuilder {
    pub log: Log,
    pub graph_store_catalog_service: Box<dyn GraphStoreCatalogService>,
    pub graph_memory_usage_application: GraphMemoryUsageApplication,
    pub drop_graph_application: DropGraphApplication,
    pub drop_node_properties_application: DropNodePropertiesApplication,
    pub drop_relationships_application: DropRelationshipsApplication,
    pub stream_node_properties_application: StreamNodePropertiesApplication,
    pub stream_relationship_properties_application: StreamRelationshipPropertiesApplication,
    pub stream_relationships_application: StreamRelationshipsApplication,
    pub write_node_properties_application: WriteNodePropertiesApplication,
    pub write_node_label_application: WriteNodeLabelApplication,
    pub write_relationship_properties_application: WriteRelationshipPropertiesApplication,
    pub write_relationships_application: WriteRelationshipsApplication,
    pub export_to_csv_application: ExportToCsvApplication,
    pub export_to_database_application: ExportToDatabaseApplication,
    pub native_project_application: NativeProjectApplication,
    pub generic_project_application: GenericProjectApplication,
    pub generate_graph_application: GenerateGraphApplication,
    pub graph_sampling_application: GraphSamplingApplication,
    pub task_registry_factory: TaskRegistryFactory,
    pub user_log_registry_factory: UserLogRegistryFactory,
}

impl DefaultGraphCatalogApplicationsBuilder {
    /// Creates a new builder with default values.
    pub fn new(log: Log) -> Self {
        Self {
            log: log.clone(),
            graph_store_catalog_service: Box::new(DefaultGraphStoreCatalogService::new()),
            graph_memory_usage_application: GraphMemoryUsageApplication::new(Box::new(DefaultGraphStoreCatalogService::new())),
            drop_graph_application: DropGraphApplication::new(Box::new(DefaultGraphStoreCatalogService::new())),
            drop_node_properties_application: DropNodePropertiesApplication::new(log.clone()),
            drop_relationships_application: DropRelationshipsApplication::new(log.clone()),
            stream_node_properties_application: StreamNodePropertiesApplication,
            stream_relationship_properties_application: StreamRelationshipPropertiesApplication,
            stream_relationships_application: StreamRelationshipsApplication,
            write_node_properties_application: WriteNodePropertiesApplication,
            write_node_label_application: WriteNodeLabelApplication,
            write_relationship_properties_application: WriteRelationshipPropertiesApplication,
            write_relationships_application: WriteRelationshipsApplication,
            export_to_csv_application: ExportToCsvApplication,
            export_to_database_application: ExportToDatabaseApplication,
            native_project_application: NativeProjectApplication,
            generic_project_application: GenericProjectApplication,
            generate_graph_application: GenerateGraphApplication,
            graph_sampling_application: GraphSamplingApplication,
            task_registry_factory: TaskRegistryFactory::new(),
            user_log_registry_factory: UserLogRegistryFactory::new(),
        }
    }
    
    /// Sets the graph store catalog service.
    pub fn with_graph_store_catalog_service(mut self, service: Box<dyn GraphStoreCatalogService>) -> Self {
        self.graph_store_catalog_service = service;
        self
    }
    
    /// Sets the task registry factory.
    pub fn with_task_registry_factory(mut self, factory: TaskRegistryFactory) -> Self {
        self.task_registry_factory = factory;
        self
    }
    
    /// Sets the user log registry factory.
    pub fn with_user_log_registry_factory(mut self, factory: UserLogRegistryFactory) -> Self {
        self.user_log_registry_factory = factory;
        self
    }
    
    /// Builds the DefaultGraphCatalogApplications.
    pub fn build(self) -> DefaultGraphCatalogApplications {
        DefaultGraphCatalogApplications::new(self)
    }
}

/// Default implementation of GraphStoreCatalogService for the builder.
#[derive(Clone, Debug)]
struct DefaultGraphStoreCatalogService;

impl DefaultGraphStoreCatalogService {
    fn new() -> Self {
        Self
    }
}

impl GraphStoreCatalogService for DefaultGraphStoreCatalogService {
    fn get_graph_store(&self, _user: &User, _database_id: &DatabaseId, _graph_name: &str) -> Box<dyn GraphStore> {
        Box::new(DefaultGraphStore::new())
    }
}

/// Default implementation of GraphStore for the builder.
#[derive(Clone, Debug)]
struct DefaultGraphStore {
    node_count: u64,
    relationship_count: u64,
}

impl DefaultGraphStore {
    fn new() -> Self {
        Self {
            node_count: 1000,
            relationship_count: 5000,
        }
    }
}

impl GraphStore for DefaultGraphStore {
    fn node_count(&self) -> u64 {
        self.node_count
    }
    
    fn relationship_count(&self) -> u64 {
        self.relationship_count
    }
}

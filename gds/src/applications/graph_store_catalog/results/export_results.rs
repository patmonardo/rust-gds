// Export result types - direct translation from Java

/// Base result for graph store export operations.
/// Mirrors Java GraphStoreExportResult abstract class.
#[derive(Clone, Debug)]
pub struct GraphStoreExportResult {
    pub graph_name: String,
    pub node_count: u64,
    pub relationship_count: u64,
    pub relationship_type_count: u64,
    pub node_property_count: u64,
    pub relationship_property_count: u64,
    pub write_millis: u64,
}

impl GraphStoreExportResult {
    pub fn new(
        graph_name: String,
        node_count: u64,
        relationship_count: u64,
        relationship_type_count: u64,
        node_property_count: u64,
        relationship_property_count: u64,
        write_millis: u64,
    ) -> Self {
        Self {
            graph_name,
            node_count,
            relationship_count,
            relationship_type_count,
            node_property_count,
            relationship_property_count,
            write_millis,
        }
    }
}

/// Result for database export operations.
/// Mirrors Java DatabaseExportResult class.
#[derive(Clone, Debug)]
pub struct DatabaseExportResult {
    pub base: GraphStoreExportResult,
    pub db_name: String,
}

impl DatabaseExportResult {
    pub fn new(
        graph_name: String,
        db_name: String,
        node_count: u64,
        relationship_count: u64,
        relationship_type_count: u64,
        node_property_count: u64,
        relationship_property_count: u64,
        write_millis: u64,
    ) -> Self {
        Self {
            base: GraphStoreExportResult::new(
                graph_name,
                node_count,
                relationship_count,
                relationship_type_count,
                node_property_count,
                relationship_property_count,
                write_millis,
            ),
            db_name,
        }
    }
}

/// Result for file export operations.
/// Mirrors Java FileExportResult class.
#[derive(Clone, Debug)]
pub struct FileExportResult {
    pub base: GraphStoreExportResult,
    pub export_name: String,
}

impl FileExportResult {
    pub fn new(
        graph_name: String,
        export_name: String,
        node_count: u64,
        relationship_count: u64,
        relationship_type_count: u64,
        node_property_count: u64,
        relationship_property_count: u64,
        write_millis: u64,
    ) -> Self {
        Self {
            base: GraphStoreExportResult::new(
                graph_name,
                node_count,
                relationship_count,
                relationship_type_count,
                node_property_count,
                relationship_property_count,
                write_millis,
            ),
            export_name,
        }
    }
}
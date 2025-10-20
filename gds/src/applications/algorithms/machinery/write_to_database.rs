use crate::api::Graph;
use crate::api::GraphStore;
use crate::api::ResultStore;
use crate::api::properties::nodes::NodePropertyValues;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::applications::algorithms::machinery::AlgorithmLabel;
use crate::config::base_types::Config;
use crate::core::utils::progress::JobId;

/// Interface for writing algorithm results to the database.
/// This is a core pattern in the Applications system for handling
/// algorithm results that need to be persisted to the database.
pub trait WriteToDatabase {
    /// Writes algorithm results to the database.
    /// 
    /// # Arguments
    /// * `graph` - The graph that was processed
    /// * `graph_store` - The graph store containing the graph
    /// * `result_store` - The result store for caching results
    /// * `config` - The algorithm configuration
    /// * `algorithm_label` - The algorithm label for tracking
    /// * `job_id` - The job ID for tracking progress
    /// * `node_properties` - The node properties to write
    /// 
    /// # Returns
    /// Metadata about what was written
    fn perform<C: Config>(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        config: &C,
        algorithm_label: AlgorithmLabel,
        job_id: JobId,
        node_properties: Box<dyn NodePropertyValues>,
    ) -> NodePropertiesWritten;

    /// Writes algorithm results to the database with explicit configuration.
    /// 
    /// # Arguments
    /// * `graph` - The graph that was processed
    /// * `graph_store` - The graph store containing the graph
    /// * `result_store` - The result store for caching results
    /// * `config` - The algorithm configuration
    /// * `write_config` - The write configuration
    /// * `algorithm_label` - The algorithm label for tracking
    /// * `job_id` - The job ID for tracking progress
    /// * `node_properties` - The node properties to write
    /// 
    /// # Returns
    /// Metadata about what was written
    fn perform_with_config<C: Config, W: Config>(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        config: &C,
        write_config: &W,
        algorithm_label: AlgorithmLabel,
        job_id: JobId,
        node_properties: Box<dyn NodePropertyValues>,
    ) -> NodePropertiesWritten;
}

/// Default implementation of WriteToDatabase.
#[derive(Clone)]
pub struct DefaultWriteToDatabase;

impl DefaultWriteToDatabase {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultWriteToDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteToDatabase for DefaultWriteToDatabase {
    fn perform<C: Config>(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        config: &C,
        algorithm_label: AlgorithmLabel,
        job_id: JobId,
        node_properties: Box<dyn NodePropertyValues>,
    ) -> NodePropertiesWritten {
        // TODO: Implement actual database writing
        // This would typically involve:
        // 1. Getting the target node labels from the config
        // 2. Getting the property name from the config
        // 3. Writing the properties to the database
        // 4. Updating the result store
        // 5. Returning metadata about what was written
        
        // For now, return a placeholder
        NodePropertiesWritten::new(graph.node_count())
    }

    fn perform_with_config<C: Config, W: Config>(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        config: &C,
        write_config: &W,
        algorithm_label: AlgorithmLabel,
        job_id: JobId,
        node_properties: Box<dyn NodePropertyValues>,
    ) -> NodePropertiesWritten {
        // TODO: Implement actual database writing with explicit config
        // This would typically involve:
        // 1. Using the write config for target node labels and property names
        // 2. Writing the properties to the database
        // 3. Updating the result store
        // 4. Returning metadata about what was written
        
        // For now, return a placeholder
        NodePropertiesWritten::new(graph.node_count())
    }
}

use crate::api::{Graph, GraphStore, ResultStore};
use crate::applications::algorithms::machinery::{WriteStep, WriteToDatabase, AlgorithmLabel};
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::core::utils::progress::JobId;
use crate::config::base_types::Config;

/// Scale Properties write step implementation
pub struct ScalePropertiesWriteStep<C: Config> {
    write_to_database: Box<dyn WriteToDatabase>,
    configuration: C,
}

impl<C: Config> ScalePropertiesWriteStep<C> {
    pub fn new(write_to_database: Box<dyn WriteToDatabase>, configuration: C) -> Self {
        Self {
            write_to_database,
            configuration,
        }
    }
}

impl<C: Config> WriteStep<crate::applications::algorithms::miscellaneous::ScalePropertiesResult, NodePropertiesWritten> for ScalePropertiesWriteStep<C> {
    fn execute(
        &self,
        graph: Graph,
        graph_store: &GraphStore,
        result_store: &ResultStore,
        result: crate::applications::algorithms::miscellaneous::ScalePropertiesResult,
        job_id: JobId,
    ) -> NodePropertiesWritten {
        // Create scaled properties node property values
        let node_property_values = ScaledPropertiesNodePropertyValues::new(
            graph.node_count(),
            result.scaled_properties()
        );

        self.write_to_database.perform(
            graph,
            graph_store,
            result_store,
            &self.configuration,
            &self.configuration,
            AlgorithmLabel::ScaleProperties,
            job_id,
            node_property_values
        )
    }
}

// Placeholder for scaled properties node property values
pub struct ScaledPropertiesNodePropertyValues {
    _node_count: u64,
    _scaled_properties: Vec<f64>,
}

impl ScaledPropertiesNodePropertyValues {
    pub fn new(node_count: u64, scaled_properties: Vec<f64>) -> Self {
        Self {
            _node_count: node_count,
            _scaled_properties: scaled_properties,
        }
    }
}

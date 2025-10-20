use super::GraphAccessGraphPropertiesConfig;

/// Configuration for streaming graph properties from the graph store.
/// 
/// Mirrors Java GraphStreamGraphPropertiesConfig interface.
/// Extends GraphAccessGraphPropertiesConfig with a factory method for construction.
pub trait GraphStreamGraphPropertiesConfig: GraphAccessGraphPropertiesConfig {
    // Factory method would be implemented by concrete types
}

/// Builder for creating GraphStreamGraphPropertiesConfig implementations.
/// 
/// In Java, this uses CypherMapWrapper for configuration parsing.
/// For now, this is a simple struct that can be extended as needed.
#[derive(Clone, Debug)]
pub struct GraphStreamGraphPropertiesConfigImpl {
    graph_name: Option<String>,
    graph_property: String,
}

impl GraphStreamGraphPropertiesConfigImpl {
    /// Creates a new GraphStreamGraphPropertiesConfig.
    pub fn new(graph_name: String, graph_property: String) -> Self {
        Self {
            graph_name: Some(graph_name),
            graph_property,
        }
    }
}

impl GraphAccessGraphPropertiesConfig for GraphStreamGraphPropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn graph_property(&self) -> String {
        self.graph_property.clone()
    }
}

impl GraphStreamGraphPropertiesConfig for GraphStreamGraphPropertiesConfigImpl {}


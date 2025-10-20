use super::{GraphNodePropertiesConfig, GraphExportNodePropertiesConfig};

/// Configuration for streaming node properties from the graph store.
/// 
/// Mirrors Java GraphStreamNodePropertiesConfig interface.
/// Extends GraphExportNodePropertiesConfig with a factory method.
pub trait GraphStreamNodePropertiesConfig: GraphExportNodePropertiesConfig {
    // Factory method would be implemented by concrete types
}

/// Builder for creating GraphStreamNodePropertiesConfig implementations.
/// 
/// In Java, this uses CypherMapWrapper for configuration parsing.
#[derive(Clone, Debug)]
pub struct GraphStreamNodePropertiesConfigImpl {
    graph_name: Option<String>,
    node_labels: Vec<String>,
    node_properties: Vec<String>,
    list_node_labels: bool,
}

impl GraphStreamNodePropertiesConfigImpl {
    /// Creates a new GraphStreamNodePropertiesConfig.
    pub fn new(
        graph_name: Option<String>,
        node_labels: Vec<String>,
        node_properties: Vec<String>,
        list_node_labels: bool,
    ) -> Self {
        Self {
            graph_name,
            node_labels,
            node_properties,
            list_node_labels,
        }
    }
}

impl GraphNodePropertiesConfig for GraphStreamNodePropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn node_labels(&self) -> Vec<String> {
        self.node_labels.clone()
    }
}

impl GraphExportNodePropertiesConfig for GraphStreamNodePropertiesConfigImpl {
    fn node_properties(&self) -> Vec<String> {
        self.node_properties.clone()
    }
    
    fn list_node_labels(&self) -> bool {
        self.list_node_labels
    }
}

impl GraphStreamNodePropertiesConfig for GraphStreamNodePropertiesConfigImpl {}

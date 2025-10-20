use super::{GraphNodePropertiesConfig, GraphExportNodePropertiesConfig};

/// Configuration for writing node properties to the database.
/// 
/// Mirrors Java GraphWriteNodePropertiesConfig interface.
/// Extends GraphNodePropertiesConfig + WriteConfig + JobIdConfig for write operations.
pub trait GraphWriteNodePropertiesConfig: GraphNodePropertiesConfig {
    /// Returns the list of node property specifications for writing.
    /// In Java, this returns List<UserInputWriteProperties.PropertySpec>.
    fn node_properties(&self) -> Vec<NodePropertySpec>;
    
    /// Parses node properties from user input.
    /// In Java, this uses UserInputWriteProperties.parse().
    fn parse_node_properties(user_input: &str) -> Vec<NodePropertySpec> {
        vec![NodePropertySpec::new(user_input.to_string(), None)]
    }
}

/// Specification for a node property to be written.
/// 
/// Mirrors Java UserInputWriteProperties.PropertySpec.
#[derive(Clone, Debug)]
pub struct NodePropertySpec {
    node_property_name: String,
    renamed_node_property: Option<String>,
}

impl NodePropertySpec {
    /// Creates a new NodePropertySpec.
    pub fn new(node_property_name: String, renamed_node_property: Option<String>) -> Self {
        Self {
            node_property_name,
            renamed_node_property,
        }
    }
    
    /// Returns the property name to write to the database.
    pub fn write_property(&self) -> String {
        self.renamed_node_property.clone().unwrap_or_else(|| self.node_property_name.clone())
    }
    
    /// Returns the original node property name.
    pub fn node_property(&self) -> String {
        self.node_property_name.clone()
    }
}

/// Builder for creating GraphWriteNodePropertiesConfig implementations.
#[derive(Clone, Debug)]
pub struct GraphWriteNodePropertiesConfigImpl {
    graph_name: Option<String>,
    node_labels: Vec<String>,
    node_properties: Vec<NodePropertySpec>,
}

impl GraphWriteNodePropertiesConfigImpl {
    /// Creates a new GraphWriteNodePropertiesConfig.
    pub fn new(
        graph_name: Option<String>,
        node_labels: Vec<String>,
        node_properties: Vec<NodePropertySpec>,
    ) -> Self {
        Self {
            graph_name,
            node_labels,
            node_properties,
        }
    }
}

impl GraphNodePropertiesConfig for GraphWriteNodePropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn node_labels(&self) -> Vec<String> {
        self.node_labels.clone()
    }
}

impl GraphWriteNodePropertiesConfig for GraphWriteNodePropertiesConfigImpl {
    fn node_properties(&self) -> Vec<NodePropertySpec> {
        self.node_properties.clone()
    }
}

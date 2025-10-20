use super::GraphNodePropertiesConfig;

/// Configuration for exporting node properties from the graph store.
/// 
/// Mirrors Java GraphExportNodePropertiesConfig interface.
/// Extends GraphNodePropertiesConfig with validation and additional methods.
pub trait GraphExportNodePropertiesConfig: GraphNodePropertiesConfig {
    /// Returns the list of node properties to export.
    fn node_properties(&self) -> Vec<String>;
    
    /// Returns whether to list node labels in the output.
    /// Defaults to false.
    fn list_node_labels(&self) -> bool {
        false
    }
    
    /// Parses node properties from user input.
    /// In Java, this uses UserInputAsStringOrListOfString.parse().
    fn parse_node_properties(user_input: &str) -> Vec<String> {
        vec![user_input.to_string()]
    }
    
    /// Validates that the specified node properties exist for the given labels.
    /// In Java, this has complex validation logic checking GraphStore.
    fn validate(&self) -> Result<(), String> {
        // Placeholder validation - in real implementation would check GraphStore
        Ok(())
    }
}

/// Builder for creating GraphExportNodePropertiesConfig implementations.
#[derive(Clone, Debug)]
pub struct GraphExportNodePropertiesConfigImpl {
    graph_name: Option<String>,
    node_labels: Vec<String>,
    node_properties: Vec<String>,
    list_node_labels: bool,
}

impl GraphExportNodePropertiesConfigImpl {
    /// Creates a new GraphExportNodePropertiesConfig.
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

impl GraphNodePropertiesConfig for GraphExportNodePropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn node_labels(&self) -> Vec<String> {
        self.node_labels.clone()
    }
}

impl GraphExportNodePropertiesConfig for GraphExportNodePropertiesConfigImpl {
    fn node_properties(&self) -> Vec<String> {
        self.node_properties.clone()
    }
    
    fn list_node_labels(&self) -> bool {
        self.list_node_labels
    }
}

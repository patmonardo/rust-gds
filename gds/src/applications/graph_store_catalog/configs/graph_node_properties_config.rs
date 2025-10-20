/// Configuration for accessing node properties from the graph store.
/// 
/// Mirrors Java GraphNodePropertiesConfig interface.
/// Base trait with default methods and parsing logic for node labels.
pub trait GraphNodePropertiesConfig {
    /// Returns the optional graph name.
    fn graph_name(&self) -> Option<String>;
    
    /// Returns the list of node labels to process.
    /// Defaults to ["*"] (all labels) if not specified.
    fn node_labels(&self) -> Vec<String> {
        vec!["*".to_string()]
    }
    
    /// Parses node labels from user input.
    /// In Java, this uses UserInputAsStringOrListOfString.parse().
    fn parse_node_labels(user_input: &str) -> Vec<String> {
        // Simple parsing - in real implementation would handle lists
        vec![user_input.to_string()]
    }
}

/// Builder for creating GraphNodePropertiesConfig implementations.
#[derive(Clone, Debug)]
pub struct GraphNodePropertiesConfigImpl {
    graph_name: Option<String>,
    node_labels: Vec<String>,
}

impl GraphNodePropertiesConfigImpl {
    /// Creates a new GraphNodePropertiesConfig.
    pub fn new(graph_name: Option<String>, node_labels: Vec<String>) -> Self {
        Self {
            graph_name,
            node_labels,
        }
    }
}

impl GraphNodePropertiesConfig for GraphNodePropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn node_labels(&self) -> Vec<String> {
        self.node_labels.clone()
    }
}

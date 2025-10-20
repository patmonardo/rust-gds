/// Configuration for dropping node properties from the graph store.
/// 
/// Mirrors Java GraphDropNodePropertiesConfig interface.
/// Extends BaseConfig + ConcurrencyConfig for dropping operations.
pub trait GraphDropNodePropertiesConfig {
    /// Returns the optional graph name.
    fn graph_name(&self) -> Option<String>;
    
    /// Returns the list of node properties to drop.
    fn node_properties(&self) -> Vec<String>;
    
    /// Returns whether to fail if properties are missing.
    /// Defaults to true.
    fn fail_if_missing(&self) -> bool {
        true
    }
    
    /// Parses node properties from user input.
    /// In Java, this uses UserInputAsStringOrListOfString.parse().
    fn parse_node_properties(user_input: &str) -> Vec<String> {
        vec![user_input.to_string()]
    }
}

/// Builder for creating GraphDropNodePropertiesConfig implementations.
/// 
/// In Java, this uses CypherMapWrapper for configuration parsing.
#[derive(Clone, Debug)]
pub struct GraphDropNodePropertiesConfigImpl {
    graph_name: Option<String>,
    node_properties: Vec<String>,
    fail_if_missing: bool,
}

impl GraphDropNodePropertiesConfigImpl {
    /// Creates a new GraphDropNodePropertiesConfig.
    pub fn new(
        graph_name: Option<String>,
        node_properties: Vec<String>,
        fail_if_missing: bool,
    ) -> Self {
        Self {
            graph_name,
            node_properties,
            fail_if_missing,
        }
    }
}

impl GraphDropNodePropertiesConfig for GraphDropNodePropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn node_properties(&self) -> Vec<String> {
        self.node_properties.clone()
    }
    
    fn fail_if_missing(&self) -> bool {
        self.fail_if_missing
    }
}

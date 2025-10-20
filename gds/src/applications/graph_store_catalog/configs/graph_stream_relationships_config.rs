/// Configuration for streaming relationships from the graph store.
/// 
/// Mirrors Java GraphStreamRelationshipsConfig interface.
/// Similar pattern to GraphNodePropertiesConfig but for relationships.
pub trait GraphStreamRelationshipsConfig {
    /// Returns the optional graph name.
    fn graph_name(&self) -> Option<String>;
    
    /// Returns the list of relationship types to process.
    /// Defaults to ["*"] (all types) if not specified.
    fn relationship_types(&self) -> Vec<String> {
        vec!["*".to_string()]
    }
    
    /// Parses relationship types from user input.
    /// In Java, this uses UserInputAsStringOrListOfString.parse().
    fn parse_relationship_types(user_input: &str) -> Vec<String> {
        vec![user_input.to_string()]
    }
    
    /// Validates that the specified relationship types exist in the graph.
    /// In Java, this has validation logic checking GraphStore.
    fn validate(&self) -> Result<(), String> {
        // Placeholder validation - in real implementation would check GraphStore
        Ok(())
    }
}

/// Builder for creating GraphStreamRelationshipsConfig implementations.
#[derive(Clone, Debug)]
pub struct GraphStreamRelationshipsConfigImpl {
    graph_name: Option<String>,
    relationship_types: Vec<String>,
}

impl GraphStreamRelationshipsConfigImpl {
    /// Creates a new GraphStreamRelationshipsConfig.
    pub fn new(graph_name: Option<String>, relationship_types: Vec<String>) -> Self {
        Self {
            graph_name,
            relationship_types,
        }
    }
}

impl GraphStreamRelationshipsConfig for GraphStreamRelationshipsConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn relationship_types(&self) -> Vec<String> {
        self.relationship_types.clone()
    }
}

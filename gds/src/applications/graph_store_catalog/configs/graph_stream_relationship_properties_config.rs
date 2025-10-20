use super::GraphStreamRelationshipsConfig;

/// Configuration for streaming relationship properties from the graph store.
/// 
/// Mirrors Java GraphStreamRelationshipPropertiesConfig interface.
/// Extends GraphStreamRelationshipsConfig with property validation.
pub trait GraphStreamRelationshipPropertiesConfig: GraphStreamRelationshipsConfig {
    /// Returns the list of relationship properties to stream.
    fn relationship_properties(&self) -> Vec<String>;
    
    /// Validates that the specified relationship properties exist for the given types.
    /// In Java, this has complex validation logic checking GraphStore.
    fn validate(&self) -> Result<(), String> {
        // Placeholder validation - in real implementation would check GraphStore
        Ok(())
    }
}

/// Builder for creating GraphStreamRelationshipPropertiesConfig implementations.
#[derive(Clone, Debug)]
pub struct GraphStreamRelationshipPropertiesConfigImpl {
    graph_name: Option<String>,
    relationship_types: Vec<String>,
    relationship_properties: Vec<String>,
}

impl GraphStreamRelationshipPropertiesConfigImpl {
    /// Creates a new GraphStreamRelationshipPropertiesConfig.
    pub fn new(
        graph_name: Option<String>,
        relationship_types: Vec<String>,
        relationship_properties: Vec<String>,
    ) -> Self {
        Self {
            graph_name,
            relationship_types,
            relationship_properties,
        }
    }
}

impl GraphStreamRelationshipsConfig for GraphStreamRelationshipPropertiesConfigImpl {
    fn graph_name(&self) -> Option<String> {
        self.graph_name.clone()
    }
    
    fn relationship_types(&self) -> Vec<String> {
        self.relationship_types.clone()
    }
}

impl GraphStreamRelationshipPropertiesConfig for GraphStreamRelationshipPropertiesConfigImpl {
    fn relationship_properties(&self) -> Vec<String> {
        self.relationship_properties.clone()
    }
}

/// Configuration for writing relationships to the database.
/// 
/// Mirrors Java GraphWriteRelationshipConfig interface.
/// Write config for relationships with type and optional property.
pub trait GraphWriteRelationshipConfig {
    /// Returns the relationship type to write.
    fn relationship_type(&self) -> String;
    
    /// Returns the optional relationship property to write.
    fn relationship_property(&self) -> Option<String>;
}

/// Builder for creating GraphWriteRelationshipConfig implementations.
/// 
/// In Java, this uses CypherMapWrapper for configuration parsing.
#[derive(Clone, Debug)]
pub struct GraphWriteRelationshipConfigImpl {
    relationship_type: String,
    relationship_property: Option<String>,
}

impl GraphWriteRelationshipConfigImpl {
    /// Creates a new GraphWriteRelationshipConfig.
    pub fn new(relationship_type: String, relationship_property: Option<String>) -> Self {
        Self {
            relationship_type,
            relationship_property,
        }
    }
}

impl GraphWriteRelationshipConfig for GraphWriteRelationshipConfigImpl {
    fn relationship_type(&self) -> String {
        self.relationship_type.clone()
    }
    
    fn relationship_property(&self) -> Option<String> {
        self.relationship_property.clone()
    }
}

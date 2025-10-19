use crate::projection::element_projection::PROJECT_ALL;
use crate::projection::{Aggregation, ElementProjection, PropertyMappings};
/// Relationship projection implementation.
///
/// Defines how relationships are projected into the graph, including
/// direction, aggregation strategy, and property mappings.
use crate::projection::{Orientation, RelationshipType};
use std::collections::HashMap;
use std::sync::Arc;

/// Projection configuration for relationships.
///
/// Specifies a relationship type, its orientation, aggregation strategy,
/// and property mappings.
#[derive(Debug, Clone)]
pub struct RelationshipProjection {
    rel_type: RelationshipType,
    orientation: Orientation,
    aggregation: Aggregation,
    index_inverse: bool,
    properties: PropertyMappings,
}

impl RelationshipProjection {
    /// Key for the type field in configuration objects.
    pub const TYPE_KEY: &'static str = "type";
    /// Key for the orientation field.
    pub const ORIENTATION_KEY: &'static str = "orientation";
    /// Key for the aggregation field.
    pub const AGGREGATION_KEY: &'static str = "aggregation";
    /// Key for the indexInverse field.
    pub const INDEX_INVERSE_KEY: &'static str = "indexInverse";

    /// Creates a new RelationshipProjection with all parameters.
    pub fn new(
        rel_type: RelationshipType,
        orientation: Orientation,
        aggregation: Aggregation,
        index_inverse: bool,
        properties: PropertyMappings,
    ) -> Self {
        RelationshipProjection {
            rel_type,
            orientation,
            aggregation,
            index_inverse,
            properties,
        }
    }

    /// Creates a RelationshipProjection with defaults (NATURAL orientation, DEFAULT aggregation).
    pub fn of(rel_type: RelationshipType) -> Self {
        RelationshipProjection {
            rel_type,
            orientation: Orientation::Natural,
            aggregation: Aggregation::Default,
            index_inverse: false,
            properties: PropertyMappings::empty(),
        }
    }

    /// Creates a projection that projects all relationships (NATURAL).
    pub fn all() -> Self {
        RelationshipProjection {
            rel_type: RelationshipType::of(PROJECT_ALL),
            orientation: Orientation::Natural,
            aggregation: Aggregation::Default,
            index_inverse: false,
            properties: PropertyMappings::empty(),
        }
    }

    /// Creates a projection that projects all relationships as UNDIRECTED.
    pub fn all_undirected() -> Self {
        RelationshipProjection {
            rel_type: RelationshipType::of(PROJECT_ALL),
            orientation: Orientation::Undirected,
            aggregation: Aggregation::Default,
            index_inverse: false,
            properties: PropertyMappings::empty(),
        }
    }

    /// Returns a builder for RelationshipProjection.
    pub fn builder() -> RelationshipProjectionBuilder {
        RelationshipProjectionBuilder::new()
    }

    /// Returns the relationship type for this projection.
    pub fn rel_type(&self) -> &RelationshipType {
        &self.rel_type
    }

    /// Returns the orientation.
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Returns the aggregation strategy.
    pub fn aggregation(&self) -> Aggregation {
        self.aggregation
    }

    /// Checks if inverse index should be built.
    pub fn index_inverse(&self) -> bool {
        self.index_inverse
    }

    /// Returns the property mappings.
    pub fn properties(&self) -> &PropertyMappings {
        &self.properties
    }

    /// Checks if this projection targets all relationships.
    pub fn project_all(&self) -> bool {
        self.rel_type.name() == PROJECT_ALL
    }

    /// Checks if this projection is undirected.
    pub fn is_undirected(&self) -> bool {
        self.orientation.is_undirected()
    }

    /// Creates a new projection with additional property mappings.
    pub fn with_additional_property_mappings(&self, mappings: PropertyMappings) -> Self {
        RelationshipProjection {
            rel_type: self.rel_type.clone(),
            orientation: self.orientation,
            aggregation: self.aggregation,
            index_inverse: self.index_inverse,
            properties: self.properties.merge(&mappings),
        }
    }

    /// Creates a new projection with a different orientation.
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Creates a new projection with a different aggregation.
    pub fn with_aggregation(mut self, aggregation: Aggregation) -> Self {
        self.aggregation = aggregation;
        self
    }

    /// Creates a new projection with index_inverse set.
    pub fn with_index_inverse(mut self, index_inverse: bool) -> Self {
        self.index_inverse = index_inverse;
        self
    }

    /// Converts to a configuration map.
    pub fn to_config(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();

        config.insert(
            Self::TYPE_KEY.to_string(),
            serde_json::Value::String(self.rel_type.name().to_string()),
        );

        config.insert(
            Self::ORIENTATION_KEY.to_string(),
            serde_json::Value::String(self.orientation.as_str().to_string()),
        );

        config.insert(
            Self::AGGREGATION_KEY.to_string(),
            serde_json::Value::String(self.aggregation.as_str().to_string()),
        );

        config.insert(
            Self::INDEX_INVERSE_KEY.to_string(),
            serde_json::Value::Bool(self.index_inverse),
        );

        if !self.properties.is_empty() {
            config.insert(
                "properties".to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }

        config
    }

    /// Validates the projection configuration.
    ///
    /// Checks for invalid combinations (e.g., indexInverse with UNDIRECTED).
    pub fn validate(&self) -> Result<(), String> {
        if self.index_inverse && self.orientation.is_undirected() {
            return Err("Cannot create inverse index for UNDIRECTED orientation".to_string());
        }
        Ok(())
    }
}

impl ElementProjection for RelationshipProjection {
    fn properties(&self) -> &PropertyMappings {
        &self.properties
    }

    fn with_additional_property_mappings(
        &self,
        mappings: PropertyMappings,
    ) -> Box<dyn ElementProjection> {
        Box::new(self.with_additional_property_mappings(mappings))
    }

    fn project_all(&self) -> bool {
        self.project_all()
    }

    fn include_aggregation(&self) -> bool {
        true // Relationships use aggregation
    }

    fn to_config(&self) -> HashMap<String, serde_json::Value> {
        self.to_config()
    }
}

/// Builder for RelationshipProjection.
#[derive(Debug)]
pub struct RelationshipProjectionBuilder {
    rel_type: Option<RelationshipType>,
    orientation: Orientation,
    aggregation: Aggregation,
    index_inverse: bool,
    properties: Option<PropertyMappings>,
}

impl Default for RelationshipProjectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipProjectionBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        RelationshipProjectionBuilder {
            rel_type: None,
            orientation: Orientation::Natural,
            aggregation: Aggregation::Default,
            index_inverse: false,
            properties: None,
        }
    }

    /// Sets the relationship type.
    pub fn rel_type(mut self, rel_type: RelationshipType) -> Self {
        self.rel_type = Some(rel_type);
        self
    }

    /// Sets the orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the aggregation strategy.
    pub fn aggregation(mut self, aggregation: Aggregation) -> Self {
        self.aggregation = aggregation;
        self
    }

    /// Sets whether to build inverse index.
    pub fn index_inverse(mut self, index_inverse: bool) -> Self {
        self.index_inverse = index_inverse;
        self
    }

    /// Sets the property mappings.
    pub fn properties(mut self, properties: PropertyMappings) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Builds the RelationshipProjection.
    ///
    /// # Panics
    /// Panics if rel_type is not set.
    pub fn build(self) -> Result<RelationshipProjection, String> {
        let projection = RelationshipProjection {
            rel_type: self.rel_type.expect("rel_type is required"),
            orientation: self.orientation,
            aggregation: self.aggregation,
            index_inverse: self.index_inverse,
            properties: self.properties.unwrap_or_else(PropertyMappings::empty),
        };

        projection.validate()?;
        Ok(projection)
    }
}

/// Type alias for a collection of relationship projections.
pub type RelationshipProjections =
    crate::projection::traits::Projections<RelationshipType, Arc<RelationshipProjection>>;

/// Type alias for a builder of relationship projections.
pub type RelationshipProjectionsBuilder =
    crate::projection::traits::ProjectionsBuilder<RelationshipType, Arc<RelationshipProjection>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_projection_creation() {
        let rel_type = RelationshipType::of("KNOWS");
        let projection = RelationshipProjection::of(rel_type.clone());

        assert_eq!(projection.rel_type().name(), "KNOWS");
        assert_eq!(projection.orientation(), Orientation::Natural);
        assert_eq!(projection.aggregation(), Aggregation::Default);
        assert!(!projection.index_inverse());
        assert!(!projection.project_all());
    }

    #[test]
    fn test_project_all() {
        let projection = RelationshipProjection::all();
        assert!(projection.project_all());
        assert_eq!(projection.orientation(), Orientation::Natural);
    }

    #[test]
    fn test_project_all_undirected() {
        let projection = RelationshipProjection::all_undirected();
        assert!(projection.project_all());
        assert!(projection.is_undirected());
    }

    #[test]
    fn test_builder() {
        let rel_type = RelationshipType::of("LIKES");
        let projection = RelationshipProjection::builder()
            .rel_type(rel_type.clone())
            .orientation(Orientation::Undirected)
            .aggregation(Aggregation::Sum)
            .build()
            .unwrap();

        assert_eq!(projection.rel_type().name(), "LIKES");
        assert_eq!(projection.orientation(), Orientation::Undirected);
        assert_eq!(projection.aggregation(), Aggregation::Sum);
    }

    #[test]
    fn test_with_methods() {
        let rel_type = RelationshipType::of("FOLLOWS");
        let projection = RelationshipProjection::of(rel_type)
            .with_orientation(Orientation::Reverse)
            .with_aggregation(Aggregation::Max)
            .with_index_inverse(true);

        assert_eq!(projection.orientation(), Orientation::Reverse);
        assert_eq!(projection.aggregation(), Aggregation::Max);
        assert!(projection.index_inverse());
    }

    #[test]
    fn test_validate_inverse_index_undirected_fails() {
        let rel_type = RelationshipType::of("CONNECTED");
        let result = RelationshipProjection::builder()
            .rel_type(rel_type)
            .orientation(Orientation::Undirected)
            .index_inverse(true)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot create inverse index"));
    }

    #[test]
    fn test_to_config() {
        let rel_type = RelationshipType::of("KNOWS");
        let projection = RelationshipProjection::of(rel_type);
        let config = projection.to_config();

        assert!(config.contains_key("type"));
        assert!(config.contains_key("orientation"));
        assert!(config.contains_key("aggregation"));
        assert!(config.contains_key("indexInverse"));
    }
}

use crate::types::schema::default_value::DefaultValue;
use std::fmt;

/// Aggregation strategy for relationship properties.
///
/// Defines how multiple property values are aggregated when
/// combining parallel relationships or multiple paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Aggregation {
    /// Use default aggregation (context-dependent).
    Default,
    /// No aggregation (fail on multiple values).
    None,
    /// Sum all values.
    Sum,
    /// Take the minimum value.
    Min,
    /// Take the maximum value.
    Max,
    /// Take the first value encountered.
    Single,
    /// Count occurrences.
    Count,
}

impl Default for Aggregation {
    fn default() -> Self {
        Aggregation::Default
    }
}

impl Aggregation {
    /// Parses an aggregation strategy from a string.
    ///
    /// # Arguments
    /// * `s` - The string to parse (case-insensitive)
    ///
    /// # Returns
    /// The parsed aggregation, or None if invalid
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "DEFAULT" => Some(Aggregation::Default),
            "NONE" => Some(Aggregation::None),
            "SUM" => Some(Aggregation::Sum),
            "MIN" => Some(Aggregation::Min),
            "MAX" => Some(Aggregation::Max),
            "SINGLE" => Some(Aggregation::Single),
            "COUNT" => Some(Aggregation::Count),
            _ => None,
        }
    }

    /// Returns the string representation of this aggregation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Aggregation::Default => "DEFAULT",
            Aggregation::None => "NONE",
            Aggregation::Sum => "SUM",
            Aggregation::Min => "MIN",
            Aggregation::Max => "MAX",
            Aggregation::Single => "SINGLE",
            Aggregation::Count => "COUNT",
        }
    }

    /// Resolves DEFAULT aggregation to a concrete strategy.
    ///
    /// # Arguments
    /// * `default` - The aggregation to use if this is Default
    pub fn resolve(&self, default: Aggregation) -> Aggregation {
        match self {
            Aggregation::Default => default,
            other => *other,
        }
    }
}

impl fmt::Display for Aggregation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a property mapping for graph elements.
///
/// Maps a property from the source graph to a projected property,
/// with support for default values and aggregation strategies.
#[derive(Debug, Clone)]
pub struct PropertyMapping {
    /// Property key in the result (e.g., Graph.nodeProperties(`propertyKey`))
    property_key: String,
    /// Property name in the source graph (e.g., a:Node {`neoPropertyKey`:xyz})
    neo_property_key: Option<String>,
    /// Default value if property is not present
    default_value: DefaultValue,
    /// Aggregation strategy for this property
    aggregation: Aggregation,
}

impl PropertyMapping {
    /// Key for the property field in configuration objects.
    pub const PROPERTY_KEY: &'static str = "property";
    /// Key for the default value field in configuration objects.
    pub const DEFAULT_VALUE_KEY: &'static str = "defaultValue";
    /// Wildcard to project all properties.
    pub const PROJECT_ALL: &'static str = "*";

    /// Creates a new PropertyMapping.
    ///
    /// # Arguments
    /// * `property_key` - The property key in the result
    /// * `neo_property_key` - Optional source property name (defaults to property_key)
    /// * `default_value` - Default value if property is missing
    /// * `aggregation` - Aggregation strategy
    pub fn new(
        property_key: impl Into<String>,
        neo_property_key: Option<String>,
        default_value: DefaultValue,
        aggregation: Aggregation,
    ) -> Result<Self, String> {
        let property_key = property_key.into();
        Self::validate_property_key(&property_key)?;

        let mapping = PropertyMapping {
            property_key,
            neo_property_key,
            default_value,
            aggregation,
        };

        mapping.validate()?;
        Ok(mapping)
    }

    /// Creates a simple PropertyMapping with default values.
    ///
    /// # Arguments
    /// * `property_key` - The property key
    pub fn of(property_key: impl Into<String>) -> Result<Self, String> {
        Self::new(
            property_key,
            None,
            DefaultValue::null(),
            Aggregation::Default,
        )
    }

    /// Creates a PropertyMapping with a specific source property.
    ///
    /// # Arguments
    /// * `property_key` - The property key in the result
    /// * `neo_property_key` - The source property name
    pub fn with_source(
        property_key: impl Into<String>,
        neo_property_key: impl Into<String>,
    ) -> Result<Self, String> {
        Self::new(
            property_key,
            Some(neo_property_key.into()),
            DefaultValue::null(),
            Aggregation::Default,
        )
    }

    /// Returns the property key in the result.
    pub fn property_key(&self) -> &str {
        &self.property_key
    }

    /// Returns the source property name (or property key if not specified).
    pub fn neo_property_key(&self) -> &str {
        self.neo_property_key.as_ref().unwrap_or(&self.property_key)
    }

    /// Returns the default value.
    pub fn default_value(&self) -> &DefaultValue {
        &self.default_value
    }

    /// Returns the aggregation strategy.
    pub fn aggregation(&self) -> Aggregation {
        self.aggregation
    }

    /// Checks if this mapping has a valid name.
    pub fn has_valid_name(&self) -> bool {
        let key = self.neo_property_key();
        !key.is_empty()
    }

    /// Checks if this property exists in the graph.
    ///
    /// Note: This is a placeholder. Actual implementation would
    /// query the graph store.
    pub fn exists(&self) -> bool {
        false
    }

    /// Sets a non-default aggregation if current is default.
    ///
    /// # Arguments
    /// * `aggregation` - The new aggregation strategy
    pub fn set_non_default_aggregation(&self, aggregation: Aggregation) -> Self {
        if aggregation == Aggregation::Default || self.aggregation != Aggregation::Default {
            return self.clone();
        }

        PropertyMapping {
            property_key: self.property_key.clone(),
            neo_property_key: self.neo_property_key.clone(),
            default_value: self.default_value.clone(),
            aggregation,
        }
    }

    /// Converts this mapping to a tuple for serialization.
    ///
    /// Returns (property_key, serialized_value) where value contains
    /// the source property, default value, and optionally aggregation.
    ///
    /// # Arguments
    /// * `include_aggregation` - Whether to include aggregation in the output
    pub fn to_object(&self, include_aggregation: bool) -> (String, String) {
        let mut parts = vec![
            format!("property={}", self.neo_property_key()),
            format!("default={:?}", self.default_value),
        ];

        if include_aggregation {
            parts.push(format!("aggregation={:?}", self.aggregation));
        }

        (self.property_key.clone(), parts.join(", "))
    }

    /// Validates the property key.
    fn validate_property_key(property_key: &str) -> Result<(), String> {
        if property_key.is_empty() {
            return Err("Property key must not be empty.".to_string());
        }
        Ok(())
    }

    /// Validates the entire mapping.
    fn validate(&self) -> Result<(), String> {
        let neo_key = self.neo_property_key();
        if neo_key == Self::PROJECT_ALL && self.aggregation != Aggregation::Count {
            return Err("A '*' property key can only be used with COUNT aggregation.".to_string());
        }
        Ok(())
    }
}

/// Builder for PropertyMapping with fluent API.
#[derive(Debug, Clone)]
pub struct PropertyMappingBuilder {
    property_key: String,
    neo_property_key: Option<String>,
    default_value: DefaultValue,
    aggregation: Aggregation,
}

impl PropertyMappingBuilder {
    /// Creates a new builder with the given property key.
    pub fn new(property_key: impl Into<String>) -> Self {
        PropertyMappingBuilder {
            property_key: property_key.into(),
            neo_property_key: None,
            default_value: DefaultValue::null(),
            aggregation: Aggregation::Default,
        }
    }

    /// Sets the source property name.
    pub fn neo_property_key(mut self, neo_property_key: impl Into<String>) -> Self {
        self.neo_property_key = Some(neo_property_key.into());
        self
    }

    /// Sets the default value.
    pub fn default_value(mut self, default_value: DefaultValue) -> Self {
        self.default_value = default_value;
        self
    }

    /// Sets the aggregation strategy.
    pub fn aggregation(mut self, aggregation: Aggregation) -> Self {
        self.aggregation = aggregation;
        self
    }

    /// Builds the PropertyMapping.
    pub fn build(self) -> Result<PropertyMapping, String> {
        PropertyMapping::new(
            self.property_key,
            self.neo_property_key,
            self.default_value,
            self.aggregation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregation_parse() {
        assert_eq!(Aggregation::parse("SUM"), Some(Aggregation::Sum));
        assert_eq!(Aggregation::parse("sum"), Some(Aggregation::Sum));
        assert_eq!(Aggregation::parse("MIN"), Some(Aggregation::Min));
        assert_eq!(Aggregation::parse("MAX"), Some(Aggregation::Max));
        assert_eq!(Aggregation::parse("INVALID"), None);
    }

    #[test]
    fn test_aggregation_resolve() {
        assert_eq!(
            Aggregation::Default.resolve(Aggregation::Sum),
            Aggregation::Sum
        );
        assert_eq!(Aggregation::Min.resolve(Aggregation::Sum), Aggregation::Min);
    }

    #[test]
    fn test_property_mapping_creation() {
        let mapping = PropertyMapping::of("age").unwrap();
        assert_eq!(mapping.property_key(), "age");
        assert_eq!(mapping.neo_property_key(), "age");
        assert_eq!(mapping.aggregation(), Aggregation::Default);
    }

    #[test]
    fn test_property_mapping_with_source() {
        let mapping = PropertyMapping::with_source("age", "person_age").unwrap();
        assert_eq!(mapping.property_key(), "age");
        assert_eq!(mapping.neo_property_key(), "person_age");
    }

    #[test]
    fn test_property_mapping_builder() {
        let mapping = PropertyMappingBuilder::new("score")
            .neo_property_key("user_score")
            .aggregation(Aggregation::Max)
            .build()
            .unwrap();

        assert_eq!(mapping.property_key(), "score");
        assert_eq!(mapping.neo_property_key(), "user_score");
        assert_eq!(mapping.aggregation(), Aggregation::Max);
    }

    #[test]
    fn test_empty_property_key_rejected() {
        let result = PropertyMapping::of("");
        assert!(result.is_err());
    }

    #[test]
    fn test_project_all_requires_count() {
        let result = PropertyMapping::new("*", None, DefaultValue::null(), Aggregation::Sum);
        assert!(result.is_err());

        let result = PropertyMapping::new(
            "count",
            Some("*".to_string()),
            DefaultValue::null(),
            Aggregation::Count,
        );
        assert!(result.is_ok());
    }
}

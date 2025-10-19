use super::property_mapping::PropertyMapping;
use std::collections::HashMap;

/// Wildcard symbol to project all properties.
pub const PROJECT_ALL: &str = "*";

/// Key used for properties in configuration objects.
pub const PROPERTIES_KEY: &str = "properties";

/// Base trait for projections of graph elements (nodes or relationships).
///
/// Defines the interface for element projections with property mappings.
pub trait ElementProjection: Send + Sync {
    /// Returns the property mappings for this projection.
    fn properties(&self) -> &PropertyMappings;

    /// Creates a new projection with additional property mappings.
    fn with_additional_property_mappings(
        &self,
        mappings: PropertyMappings,
    ) -> Box<dyn ElementProjection>;

    /// Checks if this projection includes all properties.
    fn project_all(&self) -> bool;

    /// Checks if aggregation should be included in serialized output.
    fn include_aggregation(&self) -> bool;

    /// Converts this projection to a configuration map.
    fn to_config(&self) -> HashMap<String, serde_json::Value>;
}

/// Collection of property mappings for an element projection.
///
/// Manages multiple PropertyMapping instances with efficient lookup.
#[derive(Debug, Clone)]
pub struct PropertyMappings {
    mappings: HashMap<String, PropertyMapping>,
}

impl PropertyMappings {
    /// Creates an empty PropertyMappings.
    pub fn empty() -> Self {
        PropertyMappings {
            mappings: HashMap::new(),
        }
    }

    /// Creates a new PropertyMappings from a map.
    pub fn new(mappings: HashMap<String, PropertyMapping>) -> Self {
        PropertyMappings { mappings }
    }

    /// Returns a builder for PropertyMappings.
    pub fn builder() -> PropertyMappingsBuilder {
        PropertyMappingsBuilder::new()
    }

    /// Returns all property mappings.
    pub fn mappings(&self) -> impl Iterator<Item = &PropertyMapping> {
        self.mappings.values()
    }

    /// Returns the number of mappings.
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Checks if there are no mappings.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Gets a mapping by property key.
    pub fn get(&self, key: &str) -> Option<&PropertyMapping> {
        self.mappings.get(key)
    }

    /// Checks if a property key exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.mappings.contains_key(key)
    }

    /// Returns all property keys.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.mappings.keys()
    }

    /// Merges with another PropertyMappings, preferring this one's values on conflict.
    pub fn merge(&self, other: &PropertyMappings) -> Self {
        let mut mappings = self.mappings.clone();
        for (key, mapping) in &other.mappings {
            mappings
                .entry(key.clone())
                .or_insert_with(|| mapping.clone());
        }
        PropertyMappings { mappings }
    }
}

impl Default for PropertyMappings {
    fn default() -> Self {
        Self::empty()
    }
}

/// Builder for PropertyMappings with fluent API.
#[derive(Debug, Clone)]
pub struct PropertyMappingsBuilder {
    mappings: HashMap<String, PropertyMapping>,
}

impl PropertyMappingsBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        PropertyMappingsBuilder {
            mappings: HashMap::new(),
        }
    }

    /// Adds a property mapping.
    pub fn add_mapping(mut self, mapping: PropertyMapping) -> Self {
        self.mappings
            .insert(mapping.property_key().to_string(), mapping);
        self
    }

    /// Adds a property mapping by reference.
    pub fn add_mapping_ref(&mut self, mapping: PropertyMapping) -> &mut Self {
        self.mappings
            .insert(mapping.property_key().to_string(), mapping);
        self
    }

    /// Adds a simple property by key.
    pub fn add_property(mut self, property_key: impl Into<String>) -> Result<Self, String> {
        let mapping = PropertyMapping::of(property_key)?;
        self.mappings
            .insert(mapping.property_key().to_string(), mapping);
        Ok(self)
    }

    /// Adds a property with source name.
    pub fn add_property_with_source(
        mut self,
        property_key: impl Into<String>,
        neo_property_key: impl Into<String>,
    ) -> Result<Self, String> {
        let mapping = PropertyMapping::with_source(property_key, neo_property_key)?;
        self.mappings
            .insert(mapping.property_key().to_string(), mapping);
        Ok(self)
    }

    /// Adds multiple mappings.
    pub fn add_all(mut self, mappings: impl IntoIterator<Item = PropertyMapping>) -> Self {
        for mapping in mappings {
            self.mappings
                .insert(mapping.property_key().to_string(), mapping);
        }
        self
    }

    /// Copies mappings from another PropertyMappings.
    pub fn from(&mut self, other: &PropertyMappings) -> &mut Self {
        for (key, mapping) in &other.mappings {
            self.mappings.insert(key.clone(), mapping.clone());
        }
        self
    }

    /// Builds the PropertyMappings.
    pub fn build(self) -> PropertyMappings {
        PropertyMappings {
            mappings: self.mappings,
        }
    }
}

impl Default for PropertyMappingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that support inline property addition.
///
/// Enables fluent API for adding properties during projection building.
pub trait InlineProperties: Sized {
    /// Gets the inline properties builder.
    fn inline_builder(&mut self) -> &mut InlinePropertiesBuilder;

    /// Adds a property mapping.
    fn add_property_mapping(mut self, mapping: PropertyMapping) -> Self {
        self.inline_builder()
            .properties_builder()
            .add_mapping_ref(mapping);
        self
    }

    /// Adds a simple property by key.
    fn add_property(mut self, property_key: impl Into<String>) -> Result<Self, String> {
        let mapping = PropertyMapping::of(property_key)?;
        self.inline_builder()
            .properties_builder()
            .add_mapping_ref(mapping);
        Ok(self)
    }

    /// Adds a property with source name.
    fn add_property_with_source(
        mut self,
        property_key: impl Into<String>,
        neo_property_key: impl Into<String>,
    ) -> Result<Self, String> {
        let mapping = PropertyMapping::with_source(property_key, neo_property_key)?;
        self.inline_builder()
            .properties_builder()
            .add_mapping_ref(mapping);
        Ok(self)
    }

    /// Adds multiple property mappings.
    fn add_all_properties(mut self, mappings: Vec<PropertyMapping>) -> Self {
        for mapping in mappings {
            self.inline_builder()
                .properties_builder()
                .add_mapping_ref(mapping);
        }
        self
    }

    /// Finalizes the property building process.
    fn build_properties(&mut self) {
        self.inline_builder().build();
    }
}

/// Helper for building properties inline during projection construction.
#[derive(Debug)]
pub struct InlinePropertiesBuilder {
    properties_builder: Option<PropertyMappingsBuilder>,
    properties: Option<PropertyMappings>,
}

impl InlinePropertiesBuilder {
    /// Creates a new InlinePropertiesBuilder.
    pub fn new() -> Self {
        InlinePropertiesBuilder {
            properties_builder: None,
            properties: None,
        }
    }

    /// Creates a builder with existing properties.
    pub fn with_properties(properties: PropertyMappings) -> Self {
        InlinePropertiesBuilder {
            properties_builder: None,
            properties: Some(properties),
        }
    }

    /// Gets or creates the properties builder.
    pub fn properties_builder(&mut self) -> &mut PropertyMappingsBuilder {
        if self.properties_builder.is_none() {
            let mut builder = PropertyMappingsBuilder::new();
            if let Some(ref properties) = self.properties {
                builder.from(properties);
                self.properties = None;
            }
            self.properties_builder = Some(builder);
        }
        self.properties_builder.as_mut().unwrap()
    }

    /// Builds the final PropertyMappings.
    pub fn build(&mut self) -> Option<PropertyMappings> {
        if let Some(builder) = self.properties_builder.take() {
            if self.properties.is_some() {
                panic!(
                    "Cannot have both complete mapping from `properties` \
                     and other properties from `add_property`"
                );
            }
            Some(builder.build())
        } else {
            self.properties.take()
        }
    }

    /// Gets the current properties (if not being built).
    pub fn get_properties(&self) -> Option<&PropertyMappings> {
        self.properties.as_ref()
    }

    /// Sets the properties directly.
    pub fn set_properties(&mut self, properties: PropertyMappings) {
        self.properties = Some(properties);
        self.properties_builder = None;
    }
}

impl Default for InlinePropertiesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_mappings_empty() {
        let mappings = PropertyMappings::empty();
        assert!(mappings.is_empty());
        assert_eq!(mappings.len(), 0);
    }

    #[test]
    fn test_property_mappings_builder() {
        let mappings = PropertyMappings::builder()
            .add_property("age")
            .unwrap()
            .add_property_with_source("score", "user_score")
            .unwrap()
            .build();

        assert_eq!(mappings.len(), 2);
        assert!(mappings.contains_key("age"));
        assert!(mappings.contains_key("score"));
    }

    #[test]
    fn test_property_mappings_merge() {
        let mappings1 = PropertyMappings::builder()
            .add_property("age")
            .unwrap()
            .build();

        let mappings2 = PropertyMappings::builder()
            .add_property("score")
            .unwrap()
            .build();

        let merged = mappings1.merge(&mappings2);
        assert_eq!(merged.len(), 2);
        assert!(merged.contains_key("age"));
        assert!(merged.contains_key("score"));
    }

    #[test]
    fn test_inline_properties_builder() {
        let mut builder = InlinePropertiesBuilder::new();
        builder
            .properties_builder()
            .add_mapping_ref(PropertyMapping::of("age").unwrap());

        let properties = builder.build().unwrap();
        assert_eq!(properties.len(), 1);
        assert!(properties.contains_key("age"));
    }
}

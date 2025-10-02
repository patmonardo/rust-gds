//! PropertyMappings - A collection of property mappings with validation.
//!
//! This is the foundational implementation type for property projections.
//! It wraps a collection of PropertyMapping objects and ensures:
//! - No mixing of NONE aggregation with other aggregation types
//! - Duplicate key detection
//! - Fluent builder API for construction

use crate::projection::traits::{Aggregation, PropertyMapping};
use std::collections::HashSet;

/// A collection of property mappings for configuring property filtering and aggregation.
///
/// PropertyMappings is an immutable collection that validates aggregation consistency
/// on construction. It prevents mixing NONE aggregation with other aggregation types,
/// which is a common error in graph projections.
#[derive(Debug, Clone)]
pub struct PropertyMappings {
    mappings: Vec<PropertyMapping>,
}

impl Default for PropertyMappings {
    fn default() -> Self {
        Self::empty()
    }
}

impl PropertyMappings {
    /// Creates a new PropertyMappings from a vector of mappings.
    ///
    /// # Errors
    ///
    /// Returns an error if there is mixing of NONE aggregation with other types.
    pub fn new(mappings: Vec<PropertyMapping>) -> Result<Self, String> {
        let result = Self { mappings };
        result.check_for_aggregation_mixing()?;
        Ok(result)
    }

    /// Creates an empty PropertyMappings.
    pub fn empty() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    /// Creates PropertyMappings with the given mappings.
    ///
    /// # Panics
    ///
    /// Panics if there is mixing of NONE aggregation with other types.
    pub fn of(mappings: Vec<PropertyMapping>) -> Self {
        Self::new(mappings).expect("Invalid aggregation mixing")
    }

    /// Returns the underlying mappings as a slice.
    pub fn mappings(&self) -> &[PropertyMapping] {
        &self.mappings
    }

    /// Returns the number of mappings.
    pub fn size(&self) -> usize {
        self.mappings.len()
    }

    /// Returns the number of mappings (alias for size).
    pub fn number_of_mappings(&self) -> usize {
        self.size()
    }

    /// Checks if this collection has any mappings.
    pub fn has_mappings(&self) -> bool {
        !self.mappings.is_empty()
    }

    /// Checks if this collection is empty.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Returns an iterator over the property mappings.
    pub fn iter(&self) -> impl Iterator<Item = &PropertyMapping> {
        self.mappings.iter()
    }

    /// Returns the property keys as a set.
    ///
    /// Filters out empty property keys.
    pub fn property_keys(&self) -> HashSet<String> {
        self.mappings
            .iter()
            .map(|m| m.property_key().to_string())
            .collect()
    }

    /// Converts this mappings to a structured representation for serialization.
    ///
    /// # Arguments
    ///
    /// * `include_aggregation` - Whether to include aggregation info in the output
    ///
    /// # Returns
    ///
    /// A vector of tuples (key, value) where value is the mapping's object representation.
    ///
    /// # Errors
    ///
    /// Returns an error if there are duplicate keys.
    pub fn to_object(&self, include_aggregation: bool) -> Result<Vec<(String, String)>, String> {
        let mut result = Vec::new();
        let mut seen_keys = HashSet::new();

        for mapping in &self.mappings {
            let (key, value) = mapping.to_object(include_aggregation);

            if seen_keys.contains(&key) {
                return Err(format!("Duplicate key: {}", key));
            }

            seen_keys.insert(key.clone());
            result.push((key, value));
        }

        Ok(result)
    }

    /// Merges this mappings with another, returning a new PropertyMappings.
    ///
    /// Duplicate keys from `other` are skipped (this takes precedence).
    ///
    /// # Arguments
    ///
    /// * `other` - The other PropertyMappings to merge with
    ///
    /// # Returns
    ///
    /// A new PropertyMappings containing unique mappings from both collections.
    pub fn merge_with(&self, other: &PropertyMappings) -> PropertyMappings {
        // Fast paths for empty collections
        if !self.has_mappings() {
            return other.clone();
        }

        if !other.has_mappings() {
            return self.clone();
        }

        let mut builder = PropertyMappingsBuilder::new();

        // Track seen keys
        let mut seen = HashSet::new();

        // Add all mappings from this collection
        for mapping in &self.mappings {
            let key = mapping.property_key();
            seen.insert(key.to_string());
            builder = builder.add_mapping(mapping.clone());
        }

        // Add unique mappings from other collection
        for mapping in other.mappings() {
            let key = mapping.property_key();
            if seen.contains(key) {
                continue; // Skip duplicates
            }
            builder = builder.add_mapping(mapping.clone());
        }

        builder.build()
    }

    /// Creates a new builder for PropertyMappings.
    pub fn builder() -> PropertyMappingsBuilder {
        PropertyMappingsBuilder::new()
    }

    /// Validates that there is no mixing of NONE aggregation with other types.
    ///
    /// In graph projections, NONE aggregation means "preserve all parallel relationships".
    /// Mixing NONE with other aggregations (like SUM, MAX) is ambiguous and not allowed.
    ///
    /// # Errors
    ///
    /// Returns an error if NONE is mixed with other aggregation types.
    fn check_for_aggregation_mixing(&self) -> Result<(), String> {
        let none_count = self
            .mappings
            .iter()
            .filter(|m| m.aggregation() == Aggregation::None)
            .count();

        if none_count > 0 && none_count < self.number_of_mappings() {
            return Err(
                "Conflicting relationship property aggregations, it is not allowed to mix `NONE` with aggregations.".to_string()
            );
        }

        Ok(())
    }
}

/// Implement IntoIterator for PropertyMappings
impl<'a> IntoIterator for &'a PropertyMappings {
    type Item = &'a PropertyMapping;
    type IntoIter = std::slice::Iter<'a, PropertyMapping>;

    fn into_iter(self) -> Self::IntoIter {
        self.mappings.iter()
    }
}

/// Builder for constructing PropertyMappings with fluent API.
///
/// The builder allows:
/// - Adding individual or multiple mappings
/// - Setting a default aggregation applied to all mappings
/// - Copying from existing PropertyMappings
/// - Checking for duplicate keys
#[derive(Debug, Default)]
pub struct PropertyMappingsBuilder {
    mappings: Vec<PropertyMapping>,
    default_aggregation: Aggregation,
}

impl PropertyMappingsBuilder {
    /// Creates a new PropertyMappingsBuilder.
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
            default_aggregation: Aggregation::Default,
        }
    }

    /// Creates a new PropertyMappingsBuilder (alias for new).
    pub fn create() -> Self {
        Self::new()
    }

    /// Sets the default aggregation for the builder.
    ///
    /// This aggregation will be applied to all mappings that don't explicitly
    /// override it during build.
    pub fn with_default_aggregation(mut self, aggregation: Aggregation) -> Self {
        self.default_aggregation = aggregation;
        self
    }

    /// Adds a property mapping.
    pub fn add_mapping(mut self, mapping: PropertyMapping) -> Self {
        self.mappings.push(mapping);
        self
    }

    /// Adds multiple property mappings from an iterator.
    pub fn add_mappings(mut self, mappings: impl IntoIterator<Item = PropertyMapping>) -> Self {
        self.mappings.extend(mappings);
        self
    }

    /// Adds a property mapping (alias for add_mapping).
    ///
    /// This alias matches the API expected by NodeProjectionBuilder.
    pub fn add_property(self, mapping: PropertyMapping) -> Self {
        self.add_mapping(mapping)
    }

    /// Adds multiple property mappings (variadic).
    pub fn add_properties(mut self, properties: Vec<PropertyMapping>) -> Self {
        self.mappings.extend(properties);
        self
    }

    /// Adds all property mappings from an iterable (alias for add_mappings).
    pub fn add_all_properties(self, properties: impl IntoIterator<Item = PropertyMapping>) -> Self {
        self.add_mappings(properties)
    }

    /// Copies values from another PropertyMappings.
    pub fn from(mut self, source: &PropertyMappings) -> Self {
        self.mappings.extend(source.mappings().iter().cloned());
        self
    }

    /// Checks if a mapping with the given key exists.
    pub fn has_mapping_with_key(&self, key: &str) -> bool {
        self.mappings.iter().any(|m| m.property_key() == key)
    }

    /// Builds the PropertyMappings.
    ///
    /// If a non-default aggregation was set, it will be applied to all mappings
    /// that use the DEFAULT aggregation.
    ///
    /// # Panics
    ///
    /// Panics if there is mixing of NONE aggregation with other types.
    pub fn build(mut self) -> PropertyMappings {
        // Apply default aggregation if needed
        if self.default_aggregation != Aggregation::Default {
            self.mappings = self
                .mappings
                .into_iter()
                .map(|mapping| mapping.set_non_default_aggregation(self.default_aggregation))
                .collect();
        }

        PropertyMappings::new(self.mappings).expect("Invalid aggregation mixing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DefaultValue;

    #[test]
    fn test_property_mappings_empty() {
        let mappings = PropertyMappings::empty();
        assert!(mappings.is_empty());
        assert_eq!(mappings.size(), 0);
        assert!(!mappings.has_mappings());
    }

    #[test]
    fn test_property_mappings_of() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);

        let mappings = PropertyMappings::of(vec![mapping1, mapping2]);

        assert_eq!(mappings.size(), 2);
        assert!(mappings.has_mappings());
        assert!(!mappings.is_empty());
    }

    #[test]
    fn test_property_mappings_property_keys() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);

        let mappings = PropertyMappings::of(vec![mapping1, mapping2]);
        let keys = mappings.property_keys();

        assert_eq!(keys.len(), 2);
        assert!(keys.contains("prop1"));
        assert!(keys.contains("prop2"));
    }

    #[test]
    fn test_property_mappings_merge() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);
        let mapping3 =
            PropertyMapping::new("prop3", "prop3", DefaultValue::null(), Aggregation::Default);

        let mappings1 = PropertyMappings::of(vec![mapping1.clone(), mapping2.clone()]);
        let mappings2 = PropertyMappings::of(vec![mapping2, mapping3]);

        let merged = mappings1.merge_with(&mappings2);

        // Should have prop1, prop2, prop3 but prop2 only once (from mappings1)
        assert_eq!(merged.size(), 3);
        let keys = merged.property_keys();
        assert!(keys.contains("prop1"));
        assert!(keys.contains("prop2"));
        assert!(keys.contains("prop3"));
    }

    #[test]
    fn test_property_mappings_merge_empty() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mappings1 = PropertyMappings::of(vec![mapping1]);
        let mappings2 = PropertyMappings::empty();

        let merged = mappings1.merge_with(&mappings2);
        assert_eq!(merged.size(), 1);

        let merged2 = mappings2.merge_with(&mappings1);
        assert_eq!(merged2.size(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid aggregation mixing")]
    fn test_aggregation_mixing_rejected() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::None);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Sum);

        // This should panic due to mixing NONE with other aggregations
        PropertyMappings::of(vec![mapping1, mapping2]);
    }

    #[test]
    fn test_aggregation_all_none_allowed() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::None);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::None);

        // All NONE is allowed
        let mappings = PropertyMappings::of(vec![mapping1, mapping2]);
        assert_eq!(mappings.size(), 2);
    }

    #[test]
    fn test_aggregation_no_none_allowed() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Sum);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Max);

        // No NONE is allowed
        let mappings = PropertyMappings::of(vec![mapping1, mapping2]);
        assert_eq!(mappings.size(), 2);
    }

    #[test]
    fn test_property_mappings_builder() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);

        let mappings = PropertyMappings::builder()
            .add_mapping(mapping1)
            .add_mapping(mapping2)
            .build();

        assert_eq!(mappings.size(), 2);
    }

    #[test]
    fn test_property_mappings_builder_with_default_aggregation() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);

        let mappings = PropertyMappings::builder()
            .with_default_aggregation(Aggregation::Sum)
            .add_mapping(mapping1)
            .add_mapping(mapping2)
            .build();

        assert_eq!(mappings.size(), 2);
        // Both mappings should now have Sum aggregation
        for mapping in mappings.iter() {
            assert_eq!(mapping.aggregation(), Aggregation::Sum);
        }
    }

    #[test]
    fn test_property_mappings_builder_from() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let existing = PropertyMappings::of(vec![mapping1]);

        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);

        let mappings = PropertyMappings::builder()
            .from(&existing)
            .add_mapping(mapping2)
            .build();

        assert_eq!(mappings.size(), 2);
    }

    #[test]
    fn test_property_mappings_builder_has_key() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);

        let builder = PropertyMappings::builder().add_mapping(mapping1);

        assert!(builder.has_mapping_with_key("prop1"));
        assert!(!builder.has_mapping_with_key("prop2"));
    }

    #[test]
    fn test_property_mappings_iterator() {
        let mapping1 =
            PropertyMapping::new("prop1", "prop1", DefaultValue::null(), Aggregation::Default);
        let mapping2 =
            PropertyMapping::new("prop2", "prop2", DefaultValue::null(), Aggregation::Default);

        let mappings = PropertyMappings::of(vec![mapping1, mapping2]);

        let mut count = 0;
        for _ in &mappings {
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_to_object() {
        let mapping1 = PropertyMapping::new(
            "prop1",
            "source1",
            DefaultValue::null(),
            Aggregation::Default,
        );
        let mapping2 =
            PropertyMapping::new("prop2", "source2", DefaultValue::null(), Aggregation::Sum);

        let mappings = PropertyMappings::of(vec![mapping1, mapping2]);
        let object = mappings.to_object(true).expect("Should convert to object");

        assert_eq!(object.len(), 2);
    }
}

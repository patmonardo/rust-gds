//! PregelSchema - Schema definition for node properties in Pregel computation
//!
//! Defines which properties will be stored for each node and their types.

use crate::types::ValueType;
use std::collections::{HashMap, HashSet};

/// Visibility of properties in the Pregel schema.
///
/// Properties can be marked as PUBLIC (accessible from outside) or
/// PRIVATE (only used internally within the computation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    /// Properties that can be accessed from outside the Pregel computation.
    Public,

    /// Properties that are only used internally within the Pregel computation.
    Private,
}

/// A schema element representing a single property in the Pregel schema.
///
/// Each element defines:
/// - Property name (key)
/// - Property type (long, double, long[], double[])
/// - Visibility (public or private)
/// - Optional default value
/// - Optional source property (for PropertyStore initialization)
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// The name/key of the property
    pub property_key: String,

    /// The data type of the property
    pub property_type: ValueType,

    /// The visibility of the property (PUBLIC or PRIVATE)
    pub visibility: Visibility,

    /// Optional default value for the property
    /// TODO: Replace with GdsValue when available
    pub default_value: Option<DefaultValue>,

    /// Optional source property key from PropertyStore
    ///
    /// If set, Pregel will attempt to initialize this property from the
    /// specified PropertyStore property during initialization.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Element {
    ///     property_key: "rank",
    ///     property_source: Some("seed_rank"),  // Initialize from PropertyStore
    ///     // ...
    /// }
    /// ```
    pub property_source: Option<String>,
}

/// Temporary default value representation until GdsValue is available.
///
/// # TODO
///
/// Replace with proper GdsValue type from values module.
#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    Long(i64),
    Double(f64),
    LongArray(Vec<i64>),
    DoubleArray(Vec<f64>),
}

impl Element {
    /// Create a new element with the given key, type and visibility.
    pub fn new(
        property_key: impl Into<String>,
        property_type: ValueType,
        visibility: Visibility,
    ) -> Self {
        Self {
            property_key: property_key.into(),
            property_type,
            visibility,
            default_value: None,
            property_source: None,
        }
    }

    /// Create a new element with a default value.
    pub fn with_default(
        property_key: impl Into<String>,
        default_value: DefaultValue,
        visibility: Visibility,
    ) -> Self {
        let property_type = match &default_value {
            DefaultValue::Long(_) => ValueType::Long,
            DefaultValue::Double(_) => ValueType::Double,
            DefaultValue::LongArray(_) => ValueType::LongArray,
            DefaultValue::DoubleArray(_) => ValueType::DoubleArray,
        };

        Self {
            property_key: property_key.into(),
            property_type,
            visibility,
            default_value: Some(default_value),
            property_source: None,
        }
    }
}

/// Schema describing node property layout for Pregel computation.
///
/// A PregelSchema defines all properties that will be stored for each node
/// during the computation. Each property has:
/// - A unique key (name)
/// - A value type (long, double, array, etc.)
/// - Visibility (public or private)
///
/// # Example
///
/// ```
/// use gds::pregel::{PregelSchema, Visibility};
/// use gds::values::ValueType;
///
/// let schema = PregelSchema::builder()
///     .add("rank", ValueType::Double, Visibility::Public)
///     .add("temp", ValueType::Long, Visibility::Private)
///     .build();
///
/// assert_eq!(schema.elements().len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PregelSchema {
    elements: HashSet<Element>,
}

impl PregelSchema {
    /// Create a new schema builder.
    pub fn builder() -> PregelSchemaBuilder {
        PregelSchemaBuilder::new()
    }

    /// Get all elements in this schema.
    pub fn elements(&self) -> &HashSet<Element> {
        &self.elements
    }

    /// Get a map of property keys to their types.
    pub fn properties_map(&self) -> HashMap<String, ValueType> {
        self.elements
            .iter()
            .map(|e| (e.property_key.clone(), e.property_type))
            .collect()
    }

    /// Check if a property exists in the schema.
    pub fn has_property(&self, key: &str) -> bool {
        self.elements.iter().any(|e| e.property_key == key)
    }

    /// Get the type of a property by key.
    pub fn property_type(&self, key: &str) -> Option<ValueType> {
        self.elements
            .iter()
            .find(|e| e.property_key == key)
            .map(|e| e.property_type)
    }
}

// Custom Hash and Eq for Element to use property_key as the unique identifier
impl std::hash::Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.property_key.hash(state);
    }
}

impl Eq for Element {}

/// Builder for creating PregelSchema instances.
///
/// # Example
///
/// ```
/// use gds::pregel::{PregelSchema, Visibility};
/// use gds::values::ValueType;
///
/// let schema = PregelSchema::builder()
///     .add("score", ValueType::Double, Visibility::Public)
///     .add("visited", ValueType::Long, Visibility::Private)
///     .build();
/// ```
pub struct PregelSchemaBuilder {
    elements: HashSet<Element>,
}

impl PregelSchemaBuilder {
    /// Create a new schema builder.
    pub fn new() -> Self {
        Self {
            elements: HashSet::new(),
        }
    }

    /// Add a property to the schema with specified visibility.
    pub fn add(
        mut self,
        property_key: impl Into<String>,
        property_type: ValueType,
        visibility: Visibility,
    ) -> Self {
        self.elements
            .insert(Element::new(property_key, property_type, visibility));
        self
    }

    /// Add a property with default visibility (PUBLIC).
    pub fn add_public(self, property_key: impl Into<String>, property_type: ValueType) -> Self {
        self.add(property_key, property_type, Visibility::Public)
    }

    /// Add a property with a default value.
    pub fn add_with_default(
        mut self,
        property_key: impl Into<String>,
        default_value: DefaultValue,
        visibility: Visibility,
    ) -> Self {
        self.elements.insert(Element::with_default(
            property_key,
            default_value,
            visibility,
        ));
        self
    }

    /// Set the PropertyStore source for a property.
    ///
    /// Links a Pregel property to a PropertyStore property for automatic initialization.
    /// When Pregel starts, it will attempt to load initial values from the specified
    /// PropertyStore property.
    ///
    /// # Arguments
    ///
    /// * `property_key` - The Pregel property key
    /// * `source_key` - The PropertyStore property key to read from
    ///
    /// # Example
    ///
    /// ```ignore
    /// let schema = PregelSchema::builder()
    ///     .add("rank", ValueType::Double, Visibility::Public)
    ///     .with_property_source("rank", "seed_rank")  // Initialize from PropertyStore
    ///     .build();
    /// ```
    pub fn with_property_source(
        mut self,
        property_key: impl Into<String>,
        source_key: impl Into<String>,
    ) -> Self {
        let key = property_key.into();
        let source = source_key.into();

        // Find the element and update its property_source
        // Since HashSet doesn't allow mutation, we need to rebuild
        let mut found = false;
        let elements: HashSet<Element> = self
            .elements
            .into_iter()
            .map(|mut element| {
                if element.property_key == key {
                    element.property_source = Some(source.clone());
                    found = true;
                }
                element
            })
            .collect();

        if !found {
            panic!(
                "Property '{}' not found in schema. Add the property before setting its source.",
                key
            );
        }

        self.elements = elements;
        self
    }

    /// Build the final PregelSchema.
    pub fn build(self) -> PregelSchema {
        PregelSchema {
            elements: self.elements,
        }
    }
}

impl Default for PregelSchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_builder() {
        let schema = PregelSchema::builder()
            .add("rank", ValueType::Double, Visibility::Public)
            .add("temp", ValueType::Long, Visibility::Private)
            .build();

        assert_eq!(schema.elements().len(), 2);
        assert!(schema.has_property("rank"));
        assert!(schema.has_property("temp"));
    }

    #[test]
    fn test_properties_map() {
        let schema = PregelSchema::builder()
            .add("score", ValueType::Double, Visibility::Public)
            .add("count", ValueType::Long, Visibility::Public)
            .build();

        let map = schema.properties_map();
        assert_eq!(map.get("score"), Some(&ValueType::Double));
        assert_eq!(map.get("count"), Some(&ValueType::Long));
    }

    #[test]
    fn test_property_type() {
        let schema = PregelSchema::builder()
            .add_public("value", ValueType::Double)
            .build();

        assert_eq!(schema.property_type("value"), Some(ValueType::Double));
        assert_eq!(schema.property_type("nonexistent"), None);
    }

    #[test]
    fn test_property_source() {
        let schema = PregelSchema::builder()
            .add("rank", ValueType::Double, Visibility::Public)
            .with_property_source("rank", "seed_rank")
            .build();

        let element = schema
            .elements()
            .iter()
            .find(|e| e.property_key == "rank")
            .unwrap();

        assert_eq!(element.property_source, Some("seed_rank".to_string()));
    }

    #[test]
    #[should_panic(expected = "Property 'nonexistent' not found")]
    fn test_property_source_not_found() {
        PregelSchema::builder()
            .add("rank", ValueType::Double, Visibility::Public)
            .with_property_source("nonexistent", "seed_rank")
            .build();
    }

    #[test]
    fn test_element_with_default() {
        let element = Element::with_default("initial", DefaultValue::Long(42), Visibility::Public);

        assert_eq!(element.property_key, "initial");
        assert_eq!(element.property_type, ValueType::Long);
        assert_eq!(element.visibility, Visibility::Public);
        assert_eq!(element.default_value, Some(DefaultValue::Long(42)));
    }

    #[test]
    fn test_visibility() {
        let public_elem = Element::new("pub", ValueType::Long, Visibility::Public);
        let private_elem = Element::new("priv", ValueType::Long, Visibility::Private);

        assert_eq!(public_elem.visibility, Visibility::Public);
        assert_eq!(private_elem.visibility, Visibility::Private);
    }
}

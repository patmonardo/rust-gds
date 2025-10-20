//! FormShape - Pure Form Structure Definitions
//!
//! This module defines the Pure FormShape structures that represent the
//! essential configuration forms at the Container Level.

/// A Pure FormShape represents the essential structure of a configuration
/// object. It embodies the Triadic structure:
/// - Container: The configuration struct itself
/// - Contained: The individual fields and their types  
/// - Container+Contained: The builder pattern and validation
#[derive(Debug, Clone)]
pub struct FormShape {
    /// The name of the configuration
    pub name: String,
    /// The fields that make up this FormShape
    pub fields: Vec<FieldShape>,
}

/// A FieldShape represents a single field in a Pure FormShape
#[derive(Debug, Clone)]
pub struct FieldShape {
    /// The name of the field
    pub name: String,
    /// The type of the field
    pub field_type: String,
    /// The default value for the field
    pub default_value: String,
    /// Whether the field is required
    pub required: bool,
}

impl FormShape {
    /// Create a new Pure FormShape
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    /// Add a field to the Pure FormShape
    pub fn add_field(&mut self, field: FieldShape) {
        self.fields.push(field);
    }

    /// Get the field count for this FormShape
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Check if this FormShape has any required fields
    pub fn has_required_fields(&self) -> bool {
        self.fields.iter().any(|f| f.required)
    }
}

impl FieldShape {
    /// Create a new FieldShape
    pub fn new(name: String, field_type: String, default_value: String) -> Self {
        Self {
            name,
            field_type,
            default_value,
            required: false,
        }
    }

    /// Create a required FieldShape
    pub fn required(name: String, field_type: String) -> Self {
        Self {
            name,
            field_type,
            default_value: String::new(),
            required: true,
        }
    }
}

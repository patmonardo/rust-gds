//! Miscellaneous algorithms - Faithful 1:1 translation from Java GDS
//!
//! This module contains faithful translations of Java GDS miscellaneous support classes:
//! - `ScaledPropertiesNodePropertyValues.java` → `scaled_properties_node_property_values.rs`

pub mod scaled_properties_node_property_values;

// Re-export the translated types
pub use scaled_properties_node_property_values::*;
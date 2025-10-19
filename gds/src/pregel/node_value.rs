//! NodeValue - Node property value storage for Pregel computation
//!
//! Provides storage and access to computed node property values during and after
//! Pregel execution. Properties are stored in columnar HugeArrays for memory efficiency.

use crate::collections::{HugeDoubleArray, HugeLongArray, HugeObjectArray};
use crate::concurrency::Concurrency;
use crate::pregel::{DefaultValue, Element, PregelSchema};
use crate::types::ValueType;
use std::collections::HashMap;

#[cfg(test)]
use crate::pregel::Visibility;

/// Property storage type - discriminated union of all supported array types.
enum PropertyArray {
    Double(HugeDoubleArray),
    Long(HugeLongArray),
    LongArray(HugeObjectArray<Vec<i64>>),
    DoubleArray(HugeObjectArray<Vec<f64>>),
}

/// Node property value storage for Pregel computation.
///
/// Stores all node properties defined in the PregelSchema using columnar HugeArrays.
/// Provides type-safe access with validation against the schema.
pub struct NodeValue {
    schema: PregelSchema,
    properties: HashMap<String, PropertyArray>,
    property_types: HashMap<String, ValueType>,
}

impl std::fmt::Debug for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeValue")
            .field("schema", &self.schema)
            .field("property_count", &self.properties.len())
            .finish()
    }
}

impl NodeValue {
    /// Create a NodeValue instance based on the given schema.
    pub fn of(schema: &PregelSchema, node_count: u64, _concurrency: Concurrency) -> Self {
        let mut properties = HashMap::new();

        // Initialize arrays for each property element
        for element in schema.elements() {
            let property_array = Self::init_array(element, node_count as usize);
            properties.insert(element.property_key.clone(), property_array);
        }

        let property_types = schema
            .elements()
            .iter()
            .map(|e| (e.property_key.clone(), e.property_type))
            .collect();

        NodeValue {
            schema: schema.clone(),
            properties,
            property_types,
        }
    }

    /// Create a stub NodeValue for testing.
    pub fn stub() -> Self {
        let schema = PregelSchema::builder().build();
        NodeValue {
            schema,
            properties: HashMap::new(),
            property_types: HashMap::new(),
        }
    }

    /// Get the schema used by this NodeValue.
    pub fn schema(&self) -> &PregelSchema {
        &self.schema
    }

    /// Get a double value for a specific node and property.
    pub fn double_value(&self, key: &str, node_id: usize) -> f64 {
        self.check_property(key, ValueType::Double);
        match self.properties.get(key).unwrap() {
            PropertyArray::Double(arr) => arr.get(node_id),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Get a long value for a specific node and property.
    pub fn long_value(&self, key: &str, node_id: usize) -> i64 {
        self.check_property(key, ValueType::Long);
        match self.properties.get(key).unwrap() {
            PropertyArray::Long(arr) => arr.get(node_id),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Get a long array value for a specific node and property.
    pub fn long_array_value(&self, key: &str, node_id: usize) -> &[i64] {
        self.check_property(key, ValueType::LongArray);
        match self.properties.get(key).unwrap() {
            PropertyArray::LongArray(arr) => arr.get(node_id),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Get a double array value for a specific node and property.
    pub fn double_array_value(&self, key: &str, node_id: usize) -> &[f64] {
        self.check_property(key, ValueType::DoubleArray);
        match self.properties.get(key).unwrap() {
            PropertyArray::DoubleArray(arr) => arr.get(node_id),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Set a double value for a specific node and property.
    pub fn set(&mut self, key: &str, node_id: usize, value: f64) {
        self.check_property(key, ValueType::Double);
        match self.properties.get_mut(key).unwrap() {
            PropertyArray::Double(arr) => arr.set(node_id, value),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Set a long value for a specific node and property.
    pub fn set_long(&mut self, key: &str, node_id: usize, value: i64) {
        self.check_property(key, ValueType::Long);
        match self.properties.get_mut(key).unwrap() {
            PropertyArray::Long(arr) => arr.set(node_id, value),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Set a long array value for a specific node and property.
    pub fn set_long_array(&mut self, key: &str, node_id: usize, value: Vec<i64>) {
        self.check_property(key, ValueType::LongArray);
        match self.properties.get_mut(key).unwrap() {
            PropertyArray::LongArray(arr) => arr.set(node_id, value),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Set a double array value for a specific node and property.
    pub fn set_double_array(&mut self, key: &str, node_id: usize, value: Vec<f64>) {
        self.check_property(key, ValueType::DoubleArray);
        match self.properties.get_mut(key).unwrap() {
            PropertyArray::DoubleArray(arr) => arr.set(node_id, value),
            _ => unreachable!("Type was validated"),
        }
    }

    /// Check that a property exists and has the expected type.
    fn check_property(&self, key: &str, expected_type: ValueType) {
        let actual_type = self.property_types.get(key).copied();

        if actual_type.is_none() {
            let available_keys: Vec<_> = self.property_types.keys().collect();
            panic!(
                "Property with key '{}' does not exist. Available properties: {:?}",
                key, available_keys
            );
        }

        let actual_type = actual_type.unwrap();
        if actual_type != expected_type {
            panic!(
                "Requested property type {:?} is not compatible with available property type {:?} for key '{}'",
                expected_type, actual_type, key
            );
        }
    }

    /// Initialize a property array based on the element definition.
    fn init_array(element: &Element, node_count: usize) -> PropertyArray {
        match element.property_type {
            ValueType::Double => {
                let mut arr = HugeDoubleArray::new(node_count);
                let default_value = match &element.default_value {
                    Some(DefaultValue::Double(v)) => *v,
                    None => 0.0,
                    _ => panic!("Invalid default value type for Double property"),
                };

                for i in 0..node_count {
                    arr.set(i, default_value);
                }

                PropertyArray::Double(arr)
            }
            ValueType::Long => {
                let mut arr = HugeLongArray::new(node_count);
                let default_value = match &element.default_value {
                    Some(DefaultValue::Long(v)) => *v,
                    None => 0,
                    _ => panic!("Invalid default value type for Long property"),
                };

                for i in 0..node_count {
                    arr.set(i, default_value);
                }

                PropertyArray::Long(arr)
            }
            ValueType::LongArray => {
                if element.default_value.is_some() {
                    panic!("Default value is not supported for long array properties");
                }
                PropertyArray::LongArray(HugeObjectArray::new(node_count))
            }
            ValueType::DoubleArray => {
                if element.default_value.is_some() {
                    panic!("Default value is not supported for double array properties");
                }
                PropertyArray::DoubleArray(HugeObjectArray::new(node_count))
            }
            _ => panic!("Unsupported value type: {:?}", element.property_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_value_stub() {
        let node_value = NodeValue::stub();
        assert!(node_value.schema().elements().is_empty());
    }

    #[test]
    fn test_single_double_property() {
        let schema = PregelSchema::builder()
            .add("pagerank", ValueType::Double, Visibility::Public)
            .build();

        let mut node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());

        node_values.set("pagerank", 0, 0.15);
        node_values.set("pagerank", 5, 0.85);

        assert_eq!(node_values.double_value("pagerank", 0), 0.15);
        assert_eq!(node_values.double_value("pagerank", 5), 0.85);
    }

    #[test]
    fn test_single_long_property() {
        let schema = PregelSchema::builder()
            .add("component", ValueType::Long, Visibility::Public)
            .build();

        let mut node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());

        node_values.set_long("component", 0, 42);
        node_values.set_long("component", 3, 100);

        assert_eq!(node_values.long_value("component", 0), 42);
        assert_eq!(node_values.long_value("component", 3), 100);
    }

    #[test]
    fn test_composite_properties() {
        let schema = PregelSchema::builder()
            .add("pagerank", ValueType::Double, Visibility::Public)
            .add("component", ValueType::Long, Visibility::Public)
            .build();

        let mut node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());

        node_values.set("pagerank", 0, 0.15);
        node_values.set_long("component", 0, 42);

        assert_eq!(node_values.double_value("pagerank", 0), 0.15);
        assert_eq!(node_values.long_value("component", 0), 42);
    }

    #[test]
    fn test_long_array_property() {
        let schema = PregelSchema::builder()
            .add("neighbors", ValueType::LongArray, Visibility::Private)
            .build();

        let mut node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());

        node_values.set_long_array("neighbors", 0, vec![1, 2, 3]);
        node_values.set_long_array("neighbors", 5, vec![4, 5, 6, 7]);

        assert_eq!(node_values.long_array_value("neighbors", 0), &[1, 2, 3]);
        assert_eq!(node_values.long_array_value("neighbors", 5), &[4, 5, 6, 7]);
    }

    #[test]
    fn test_double_array_property() {
        let schema = PregelSchema::builder()
            .add("embeddings", ValueType::DoubleArray, Visibility::Private)
            .build();

        let mut node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());

        node_values.set_double_array("embeddings", 0, vec![0.1, 0.2, 0.3]);
        node_values.set_double_array("embeddings", 2, vec![0.4, 0.5]);

        assert_eq!(
            node_values.double_array_value("embeddings", 0),
            &[0.1, 0.2, 0.3]
        );
        assert_eq!(node_values.double_array_value("embeddings", 2), &[0.4, 0.5]);
    }

    #[test]
    #[should_panic(expected = "does not exist")]
    fn test_missing_property() {
        let schema = PregelSchema::builder()
            .add("pagerank", ValueType::Double, Visibility::Public)
            .build();

        let node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());
        let _ = node_values.double_value("missing", 0);
    }

    #[test]
    #[should_panic(expected = "not compatible")]
    fn test_wrong_property_type() {
        let schema = PregelSchema::builder()
            .add("component", ValueType::Long, Visibility::Public)
            .build();

        let node_values = NodeValue::of(&schema, 10, Concurrency::new(1).unwrap());
        // Try to access Long property as Double
        let _ = node_values.double_value("component", 0);
    }
}

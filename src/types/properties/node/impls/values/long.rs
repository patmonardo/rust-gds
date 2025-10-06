use crate::types::properties::node::node_property_values::{
    LongNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::property::ValueType;
use crate::types::property_value::PropertyValue;
use crate::{node_long_property_values_impl, property_values_impl};

/// Default implementation for long node property values.
///
/// Storage: Vec<i64> - suitable for pure in-memory graphs.
/// Future: Can be replaced with Arrow2 Int64Array for columnar storage.
#[derive(Debug, Clone)]
pub struct DefaultLongNodePropertyValues {
    values: Vec<i64>,
    node_count: usize,
}

impl DefaultLongNodePropertyValues {
    pub fn new(values: Vec<i64>, node_count: usize) -> Self {
        DefaultLongNodePropertyValues { values, node_count }
    }
}

// Generate PropertyValues trait implementation
property_values_impl!(
    DefaultLongNodePropertyValues,
    Long,
    i64,
    PropertyValue::Long
);

// Generate complete NodePropertyValues trait implementation with all conversions and errors
node_long_property_values_impl!(DefaultLongNodePropertyValues);

// Specialized unchecked accessor for Long values
impl LongNodePropertyValues for DefaultLongNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.values[node_id as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::property_values::PropertyValues;
    use crate::types::property::ValueType;

    #[test]
    fn test_long_node_property_values() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3, 4, 5], 5);
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.node_count(), 5);
        assert_eq!(values.long_value(0).unwrap(), 1);
        assert_eq!(values.long_value(4).unwrap(), 5);
        assert_eq!(values.double_value(2).unwrap(), 3.0);
        assert_eq!(values.dimension(), Some(1));
        assert_eq!(values.get_max_long_property_value(), Some(5));
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }

    #[test]
    fn test_long_to_double_conversion() {
        let values = DefaultLongNodePropertyValues::new(vec![100, 200, 300], 3);
        assert_eq!(values.double_value(0).unwrap(), 100.0);
        assert_eq!(values.double_value(2).unwrap(), 300.0);
    }

    #[test]
    fn test_unsupported_array_access() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3);
        assert!(values.double_array_value(0).is_err());
        assert!(values.float_array_value(0).is_err());
        assert!(values.long_array_value(0).is_err());
    }
}

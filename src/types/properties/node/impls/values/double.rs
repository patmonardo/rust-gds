use crate::types::properties::node::node_property_values::{
    DoubleNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::value_type::ValueType;
use crate::{node_double_property_values_impl, property_values_impl};

/// Default implementation for double node property values.
///
/// Storage: Vec<f64> - suitable for pure in-memory graphs.
/// Future: Can be replaced with Arrow2 Float64Array for columnar storage.
#[derive(Debug, Clone)]
pub struct DefaultDoubleNodePropertyValues {
    values: Vec<f64>,
    node_count: usize,
}

impl DefaultDoubleNodePropertyValues {
    pub fn new(values: Vec<f64>, node_count: usize) -> Self {
        DefaultDoubleNodePropertyValues { values, node_count }
    }
}

// Generate PropertyValues trait implementation
property_values_impl!(DefaultDoubleNodePropertyValues, Double);

// Generate complete NodePropertyValues trait implementation with all conversions and errors
node_double_property_values_impl!(DefaultDoubleNodePropertyValues);

// Specialized unchecked accessor for Double values
impl DoubleNodePropertyValues for DefaultDoubleNodePropertyValues {
    fn double_value_unchecked(&self, node_id: u64) -> f64 {
        self.values[node_id as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::property_values::PropertyValues;
    use crate::types::value_type::ValueType;

    #[test]
    fn test_double_node_property_values() {
        let values = DefaultDoubleNodePropertyValues::new(vec![1.5, 2.5, 3.5], 3);
        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.node_count(), 3);
        assert_eq!(values.double_value(0).unwrap(), 1.5);
        assert_eq!(values.double_value(2).unwrap(), 3.5);
    }

    #[test]
    fn test_double_to_long_conversion() {
        let values = DefaultDoubleNodePropertyValues::new(vec![1.9, 2.1, 3.7], 3);
        assert_eq!(values.long_value(0).unwrap(), 1);
        assert_eq!(values.long_value(2).unwrap(), 3);
    }

    #[test]
    fn test_max_values() {
        let values = DefaultDoubleNodePropertyValues::new(vec![1.0, 5.0, 3.0], 3);
        assert_eq!(values.get_max_double_property_value(), Some(5.0));
        assert_eq!(values.get_max_long_property_value(), Some(5));
    }
}

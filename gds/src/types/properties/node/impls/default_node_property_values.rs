use crate::types::properties::node::{
    DoubleArrayNodePropertyValues, DoubleNodePropertyValues, FloatArrayNodePropertyValues,
    LongArrayNodePropertyValues, LongNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;
use crate::{
    node_double_array_property_values_impl, node_double_property_values_impl,
    node_float_array_property_values_impl, node_long_array_property_values_impl,
    node_long_property_values_impl, property_values_impl,
};

// Java GDS Parity: Generate exactly 4 Node types using macros
crate::node_long_adapter!(DefaultLongNodePropertyValues, Vec<i64>);
crate::node_double_adapter!(DefaultDoubleNodePropertyValues, Vec<f64>);
crate::node_double_array_adapter!(DefaultDoubleArrayNodePropertyValues, Vec<Option<Vec<f64>>>);
crate::node_long_array_adapter!(DefaultLongArrayNodePropertyValues, Vec<Option<Vec<i64>>>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ValueType;

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
    fn test_double_array_node_property_values() {
        let values = DefaultDoubleArrayNodePropertyValues::new(
            vec![Some(vec![1.0, 2.0, 3.0]), Some(vec![4.0, 5.0, 6.0]), None],
            3,
        );

        assert_eq!(values.value_type(), ValueType::DoubleArray);
        assert_eq!(values.node_count(), 3);
        assert_eq!(values.double_array_value(0).unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(values.dimension(), Some(3));
        assert!(values.has_value(0));
        assert!(!values.has_value(2));
    }
}

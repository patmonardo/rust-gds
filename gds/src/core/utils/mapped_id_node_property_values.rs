//! Node property values implementation where the property value is the same as the node's internal ID.
//!
//! This is a lightweight implementation that returns the mapped node ID itself as the property value,
//! useful for algorithms that need to track node identities or for testing and debugging.

use crate::types::graph::id_map::IdMap;
use crate::types::properties::node::LongNodePropertyValues;
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;

/// A node property values implementation where the property value is the same as the node's mapped ID.
///
/// This implementation provides a zero-overhead way to expose node IDs as properties,
/// commonly used for:
/// - Algorithms that need to track node identities
/// - Testing and debugging graph operations
/// - Export scenarios where internal IDs are needed
///
/// # Examples
///
/// ```
/// use gds::core::utils::MappedIdNodePropertyValues;
///
/// // Create from node count
/// let props = MappedIdNodePropertyValues::new(1000);
/// assert_eq!(props.node_count(), 1000);
///
/// // Property value is the node ID itself
/// assert_eq!(props.long_value(42).unwrap(), 42);
/// assert_eq!(props.long_value(999).unwrap(), 999);
/// ```
#[derive(Debug, Clone)]
pub struct MappedIdNodePropertyValues {
    node_count: usize,
}

impl MappedIdNodePropertyValues {
    /// Creates a new mapped ID node property values from an IdMap.
    ///
    /// # Arguments
    ///
    /// * `id_map` - ID map to get the node count from
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use gds::core::utils::MappedIdNodePropertyValues;
    /// use gds::types::graph::id_map::IdMap;
    ///
    /// let id_map = /* some IdMap implementation */;
    /// let props = MappedIdNodePropertyValues::from_id_map(&id_map);
    /// ```
    pub fn from_id_map<T: IdMap + ?Sized>(id_map: &T) -> Self {
        Self {
            node_count: id_map.node_count(),
        }
    }

    /// Creates a new mapped ID node property values with the given node count.
    ///
    /// # Arguments
    ///
    /// * `node_count` - The number of nodes
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::MappedIdNodePropertyValues;
    ///
    /// let props = MappedIdNodePropertyValues::new(5000);
    /// assert_eq!(props.node_count(), 5000);
    /// ```
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl PropertyValues for MappedIdNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl crate::types::properties::node::NodePropertyValues for MappedIdNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::unsupported_operation(
            "MappedIdNodePropertyValues does not support double values",
        ))
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        Ok(node_id as i64)
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::unsupported_operation(
            "MappedIdNodePropertyValues does not support double array values",
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::unsupported_operation(
            "MappedIdNodePropertyValues does not support float array values",
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_operation(
            "MappedIdNodePropertyValues does not support long array values",
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(node_id as i64))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        if self.node_count > 0 {
            Some((self.node_count - 1) as i64)
        } else {
            None
        }
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.node_count
    }

    fn node_count(&self) -> usize {
        self.node_count
    }
}

impl LongNodePropertyValues for MappedIdNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        node_id as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::NodePropertyValues;

    #[test]
    fn test_new() {
        let props = MappedIdNodePropertyValues::new(100);
        assert_eq!(props.node_count(), 100);
    }

    #[test]
    fn test_long_value_returns_node_id() {
        let props = MappedIdNodePropertyValues::new(1000);

        assert_eq!(props.long_value(0).unwrap(), 0);
        assert_eq!(props.long_value(42).unwrap(), 42);
        assert_eq!(props.long_value(999).unwrap(), 999);
    }

    #[test]
    fn test_long_value_unchecked() {
        let props = MappedIdNodePropertyValues::new(1000);

        assert_eq!(props.long_value_unchecked(0), 0);
        assert_eq!(props.long_value_unchecked(500), 500);
        assert_eq!(props.long_value_unchecked(999), 999);
    }

    #[test]
    fn test_value_type() {
        let props = MappedIdNodePropertyValues::new(100);
        assert_eq!(props.value_type(), ValueType::Long);
    }

    #[test]
    fn test_element_count() {
        let props = MappedIdNodePropertyValues::new(5000);
        assert_eq!(props.element_count(), 5000);
    }

    #[test]
    fn test_dimension() {
        let props = MappedIdNodePropertyValues::new(100);
        assert_eq!(props.dimension(), Some(1));
    }

    #[test]
    fn test_get_max_long_property_value() {
        let props = MappedIdNodePropertyValues::new(1000);
        assert_eq!(props.get_max_long_property_value(), Some(999));

        let empty_props = MappedIdNodePropertyValues::new(0);
        assert_eq!(empty_props.get_max_long_property_value(), None);
    }

    #[test]
    fn test_get_max_double_property_value() {
        let props = MappedIdNodePropertyValues::new(100);
        assert_eq!(props.get_max_double_property_value(), None);
    }

    #[test]
    fn test_has_value() {
        let props = MappedIdNodePropertyValues::new(100);

        assert!(props.has_value(0));
        assert!(props.has_value(50));
        assert!(props.has_value(99));
        assert!(!props.has_value(100));
        assert!(!props.has_value(1000));
    }

    #[test]
    fn test_double_value_unsupported() {
        let props = MappedIdNodePropertyValues::new(100);
        assert!(props.double_value(0).is_err());
    }

    #[test]
    fn test_array_values_unsupported() {
        let props = MappedIdNodePropertyValues::new(100);
        assert!(props.double_array_value(0).is_err());
        assert!(props.float_array_value(0).is_err());
        assert!(props.long_array_value(0).is_err());
    }

    #[test]
    fn test_get_object() {
        let props = MappedIdNodePropertyValues::new(100);
        let obj = props.get_object(42).unwrap();
        let value = obj.downcast_ref::<i64>().unwrap();
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_zero_node_count() {
        let props = MappedIdNodePropertyValues::new(0);
        assert_eq!(props.node_count(), 0);
        assert_eq!(props.get_max_long_property_value(), None);
        assert!(!props.has_value(0));
    }

    #[test]
    fn test_large_node_count() {
        let props = MappedIdNodePropertyValues::new(1_000_000);
        assert_eq!(props.node_count(), 1_000_000);
        assert_eq!(props.long_value(999_999).unwrap(), 999_999);
        assert_eq!(props.get_max_long_property_value(), Some(999_999));
    }
}

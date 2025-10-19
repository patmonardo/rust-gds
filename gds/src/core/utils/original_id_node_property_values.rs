//! Node property values implementation where the property value is the original ID of the node.
//!
//! This implementation maps internal (mapped) node IDs back to their original IDs from the source graph,
//! essential for algorithms that need to output results in terms of the original node identifiers.

use crate::types::graph::id_map::IdMap;
use crate::types::properties::node::LongNodePropertyValues;
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;

/// A node property values implementation where the property value is the original ID of the node.
///
/// This implementation provides a way to expose original node IDs (typically from the source database)
/// as properties, commonly used for:
/// - Export scenarios where original IDs are required
/// - Result formatting that needs to reference source graph IDs
/// - Algorithms that track nodes by their database identifiers
///
/// # Examples
///
/// ```ignore
/// use gds::core::utils::OriginalIdNodePropertyValues;
/// use gds::types::graph::id_map::IdMap;
///
/// let id_map = /* some IdMap implementation */;
/// let props = OriginalIdNodePropertyValues::from_id_map(&id_map);
///
/// // Property value is the original ID from the source graph
/// let original_id = props.long_value(42).unwrap();
/// ```
pub struct OriginalIdNodePropertyValues {
    to_original_node_id: Box<dyn Fn(u64) -> Option<i64> + Send + Sync>,
    node_count: usize,
}

impl std::fmt::Debug for OriginalIdNodePropertyValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OriginalIdNodePropertyValues")
            .field("node_count", &self.node_count)
            .field("to_original_node_id", &"<closure>")
            .finish()
    }
}

impl OriginalIdNodePropertyValues {
    /// Creates a new original ID node property values from an IdMap.
    ///
    /// # Arguments
    ///
    /// * `id_map` - ID map used to map node IDs to their original IDs
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use gds::core::utils::OriginalIdNodePropertyValues;
    /// use gds::types::graph::id_map::IdMap;
    ///
    /// let id_map = /* some IdMap implementation */;
    /// let props = OriginalIdNodePropertyValues::from_id_map(&id_map);
    /// ```
    pub fn from_id_map<T: IdMap + ?Sized>(id_map: &T) -> Self {
        // Capture the original IDs at creation time to avoid lifetime issues
        let node_count = id_map.node_count();
        let original_ids: Vec<Option<i64>> = (0..node_count)
            .map(|i| id_map.to_original_node_id(i as u64))
            .collect();

        Self {
            to_original_node_id: Box::new(move |node_id| {
                original_ids.get(node_id as usize).and_then(|&id| id)
            }),
            node_count,
        }
    }

    /// Creates a new original ID node property values with a custom mapping function.
    ///
    /// # Arguments
    ///
    /// * `to_original_node_id` - Function that maps node IDs to their original IDs
    /// * `node_count` - The number of nodes
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::OriginalIdNodePropertyValues;
    ///
    /// // Simple identity mapping (mapped ID = original ID)
    /// let props = OriginalIdNodePropertyValues::new(
    ///     |node_id| Some(node_id as i64),
    ///     1000
    /// );
    ///
    /// assert_eq!(props.long_value(42).unwrap(), 42);
    /// ```
    pub fn new<F>(to_original_node_id: F, node_count: usize) -> Self
    where
        F: Fn(u64) -> Option<i64> + Send + Sync + 'static,
    {
        Self {
            to_original_node_id: Box::new(to_original_node_id),
            node_count,
        }
    }
}

impl PropertyValues for OriginalIdNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl crate::types::properties::node::NodePropertyValues for OriginalIdNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::unsupported_operation(
            "OriginalIdNodePropertyValues does not support double values",
        ))
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        (self.to_original_node_id)(node_id).ok_or_else(|| {
            PropertyValuesError::unsupported_operation(format!(
                "No original ID for node {}",
                node_id
            ))
        })
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::unsupported_operation(
            "OriginalIdNodePropertyValues does not support double array values",
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::unsupported_operation(
            "OriginalIdNodePropertyValues does not support float array values",
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_operation(
            "OriginalIdNodePropertyValues does not support long array values",
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        self.long_value(node_id)
            .map(|id| Box::new(id) as Box<dyn std::any::Any>)
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        // We don't know the max original ID without scanning all mappings
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.node_count && (self.to_original_node_id)(node_id).is_some()
    }

    fn node_count(&self) -> usize {
        self.node_count
    }
}

impl LongNodePropertyValues for OriginalIdNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        (self.to_original_node_id)(node_id).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::NodePropertyValues;

    #[test]
    fn test_new_with_identity_mapping() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);

        assert_eq!(props.node_count(), 100);
        assert_eq!(props.long_value(0).unwrap(), 0);
        assert_eq!(props.long_value(42).unwrap(), 42);
        assert_eq!(props.long_value(99).unwrap(), 99);
    }

    #[test]
    fn test_new_with_offset_mapping() {
        // Original IDs are offset by 1000
        let props = OriginalIdNodePropertyValues::new(|node_id| Some((node_id + 1000) as i64), 100);

        assert_eq!(props.long_value(0).unwrap(), 1000);
        assert_eq!(props.long_value(42).unwrap(), 1042);
        assert_eq!(props.long_value(99).unwrap(), 1099);
    }

    #[test]
    fn test_new_with_custom_mapping() {
        // Custom mapping: even nodes map to themselves * 2, odd nodes to themselves * 3
        let props = OriginalIdNodePropertyValues::new(
            |node_id| {
                if node_id % 2 == 0 {
                    Some((node_id * 2) as i64)
                } else {
                    Some((node_id * 3) as i64)
                }
            },
            100,
        );

        assert_eq!(props.long_value(0).unwrap(), 0);
        assert_eq!(props.long_value(2).unwrap(), 4);
        assert_eq!(props.long_value(1).unwrap(), 3);
        assert_eq!(props.long_value(3).unwrap(), 9);
    }

    #[test]
    fn test_new_with_sparse_mapping() {
        // Only even node IDs have mappings
        let props = OriginalIdNodePropertyValues::new(
            |node_id| {
                if node_id % 2 == 0 {
                    Some((node_id * 10) as i64)
                } else {
                    None
                }
            },
            100,
        );

        assert_eq!(props.long_value(0).unwrap(), 0);
        assert_eq!(props.long_value(2).unwrap(), 20);
        assert!(props.long_value(1).is_err());
        assert!(props.long_value(3).is_err());
    }

    #[test]
    fn test_long_value_unchecked() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some((node_id + 100) as i64), 100);

        assert_eq!(props.long_value_unchecked(0), 100);
        assert_eq!(props.long_value_unchecked(50), 150);
        assert_eq!(props.long_value_unchecked(99), 199);
    }

    #[test]
    fn test_long_value_unchecked_with_none() {
        let props = OriginalIdNodePropertyValues::new(|_| None, 100);

        // Should return -1 for unmapped nodes
        assert_eq!(props.long_value_unchecked(0), -1);
        assert_eq!(props.long_value_unchecked(50), -1);
    }

    #[test]
    fn test_value_type() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        assert_eq!(props.value_type(), ValueType::Long);
    }

    #[test]
    fn test_element_count() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 5000);
        assert_eq!(props.element_count(), 5000);
    }

    #[test]
    fn test_dimension() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        assert_eq!(props.dimension(), Some(1));
    }

    #[test]
    fn test_get_max_long_property_value() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        // We don't know the max without scanning, so it returns None
        assert_eq!(props.get_max_long_property_value(), None);
    }

    #[test]
    fn test_get_max_double_property_value() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        assert_eq!(props.get_max_double_property_value(), None);
    }

    #[test]
    fn test_has_value() {
        let props = OriginalIdNodePropertyValues::new(
            |node_id| {
                if node_id % 2 == 0 {
                    Some(node_id as i64)
                } else {
                    None
                }
            },
            100,
        );

        assert!(props.has_value(0));
        assert!(props.has_value(2));
        assert!(!props.has_value(1));
        assert!(!props.has_value(3));
        assert!(!props.has_value(100)); // Out of bounds
    }

    #[test]
    fn test_double_value_unsupported() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        assert!(props.double_value(0).is_err());
    }

    #[test]
    fn test_array_values_unsupported() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        assert!(props.double_array_value(0).is_err());
        assert!(props.float_array_value(0).is_err());
        assert!(props.long_array_value(0).is_err());
    }

    #[test]
    fn test_get_object() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 100);
        let obj = props.get_object(42).unwrap();
        let value = obj.downcast_ref::<i64>().unwrap();
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_zero_node_count() {
        let props = OriginalIdNodePropertyValues::new(|node_id| Some(node_id as i64), 0);
        assert_eq!(props.node_count(), 0);
        assert!(!props.has_value(0));
    }

    #[test]
    fn test_large_node_count() {
        let props =
            OriginalIdNodePropertyValues::new(|node_id| Some((node_id * 1000) as i64), 1_000_000);
        assert_eq!(props.node_count(), 1_000_000);
        assert_eq!(props.long_value(999_999).unwrap(), 999_999_000);
    }

    #[test]
    fn test_database_id_scenario() {
        // Simulate database IDs that are non-sequential
        let db_ids = vec![1001, 2003, 3005, 4007, 5009];
        let node_count = db_ids.len();
        let props = OriginalIdNodePropertyValues::new(
            move |node_id| db_ids.get(node_id as usize).copied(),
            node_count,
        );

        assert_eq!(props.long_value(0).unwrap(), 1001);
        assert_eq!(props.long_value(1).unwrap(), 2003);
        assert_eq!(props.long_value(2).unwrap(), 3005);
        assert_eq!(props.long_value(3).unwrap(), 4007);
        assert_eq!(props.long_value(4).unwrap(), 5009);
    }
}

//! Consecutive Community ID Mapping
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.community.ConsecutiveLongNodePropertyValues`
//!
//! This module provides node property values that remap community IDs to consecutive integers.

use super::super::stubs::{LongNodePropertyValues, FilteredNodePropertyValuesMarker, ValueType, NodePropertyValues};
use std::collections::HashMap;

/// Node property values that remap community IDs to consecutive integers
///
/// Translation of: `org.neo4j.gds.algorithms.community.ConsecutiveLongNodePropertyValues`
///
/// ## Java GDS Source
///
/// ```java
/// public class ConsecutiveLongNodePropertyValues implements LongNodePropertyValues, FilteredNodePropertyValuesMarker {
///     private static final long MAPPING_SIZE_QUOTIENT = 10L;
///     private static final long NO_VALUE = -1L;
///     
///     private final HugeLongArray communities;
///     
///     public ConsecutiveLongNodePropertyValues(LongNodePropertyValues inputProperties) {
///         // Build mapping from original IDs to consecutive IDs
///         // Store remapped values in communities array
///     }
/// }
/// ```
///
/// ## Purpose
///
/// Community detection algorithms may produce non-consecutive community IDs (e.g., 5, 10, 15, 20).
/// This class remaps them to consecutive integers (0, 1, 2, 3) for cleaner output and storage.
///
/// ## Example
///
/// ```rust,ignore
/// use gds::procedures::algorithms::community::ConsecutiveLongNodePropertyValues;
///
/// let original_props = /* community results with IDs [10, 20, 10, 30, 20] */;
/// let consecutive = ConsecutiveLongNodePropertyValues::new(original_props);
/// 
/// // Now consecutive.long_value(0) = 0, consecutive.long_value(1) = 1, etc.
/// // Original ID 10 -> consecutive ID 0
/// // Original ID 20 -> consecutive ID 1  
/// // Original ID 30 -> consecutive ID 2
/// ```
pub struct ConsecutiveLongNodePropertyValues {
    /// Remapped community IDs (original ID -> consecutive ID)
    communities: Vec<i64>,
    /// Number of nodes
    node_count: usize,
}

impl ConsecutiveLongNodePropertyValues {
    /// Sentinel value indicating no community assignment
    const NO_VALUE: i64 = -1;

    /// Create consecutive community ID mapping
    ///
    /// Translation of: Constructor (lines 36-59)
    ///
    /// ## Parameters
    ///
    /// - `input_properties`: Original community property values with non-consecutive IDs
    ///
    /// ## Process
    ///
    /// 1. Iterate through all nodes
    /// 2. For each node with a community ID, map it to a consecutive ID
    /// 3. Store the consecutive ID in the communities array
    /// 4. Nodes without communities get `NO_VALUE` (-1)
    pub fn new(input_properties: Box<dyn LongNodePropertyValues>) -> Self {
        let node_count = input_properties.node_count();
        let mut communities = vec![Self::NO_VALUE; node_count];
        let mut set_id_to_consecutive_id: HashMap<i64, i64> = HashMap::new();
        let mut next_consecutive_id = -1i64;

        // Build mapping from original community IDs to consecutive IDs
        // Translation of: lines 46-58
        for node_id in 0..node_count {
            if input_properties.has_value(node_id) {
                let set_id = input_properties.long_value(node_id);
                let community_id = *set_id_to_consecutive_id
                    .entry(set_id)
                    .or_insert_with(|| {
                        next_consecutive_id += 1;
                        next_consecutive_id
                    });
                communities[node_id] = community_id;
            }
        }

        Self {
            communities,
            node_count,
        }
    }
}

impl LongNodePropertyValues for ConsecutiveLongNodePropertyValues {
    /// Get the consecutive community ID for a node
    ///
    /// Translation of: `longValue()` (lines 69-76)
    ///
    /// ## Returns
    ///
    /// - Consecutive community ID if node has a community
    /// - `i64::MIN` if node has no community (sentinel value for filtering)
    fn long_value(&self, node_id: usize) -> i64 {
        let value = self.communities[node_id];
        if value == Self::NO_VALUE {
            i64::MIN // Sentinel value indicating "no value"
        } else {
            value
        }
    }

    /// Check if a node has a community assignment
    ///
    /// Translation of: `hasValue()` (lines 78-81)
    fn has_value(&self, node_id: usize) -> bool {
        self.communities[node_id] != Self::NO_VALUE
    }
}

impl NodePropertyValues for ConsecutiveLongNodePropertyValues {
    /// Get the number of nodes
    ///
    /// Translation of: `nodeCount()` (lines 83-86)
    fn node_count(&self) -> usize {
        self.node_count
    }

    /// Get the value type
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }
}

/// Marker trait indicating this is a filtered property values implementation
impl FilteredNodePropertyValuesMarker for ConsecutiveLongNodePropertyValues {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::algorithms::stubs::{LongNodePropertyValues, ValueType};

    // Test implementation
    struct TestLongProperty {
        values: Vec<i64>,
    }

    impl NodePropertyValues for TestLongProperty {
        fn node_count(&self) -> usize {
            self.values.len()
        }

        fn value_type(&self) -> ValueType {
            ValueType::Long
        }
    }

    impl LongNodePropertyValues for TestLongProperty {
        fn long_value(&self, node_id: usize) -> i64 {
            self.values[node_id]
        }

        fn has_value(&self, node_id: usize) -> bool {
            self.values[node_id] != i64::MIN
        }
    }

    #[test]
    fn test_consecutive_mapping_basic() {
        let input = Box::new(TestLongProperty {
            values: vec![10, 20, 10, 30, 20], // Non-consecutive IDs
        });

        let consecutive = ConsecutiveLongNodePropertyValues::new(input);

        assert_eq!(consecutive.node_count(), 5);
        assert_eq!(consecutive.value_type(), ValueType::Long);

        // Should remap: 10->0, 20->1, 30->2
        assert_eq!(consecutive.long_value(0), 0); // 10 -> 0
        assert_eq!(consecutive.long_value(1), 1); // 20 -> 1
        assert_eq!(consecutive.long_value(2), 0); // 10 -> 0
        assert_eq!(consecutive.long_value(3), 2); // 30 -> 2
        assert_eq!(consecutive.long_value(4), 1); // 20 -> 1

        // All nodes should have values
        for i in 0..5 {
            assert!(consecutive.has_value(i));
        }
    }

    #[test]
    fn test_consecutive_mapping_with_no_values() {
        let input = Box::new(TestLongProperty {
            values: vec![10, i64::MIN, 20, i64::MIN, 30], // Some nodes have no community
        });

        let consecutive = ConsecutiveLongNodePropertyValues::new(input);

        assert_eq!(consecutive.node_count(), 5);

        // Should remap: 10->0, 20->1, 30->2
        assert_eq!(consecutive.long_value(0), 0); // 10 -> 0
        assert_eq!(consecutive.long_value(1), i64::MIN); // No value
        assert_eq!(consecutive.long_value(2), 1); // 20 -> 1
        assert_eq!(consecutive.long_value(3), i64::MIN); // No value
        assert_eq!(consecutive.long_value(4), 2); // 30 -> 2

        // Check has_value
        assert!(consecutive.has_value(0));
        assert!(!consecutive.has_value(1));
        assert!(consecutive.has_value(2));
        assert!(!consecutive.has_value(3));
        assert!(consecutive.has_value(4));
    }

    #[test]
    fn test_consecutive_mapping_single_community() {
        let input = Box::new(TestLongProperty {
            values: vec![100, 100, 100, 100], // All same community
        });

        let consecutive = ConsecutiveLongNodePropertyValues::new(input);

        assert_eq!(consecutive.node_count(), 4);

        // All should map to 0
        for i in 0..4 {
            assert_eq!(consecutive.long_value(i), 0);
            assert!(consecutive.has_value(i));
        }
    }

    #[test]
    fn test_consecutive_mapping_empty() {
        let input = Box::new(TestLongProperty {
            values: vec![],
        });

        let consecutive = ConsecutiveLongNodePropertyValues::new(input);

        assert_eq!(consecutive.node_count(), 0);
    }

    #[test]
    fn test_consecutive_mapping_all_no_values() {
        let input = Box::new(TestLongProperty {
            values: vec![i64::MIN, i64::MIN, i64::MIN],
        });

        let consecutive = ConsecutiveLongNodePropertyValues::new(input);

        assert_eq!(consecutive.node_count(), 3);

        // All should have no values
        for i in 0..3 {
            assert_eq!(consecutive.long_value(i), i64::MIN);
            assert!(!consecutive.has_value(i));
        }
    }
}

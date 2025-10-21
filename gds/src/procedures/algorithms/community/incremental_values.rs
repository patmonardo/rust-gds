//! Incremental Community Detection Support
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.community.LongIfChangedNodePropertyValues`
//!
//! This module provides node property values that only writes changed values for incremental algorithms.

use super::super::stubs::{LongNodePropertyValues, FilteredNodePropertyValuesMarker, ValueType, NodePropertyValues};

/// Node property values that only writes changed values (for incremental algorithms)
///
/// Translation of: `org.neo4j.gds.algorithms.community.LongIfChangedNodePropertyValues`
///
/// ## Java GDS Source
///
/// ```java
/// public final class LongIfChangedNodePropertyValues implements LongNodePropertyValues, FilteredNodePropertyValuesMarker {
///     private final NodePropertyValues seedProperties;
///     private final NodePropertyValues newProperties;
///     
///     public static LongNodePropertyValues of(NodeProperty seedProperty, LongNodePropertyValues newProperties) {
///         // Check if seed property is persistent
///         // Return LongIfChangedNodePropertyValues or newProperties based on property state
///     }
///     
///     @Override
///     public long longValue(long nodeId) {
///         var seedValue = seedProperties.longValue(nodeId);
///         var writeValue = newProperties.longValue(nodeId);
///         return (seedValue != writeValue) ? writeValue : Long.MIN_VALUE;
///     }
/// }
/// ```
///
/// ## Purpose
///
/// For incremental community detection algorithms (e.g., iterative Louvain), we only want to write
/// nodes whose community assignment changed. This class filters out unchanged values to reduce
/// I/O and improve performance.
///
/// ## Example
///
/// ```rust,ignore
/// use gds::procedures::algorithms::community::LongIfChangedNodePropertyValues;
///
/// let seed_props = /* previous iteration results */;
/// let new_props = /* current iteration results */;
/// let incremental = LongIfChangedNodePropertyValues::new(seed_props, new_props);
/// 
/// // Only nodes whose community changed will have values written
/// // Unchanged nodes return i64::MIN (sentinel value)
/// ```
pub struct LongIfChangedNodePropertyValues {
    /// Previous iteration property values (seed)
    seed_properties: Box<dyn LongNodePropertyValues>,
    /// Current iteration property values (new)
    new_properties: Box<dyn LongNodePropertyValues>,
}

impl LongIfChangedNodePropertyValues {
    /// Create incremental property values
    ///
    /// Translation of: `of()` (lines 36-54)
    ///
    /// ## Parameters
    ///
    /// - `seed_properties`: Previous iteration property values
    /// - `new_properties`: Current iteration property values
    ///
    /// ## Returns
    ///
    /// Node property values that only writes changed values
    ///
    /// ## Note
    ///
    /// In the Java version, this method checks if the seed property is persistent.
    /// If not persistent, it returns the new properties directly. For now, we always
    /// create the incremental wrapper.
    pub fn new(
        seed_properties: Box<dyn LongNodePropertyValues>,
        new_properties: Box<dyn LongNodePropertyValues>,
    ) -> Self {
        Self {
            seed_properties,
            new_properties,
        }
    }
}

impl LongNodePropertyValues for LongIfChangedNodePropertyValues {
    /// Only return value if it changed from seed
    ///
    /// Translation of: `longValue()` (lines 72-78)
    ///
    /// ## Returns
    ///
    /// - New value if it differs from seed value
    /// - `i64::MIN` if value unchanged (sentinel value for filtering)
    fn long_value(&self, node_id: usize) -> i64 {
        let seed_value = if node_id < self.seed_properties.node_count() {
            self.seed_properties.long_value(node_id)
        } else {
            i64::MIN
        };
        let write_value = if node_id < self.new_properties.node_count() {
            self.new_properties.long_value(node_id)
        } else {
            i64::MIN
        };

        if seed_value != write_value {
            write_value
        } else {
            i64::MIN // Sentinel: don't write unchanged values
        }
    }

    /// Has value only if it changed
    ///
    /// Translation of: `hasValue()` (lines 86-90)
    ///
    /// ## Returns
    ///
    /// - `true` if seed had no value OR if new value differs from seed
    /// - `false` if new value same as seed
    fn has_value(&self, node_id: usize) -> bool {
        let seed_value = if node_id < self.seed_properties.node_count() {
            self.seed_properties.long_value(node_id)
        } else {
            i64::MIN
        };
        let write_value = if node_id < self.new_properties.node_count() {
            self.new_properties.long_value(node_id)
        } else {
            i64::MIN
        };
        seed_value == i64::MIN || (seed_value != write_value)
    }
}

impl NodePropertyValues for LongIfChangedNodePropertyValues {
    /// Get the number of nodes
    ///
    /// Translation of: `nodeCount()` (lines 80-83)
    ///
    /// ## Returns
    ///
    /// Maximum of seed and new property node counts
    fn node_count(&self) -> usize {
        self.seed_properties.node_count().max(self.new_properties.node_count())
    }

    /// Get the value type
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }
}

/// Marker trait indicating this is a filtered property values implementation
impl FilteredNodePropertyValuesMarker for LongIfChangedNodePropertyValues {}

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
    fn test_incremental_values_changed() {
        let seed = Box::new(TestLongProperty {
            values: vec![1, 2, 3, 4, 5],
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 6, 4, 7], // Changed: index 2 (3->6), index 4 (5->7)
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        assert_eq!(incremental.node_count(), 5);
        assert_eq!(incremental.value_type(), ValueType::Long);

        // Unchanged values should return i64::MIN
        assert_eq!(incremental.long_value(0), i64::MIN); // 1 -> 1 (unchanged)
        assert_eq!(incremental.long_value(1), i64::MIN); // 2 -> 2 (unchanged)
        assert_eq!(incremental.long_value(3), i64::MIN); // 4 -> 4 (unchanged)

        // Changed values should return new value
        assert_eq!(incremental.long_value(2), 6); // 3 -> 6 (changed)
        assert_eq!(incremental.long_value(4), 7); // 5 -> 7 (changed)

        // Check has_value
        assert!(!incremental.has_value(0)); // Unchanged
        assert!(!incremental.has_value(1)); // Unchanged
        assert!(incremental.has_value(2));  // Changed
        assert!(!incremental.has_value(3)); // Unchanged
        assert!(incremental.has_value(4));  // Changed
    }

    #[test]
    fn test_incremental_values_no_seed() {
        let seed = Box::new(TestLongProperty {
            values: vec![i64::MIN, i64::MIN, i64::MIN], // No seed values
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 3], // New values
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        assert_eq!(incremental.node_count(), 3);

        // All values should be written (no seed values)
        assert_eq!(incremental.long_value(0), 1);
        assert_eq!(incremental.long_value(1), 2);
        assert_eq!(incremental.long_value(2), 3);

        assert!(incremental.has_value(0));
        assert!(incremental.has_value(1));
        assert!(incremental.has_value(2));
    }

    #[test]
    fn test_incremental_values_all_unchanged() {
        let seed = Box::new(TestLongProperty {
            values: vec![1, 2, 3],
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 3], // Same values
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        assert_eq!(incremental.node_count(), 3);

        // All values should be filtered out (unchanged)
        assert_eq!(incremental.long_value(0), i64::MIN);
        assert_eq!(incremental.long_value(1), i64::MIN);
        assert_eq!(incremental.long_value(2), i64::MIN);

        assert!(!incremental.has_value(0));
        assert!(!incremental.has_value(1));
        assert!(!incremental.has_value(2));
    }

    #[test]
    fn test_incremental_values_different_sizes() {
        let seed = Box::new(TestLongProperty {
            values: vec![1, 2], // Smaller
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 3], // Larger
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        // Should use maximum size
        assert_eq!(incremental.node_count(), 3);

        // Existing nodes
        assert_eq!(incremental.long_value(0), i64::MIN); // 1 -> 1 (unchanged)
        assert_eq!(incremental.long_value(1), i64::MIN); // 2 -> 2 (unchanged)

        // New node (no seed value)
        assert_eq!(incremental.long_value(2), 3); // No seed -> 3 (changed)

        assert!(!incremental.has_value(0));
        assert!(!incremental.has_value(1));
        assert!(incremental.has_value(2));
    }
}

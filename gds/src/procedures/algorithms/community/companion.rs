//! Community Detection Companion Utilities
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.community.CommunityCompanion`
//!
//! This module provides utility functions for community detection algorithms.

use super::super::stubs::{LongNodePropertyValues, NodePropertyValues, ValueType};
use super::consecutive_values::ConsecutiveLongNodePropertyValues;
use super::incremental_values::LongIfChangedNodePropertyValues;

/// Community algorithm companion utilities
///
/// Translation of: `org.neo4j.gds.algorithms.community.CommunityCompanion`
///
/// ## Java GDS Source
///
/// ```java
/// public final class CommunityCompanion {
///     public static NodePropertyValues nodePropertyValues(
///         boolean consecutiveIds,
///         LongNodePropertyValues nodeProperties
///     ) { /* ... */ }
///     
///     public static NodePropertyValues nodePropertyValues(
///         boolean incremental,
///         String resultProperty,
///         String seedProperty,
///         boolean consecutiveIds,
///         LongNodePropertyValues nodeProperties,
///         Optional<Long> minCommunitySize,
///         Concurrency concurrency,
///         Supplier<NodeProperty> seedPropertySupplier
///     ) { /* ... */ }
/// }
/// ```
pub struct CommunityCompanion;

impl CommunityCompanion {
    /// Create node property values, optionally with consecutive IDs
    ///
    /// Translation of: `nodePropertyValues(boolean consecutiveIds, LongNodePropertyValues nodeProperties)`
    /// (lines 44-54)
    ///
    /// ## Parameters
    ///
    /// - `consecutive_ids`: Whether to remap community IDs to consecutive integers
    /// - `node_properties`: Original community property values
    ///
    /// ## Returns
    ///
    /// Node property values with optional consecutive ID mapping
    pub fn node_property_values(
        consecutive_ids: bool,
        node_properties: Box<dyn LongNodePropertyValues>,
    ) -> Box<dyn LongNodePropertyValues> {
        if consecutive_ids {
            Box::new(ConsecutiveLongNodePropertyValues::new(node_properties))
        } else {
            node_properties
        }
    }

    /// Create node property values with incremental detection support
    ///
    /// Translation of: `nodePropertyValues()` with incremental parameters (lines 56-98)
    ///
    /// ## Parameters
    ///
    /// - `incremental`: Whether this is an incremental algorithm
    /// - `result_property`: Name of result property
    /// - `seed_property`: Name of seed property
    /// - `consecutive_ids`: Whether to remap community IDs to consecutive integers
    /// - `node_properties`: Original community property values
    /// - `min_community_size`: Minimum community size filter (optional)
    /// - `concurrency`: Number of threads for parallel processing
    ///
    /// ## Returns
    ///
    /// Node property values with optional incremental filtering and consecutive ID mapping
    pub fn node_property_values_with_incremental(
        incremental: bool,
        result_property: &str,
        seed_property: &str,
        consecutive_ids: bool,
        node_properties: Box<dyn LongNodePropertyValues>,
        min_community_size: Option<usize>,
        concurrency: usize,
    ) -> Box<dyn LongNodePropertyValues> {
        // Apply minimum community size filter if specified
        // Translation of: applySizeFilter() (lines 86-88)
        let filtered = if let Some(min_size) = min_community_size {
            // TODO: Implement CommunitySizeFilter when we have HugeSparseLongArray
            // For now, return original properties
            node_properties
        } else {
            node_properties
        };

        // Apply incremental filtering if needed
        // Translation of: lines 68-70
        let incremental_filtered = if incremental && result_property == seed_property {
            // TODO: Implement LongIfChangedNodePropertyValues when we have seed property supplier
            // For now, return filtered properties
            filtered
        } else {
            filtered
        };

        // Apply consecutive ID mapping
        Self::node_property_values(consecutive_ids, incremental_filtered)
    }

    /// Create node property values with minimum community size filter
    ///
    /// Translation of: `nodePropertyValues()` with min community size (lines 100-111)
    ///
    /// ## Parameters
    ///
    /// - `consecutive_ids`: Whether to remap community IDs to consecutive integers
    /// - `node_properties`: Original community property values
    /// - `min_community_size`: Minimum community size filter (optional)
    /// - `concurrency`: Number of threads for parallel processing
    ///
    /// ## Returns
    ///
    /// Node property values with optional community size filtering and consecutive ID mapping
    pub fn node_property_values_with_filter(
        consecutive_ids: bool,
        node_properties: Box<dyn LongNodePropertyValues>,
        min_community_size: Option<usize>,
        concurrency: usize,
    ) -> Box<dyn LongNodePropertyValues> {
        let filtered = if let Some(min_size) = min_community_size {
            // TODO: Implement CommunitySizeFilter when we have HugeSparseLongArray
            // For now, return original properties
            node_properties
        } else {
            node_properties
        };

        Self::node_property_values(consecutive_ids, filtered)
    }

    /// Extract seeding node property values
    ///
    /// Translation of: `extractSeedingNodePropertyValues()` (lines 197-211)
    ///
    /// ## Parameters
    ///
    /// - `property_name`: Name of the seeding property
    ///
    /// ## Returns
    ///
    /// Node property values for seeding, or error if property not found or wrong type
    ///
    /// ## TODO
    ///
    /// This method requires `NodePropertyContainer` which we haven't translated yet.
    /// Will be implemented when we translate the core API.
    pub fn extract_seeding_property(
        property_name: &str,
        // TODO: Accept NodePropertyContainer when available
    ) -> Result<Box<dyn NodePropertyValues>, String> {
        // Stub for now - will be implemented when we have full property system
        Err(format!(
            "Not yet implemented - requires NodePropertyContainer for property '{}'",
            property_name
        ))
    }
}

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
    fn test_node_property_values_basic() {
        let props = Box::new(TestLongProperty {
            values: vec![1, 2, 3, 4, 5],
        });

        // Test without consecutive IDs
        let result = CommunityCompanion::node_property_values(false, props);
        assert_eq!(result.node_count(), 5);
        assert_eq!(result.long_value(0), 1);
        assert_eq!(result.long_value(4), 5);
    }

    #[test]
    fn test_node_property_values_with_consecutive_ids() {
        let props = Box::new(TestLongProperty {
            values: vec![10, 20, 10, 30, 20], // Non-consecutive IDs
        });

        // Test with consecutive IDs
        let result = CommunityCompanion::node_property_values(true, props);
        assert_eq!(result.node_count(), 5);
        
        // Should remap 10->0, 20->1, 30->2
        assert_eq!(result.long_value(0), 0); // 10 -> 0
        assert_eq!(result.long_value(1), 1); // 20 -> 1
        assert_eq!(result.long_value(2), 0); // 10 -> 0
        assert_eq!(result.long_value(3), 2); // 30 -> 2
        assert_eq!(result.long_value(4), 1); // 20 -> 1
    }

    #[test]
    fn test_node_property_values_with_filter() {
        let props = Box::new(TestLongProperty {
            values: vec![1, 2, 3, 4, 5],
        });

        // Test with minimum community size filter
        let result = CommunityCompanion::node_property_values_with_filter(
            false, // consecutive_ids
            props,
            Some(3), // min_community_size
            1,       // concurrency
        );

        // TODO: When CommunitySizeFilter is implemented, this should filter communities
        // For now, just verify it returns something
        assert_eq!(result.node_count(), 5);
    }

    #[test]
    fn test_extract_seeding_property_stub() {
        let result = CommunityCompanion::extract_seeding_property("test_property");
        assert!(result.is_err());
        // Just verify we get an error - don't try to unwrap it due to Debug trait requirements
    }
}

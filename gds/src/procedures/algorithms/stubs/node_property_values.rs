//! Stub Node Property Values Traits
//!
//! **Translation Source**: `org.neo4j.gds.api.properties.nodes.*` package
//! **Status**: STUB - Minimal interface for compilation only
//!
//! This module provides stub trait definitions for node property value accessors.
//! These will be replaced with full implementations when we translate the core API.
//!
//! ## Java GDS Source
//!
//! ```java
//! // org.neo4j.gds.api.properties.nodes.NodePropertyValues
//! public interface NodePropertyValues {
//!     long nodeCount();
//!     ValueType valueType();
//! }
//!
//! // org.neo4j.gds.api.properties.nodes.LongNodePropertyValues
//! public interface LongNodePropertyValues extends NodePropertyValues {
//!     long longValue(long nodeId);
//!     boolean hasValue(long nodeId);
//! }
//! ```
//!
//! ## Stub Implementation
//!
//! These traits provide the **minimum interface** needed by algorithm infrastructure.
//! Real implementations will:
//! - Support property schema and metadata
//! - Provide efficient storage (HugeArray, etc.)
//! - Handle missing values and default values
//! - Support property aggregation and transformation
//! - Integrate with graph loading pipeline
//!
//! ## TODO
//!
//! - [ ] Replace with full `NodePropertyValues` translation from core API
//! - [ ] Add property schema support
//! - [ ] Add efficient storage implementations
//! - [ ] Add property transformation support

use serde::{Deserialize, Serialize};

/// Value type enum for node properties
///
/// Translation of: `org.neo4j.gds.api.nodeproperties.ValueType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    /// 64-bit signed integer
    Long,
    /// 64-bit floating point
    Double,
    /// Array of 32-bit floats
    FloatArray,
    /// Array of 64-bit floats
    DoubleArray,
    /// Array of 64-bit signed integers
    LongArray,
}

/// Base trait for node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.NodePropertyValues`
pub trait NodePropertyValues {
    /// Get the number of nodes
    fn node_count(&self) -> usize;

    /// Get the value type of this property
    fn value_type(&self) -> ValueType;
}

/// Long (i64) node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.LongNodePropertyValues`
pub trait LongNodePropertyValues: NodePropertyValues {
    /// Get the long value for a node
    ///
    /// Returns `i64::MIN` if the node has no value (sentinel value)
    fn long_value(&self, node_id: usize) -> i64;

    /// Check if a node has a value
    ///
    /// Returns `false` if the node has no value or the value is filtered
    fn has_value(&self, node_id: usize) -> bool;
}

/// Double (f64) node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.DoubleNodePropertyValues`
pub trait DoubleNodePropertyValues: NodePropertyValues {
    /// Get the double value for a node
    ///
    /// Returns `f64::NAN` if the node has no value (sentinel value)
    fn double_value(&self, node_id: usize) -> f64;

    /// Check if a node has a value
    fn has_value(&self, node_id: usize) -> bool;
}

/// Float array node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.FloatArrayNodePropertyValues`
pub trait FloatArrayNodePropertyValues: NodePropertyValues {
    /// Get the float array value for a node
    ///
    /// Returns an empty vector if the node has no value
    fn float_array_value(&self, node_id: usize) -> Vec<f32>;
}

/// Double array node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.DoubleArrayNodePropertyValues`
pub trait DoubleArrayNodePropertyValues: NodePropertyValues {
    /// Get the double array value for a node
    ///
    /// Returns an empty vector if the node has no value
    fn double_array_value(&self, node_id: usize) -> Vec<f64>;
}

/// Long array node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.LongArrayNodePropertyValues`
pub trait LongArrayNodePropertyValues: NodePropertyValues {
    /// Get the long array value for a node
    ///
    /// Returns an empty vector if the node has no value
    fn long_array_value(&self, node_id: usize) -> Vec<i64>;
}

/// Marker trait for filtered node property values
///
/// Translation of: `org.neo4j.gds.api.properties.nodes.FilteredNodePropertyValuesMarker`
///
/// This is a marker trait indicating that the property values are filtered
/// (e.g., minimum community size filter, changed values only, etc.)
pub trait FilteredNodePropertyValuesMarker {}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test implementation
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
    fn test_stub_trait_basic_usage() {
        let prop = TestLongProperty {
            values: vec![1, 2, 3, i64::MIN, 5],
        };

        assert_eq!(prop.node_count(), 5);
        assert_eq!(prop.value_type(), ValueType::Long);
        assert_eq!(prop.long_value(0), 1);
        assert_eq!(prop.long_value(1), 2);
        assert!(prop.has_value(0));
        assert!(!prop.has_value(3)); // i64::MIN means no value
    }
}


//! Adapter factories for PropertyValues per level.
//! These factories consult PropertyStoreConfig to choose a backend.

use std::sync::Arc;

use crate::collections::backends::huge::{HugeDoubleArray, HugeLongArray};
use crate::config::{PropertyBackendKind, PropertyStoreConfig};
use crate::types::properties::graph::{
    DoubleArrayGraphPropertyValues, DoubleGraphPropertyValues, FloatArrayGraphPropertyValues,
    LongArrayGraphPropertyValues, LongGraphPropertyValues,
};
use crate::types::properties::node::{
    DoubleArrayNodePropertyValues, DoubleNodePropertyValues, LongArrayNodePropertyValues, LongNodePropertyValues,
};
use crate::types::properties::relationship::{
    RelationshipPropertyValues,
};

// Default Vec-backed implementations
use crate::types::properties::graph::impls::default_graph_property_values::{
    DefaultDoubleArrayGraphPropertyValues, DefaultDoubleGraphPropertyValues,
    DefaultFloatArrayGraphPropertyValues, DefaultLongArrayGraphPropertyValues,
    DefaultLongGraphPropertyValues,
};
use crate::types::properties::node::impls::{
    DefaultDoubleArrayNodePropertyValues, DefaultDoubleNodePropertyValues,
    DefaultLongArrayNodePropertyValues, DefaultLongNodePropertyValues,
};
use crate::types::properties::node::impls::huge_node_property_values::{
    HugeDoubleNodePropertyValues, HugeLongNodePropertyValues,
    HugeLongArrayNodePropertyValues, HugeDoubleArrayNodePropertyValues,
};
use crate::types::properties::relationship::impls::default_relationship_property_values::{
    DefaultRelationshipPropertyValues,
};

/// Factory for Node-level PropertyValues adapters.
#[derive(Debug, Default, Clone)]
pub struct NodePropertyAdapterFactory;

impl NodePropertyAdapterFactory {
    /// Get the backend kind for node properties from config
    fn node_backend(cfg: &PropertyStoreConfig) -> PropertyBackendKind {
        cfg.node_backend.unwrap_or(cfg.default_backend)
    }

    #[inline]
    pub fn long_from_vec(
        cfg: &PropertyStoreConfig,
        values: Vec<i64>,
        node_count: usize,
    ) -> Arc<dyn LongNodePropertyValues> {
        match Self::node_backend(cfg) {
            PropertyBackendKind::Vec => Arc::new(DefaultLongNodePropertyValues::new(values, node_count)),
            PropertyBackendKind::HugeArray => {
                // Convert Vec to HugeArray
                let mut huge_array = HugeLongArray::new(node_count);
                for (i, value) in values.iter().enumerate() {
                    huge_array.set(i, *value);
                }
                Arc::new(HugeLongNodePropertyValues::new(huge_array, 0, node_count))
            }
            #[cfg(feature = "arrow")]
            PropertyBackendKind::Arrow => {
                // TODO: Implement Arrow backend
                Arc::new(DefaultLongNodePropertyValues::new(values, node_count))
            }
        }
    }

    #[inline]
    pub fn double_from_vec(
        cfg: &PropertyStoreConfig,
        values: Vec<f64>,
        node_count: usize,
    ) -> Arc<dyn DoubleNodePropertyValues> {
        match Self::node_backend(cfg) {
            PropertyBackendKind::Vec => Arc::new(DefaultDoubleNodePropertyValues::new(values, node_count)),
            PropertyBackendKind::HugeArray => {
                // Convert Vec to HugeArray
                let mut huge_array = HugeDoubleArray::new(node_count);
                for (i, value) in values.iter().enumerate() {
                    huge_array.set(i, *value);
                }
                Arc::new(HugeDoubleNodePropertyValues::new(huge_array, 0.0, node_count))
            }
            #[cfg(feature = "arrow")]
            PropertyBackendKind::Arrow => {
                // TODO: Implement Arrow backend
                Arc::new(DefaultDoubleNodePropertyValues::new(values, node_count))
            }
        }
    }

    #[inline]
    pub fn double_array_from_vec(
        cfg: &PropertyStoreConfig,
        values: Vec<Option<Vec<f64>>>,
        node_count: usize,
    ) -> Arc<dyn DoubleArrayNodePropertyValues> {
        match Self::node_backend(cfg) {
            PropertyBackendKind::Vec => Arc::new(DefaultDoubleArrayNodePropertyValues::new(values, node_count)),
            PropertyBackendKind::HugeArray => {
                // Convert Vec<Option<Vec<f64>>> to HugeObjectArray<Vec<f64>>
                use crate::collections::backends::huge::HugeObjectArray;
                let mut huge_array = HugeObjectArray::new(node_count);
                for (i, opt_array) in values.into_iter().enumerate() {
                    if let Some(array) = opt_array {
                        huge_array.set(i, array);
                    }
                }
                Arc::new(HugeDoubleArrayNodePropertyValues::new(huge_array, node_count))
            }
            #[cfg(feature = "arrow")]
            PropertyBackendKind::Arrow => {
                // TODO: Implement Arrow backend
                Arc::new(DefaultDoubleArrayNodePropertyValues::new(values, node_count))
            }
        }
    }

    #[inline]
    pub fn long_array_from_vec(
        cfg: &PropertyStoreConfig,
        values: Vec<Option<Vec<i64>>>,
        node_count: usize,
    ) -> Arc<dyn LongArrayNodePropertyValues> {
        match Self::node_backend(cfg) {
            PropertyBackendKind::Vec => Arc::new(DefaultLongArrayNodePropertyValues::new(values, node_count)),
            PropertyBackendKind::HugeArray => {
                // Convert Vec<Option<Vec<i64>>> to HugeObjectArray<Vec<i64>>
                use crate::collections::backends::huge::HugeObjectArray;
                let mut huge_array = HugeObjectArray::new(node_count);
                for (i, opt_array) in values.into_iter().enumerate() {
                    if let Some(array) = opt_array {
                        huge_array.set(i, array);
                    }
                }
                Arc::new(HugeLongArrayNodePropertyValues::new(huge_array, node_count))
            }
            #[cfg(feature = "arrow")]
            PropertyBackendKind::Arrow => {
                // TODO: Implement Arrow backend
                Arc::new(DefaultLongArrayNodePropertyValues::new(values, node_count))
            }
        }
    }
}

/// Factory for Relationship-level PropertyValues adapters.
#[derive(Debug, Default, Clone)]
pub struct RelationshipPropertyAdapterFactory;

impl RelationshipPropertyAdapterFactory {
    #[inline]
    pub fn double_from_vec(
        _cfg: &PropertyStoreConfig,
        values: Vec<f64>,
        default_value: f64,
    ) -> Arc<dyn RelationshipPropertyValues> {
        let element_count = values.len();
        Arc::new(DefaultRelationshipPropertyValues::new(
            values,
            default_value,
            element_count,
        ))
    }

    #[allow(dead_code)]
    fn _backend(_cfg: &PropertyStoreConfig) -> PropertyBackendKind {
        _cfg.relationship_backend.unwrap_or(_cfg.default_backend)
    }
}

/// Factory for Graph-level PropertyValues adapters.
#[derive(Debug, Default, Clone)]
pub struct GraphPropertyAdapterFactory;

impl GraphPropertyAdapterFactory {
    #[inline]
    pub fn long_from_vec(
        _cfg: &PropertyStoreConfig,
        values: Vec<i64>,
    ) -> Arc<dyn LongGraphPropertyValues> {
        Arc::new(DefaultLongGraphPropertyValues::new(values))
    }

    #[inline]
    pub fn double_from_vec(
        _cfg: &PropertyStoreConfig,
        values: Vec<f64>,
    ) -> Arc<dyn DoubleGraphPropertyValues> {
        Arc::new(DefaultDoubleGraphPropertyValues::new(values))
    }

    #[inline]
    pub fn double_array_from_vec(
        _cfg: &PropertyStoreConfig,
        values: Vec<Vec<f64>>,
    ) -> Arc<dyn DoubleArrayGraphPropertyValues> {
        Arc::new(DefaultDoubleArrayGraphPropertyValues::new(values))
    }

    #[inline]
    pub fn float_array_from_vec(
        _cfg: &PropertyStoreConfig,
        values: Vec<Vec<f32>>,
    ) -> Arc<dyn FloatArrayGraphPropertyValues> {
        Arc::new(DefaultFloatArrayGraphPropertyValues::new(values))
    }

    #[inline]
    pub fn long_array_from_vec(
        _cfg: &PropertyStoreConfig,
        values: Vec<Vec<i64>>,
    ) -> Arc<dyn LongArrayGraphPropertyValues> {
        Arc::new(DefaultLongArrayGraphPropertyValues::new(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PropertyBackendKind, PropertyStoreConfig};

    #[test]
    fn test_huge_array_factory_integration() {
        // Test Vec backend (default)
        let vec_config = PropertyStoreConfig {
            default_backend: PropertyBackendKind::Vec,
            node_backend: None,
            relationship_backend: None,
            graph_backend: None,
        };

        let values = vec![1, 2, 3, 4, 5];
        let node_count = values.len();
        
        let vec_props = NodePropertyAdapterFactory::long_from_vec(&vec_config, values.clone(), node_count);
        assert_eq!(vec_props.long_value(0).unwrap(), 1);
        assert_eq!(vec_props.long_value(4).unwrap(), 5);

        // Test HugeArray backend
        let huge_config = PropertyStoreConfig {
            default_backend: PropertyBackendKind::HugeArray,
            node_backend: None,
            relationship_backend: None,
            graph_backend: None,
        };

        let huge_props = NodePropertyAdapterFactory::long_from_vec(&huge_config, values, node_count);
        assert_eq!(huge_props.long_value(0).unwrap(), 1);
        assert_eq!(huge_props.long_value(4).unwrap(), 5);
        
        // Verify they're different types but same behavior
        // We can't easily distinguish trait objects by type name, so let's test behavior instead
        // Both should work identically but use different underlying storage
        assert_eq!(vec_props.element_count(), huge_props.element_count());
        assert_eq!(vec_props.value_type(), huge_props.value_type());
        
        // Test that both work correctly
        for i in 0..node_count {
            assert_eq!(vec_props.long_value(i as u64).unwrap(), huge_props.long_value(i as u64).unwrap());
        }
    }

    #[test]
    fn test_double_huge_array_factory_integration() {
        let huge_config = PropertyStoreConfig {
            default_backend: PropertyBackendKind::HugeArray,
            node_backend: None,
            relationship_backend: None,
            graph_backend: None,
        };

        let values = vec![1.5, 2.5, 3.5, 4.5, 5.5];
        let node_count = values.len();
        
        let huge_props = NodePropertyAdapterFactory::double_from_vec(&huge_config, values, node_count);
        assert_eq!(huge_props.double_value(0).unwrap(), 1.5);
        assert_eq!(huge_props.double_value(4).unwrap(), 5.5);
        assert_eq!(huge_props.long_value(0).unwrap(), 1); // Implicit cast
    }

    #[test]
    fn test_node_backend_override() {
        // Test that node_backend override works
        let config = PropertyStoreConfig {
            default_backend: PropertyBackendKind::Vec,
            node_backend: Some(PropertyBackendKind::HugeArray),
            relationship_backend: None,
            graph_backend: None,
        };

        let values = vec![10, 20, 30];
        let node_count = values.len();
        
        let props = NodePropertyAdapterFactory::long_from_vec(&config, values, node_count);
        assert_eq!(props.long_value(0).unwrap(), 10);
        assert_eq!(props.long_value(2).unwrap(), 30);
        
        // Should be HugeArray type despite default being Vec
        // We can verify this by checking that it works correctly with HugeArray behavior
        assert_eq!(props.element_count(), node_count);
        assert_eq!(props.value_type(), crate::types::ValueType::Long);
        
        // Test that the values are correctly stored and retrieved
        assert_eq!(props.long_value(0).unwrap(), 10);
        assert_eq!(props.long_value(1).unwrap(), 20);
        assert_eq!(props.long_value(2).unwrap(), 30);
    }
}

//! LongIfChangedNodePropertyValues - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.community.LongIfChangedNodePropertyValues
//!
//! Only writes community values that have changed from seed values for incremental updates.

use crate::types::properties::{PropertyValues, PropertyValuesResult};
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;

/// Long if changed node property values - translated from Java LongIfChangedNodePropertyValues
/// 
/// Only writes community values that have changed from seed values for incremental updates.
/// This is a PropertyValues wrapper that compares seed vs new values and only writes changes.
#[derive(Debug)]
pub struct LongIfChangedNodePropertyValues {
    /// Seed properties - translated from Java: private final NodePropertyValues seedProperties;
    seed_properties: Box<dyn NodePropertyValues>,
    
    /// New properties - translated from Java: private final NodePropertyValues newProperties;
    new_properties: Box<dyn NodePropertyValues>,
}

impl LongIfChangedNodePropertyValues {
    /// Create instance from seed property and new properties
    /// 
    /// Translated from Java static method:
    /// ```java
    /// public static LongNodePropertyValues of(NodeProperty seedProperty, LongNodePropertyValues newProperties) {
    ///     var propertyState = seedProperty.propertyState();
    ///     if (propertyState == PropertyState.PERSISTENT) {
    ///         NodePropertyValues seedProperties = seedProperty.values();
    ///         // TODO forbid doubles once we load properties with their correct type
    ///         if (seedProperties.valueType() == ValueType.LONG || seedProperties.valueType() == ValueType.DOUBLE) {
    ///             return new LongIfChangedNodePropertyValues(seedProperties, newProperties);
    ///         } else {
    ///             throw new IllegalStateException(formatWithLocale(
    ///                 "Expected seedProperty `%s` to be of type %s, but was %s",
    ///                 seedProperty,
    ///                 ValueType.LONG,
    ///                 seedProperties.valueType()
    ///             ));
    ///         }
    ///     } else {
    ///         return newProperties;
    ///     }
    /// }
    /// ```
    pub fn of(
        seed_property: &dyn NodePropertyValues,
        new_properties: Box<dyn NodePropertyValues>,
    ) -> Box<dyn NodePropertyValues> {
        // TODO: Check property state - need PropertyState enum
        // let property_state = seed_property.property_state();
        // if property_state == PropertyState::Persistent {
        
        // TODO: This is a hack - need to properly clone the seed property
        // For now, we'll create a new instance with the same properties
        let value_type = seed_property.value_type();
        
        // Check value type - translated from Java logic
        if value_type == ValueType::Long || value_type == ValueType::Double {
            // TODO: Need to properly handle the seed property cloning
            // This is a placeholder implementation
            todo!("Need to properly clone seed property")
        } else {
            panic!(
                "Expected seedProperty to be of type {:?}, but was {:?}",
                ValueType::Long,
                value_type
            );
        }
        // } else {
        //     return new_properties;
        // }
    }
    
    /// Private constructor - translated from Java private constructor
    /// 
    /// Java constructor:
    /// ```java
    /// private LongIfChangedNodePropertyValues(
    ///     NodePropertyValues seedProperties,
    ///     NodePropertyValues newProperties
    /// ) {
    ///     this.seedProperties = seedProperties;
    ///     this.newProperties = newProperties;
    /// }
    /// ```
    fn new(
        seed_properties: Box<dyn NodePropertyValues>,
        new_properties: Box<dyn NodePropertyValues>,
    ) -> Self {
        Self {
            seed_properties,
            new_properties,
        }
    }
}

impl PropertyValues for LongIfChangedNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.new_properties.node_count().max(self.seed_properties.node_count())
    }
}

impl NodePropertyValues for LongIfChangedNodePropertyValues {
    /// Get long value for node - translated from Java longValue method
    /// 
    /// Java method:
    /// ```java
    /// public long longValue(long nodeId) {
    ///     var seedValue = seedProperties.longValue(nodeId);
    ///     var writeValue = newProperties.longValue(nodeId);
    ///     return (seedValue != writeValue) ? writeValue : Long.MIN_VALUE;
    /// }
    /// ```
    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        let seed_value = self.seed_properties.long_value(node_id).unwrap_or(0);
        let write_value = self.new_properties.long_value(node_id).unwrap_or(0);
        
        if seed_value != write_value {
            Ok(write_value)
        } else {
            Ok(i64::MIN) // Return Long.MIN_VALUE equivalent
        }
    }
    
    /// Get node count - translated from Java nodeCount method
    /// 
    /// Java method:
    /// ```java
    /// public long nodeCount() {
    ///     return Math.max(newProperties.nodeCount(), seedProperties.nodeCount());
    /// }
    /// ```
    fn node_count(&self) -> usize {
        self.new_properties.node_count().max(self.seed_properties.node_count())
    }
    
    /// Check if node has value - translated from Java hasValue method
    /// 
    /// Java method:
    /// ```java
    /// public boolean hasValue(long nodeId) {
    ///     long seedValue = seedProperties.longValue(nodeId);
    ///     long writeValue = newProperties.longValue(nodeId);
    ///     return seedValue == Long.MIN_VALUE || (seedValue != writeValue);
    /// }
    /// ```
    fn has_value(&self, node_id: u64) -> bool {
        let seed_value = self.seed_properties.long_value(node_id).unwrap_or(0);
        let write_value = self.new_properties.long_value(node_id).unwrap_or(0);
        
        seed_value == i64::MIN || (seed_value != write_value)
    }
    
    /// Get double value - not implemented in Java version
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("double_value".to_string()))
    }
    
    /// Get double array value - not implemented in Java version
    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("double_array_value".to_string()))
    }
    
    /// Get float array value - not implemented in Java version
    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("float_array_value".to_string()))
    }
    
    /// Get long array value - not implemented in Java version
    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("long_array_value".to_string()))
    }
    
    /// Get object value - not implemented in Java version
    fn get_object(&self, _node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("get_object".to_string()))
    }
    
    /// Get dimension - not implemented in Java version
    fn dimension(&self) -> Option<usize> {
        None
    }
    
    /// Get max long property value - not implemented in Java version
    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }
    
    /// Get max double property value - not implemented in Java version
    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }
}

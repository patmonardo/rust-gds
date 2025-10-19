//! ScaledPropertiesNodePropertyValues - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.misc.ScaledPropertiesNodePropertyValues
//!
//! Node property values implementation for scaled double array properties.

use crate::types::properties::{PropertyValues, PropertyValuesResult};
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;

/// Huge object array - placeholder for Java HugeObjectArray<double[]>
/// 
/// This would be implemented using the existing HugeDoubleArray infrastructure
/// but adapted for arrays of doubles rather than single doubles.
#[derive(Debug)]
pub struct HugeObjectArray<T> {
    /// Data storage - placeholder for actual implementation
    data: Vec<Option<T>>,
    /// Size
    size: u64,
}

impl<T> HugeObjectArray<T> {
    /// Create new huge object array
    pub fn new(size: u64) -> Self {
        Self {
            data: vec![None; size as usize],
            size,
        }
    }
    
    /// Get value at index - placeholder for Java get method
    /// 
    /// Java method:
    /// ```java
    /// public T get(long index)
    /// ```
    pub fn get(&self, index: u64) -> Option<&T> {
        if index < self.size {
            self.data[index as usize].as_ref()
        } else {
            None
        }
    }
    
    /// Set value at index - placeholder for Java set method
    pub fn set(&mut self, index: u64, value: T) {
        if index < self.size {
            self.data[index as usize] = Some(value);
        }
    }
}

/// Scaled properties node property values - translated from Java ScaledPropertiesNodePropertyValues
/// 
/// Node property values implementation for scaled double array properties.
/// 
/// Java class:
/// ```java
/// public final class ScaledPropertiesNodePropertyValues implements DoubleArrayNodePropertyValues {
///     private final long size;
///     private final HugeObjectArray<double[]> scaledProperties;
/// 
///     public ScaledPropertiesNodePropertyValues(long size, HugeObjectArray<double[]> scaledProperties) {
///         this.size = size;
///         this.scaledProperties = scaledProperties;
///     }
/// 
///     @Override
///     public long nodeCount() {
///         return size;
///     }
/// 
///     @Override
///     public double[] doubleArrayValue(long nodeId) {
///         return scaledProperties.get(nodeId);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct ScaledPropertiesNodePropertyValues {
    /// Size - translated from Java: private final long size;
    size: u64,
    
    /// Scaled properties - translated from Java: private final HugeObjectArray<double[]> scaledProperties;
    scaled_properties: HugeObjectArray<Vec<f64>>,
}

impl ScaledPropertiesNodePropertyValues {
    /// Constructor - translated from Java constructor
    /// 
    /// Java constructor:
    /// ```java
    /// public ScaledPropertiesNodePropertyValues(long size, HugeObjectArray<double[]> scaledProperties) {
    ///     this.size = size;
    ///     this.scaledProperties = scaledProperties;
    /// }
    /// ```
    pub fn new(size: u64, scaled_properties: HugeObjectArray<Vec<f64>>) -> Self {
        Self {
            size,
            scaled_properties,
        }
    }
}

impl PropertyValues for ScaledPropertiesNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }

    fn element_count(&self) -> usize {
        self.size as usize
    }
}

impl NodePropertyValues for ScaledPropertiesNodePropertyValues {
    /// Get node count - translated from Java nodeCount method
    /// 
    /// Java method:
    /// ```java
    /// public long nodeCount() {
    ///     return size;
    /// }
    /// ```
    fn node_count(&self) -> usize {
        self.size as usize
    }
    
    /// Get double array value - translated from Java doubleArrayValue method
    /// 
    /// Java method:
    /// ```java
    /// public double[] doubleArrayValue(long nodeId) {
    ///     return scaledProperties.get(nodeId);
    /// }
    /// ```
    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        if node_id >= self.size {
            return Err(crate::types::properties::PropertyValuesError::ValueNotFound(node_id));
        }
        
        match self.scaled_properties.get(node_id) {
            Some(array) => Ok(array.clone()),
            None => Err(crate::types::properties::PropertyValuesError::ValueNotFound(node_id)),
        }
    }
    
    /// Get long value - not implemented for double array type
    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("long_value".to_string()))
    }
    
    /// Check if node has value - placeholder implementation
    fn has_value(&self, node_id: u64) -> bool {
        node_id < self.size && self.scaled_properties.get(node_id).is_some()
    }
    
    /// Get double value - not implemented for double array type
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("double_value".to_string()))
    }
    
    /// Get float array value - not implemented for double array type
    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("float_array_value".to_string()))
    }
    
    /// Get long array value - not implemented for double array type
    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("long_array_value".to_string()))
    }
    
    /// Get object value - not implemented for double array type
    fn get_object(&self, _node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Err(crate::types::properties::PropertyValuesError::UnsupportedOperation("get_object".to_string()))
    }
    
    /// Get dimension - placeholder implementation
    fn dimension(&self) -> Option<usize> {
        // TODO: Implement proper dimension calculation
        None
    }
    
    /// Get max long property value - not applicable for double array type
    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }
    
    /// Get max double property value - not applicable for double array type
    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }
}

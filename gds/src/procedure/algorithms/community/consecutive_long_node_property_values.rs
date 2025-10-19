//! ConsecutiveLongNodePropertyValues - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.community.ConsecutiveLongNodePropertyValues
//!
//! Maps sparse community IDs to consecutive dense ranges for memory optimization.

use crate::types::properties::{PropertyValues, PropertyValuesResult};
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;

/// Consecutive long node property values - translated from Java ConsecutiveLongNodePropertyValues
/// 
/// Maps sparse community IDs to consecutive dense ranges for memory optimization.
/// This is a PropertyValues wrapper that converts sparse community IDs to consecutive ranges.
#[derive(Debug)]
pub struct ConsecutiveLongNodePropertyValues {
    /// Communities array - translated from Java: private final HugeLongArray communities;
    communities: Vec<Option<u64>>,
    
    /// Node count - translated from Java: long maxIdx = inputProperties.nodeCount();
    node_count: u64,
}

impl ConsecutiveLongNodePropertyValues {
    /// Mapping size quotient - translated from Java: private static final long MAPPING_SIZE_QUOTIENT = 10L;
    const MAPPING_SIZE_QUOTIENT: u64 = 10;
    
    /// No value constant - translated from Java: private static final long NO_VALUE = -1L;
    const NO_VALUE: u64 = u64::MAX; // Using MAX instead of -1 for unsigned
    
    /// Constructor - translated from Java constructor
    /// 
    /// Java constructor:
    /// ```java
    /// public ConsecutiveLongNodePropertyValues(LongNodePropertyValues inputProperties) {
    ///     var nextConsecutiveId = -1L;
    ///     long maxIdx = inputProperties.nodeCount();
    ///     var setIdToConsecutiveId = new HugeLongLongMap(BitUtil.ceilDiv(maxIdx, MAPPING_SIZE_QUOTIENT));
    ///     this.communities = HugeLongArray.newArray(maxIdx);
    ///     
    ///     for (var nodeId = 0; nodeId < maxIdx; nodeId++) {
    ///         if (inputProperties.hasValue(nodeId)) {
    ///             var setId = inputProperties.longValue(nodeId);
    ///             var communityId = setIdToConsecutiveId.getOrDefault(setId, -1);
    ///             if (communityId == -1) {
    ///                 setIdToConsecutiveId.addTo(setId, ++nextConsecutiveId);
    ///                 communityId = nextConsecutiveId;
    ///             }
    ///             communities.set(nodeId, communityId);
    ///         } else {
    ///             communities.set(nodeId, NO_VALUE);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn new(input_properties: &dyn NodePropertyValues) -> Self {
        let mut next_consecutive_id = 0u64;
        let max_idx = input_properties.node_count() as u64;
        let mut set_id_to_consecutive_id = std::collections::HashMap::new();
        
        let mut communities = vec![None; max_idx as usize];
        
        for node_id in 0..max_idx {
            if input_properties.has_value(node_id) {
                let set_id = input_properties.long_value(node_id).unwrap_or(0) as u64;
                let community_id = set_id_to_consecutive_id.get(&set_id).copied();
                
                let consecutive_id = match community_id {
                    Some(id) => id,
                    None => {
                        let new_id = next_consecutive_id;
                        next_consecutive_id += 1;
                        set_id_to_consecutive_id.insert(set_id, new_id);
                        new_id
                    }
                };
                
                communities[node_id as usize] = Some(consecutive_id);
            } else {
                communities[node_id as usize] = None;
            }
        }
        
        Self {
            communities,
            node_count: max_idx,
        }
    }
}

impl PropertyValues for ConsecutiveLongNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.node_count as usize
    }
}

impl NodePropertyValues for ConsecutiveLongNodePropertyValues {
    /// Get long value for node - translated from Java longValue method
    /// 
    /// Java method:
    /// ```java
    /// public long longValue(long nodeId) {
    ///     long l = communities.get(nodeId);
    ///     if (l == NO_VALUE) {
    ///         return Long.MIN_VALUE;
    ///     }
    ///     return l;
    /// }
    /// ```
    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        if node_id >= self.node_count {
            return Err(crate::types::properties::PropertyValuesError::ValueNotFound(node_id));
        }
        
        match self.communities[node_id as usize] {
            Some(value) => Ok(value as i64),
            None => Ok(i64::MIN), // Return Long.MIN_VALUE equivalent
        }
    }
    
    /// Check if node has value - translated from Java hasValue method
    /// 
    /// Java method:
    /// ```java
    /// public boolean hasValue(long nodeId) {
    ///     return communities.get(nodeId) != NO_VALUE;
    /// }
    /// ```
    fn has_value(&self, node_id: u64) -> bool {
        if node_id >= self.node_count {
            return false;
        }
        
        self.communities[node_id as usize].is_some()
    }
    
    /// Get node count - translated from Java nodeCount method
    /// 
    /// Java method:
    /// ```java
    /// public long nodeCount() {
    ///     return communities.size();
    /// }
    /// ```
    fn node_count(&self) -> usize {
        self.node_count as usize
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

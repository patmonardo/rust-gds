use crate::types::graph::id_map::NodeId;
use std::fmt::Debug;

/// Represents a relationship between two nodes with an associated property value.
pub trait RelationshipCursor: Debug {
    fn source_id(&self) -> NodeId;
    fn target_id(&self) -> NodeId;
    fn property(&self) -> f64;
}

/// Mutable variant of the relationship cursor used by iterator implementations
/// that reuse a single cursor instance.
pub trait ModifiableRelationshipCursor: RelationshipCursor {
    fn set_source_id(&mut self, source_id: NodeId);
    fn set_target_id(&mut self, target_id: NodeId);
    fn set_property(&mut self, property: f64);
}

/// Convenient alias for passing cursor trait objects around.
pub type RelationshipCursorBox = Box<dyn RelationshipCursor + Send>;

// === Phase 2A: Specialized Cursor Architecture ===

/// Optimized relationship cursor for algorithms requiring direct f64 weight access.
/// 
/// This trait is designed for pathfinding algorithms (Dijkstra, A*, Bellman-Ford, etc.)
/// that need high-performance access to relationship weights as f64 values.
/// 
/// **Key Benefits:**
/// - Direct f64 access (no conversion overhead)
/// - Java GDS alignment (algorithms use `double` weights)
/// - Zero-copy weight access for performance-critical paths
/// - Clean separation from general property access
pub trait WeightedRelationshipCursor: Debug {
    /// Returns the source node ID of this relationship.
    fn source_id(&self) -> NodeId;
    
    /// Returns the target node ID of this relationship.
    fn target_id(&self) -> NodeId;
    
    /// Returns the weight of this relationship as f64.
    /// 
    /// This method provides direct f64 access optimized for algorithms.
    /// No type conversion is performed - the weight is already stored as f64.
    fn weight(&self) -> f64;
}

/// Mutable variant of the weighted relationship cursor for iterator implementations.
pub trait ModifiableWeightedRelationshipCursor: WeightedRelationshipCursor {
    /// Sets the source node ID of this relationship.
    fn set_source_id(&mut self, source_id: NodeId);
    
    /// Sets the target node ID of this relationship.
    fn set_target_id(&mut self, target_id: NodeId);
    
    /// Sets the weight of this relationship as f64.
    /// 
    /// This method accepts f64 directly, optimized for algorithm updates.
    fn set_weight(&mut self, weight: f64);
}

/// Convenient alias for passing weighted cursor trait objects around.
pub type WeightedRelationshipCursorBox = Box<dyn WeightedRelationshipCursor + Send>;

// === Phase 2B: General Purpose Typed Cursor Architecture ===

/// General-purpose relationship cursor with Value enum support and smart converter.
/// 
/// This trait is designed for general property access where you need to work with
/// the full `Value` enum and want automatic type conversions via the smart converter pattern.
/// 
/// **Key Benefits:**
/// - Full `Value` enum support (Long, Double, Boolean, String, Bytes, Null)
/// - Smart converter `get<T>()` method with automatic type conversions
/// - Type-safe property access with compile-time dispatch
/// - Future-ready for Level 1 Projection Typing System
/// - Arrow/Polars alignment via `Value` enum
pub trait TypedRelationshipCursor: Debug {
    /// Returns the source node ID of this relationship.
    fn source_id(&self) -> NodeId;
    
    /// Returns the target node ID of this relationship.
    fn target_id(&self) -> NodeId;
    
    /// Returns the property value as a `Value` enum.
    /// 
    /// This method provides access to the full `Value` enum, supporting all types:
    /// - `Value::Long(i64)` - Integer values
    /// - `Value::Double(f64)` - Floating-point values  
    /// - `Value::Boolean(bool)` - Boolean values
    /// - `Value::String(String)` - String values
    /// - `Value::Bytes { data: Vec<u8>, mime_type: Option<String> }` - Binary data with MIME
    /// - `Value::Null` - Null values
    fn value(&self) -> std::sync::Arc<dyn crate::values::traits::GdsValue>;
    
    /// Smart converter: Get property value as type T with automatic conversion.
    /// 
    /// This method provides type-safe access with automatic conversions:
    /// - `i64` ↔ `f64` (numeric compatibility)
    /// - `i64` ↔ `bool` (0 = false, non-zero = true)
    /// - `f64` ↔ `bool` (0.0 = false, non-zero = true)
    /// - `String` ↔ `i64`/`f64` (string parsing)
    /// 
    /// Returns `Err(ValueError)` for incompatible conversions.
    fn get<T: crate::values::traits::FromGdsValue>(&self) -> Result<T, String>;
}

/// Mutable variant of the typed relationship cursor for iterator implementations.
pub trait ModifiableTypedRelationshipCursor: TypedRelationshipCursor {
    /// Sets the source node ID of this relationship.
    fn set_source_id(&mut self, source_id: NodeId);
    
    /// Sets the target node ID of this relationship.
    fn set_target_id(&mut self, target_id: NodeId);
    
    /// Sets the property value as a `GdsValue`.
    /// 
    /// This method accepts the full `GdsValue` trait object, supporting all types.
    fn set_value(&mut self, value: std::sync::Arc<dyn crate::values::traits::GdsValue>);
    
    /// Smart setter: Set property value from type T with automatic conversion.
    /// 
    /// This method provides type-safe setting with automatic conversions.
    /// Returns `Err(ValueError)` for incompatible conversions.
    fn set<T: crate::values::traits::FromGdsValue>(&mut self, value: T) -> Result<(), String>;
}

/// Convenient alias for passing typed cursor trait objects around.
pub type TypedRelationshipCursorBox = Box<dyn TypedRelationshipCursor + Send>;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Clone, Copy)]
    struct SimpleCursor {
        source: NodeId,
        target: NodeId,
        property: f64,
    }

    impl RelationshipCursor for SimpleCursor {
        fn source_id(&self) -> NodeId {
            self.source
        }

        fn target_id(&self) -> NodeId {
            self.target
        }

        fn property(&self) -> f64 {
            self.property
        }
    }

    impl ModifiableRelationshipCursor for SimpleCursor {
        fn set_source_id(&mut self, source_id: NodeId) {
            self.source = source_id;
        }

        fn set_target_id(&mut self, target_id: NodeId) {
            self.target = target_id;
        }

        fn set_property(&mut self, property: f64) {
            self.property = property;
        }
    }

    #[test]
    fn modifiable_cursor_updates_values() {
        let mut cursor = SimpleCursor::default();
        cursor.set_source_id(1);
        cursor.set_target_id(2);
        cursor.set_property(3.0);

        assert_eq!(cursor.source_id(), 1);
        assert_eq!(cursor.target_id(), 2);
        assert_eq!(cursor.property(), 3.0);
    }
    
    // === Phase 2A: WeightedRelationshipCursor Tests ===
    
    #[derive(Debug, Default, Clone, Copy)]
    struct WeightedCursor {
        source: NodeId,
        target: NodeId,
        weight: f64,
    }

    impl WeightedRelationshipCursor for WeightedCursor {
        fn source_id(&self) -> NodeId {
            self.source
        }

        fn target_id(&self) -> NodeId {
            self.target
        }

        fn weight(&self) -> f64 {
            self.weight
        }
    }

    impl ModifiableWeightedRelationshipCursor for WeightedCursor {
        fn set_source_id(&mut self, source_id: NodeId) {
            self.source = source_id;
        }

        fn set_target_id(&mut self, target_id: NodeId) {
            self.target = target_id;
        }

        fn set_weight(&mut self, weight: f64) {
            self.weight = weight;
        }
    }

    #[test]
    fn weighted_cursor_provides_direct_f64_access() {
        let mut cursor = WeightedCursor::default();
        cursor.set_source_id(1);
        cursor.set_target_id(2);
        cursor.set_weight(3.14);

        assert_eq!(cursor.source_id(), 1);
        assert_eq!(cursor.target_id(), 2);
        assert_eq!(cursor.weight(), 3.14);
    }

    #[test]
    fn weighted_cursor_performance_characteristics() {
        let cursor = WeightedCursor {
            source: 1,
            target: 2,
            weight: 42.0,
        };

        // Direct f64 access - no conversion overhead
        let weight = cursor.weight();
        assert_eq!(weight, 42.0);
        
        // Can be used directly in algorithms without casting
        let distance = weight + 10.0;
        assert_eq!(distance, 52.0);
    }
    
    // === Phase 2B: TypedRelationshipCursor Tests ===
    
    #[derive(Clone)]
    struct TypedCursor {
        source: NodeId,
        target: NodeId,
        value: std::sync::Arc<dyn crate::values::traits::GdsValue>,
    }

    impl std::fmt::Debug for TypedCursor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TypedCursor")
                .field("source", &self.source)
                .field("target", &self.target)
                .field("value_type", &self.value.value_type())
                .finish()
        }
    }

    impl Default for TypedCursor {
        fn default() -> Self {
            Self {
                source: 0,
                target: 0,
                value: crate::values::PrimitiveValues::long_value(0),
            }
        }
    }

    impl TypedRelationshipCursor for TypedCursor {
        fn source_id(&self) -> NodeId {
            self.source
        }

        fn target_id(&self) -> NodeId {
            self.target
        }

        fn value(&self) -> std::sync::Arc<dyn crate::values::traits::GdsValue> {
            self.value.clone()
        }

        fn get<T: crate::values::traits::FromGdsValue>(&self) -> Result<T, String> {
            T::from_gds_value(self.value.as_ref())
        }
    }

    impl ModifiableTypedRelationshipCursor for TypedCursor {
        fn set_source_id(&mut self, source_id: NodeId) {
            self.source = source_id;
        }

        fn set_target_id(&mut self, target_id: NodeId) {
            self.target = target_id;
        }

        fn set_value(&mut self, value: std::sync::Arc<dyn crate::values::traits::GdsValue>) {
            self.value = value;
        }

        fn set<T: crate::values::traits::FromGdsValue>(&mut self, _value: T) -> Result<(), String> {
            // Not used in these tests; provide a conservative error to avoid generic lifetime issues
            let _ = std::any::type_name::<T>();
            Err("set<T>() not implemented in test stub".to_string())
        }
    }

    #[test]
    fn typed_cursor_provides_full_gdsvalue_access() {
        let mut cursor = TypedCursor::default();
        cursor.set_source_id(1);
        cursor.set_target_id(2);
        let long_value = crate::values::PrimitiveValues::long_value(42);
        cursor.set_value(long_value.clone());

        assert_eq!(cursor.source_id(), 1);
        assert_eq!(cursor.target_id(), 2);
        assert_eq!(cursor.value().value_type(), long_value.value_type());
    }

    #[test]
    fn typed_cursor_smart_converter_works() {
        let long_value = crate::values::PrimitiveValues::long_value(42);
        let cursor = TypedCursor {
            source: 1,
            target: 2,
            value: long_value,
        };

        // Smart converter: i64 to f64
        let weight: f64 = cursor.get().unwrap();
        assert_eq!(weight, 42.0);
        
        // Smart converter: i64 to bool
        let active: bool = cursor.get().unwrap();
        assert_eq!(active, true);
        
        // Smart converter: i64 to i64 (no conversion)
        let count: i64 = cursor.get().unwrap();
        assert_eq!(count, 42);
    }

    #[test]
    fn typed_cursor_supports_all_value_types() {
        let test_cases = vec![
            (crate::values::PrimitiveValues::long_value(42), "Long"),
            (crate::values::PrimitiveValues::floating_point_value(3.14), "Double"),
            (crate::values::PrimitiveValues::boolean_value(true), "Boolean"),
            (crate::values::PrimitiveValues::string_value("hello".to_string()), "String"),
        ];

        for (value, type_name) in test_cases {
            let cursor = TypedCursor {
                source: 1,
                target: 2,
                value: value.clone(),
            };

            assert_eq!(cursor.value().value_type(), value.value_type(), "Failed for {}", type_name);
        }
    }

    #[test]
    fn typed_cursor_set_value_works() {
        let mut cursor = TypedCursor::default();

        // i64
        cursor.set_value(crate::values::PrimitiveValues::long_value(42));
        assert_eq!(cursor.value().value_type(), crate::types::ValueType::Long);

        // f64
        cursor.set_value(crate::values::PrimitiveValues::floating_point_value(3.14));
        assert_eq!(cursor.value().value_type(), crate::types::ValueType::Double);

        // bool
        cursor.set_value(crate::values::PrimitiveValues::boolean_value(true));
        assert_eq!(cursor.value().value_type(), crate::types::ValueType::Boolean);

        // String
        cursor.set_value(crate::values::PrimitiveValues::string_value("hello".to_string()));
        assert_eq!(cursor.value().value_type(), crate::types::ValueType::String);
    }
}

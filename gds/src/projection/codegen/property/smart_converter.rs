//! Smart Converter Pattern: Type-Driven Property Access
//!
//! This module implements the smart converter pattern for property cursors,
//! providing a single `get<T>()` method with automatic type conversions.
//!
//! **Pattern**: Instead of `get_f64()`, `get_i64()`, `get_bool()` methods,
//! we provide one generic `get<T>()` method that handles type-driven dispatch
//! and automatic conversions (e.g., `i64` to `f64`).

use crate::values::traits::{GdsValue, FromGdsValue};
use crate::types::ValueType;
use std::fmt::Debug;
use std::sync::Arc;

/// Smart converter trait for property access
/// 
/// This trait enables the `get<T>()` pattern where a single method
/// can return different types based on the generic parameter, with
/// automatic type conversions handled by the `FromGdsValue` trait.
pub trait SmartConverter {
    /// Gets a property value with automatic type conversion
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Type-driven dispatch with automatic conversions
    /// let weight: f64 = cursor.get("weight")?;
    /// let count: i64 = cursor.get("count")?;
    /// let active: bool = cursor.get("active")?;
    /// let name: String = cursor.get("name")?;
    /// ```
    fn get<T: FromGdsValue>(&self, key: &str) -> Result<T, String>;
    
    /// Gets a property value with explicit ValueType validation
    /// 
    /// This method validates that the property has the expected type
    /// before attempting conversion, providing better error messages.
    fn get_typed<T: FromGdsValue>(&self, key: &str, expected_type: ValueType) -> Result<T, String>;
    
    /// Gets the raw GdsValue for a property key
    /// 
    /// This is the lowest-level access method that returns the actual
    /// GdsValue without any type conversion.
    fn get_value(&self, key: &str) -> Result<Arc<dyn GdsValue>, String>;
    
    /// Gets the ValueType for a property key
    /// 
    /// This method returns the type metadata for a property without
    /// accessing the actual value.
    fn get_value_type(&self, key: &str) -> Result<ValueType, String>;
}

/// Extension trait for existing PropertyCursor to add smart converter methods
/// 
/// This trait bridges the existing `PropertyCursor` API with the new
/// smart converter pattern, maintaining backward compatibility.
pub trait PropertyCursorExt: Debug {
    /// Smart converter: gets the next property value with type conversion
    /// 
    /// This method provides the new smart converter pattern for cursor iteration.
    fn next<T: FromGdsValue>(&mut self) -> Result<Option<T>, String>;
    
    /// Gets the next property value with explicit type validation
    fn next_typed<T: FromGdsValue>(&mut self, expected_type: ValueType) -> Result<Option<T>, String>;
}

/// Extension trait for RelationshipCursor to add smart converter methods
/// 
/// This trait extends the existing `RelationshipCursor` API with smart
/// converter methods for property access.
pub trait RelationshipCursorExt: Debug {
    /// Smart converter: gets the property value with type conversion
    /// 
    /// This method provides the new smart converter pattern for relationship
    /// property access.
    fn get<T: FromGdsValue>(&self) -> Result<T, String>;
    
    /// Gets the property value with explicit type validation
    fn get_typed<T: FromGdsValue>(&self, expected_type: ValueType) -> Result<T, String>;
    
    /// Gets the raw GdsValue for the relationship property
    fn get_value(&self) -> Result<Arc<dyn GdsValue>, String>;
    
    /// Gets the ValueType for the relationship property
    fn get_value_type(&self) -> Result<ValueType, String>;
}

/// Default implementation of SmartConverter for PropertyCursor
/// 
/// This implementation provides smart converter methods for any type
/// that implements the basic PropertyCursor interface.
impl<T> PropertyCursorExt for T
where
    T: crate::types::properties::relationship::PropertyCursor,
{
    fn next<U: FromGdsValue>(&mut self) -> Result<Option<U>, String> {
        match self.next() {
            Some(value) => {
                // Create a GdsValue from the f64 property
                let gds_value = crate::values::PrimitiveValues::floating_point_value(value);
                Ok(Some(U::from_gds_value(gds_value.as_ref())?))
            }
            None => Ok(None),
        }
    }
    
    fn next_typed<U: FromGdsValue>(&mut self, expected_type: ValueType) -> Result<Option<U>, String> {
        match self.next() {
            Some(value) => {
                let gds_value = crate::values::PrimitiveValues::floating_point_value(value);
                if gds_value.value_type() != expected_type {
                    return Err(format!("Type mismatch: expected {:?}, got {:?}", expected_type, gds_value.value_type()));
                }
                Ok(Some(U::from_gds_value(gds_value.as_ref())?))
            }
            None => Ok(None),
        }
    }
}

/// Default implementation of SmartConverter for RelationshipCursor
/// 
/// This implementation provides smart converter methods for any type
/// that implements the basic RelationshipCursor interface.
impl<T> RelationshipCursorExt for T
where
    T: crate::types::properties::relationship::RelationshipCursor,
{
    fn get<U: FromGdsValue>(&self) -> Result<U, String> {
        let value = self.property();
        let gds_value = crate::values::PrimitiveValues::floating_point_value(value);
        U::from_gds_value(gds_value.as_ref())
    }
    
    fn get_typed<U: FromGdsValue>(&self, expected_type: ValueType) -> Result<U, String> {
        let value = self.property();
        let gds_value = crate::values::PrimitiveValues::floating_point_value(value);
        if gds_value.value_type() != expected_type {
            return Err(format!("Type mismatch: expected {:?}, got {:?}", expected_type, gds_value.value_type()));
        }
        U::from_gds_value(gds_value.as_ref())
    }
    
    fn get_value(&self) -> Result<Arc<dyn GdsValue>, String> {
        let value = self.property();
        Ok(crate::values::PrimitiveValues::floating_point_value(value))
    }
    
    fn get_value_type(&self) -> Result<ValueType, String> {
        Ok(ValueType::Double)
    }
}
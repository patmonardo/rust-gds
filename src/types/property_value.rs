use crate::types::property::ValueType;
use std::collections::HashMap;

/// A unified value representation that can hold scalars, arrays, and maps.
/// This enables rich property types beyond Java GDS's scalar limitations.
/// Designed to be extensible to Arrow2 backing for columnar storage.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    // Scalars
    Long(i64),
    Double(f64),
    Boolean(bool),
    String(String),

    // Arrays (can be backed by Arrow2 ListArray)
    LongArray(Vec<i64>),
    DoubleArray(Vec<f64>),
    FloatArray(Vec<f32>),

    // Maps (can be backed by Arrow2 MapArray)
    StringMap(std::collections::HashMap<String, String>),
    LongMap(std::collections::HashMap<String, i64>),
    DoubleMap(std::collections::HashMap<String, f64>),
    BooleanMap(std::collections::HashMap<String, bool>),

    // Nested structures
    StringMapArray(Vec<std::collections::HashMap<String, String>>),
    LongMapArray(Vec<std::collections::HashMap<String, i64>>),
    DoubleMapArray(Vec<std::collections::HashMap<String, f64>>),

    // Untyped variants for flexibility
    UntypedArray(Vec<PropertyValue>),
    UntypedMap(std::collections::HashMap<String, PropertyValue>),
}

impl PropertyValue {
    /// Returns the ValueType of this property value.
    pub fn value_type(&self) -> ValueType {
        match self {
            PropertyValue::Long(_) => ValueType::Long,
            PropertyValue::Double(_) => ValueType::Double,
            PropertyValue::Boolean(_) => ValueType::Boolean,
            PropertyValue::String(_) => ValueType::String,
            PropertyValue::LongArray(_) => ValueType::LongArray,
            PropertyValue::DoubleArray(_) => ValueType::DoubleArray,
            PropertyValue::FloatArray(_) => ValueType::FloatArray,
            // For now, map to String for maps (can extend ValueType later)
            PropertyValue::StringMap(_) => ValueType::String,
            PropertyValue::LongMap(_) => ValueType::Long,
            PropertyValue::DoubleMap(_) => ValueType::Double,
            PropertyValue::BooleanMap(_) => ValueType::Boolean,
            PropertyValue::StringMapArray(_) => ValueType::String,
            PropertyValue::LongMapArray(_) => ValueType::LongArray,
            PropertyValue::DoubleMapArray(_) => ValueType::DoubleArray,
            PropertyValue::UntypedArray(_) => ValueType::LongArray, // fallback
            PropertyValue::UntypedMap(_) => ValueType::String,      // fallback
        }
    }

    /// Check if this value is a scalar.
    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            PropertyValue::Long(_)
                | PropertyValue::Double(_)
                | PropertyValue::Boolean(_)
                | PropertyValue::String(_)
        )
    }

    /// Check if this value is an array.
    pub fn is_array(&self) -> bool {
        matches!(
            self,
            PropertyValue::LongArray(_)
                | PropertyValue::DoubleArray(_)
                | PropertyValue::FloatArray(_)
                | PropertyValue::StringMapArray(_)
                | PropertyValue::LongMapArray(_)
                | PropertyValue::DoubleMapArray(_)
                | PropertyValue::UntypedArray(_)
        )
    }

    /// Check if this value is a map.
    pub fn is_map(&self) -> bool {
        matches!(
            self,
            PropertyValue::StringMap(_)
                | PropertyValue::LongMap(_)
                | PropertyValue::DoubleMap(_)
                | PropertyValue::BooleanMap(_)
                | PropertyValue::UntypedMap(_)
        )
    }

    /// Gets the long value if this is a Long variant.
    pub fn as_long(&self) -> Option<i64> {
        match self {
            PropertyValue::Long(v) => Some(*v),
            _ => None,
        }
    }

    /// Gets the double value if this is a Double variant.
    pub fn as_double(&self) -> Option<f64> {
        match self {
            PropertyValue::Double(v) => Some(*v),
            _ => None,
        }
    }

    /// Gets the string value if this is a String variant.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            PropertyValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the long array if this is a LongArray variant.
    pub fn as_long_array(&self) -> Option<&[i64]> {
        match self {
            PropertyValue::LongArray(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the double array if this is a DoubleArray variant.
    pub fn as_double_array(&self) -> Option<&[f64]> {
        match self {
            PropertyValue::DoubleArray(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the string map if this is a StringMap variant.
    pub fn as_string_map(&self) -> Option<&HashMap<String, String>> {
        match self {
            PropertyValue::StringMap(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the long map if this is a LongMap variant.
    pub fn as_long_map(&self) -> Option<&HashMap<String, i64>> {
        match self {
            PropertyValue::LongMap(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the double map if this is a DoubleMap variant.
    pub fn as_double_map(&self) -> Option<&HashMap<String, f64>> {
        match self {
            PropertyValue::DoubleMap(v) => Some(v),
            _ => None,
        }
    }

    /// Gets the untyped map if this is an UntypedMap variant.
    pub fn as_untyped_map(&self) -> Option<&HashMap<String, PropertyValue>> {
        match self {
            PropertyValue::UntypedMap(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_types() {
        assert_eq!(PropertyValue::Long(42).value_type(), ValueType::Long);
        assert_eq!(PropertyValue::Double(3.14).value_type(), ValueType::Double);
        assert_eq!(
            PropertyValue::LongArray(vec![1, 2, 3]).value_type(),
            ValueType::LongArray
        );
        assert_eq!(
            PropertyValue::StringMap(HashMap::new()).value_type(),
            ValueType::String
        );
    }

    #[test]
    fn test_is_scalar() {
        assert!(PropertyValue::Long(42).is_scalar());
        assert!(PropertyValue::String("hello".to_string()).is_scalar());
        assert!(!PropertyValue::LongArray(vec![1, 2]).is_scalar());
        assert!(!PropertyValue::StringMap(HashMap::new()).is_scalar());
    }

    #[test]
    fn test_accessors() {
        let long_val = PropertyValue::Long(42);
        assert_eq!(long_val.as_long(), Some(42));
        assert_eq!(long_val.as_double(), None);

        let double_arr = PropertyValue::DoubleArray(vec![1.0, 2.0, 3.0]);
        assert_eq!(
            double_arr.as_double_array(),
            Some([1.0, 2.0, 3.0].as_slice())
        );
        assert_eq!(double_arr.as_long(), None);
    }
}

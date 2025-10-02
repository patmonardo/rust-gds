use crate::types::value_type::ValueType;
use crate::values::primitive::array_equals as eq;
use crate::values::primitive::traits::*;
use serde_json::Value as JsonValue;
use std::sync::Arc;

/// Owned long-array backed by Vec<i64>
#[derive(Clone)]
pub struct DefLongArray {
    data: Arc<Vec<i64>>,
}
impl DefLongArray {
    pub fn new(data: Vec<i64>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}
impl LongArray for DefLongArray {
    fn long_array_value(&self) -> Vec<i64> {
        (*self.data).clone()
    }
    fn long_value_at(&self, idx: usize) -> i64 {
        self.data[idx]
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl GdsValue for DefLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

/// Int32-backed long array
#[derive(Clone)]
pub struct DefIntLongArray {
    data: Arc<Vec<i32>>,
}
impl DefIntLongArray {
    pub fn new(data: Vec<i32>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}
impl LongArray for DefIntLongArray {
    fn long_array_value(&self) -> Vec<i64> {
        self.data.iter().map(|v| *v as i64).collect()
    }
    fn long_value_at(&self, idx: usize) -> i64 {
        self.data[idx] as i64
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl GdsValue for DefIntLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

/// Int16-backed long array
#[derive(Clone)]
pub struct DefShortLongArray {
    data: Arc<Vec<i16>>,
}
impl DefShortLongArray {
    pub fn new(data: Vec<i16>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}
impl LongArray for DefShortLongArray {
    fn long_array_value(&self) -> Vec<i64> {
        self.data.iter().map(|v| *v as i64).collect()
    }
    fn long_value_at(&self, idx: usize) -> i64 {
        self.data[idx] as i64
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl GdsValue for DefShortLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

/// Byte (u8)-backed long array
#[derive(Clone)]
pub struct DefByteLongArray {
    data: Arc<Vec<u8>>,
}
impl DefByteLongArray {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}
impl LongArray for DefByteLongArray {
    fn long_array_value(&self) -> Vec<i64> {
        self.data.iter().map(|v| *v as i64).collect()
    }
    fn long_value_at(&self, idx: usize) -> i64 {
        self.data[idx] as i64
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl DefByteLongArray {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}
impl GdsValue for DefByteLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

/// Scalar long value
#[derive(Clone)]
pub struct DefLongValue(pub i64);
impl IntegralValue for DefLongValue {
    fn long_value(&self) -> i64 {
        self.0
    }
}
impl GdsValue for DefLongValue {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.0)
    }
}

/// Scalar floating point value
#[derive(Clone)]
pub struct DefFloatingPointValue(pub f64);
impl FloatingPointValue for DefFloatingPointValue {
    fn double_value(&self) -> f64 {
        self.0
    }
}
impl GdsValue for DefFloatingPointValue {
    fn value_type(&self) -> ValueType {
        ValueType::Double
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.0)
    }
}

/// Double-array backed by Vec<f64>
#[derive(Clone)]
pub struct DefDoubleArray {
    data: Arc<Vec<f64>>,
}
impl DefDoubleArray {
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}
impl DoubleArray for DefDoubleArray {
    fn double_array_value(&self) -> Vec<f64> {
        (*self.data).clone()
    }
    fn double_value_at(&self, idx: usize) -> f64 {
        self.data[idx]
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl GdsValue for DefDoubleArray {
    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.double_array_value())
    }
}

/// Float-array backed by Vec<f32>
#[derive(Clone)]
pub struct DefFloatArray {
    data: Arc<Vec<f32>>,
}
impl DefFloatArray {
    pub fn new(data: Vec<f32>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }
}
impl FloatArray for DefFloatArray {
    fn float_array_value(&self) -> Vec<f32> {
        (*self.data).clone()
    }
    fn double_value_at(&self, idx: usize) -> f64 {
        self.data[idx] as f64
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
impl GdsValue for DefFloatArray {
    fn value_type(&self) -> ValueType {
        ValueType::FloatArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(
            self.float_array_value()
                .into_iter()
                .map(|f| f as f64)
                .collect::<Vec<_>>(),
        )
    }
}

use std::sync::Arc;

use arrow2::array::{Array, BooleanArray, ListArray, PrimitiveArray, Utf8Array};
use arrow2::datatypes::DataType as ArrowDataType;
use arrow2::types::NativeType;
use serde_json::Value as JsonValue;

use crate::types::node_property_values::NodePropertyValues;
use crate::types::value_type::ValueType;

/// Adapter that wraps an Arrow2 Array (zero-copy) and implements NodePropertyValues.
///
/// NOTE: This struct does NOT own or pin the mmap itself. The caller must ensure
/// that whatever produced the Arrow2 Array (e.g. Arrow2MmapColumn inside your mmap store)
/// remains alive for as long as this adapter is used. The Array may reference the mmap bytes.
#[derive(Clone)]
pub struct Arrow2NodePropertyValues {
    array: Arc<dyn Array>,
}

impl Arrow2NodePropertyValues {
    /// Construct from an Arc<dyn Array> produced by arrow2 parsing over an mmaped buffer.
    pub fn new(array: Arc<dyn Array>) -> Self {
        Self { array }
    }

    fn any_to_json_from_array(&self, idx: usize) -> Option<JsonValue> {
        match &self.array.data_type() {
            ArrowDataType::Float64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<f64>>()?;
                if pa.is_valid(idx) {
                    Some(JsonValue::from(pa.values()[idx]))
                } else {
                    Some(JsonValue::Null)
                }
            }
            ArrowDataType::Int64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<i64>>()?;
                if pa.is_valid(idx) {
                    Some(JsonValue::from(pa.values()[idx]))
                } else {
                    Some(JsonValue::Null)
                }
            }
            ArrowDataType::UInt64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<u64>>()?;
                if pa.is_valid(idx) {
                    Some(JsonValue::from(pa.values()[idx]))
                } else {
                    Some(JsonValue::Null)
                }
            }
            ArrowDataType::Utf8 => {
                let ua = self.array.as_any().downcast_ref::<Utf8Array<i32>>()?;
                match ua.value(idx) {
                    Some(s) => Some(JsonValue::from(s.to_string())),
                    None => Some(JsonValue::Null),
                }
            }
            ArrowDataType::Boolean => {
                let ba = self.array.as_any().downcast_ref::<BooleanArray>()?;
                if ba.is_valid(idx) {
                    Some(JsonValue::from(ba.values()[idx]))
                } else {
                    Some(JsonValue::Null)
                }
            }
            ArrowDataType::List(_) => {
                let la = self.array.as_any().downcast_ref::<ListArray<i32>>()?;
                if !la.is_valid(idx) {
                    return Some(JsonValue::Null);
                }
                let value = la.value(idx);
                // try to convert inner array elements to json (best-effort)
                let mut out = Vec::with_capacity(value.len());
                for i in 0..value.len() {
                    // recursively convert using a small helper that handles common types
                    match value.data_type() {
                        ArrowDataType::Float64 => {
                            if let Some(pa) = value.as_any().downcast_ref::<PrimitiveArray<f64>>() {
                                if pa.is_valid(i) {
                                    out.push(JsonValue::from(pa.values()[i]));
                                } else {
                                    out.push(JsonValue::Null);
                                }
                            }
                        }
                        ArrowDataType::Int64 => {
                            if let Some(pa) = value.as_any().downcast_ref::<PrimitiveArray<i64>>() {
                                if pa.is_valid(i) {
                                    out.push(JsonValue::from(pa.values()[i]));
                                } else {
                                    out.push(JsonValue::Null);
                                }
                            }
                        }
                        ArrowDataType::Utf8 => {
                            if let Some(ua) = value.as_any().downcast_ref::<Utf8Array<i32>>() {
                                match ua.value(i) {
                                    Some(s) => out.push(JsonValue::from(s.to_string())),
                                    None => out.push(JsonValue::Null),
                                }
                            }
                        }
                        _ => {
                            out.push(JsonValue::from(format!("{:?}", value.data_type())));
                        }
                    }
                }
                Some(JsonValue::from(out))
            }
            other => Some(JsonValue::from(format!("unsupported type {:?}", other))),
        }
    }
}

/// Helper to downcast primitive arrays generically for max computations.
fn primitive_max<T>(arr: &PrimitiveArray<T>) -> Option<T>
where
    T: NativeType + PartialOrd + Copy,
{
    let mut max: Option<T> = None;
    for i in 0..arr.len() {
        if !arr.is_valid(i) {
            continue;
        }
        let v = arr.values()[i];
        max = Some(match max {
            None => v,
            Some(curr) => {
                if v > curr {
                    v
                } else {
                    curr
                }
            }
        });
    }
    max
}

impl NodePropertyValues for Arrow2NodePropertyValues {
    fn value_type(&self) -> ValueType {
        match &self.array.data_type() {
            ArrowDataType::Utf8 => ValueType::String,
            ArrowDataType::Int64 => ValueType::Long,
            ArrowDataType::Float64 => ValueType::Double,
            ArrowDataType::Boolean => ValueType::Boolean,
            other => ValueType::Other(format!("{:?}", other)),
        }
    }

    fn double_value(&self, idx: usize) -> Option<f64> {
        if idx >= self.array.len() {
            return None;
        }
        match &self.array.data_type() {
            ArrowDataType::Float64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<f64>>()?;
                if pa.is_valid(idx) {
                    Some(pa.values()[idx])
                } else {
                    None
                }
            }
            ArrowDataType::Int64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<i64>>()?;
                if pa.is_valid(idx) {
                    Some(pa.values()[idx] as f64)
                } else {
                    None
                }
            }
            ArrowDataType::UInt64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<u64>>()?;
                if pa.is_valid(idx) {
                    Some(pa.values()[idx] as f64)
                } else {
                    None
                }
            }
            ArrowDataType::Utf8 => {
                // attempt parse
                let ua = self.array.as_any().downcast_ref::<Utf8Array<i32>>()?;
                ua.value(idx).and_then(|s| s.parse::<f64>().ok())
            }
            _ => None,
        }
    }

    fn long_value(&self, idx: usize) -> Option<i64> {
        if idx >= self.array.len() {
            return None;
        }
        match &self.array.data_type() {
            ArrowDataType::Int64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<i64>>()?;
                if pa.is_valid(idx) {
                    Some(pa.values()[idx])
                } else {
                    None
                }
            }
            ArrowDataType::UInt64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<u64>>()?;
                if pa.is_valid(idx) {
                    Some(pa.values()[idx] as i64)
                } else {
                    None
                }
            }
            ArrowDataType::Float64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<f64>>()?;
                if pa.is_valid(idx) {
                    Some(pa.values()[idx] as i64)
                } else {
                    None
                }
            }
            ArrowDataType::Utf8 => {
                let ua = self.array.as_any().downcast_ref::<Utf8Array<i32>>()?;
                ua.value(idx).and_then(|s| s.parse::<i64>().ok())
            }
            _ => None,
        }
    }

    fn get_object(&self, idx: usize) -> Option<JsonValue> {
        if idx >= self.array.len() {
            return None;
        }
        self.any_to_json_from_array(idx)
    }

    fn node_count(&self) -> usize {
        self.array.len()
    }

    fn dimension(&self) -> Option<usize> {
        match &self.array.data_type() {
            ArrowDataType::List(_) => {
                let la = self.array.as_any().downcast_ref::<ListArray<i32>>()?;
                if la.len() == 0 {
                    return None;
                }
                if !la.is_valid(0) {
                    return None;
                }
                let inner = la.value(0);
                Some(inner.len())
            }
            _ => Some(1),
        }
    }

    fn max_long(&self) -> Option<i64> {
        match &self.array.data_type() {
            ArrowDataType::Int64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<i64>>()?;
                primitive_max(pa)
            }
            ArrowDataType::UInt64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<u64>>()?;
                primitive_max(pa).map(|v| v as i64)
            }
            ArrowDataType::Float64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<f64>>()?;
                primitive_max(pa).map(|v| v as i64)
            }
            _ => None,
        }
    }

    fn max_double(&self) -> Option<f64> {
        match &self.array.data_type() {
            ArrowDataType::Float64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<f64>>()?;
                primitive_max(pa)
            }
            ArrowDataType::Int64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<i64>>()?;
                primitive_max(pa).map(|v| v as f64)
            }
            ArrowDataType::UInt64 => {
                let pa = self.array.as_any().downcast_ref::<PrimitiveArray<u64>>()?;
                primitive_max(pa).map(|v| v as f64)
            }
            _ => None,
        }
    }
}

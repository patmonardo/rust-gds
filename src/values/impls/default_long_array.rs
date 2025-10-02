use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::types::value_type::ValueType;
use crate::values::primitive::traits::{GdsValue, LongArray};

/// Owned long-array backed by Vec<i64>
#[derive(Clone, Debug)]
pub struct DefaultLongArray {
    data: Arc<Vec<i64>>,
}

impl DefaultLongArray {
    pub fn new(data: Vec<i64>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    pub fn from_arc(data: Arc<Vec<i64>>) -> Self {
        Self { data }
    }

    pub fn as_slice(&self) -> &[i64] {
        &self.data
    }
}

impl LongArray for DefaultLongArray {
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

impl GdsValue for DefaultLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }

    fn as_object(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

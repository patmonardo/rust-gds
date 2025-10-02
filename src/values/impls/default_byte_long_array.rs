use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::types::value_type::ValueType;
use crate::values::primitive::traits::{GdsValue, LongArray};

/// Byte (u8)-backed long array — explicit "Default" naming for clarity.
#[derive(Clone, Debug)]
pub struct DefaultByteLongArray {
    data: Arc<Vec<u8>>,
}

impl DefaultByteLongArray {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    /// Return underlying bytes slice (zero-copy view)
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl LongArray for DefaultByteLongArray {
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

impl GdsValue for DefaultByteLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }

    fn as_object(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

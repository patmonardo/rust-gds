use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::types::value_type::ValueType;
use crate::values::primitive::traits::{GdsValue, LongArray};

/// Short (i16)-backed long array — renamed to use explicit "Default" prefix.
#[derive(Clone, Debug)]
pub struct DefaultShortLongArray {
    data: Arc<Vec<i16>>,
}

impl DefaultShortLongArray {
    pub fn new(data: Vec<i16>) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    pub fn as_slice(&self) -> &[i16] {
        &self.data
    }
}

impl LongArray for DefaultShortLongArray {
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

impl GdsValue for DefaultShortLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }

    fn as_object(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

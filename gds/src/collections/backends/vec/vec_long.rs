//! VecLong: Vec-based i64 Collections implementation
#[allow(unused_imports)]
use crate::collections::traits::Collections;

/// Vec-based i64 Collections implementation
#[derive(Debug, Clone)]
pub struct VecLong {
    pub data: Vec<i64>,
}

impl Default for VecLong {
    fn default() -> Self {
        Self::new()
    }
}

impl VecLong {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: i64) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
}

impl From<Vec<i64>> for VecLong {
    fn from(data: Vec<i64>) -> Self {
        Self { data }
    }
}

use crate::vec_collections;
use crate::types::ValueType;
use crate::collections::traits::PropertyValuesAdapter;

vec_collections!(VecLong, i64, ValueType::Long, 0i64, to_f64 = |x: i64| x as f64, kind = Ord);

// Implement PropertyValuesAdapter (marker trait)
impl PropertyValuesAdapter<i64> for VecLong {}

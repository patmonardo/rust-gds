//! VecDouble: Vec-based f64 Collections implementation

use crate::collections::traits::Collections;

/// Vec-based f64 Collections implementation
#[derive(Debug, Clone)]
pub struct VecDouble {
    pub data: Vec<f64>,
}

impl VecDouble {
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

    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.data.pop()
    }
}

impl From<Vec<f64>> for VecDouble {
    fn from(data: Vec<f64>) -> Self {
        Self { data }
    }
}

use crate::vec_collections;
use crate::types::ValueType;
vec_collections!(VecDouble, f64, ValueType::Double, 0.0f64, kind = Float);

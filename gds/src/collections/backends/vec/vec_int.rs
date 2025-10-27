//! VecInt: Vec-based i32 Collections implementation
use crate::collections::traits::Collections;
/// Vec-based i32 Collections implementation
#[derive(Debug, Clone)]
pub struct VecInt {
    pub data: Vec<i32>,
}
impl VecInt {
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
    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
}
impl From<Vec<i32>> for VecInt {
    fn from(data: Vec<i32>) -> Self {
        Self { data }
    }
}
use crate::vec_collections;
use crate::types::ValueType;
vec_collections!(VecInt, i32, ValueType::Int, 0i32, to_f64 = |x: i32| x as f64, kind = Ord);

use crate::collections::traits::PropertyValuesAdapter;

// Implement PropertyValuesAdapter (marker trait)
impl PropertyValuesAdapter<i32> for VecInt {}

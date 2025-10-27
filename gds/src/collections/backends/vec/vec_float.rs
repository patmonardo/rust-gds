//! VecFloat: Vec-based f32 Collections implementation
#[allow(unused_imports)]
use crate::collections::traits::Collections;

/// Vec-based f32 Collections implementation
#[derive(Debug, Clone)]
pub struct VecFloat {
    pub data: Vec<f32>,
}
impl Default for VecFloat {
    fn default() -> Self {
        Self::new()
    }
}

impl VecFloat {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}
impl From<Vec<f32>> for VecFloat {
    fn from(data: Vec<f32>) -> Self {
        Self { data }
    }
}
use crate::vec_collections;
use crate::types::ValueType;
vec_collections!(VecFloat, f32, ValueType::Float, 0.0f32, kind = Float);

use crate::collections::traits::PropertyValuesAdapter;

// Implement PropertyValuesAdapter (marker trait)
impl PropertyValuesAdapter<f32> for VecFloat {}

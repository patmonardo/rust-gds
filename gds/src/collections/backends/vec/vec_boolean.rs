//! VecBoolean: Vec-based bool Collections implementation
#[derive(Debug, Clone)]
pub struct VecBoolean { pub data: Vec<bool> }
impl VecBoolean { pub fn new() -> Self { Self { data: Vec::new() } } }
use crate::vec_collections;
use crate::types::ValueType;
vec_collections!(VecBoolean, bool, ValueType::Boolean, false, kind = OrdNoAgg);

use crate::collections::traits::PropertyValuesAdapter;

// Implement PropertyValuesAdapter (marker trait)
impl PropertyValuesAdapter<bool> for VecBoolean {}

//! VecByte: Vec-based i8 Collections implementation
#[derive(Debug, Clone)]
pub struct VecByte { pub data: Vec<i8> }
impl VecByte { pub fn new() -> Self { Self { data: Vec::new() } } }
use crate::vec_collections;
use crate::types::ValueType;
vec_collections!(VecByte, i8, ValueType::Byte, 0i8, to_f64 = |x: i8| x as f64, kind = Ord);

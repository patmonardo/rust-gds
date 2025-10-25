//! VecChar: Vec-based char Collections implementation
#[derive(Debug, Clone)]
pub struct VecChar { pub data: Vec<char> }
impl VecChar { pub fn new() -> Self { Self { data: Vec::new() } } }
use crate::vec_collections;
use crate::types::ValueType;
vec_collections!(VecChar, char, ValueType::Char, '\0', kind = OrdNoAgg);

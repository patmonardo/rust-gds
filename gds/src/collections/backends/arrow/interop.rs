use std::sync::Arc;

use arrow2::array::{Array, PrimitiveArray};
use arrow2::chunk::Chunk;

use crate::collections::backends::arrow::array::ArrowPrimitiveArray;
use crate::collections::backends::arrow::chunk::ArrowChunk;
use crate::collections::backends::arrow::error::ArrowResult;

/// Entry points for translating between Arrow-native tables and Collections.
#[derive(Debug, Default)]
pub struct ArrowInterOp;

impl ArrowInterOp {
    pub fn chunk_from_chunk(chunk: Chunk<Arc<dyn Array>>) -> ArrowChunk {
        ArrowChunk::from_chunk(chunk)
    }

    pub fn primitive_from_array<T>(array: Arc<PrimitiveArray<T>>) -> ArrowPrimitiveArray<T>
    where
        T: arrow2::types::NativeType,
    {
        ArrowPrimitiveArray::from_arc(array)
    }

    pub fn validate_chunk(_chunk: &ArrowChunk) -> ArrowResult<()> {
        Ok(())
    }
}

/// Shared view type so importer code can keep metadata at hand.
#[derive(Debug, Clone)]
pub struct ArrowTableView {
    pub name: String,
    pub row_count: usize,
}

use std::sync::Arc;

use arrow2::array::{Array, MutablePrimitiveArray, PrimitiveArray};
use arrow2::chunk::Chunk;
use arrow2::types::NativeType;

use crate::collections::backends::arrow::array::ArrowPrimitiveArray;
use crate::collections::backends::arrow::chunk::ArrowChunk;

/// Mutable buffer for populating Arrow primitive arrays prior to freezing.
#[derive(Debug)]
pub struct ArrowPrimitiveBuilder<T>
where
    T: NativeType,
{
    mutable: MutablePrimitiveArray<T>,
}

impl<T> ArrowPrimitiveBuilder<T>
where
    T: NativeType,
{
    pub fn new() -> Self {
        Self {
            mutable: MutablePrimitiveArray::new(),
        }
    }

    pub fn push(&mut self, value: Option<T>) {
        self.mutable.push(value);
    }

    pub fn freeze(self) -> ArrowPrimitiveArray<T> {
        let primitive: PrimitiveArray<T> = self.mutable.into();
        let array = Arc::new(primitive);
        ArrowPrimitiveArray::from_arc(array)
    }
}

/// Container builder for assembling `ArrowChunk` instances.
#[derive(Debug, Default)]
pub struct ArrowChunkBuilder {
    columns: Vec<Arc<dyn Array>>,
}

impl ArrowChunkBuilder {
    pub fn with_column(mut self, column: Arc<dyn Array>) -> Self {
        self.columns.push(column);
        self
    }

    pub fn columns(&self) -> &[Arc<dyn Array>] {
        &self.columns
    }

    pub fn build(self) -> ArrowChunk {
        ArrowChunk::from_chunk(Chunk::new(self.columns))
    }
}

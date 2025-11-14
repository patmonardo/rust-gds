use std::sync::Arc;

use arrow2::array::Array;
use arrow2::chunk::Chunk;

/// Wrapper that keeps Arrow record batch semantics explicit inside Collections.
#[derive(Clone, Debug)]
pub struct ArrowChunk {
    chunk: Chunk<Arc<dyn Array>>,
}

impl ArrowChunk {
    pub fn from_chunk(chunk: Chunk<Arc<dyn Array>>) -> Self {
        Self { chunk }
    }

    pub fn len(&self) -> usize {
        self.chunk.len()
    }

    pub fn column(&self, index: usize) -> Option<&Arc<dyn Array>> {
        self.chunk.arrays().get(index)
    }

    pub fn columns(&self) -> &[Arc<dyn Array>] {
        self.chunk.arrays()
    }
}

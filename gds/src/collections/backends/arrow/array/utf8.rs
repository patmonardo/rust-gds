use std::sync::Arc;

use arrow2::array::{Array, Utf8Array};

use super::ArrowArrayBehavior;
use crate::collections::backends::arrow::bitmap::ArrowBitmap;

#[derive(Clone, Debug)]
pub struct ArrowUtf8Array {
    array: Arc<Utf8Array<i32>>,
    bitmap: Option<ArrowBitmap>,
}

impl ArrowUtf8Array {
    pub fn from_arc(array: Arc<Utf8Array<i32>>) -> Self {
        let bitmap = array.validity().cloned().map(ArrowBitmap::from_bitmap);
        Self { array, bitmap }
    }

    pub fn values(&self) -> &Utf8Array<i32> {
        &self.array
    }
}

impl ArrowArrayBehavior for ArrowUtf8Array {
    fn len(&self) -> usize {
        self.array.len()
    }

    fn null_count(&self) -> usize {
        self.array.null_count()
    }

    fn is_null(&self, index: usize) -> bool {
        self.array.is_null(index)
    }
}

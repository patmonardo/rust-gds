use std::sync::Arc;

use arrow2::array::{Array, BooleanArray};

use super::ArrowArrayBehavior;
use crate::collections::backends::arrow::bitmap::ArrowBitmap;

#[derive(Clone, Debug)]
pub struct ArrowBooleanArray {
    array: Arc<BooleanArray>,
    bitmap: Option<ArrowBitmap>,
}

impl ArrowBooleanArray {
    pub fn from_arc(array: Arc<BooleanArray>) -> Self {
        let bitmap = array.validity().cloned().map(ArrowBitmap::from_bitmap);
        Self { array, bitmap }
    }

    pub fn values(&self) -> &arrow2::bitmap::Bitmap {
        self.array.values()
    }

    pub fn array(&self) -> &BooleanArray {
        &self.array
    }
}

impl ArrowArrayBehavior for ArrowBooleanArray {
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

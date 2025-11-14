use std::sync::Arc;

use arrow2::array::{Array, PrimitiveArray};
use arrow2::types::NativeType;

use super::ArrowArrayBehavior;
use crate::collections::backends::arrow::bitmap::ArrowBitmap;

/// Immutable wrapper for Arrow primitive arrays.
#[derive(Clone, Debug)]
pub struct ArrowPrimitiveArray<T>
where
    T: NativeType,
{
    array: Arc<PrimitiveArray<T>>,
    bitmap: Option<ArrowBitmap>,
}

impl<T> ArrowPrimitiveArray<T>
where
    T: NativeType,
{
    pub fn from_arc(array: Arc<PrimitiveArray<T>>) -> Self {
        let bitmap = array.validity().cloned().map(ArrowBitmap::from_bitmap);
        Self { array, bitmap }
    }

    pub fn values(&self) -> &[T] {
        self.array.values()
    }

    pub fn array(&self) -> &PrimitiveArray<T> {
        &self.array
    }
}

impl<T> ArrowArrayBehavior for ArrowPrimitiveArray<T>
where
    T: NativeType,
{
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

use std::ops::Add;

use arrow2::types::NativeType;

use crate::collections::backends::arrow::array::ArrowPrimitiveArray;
use crate::collections::backends::arrow::error::{ArrowKernelError, ArrowResult};

/// Placeholder for SIMD-aware kernel helpers.
#[derive(Debug, Default, Clone, Copy)]
pub struct KernelExtensions;

impl KernelExtensions {
    pub fn sum<T>(array: &ArrowPrimitiveArray<T>) -> ArrowResult<Option<T>>
    where
        T: NativeType + Add<Output = T> + Default,
    {
        let mut acc = T::default();
        for value in array.values() {
            acc = acc + *value;
        }
        Ok(Some(acc))
    }

    pub fn validate_bitmap(len: usize, bitmap_len: usize) -> ArrowResult<()> {
        if bitmap_len < len {
            return Err(ArrowKernelError::Bitmap(format!(
                "bitmap length {bitmap_len} shorter than array length {len}"
            ))
            .into());
        }
        Ok(())
    }
}

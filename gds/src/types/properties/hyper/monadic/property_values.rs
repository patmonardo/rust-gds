//! Minimal monadic property values for the HyperStore proof-of-concept.
use crate::collections::backends::vec::VecLong;
use crate::types::properties::property_values::PropertyValues;
use crate::types::ValueType;
use std::fmt::Debug;

/// Long-typed monadic property values backed by a VecLong collections backend.
#[derive(Clone, Debug)]
pub struct MonadicLongPropertyValues {
    pub inner: VecLong,
    pub default_index: usize,
}

impl MonadicLongPropertyValues {
    pub fn new(inner: VecLong, default_index: usize) -> Self {
        Self {
            inner,
            default_index,
        }
    }
}

impl PropertyValues for MonadicLongPropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.inner.data.len()
    }
}

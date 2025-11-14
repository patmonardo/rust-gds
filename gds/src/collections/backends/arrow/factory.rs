use std::marker::PhantomData;
use std::sync::Arc;

use arrow2::array::PrimitiveArray;

use crate::collections::traits::CollectionsFactory;
use crate::types::ValueType;

use super::array::ArrowPrimitiveArray;
use super::descriptor::ArrowBackendDescriptor;

/// Factory for constructing Arrow-backed primitive collections.
#[derive(Debug, Clone)]
pub struct ArrowCollectionsFactory<T>
where
    T: arrow2::types::NativeType,
{
    descriptor: ArrowBackendDescriptor,
    _marker: PhantomData<T>,
}

impl<T> ArrowCollectionsFactory<T>
where
    T: arrow2::types::NativeType,
{
    fn default_descriptor() -> ArrowBackendDescriptor {
        ArrowBackendDescriptor::new("arrow", ValueType::Unknown)
    }

    pub fn new(descriptor: ArrowBackendDescriptor) -> Self {
        Self {
            descriptor,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn descriptor(&self) -> &ArrowBackendDescriptor {
        &self.descriptor
    }
}

impl<T> CollectionsFactory<T> for ArrowCollectionsFactory<T>
where
    T: arrow2::types::NativeType,
{
    fn new() -> Self {
        ArrowCollectionsFactory::new(Self::default_descriptor())
    }

    fn with_capacity(_capacity: usize) -> Self {
        ArrowCollectionsFactory::new(Self::default_descriptor())
    }

    fn from_vec(_values: Vec<T>) -> Self {
        ArrowCollectionsFactory::new(Self::default_descriptor())
    }

    fn from_slice(_slice: &[T]) -> Self {
        ArrowCollectionsFactory::new(Self::default_descriptor())
    }

    fn with_defaults(_count: usize, _default_value: T) -> Self {
        ArrowCollectionsFactory::new(Self::default_descriptor())
    }
}

impl<T> ArrowCollectionsFactory<T>
where
    T: arrow2::types::NativeType,
{
    pub fn from_arrow(array: Arc<PrimitiveArray<T>>) -> ArrowPrimitiveArray<T> {
        ArrowPrimitiveArray::from_arc(array)
    }
}

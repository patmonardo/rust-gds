//! Tensor trait for ML computations.
//!
//! Translated from Java GDS ml-core Tensor.java.

use std::fmt;

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait Tensor: fmt::Debug + fmt::Display + AsAny {
    fn dimensions(&self) -> &[usize];
    fn data(&self) -> &[f64];
    fn set_data_at(&mut self, idx: usize, new_value: f64);
    fn clone_box(&self) -> Box<dyn Tensor>;
    fn create_with_same_dimensions(&self) -> Box<dyn Tensor>;
    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor>;
    fn add_inplace(&mut self, other: &dyn Tensor);
    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor>;
    fn scalar_multiply_mutate(&mut self, scalar: f64);
    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor>;
    fn elementwise_product_mutate(&mut self, other: &dyn Tensor);
    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor>;
    fn map_inplace(&mut self, f: fn(f64) -> f64);
    fn ones_like(&self) -> Box<dyn Tensor>;
    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool;

    fn total_size(&self) -> usize {
        self.dimensions().iter().product()
    }

    fn aggregate_sum(&self) -> f64 {
        self.data().iter().sum()
    }
}

pub fn size_in_bytes(dimensions: &[usize]) -> usize {
    let total_elements: usize = dimensions.iter().product();
    total_elements * std::mem::size_of::<f64>()
}

//! Scalar tensor - translated from Scalar.java

use super::tensor::Tensor;
use crate::ml::core::dimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct Scalar {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        Self {
            data: vec![value],
            dimensions: dimensions::scalar(),
        }
    }

    pub fn value(&self) -> f64 {
        self.data[0]
    }
}

impl Tensor for Scalar {
    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    fn data(&self) -> &[f64] {
        &self.data
    }

    fn set_data_at(&mut self, idx: usize, new_value: f64) {
        self.data[idx] = new_value;
    }

    fn clone_box(&self) -> Box<dyn Tensor> {
        Box::new(self.clone())
    }

    fn create_with_same_dimensions(&self) -> Box<dyn Tensor> {
        Box::new(Scalar::new(0.0))
    }

    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        Box::new(Scalar::new(self.value() + other_scalar.value()))
    }

    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        self.data[0] += other_scalar.data[0];
    }

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        Box::new(Scalar::new(self.value() * scalar))
    }

    fn scalar_multiply_mutate(&mut self, scalar: f64) {
        self.data[0] *= scalar;
    }

    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        Box::new(Scalar::new(self.value() * other_scalar.value()))
    }

    fn elementwise_product_mutate(&mut self, other: &dyn Tensor) {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        self.data[0] *= other_scalar.data[0];
    }

    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        Box::new(Scalar::new(f(self.value())))
    }

    fn map_inplace(&mut self, f: fn(f64) -> f64) {
        self.data[0] = f(self.data[0]);
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Scalar::new(1.0))
    }

    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_scalar) = other.as_any().downcast_ref::<Scalar>() {
            (self.value() - other_scalar.value()).abs() <= tolerance
        } else {
            false
        }
    }
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scalar: {}", self.value())
    }
}

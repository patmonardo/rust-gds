//! Vector tensor - translated from Vector.java

use super::tensor::Tensor;
use crate::ml::core::dimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl Vector {
    pub fn new(data: Vec<f64>) -> Self {
        let len = data.len();
        Self {
            data,
            dimensions: dimensions::vector(len),
        }
    }

    pub fn with_size(size: usize) -> Self {
        Self::new(vec![0.0; size])
    }

    pub fn create(value: f64, size: usize) -> Self {
        Self::new(vec![value; size])
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

impl Tensor for Vector {
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
        Box::new(Vector::with_size(self.length()))
    }

    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        assert_eq!(self.length(), other_vector.length());
        
        let result_data: Vec<f64> = self.data.iter()
            .zip(other_vector.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Box::new(Vector::new(result_data))
    }

    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        for (a, b) in self.data.iter_mut().zip(other_vector.data.iter()) {
            *a += b;
        }
    }

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let result_data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Box::new(Vector::new(result_data))
    }

    fn scalar_multiply_mutate(&mut self, scalar: f64) {
        for x in &mut self.data {
            *x *= scalar;
        }
    }

    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        let result_data: Vec<f64> = self.data.iter()
            .zip(other_vector.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        Box::new(Vector::new(result_data))
    }

    fn elementwise_product_mutate(&mut self, other: &dyn Tensor) {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        for (a, b) in self.data.iter_mut().zip(other_vector.data.iter()) {
            *a *= b;
        }
    }

    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let result_data: Vec<f64> = self.data.iter().map(|&x| f(x)).collect();
        Box::new(Vector::new(result_data))
    }

    fn map_inplace(&mut self, f: fn(f64) -> f64) {
        for x in &mut self.data {
            *x = f(*x);
        }
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Vector::create(1.0, self.length()))
    }

    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_vector) = other.as_any().downcast_ref::<Vector>() {
            if self.length() != other_vector.length() {
                return false;
            }
            self.data.iter()
                .zip(other_vector.data.iter())
                .all(|(a, b)| (a - b).abs() <= tolerance)
        } else {
            false
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}): {:?}", self.length(), self.data)
    }
}

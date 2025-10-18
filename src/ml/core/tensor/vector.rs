//! Vector tensor - translated from Vector.java
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This Vector wraps a TensorData (composition) to share storage and methods.
//! This matches Java's inheritance: Vector extends Tensor<Vector>
//!
//! - TensorData provides: data, dimensions, aggregate_sum(), map(), etc.
//! - Vector adds: length(), convenience accessors
//! - Vector delegates shared operations to inner TensorData

use super::tensor::Tensor;
use super::tensor_data::TensorData;
use crate::ml::core::dimensions;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    tensor: TensorData, // COMPOSITION: wraps shared storage/methods
}

impl Vector {
    // ========================================================================
    // Constructors
    // ========================================================================

    pub fn new(data: Vec<f64>) -> Self {
        let len = data.len();
        let tensor = TensorData::new(data, dimensions::vector(len));
        Self { tensor }
    }

    pub fn with_size(size: usize) -> Self {
        let tensor = TensorData::zeros(dimensions::vector(size));
        Self { tensor }
    }

    pub fn zeros(size: usize) -> Self {
        Self::with_size(size)
    }

    pub fn create(value: f64, size: usize) -> Self {
        let tensor = TensorData::filled(value, dimensions::vector(size));
        Self { tensor }
    }

    // ========================================================================
    // Vector-specific accessors
    // ========================================================================

    pub fn length(&self) -> usize {
        self.tensor.data().len()
    }

    pub fn len(&self) -> usize {
        self.length()
    }

    /// DELEGATION: Get value at index (convenience wrapper).
    pub fn data_at(&self, index: usize) -> f64 {
        self.tensor.data_at(index)
    }

    /// DELEGATION: Set value at index (convenience wrapper).
    pub fn set_data_at(&mut self, index: usize, value: f64) {
        self.tensor.set_data_at(index, value);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.tensor.data().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.tensor.data_mut().iter_mut()
    }

    // ========================================================================
    // Delegation to TensorData (shared methods)
    // ========================================================================

    /// DELEGATION: Get raw data slice.
    pub fn data(&self) -> &[f64] {
        self.tensor.data()
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.tensor.data().to_vec()
    }
}

// ============================================================================
// Tensor Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Most methods delegate to the inner TensorData.
// Vector-specific logic wraps the result in a new Vector.

impl Tensor for Vector {
    // DELEGATION: Forward to TensorData
    fn dimensions(&self) -> &[usize] {
        self.tensor.dimensions()
    }

    // DELEGATION: Forward to TensorData
    fn data(&self) -> &[f64] {
        self.tensor.data()
    }

    // DELEGATION: Forward to TensorData
    fn set_data_at(&mut self, idx: usize, new_value: f64) {
        self.tensor.set_data_at(idx, new_value);
    }

    fn clone_box(&self) -> Box<dyn Tensor> {
        Box::new(self.clone())
    }

    fn create_with_same_dimensions(&self) -> Box<dyn Tensor> {
        Box::new(Vector::with_size(self.length()))
    }

    // DELEGATION: Use TensorData.add(), wrap result
    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        assert_eq!(self.length(), other_vector.length());

        let result_tensor = self.tensor.add(&other_vector.tensor);
        Box::new(Vector {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.add_inplace()
    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        self.tensor.add_inplace(&other_vector.tensor);
    }

    // DELEGATION: Use TensorData.scalar_multiply(), wrap result
    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let result_tensor = self.tensor.scalar_multiply(scalar);
        Box::new(Vector {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.scalar_multiply_mutate()
    fn scalar_multiply_mutate(&mut self, scalar: f64) {
        self.tensor.scalar_multiply_mutate(scalar);
    }

    // DELEGATION: Use TensorData.elementwise_product(), wrap result
    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        let result_tensor = self.tensor.elementwise_product(&other_vector.tensor);
        Box::new(Vector {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.elementwise_product_mutate()
    fn elementwise_product_mutate(&mut self, other: &dyn Tensor) {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        self.tensor.elementwise_product_mutate(&other_vector.tensor);
    }

    // DELEGATION: Use TensorData.map(), wrap result
    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let result_tensor = self.tensor.map(f);
        Box::new(Vector {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.map_inplace()
    fn map_inplace(&mut self, f: fn(f64) -> f64) {
        self.tensor.map_inplace(f);
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Vector::create(1.0, self.length()))
    }

    // DELEGATION: Forward to TensorData.equals()
    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_vector) = other.as_any().downcast_ref::<Vector>() {
            if self.length() != other_vector.length() {
                return false;
            }
            self.tensor.equals(&other_vector.tensor, tolerance)
        } else {
            false
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tensor.data()[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tensor.data_mut()[index]
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}): {:?}", self.length(), self.data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_produces_new_vector() {
        let lhs = Vector::new(vec![1.0, 2.0, 3.0]);
        let rhs = Vector::new(vec![4.0, 5.0, 6.0]);

        let result = lhs.add(&rhs);
        assert_eq!(result.dimensions(), &[3, 1]);
        assert_eq!(result.data(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn add_inplace_updates_left_operand() {
        let mut lhs = Vector::new(vec![1.0, 2.0, 3.0]);
        let rhs = Vector::new(vec![0.5, 1.5, 2.5]);

        lhs.add_inplace(&rhs);

        assert_eq!(lhs.data(), &[1.5, 3.5, 5.5]);
    }

    #[test]
    fn elementwise_product_matches_componentwise() {
        let lhs = Vector::new(vec![1.0, 2.0, 3.0]);
        let rhs = Vector::new(vec![4.0, 5.0, 6.0]);

        let result = lhs.elementwise_product(&rhs);
        assert_eq!(result.data(), &[4.0, 10.0, 18.0]);
    }

    #[test]
    fn scalar_multiply_respects_factor() {
        let vector = Vector::new(vec![2.0, -4.0, 6.0]);

        let result = vector.scalar_multiply(0.5);
        assert_eq!(result.data(), &[1.0, -2.0, 3.0]);
    }
}

//! Vector tensor - translated from Vector.java
//!
//! This directly mirrors Java's `Vector extends Tensor<Vector>` pattern.
//! Contains data and dimensions directly, not wrapped in TensorData.

use super::tensor::{Tensor, AsAny};
use crate::ml::core::dimensions;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl Vector {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create vector from data array.
    /// Java: `public Vector(double... elements)`
    pub fn new(data: Vec<f64>) -> Self {
        let len = data.len();
        let dimensions = dimensions::vector(len);
        Self { data, dimensions }
    }

    /// Create vector with given size, filled with zeros.
    /// Java: `public Vector(int size)`
    pub fn with_size(size: usize) -> Self {
        let data = vec![0.0; size];
        Self::new(data)
    }

    /// Create vector filled with a constant value.
    /// Java: `public static Vector create(double v, int size)`
    pub fn create(value: f64, size: usize) -> Self {
        let data = vec![value; size];
        Self::new(data)
    }

    // ========================================================================
    // Vector-specific accessors
    // ========================================================================

    /// Get vector length.
    /// Java: `public int length()`
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Get vector length (alias).
    pub fn len(&self) -> usize {
        self.length()
    }

    /// Check if vector is empty.
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Get value at index.
    /// Java: `public double dataAt(int idx)`
    pub fn data_at(&self, index: usize) -> f64 {
        self.data[index]
    }

    /// Set value at index.
    /// Java: `public void setDataAt(int idx, double value)`
    pub fn set_data_at(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }

    /// Get iterator over data.
    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.data.iter()
    }

    /// Get mutable iterator over data.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.data.iter_mut()
    }

    /// Convert to Vec<f64>.
    pub fn to_vec(&self) -> Vec<f64> {
        self.data.clone()
    }
}

// ============================================================================
// Tensor Trait Implementation
// ============================================================================

impl Tensor for Vector {
    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    fn data(&self) -> &[f64] {
        &self.data
    }

    fn data_at(&self, idx: usize) -> f64 {
        self.data[idx]
    }

    fn dimension(&self, dimension_index: usize) -> usize {
        self.dimensions[dimension_index]
    }

    fn create_with_same_dimensions(&self) -> Box<dyn Tensor> {
        Box::new(Vector::with_size(self.length()))
    }

    fn clone_box(&self) -> Box<dyn Tensor> {
        Box::new(self.clone())
    }

    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        assert_eq!(
            self.length(), other_vector.length(),
            "Vector lengths must be equal, got {} + {} lengths",
            self.length(), other_vector.length()
        );
        
        let mut result = Vector::with_size(self.length());
        for i in 0..self.length() {
            result.data[i] = self.data[i] + other_vector.data[i];
        }
        Box::new(result)
    }

    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let new_data: Vec<f64> = self.data.iter().map(|&x| f(x)).collect();
        Box::new(Vector {
            data: new_data,
            dimensions: self.dimensions.clone(),
        })
    }

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let new_data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Box::new(Vector {
            data: new_data,
            dimensions: self.dimensions.clone(),
        })
    }

    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        let mut result = Vector::with_size(self.length());
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] * other_vector.data[i];
        }
        Box::new(result)
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Vector::create(1.0, self.length()))
    }

    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_vector = other.as_any().downcast_ref::<Vector>().unwrap();
        for i in 0..self.data.len() {
            self.data[i] += other_vector.data[i];
        }
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

    fn short_description(&self) -> String {
        format!("Vector({})", self.length())
    }
}

impl AsAny for Vector {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.short_description(), self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(vector.length(), 3);
        assert_eq!(vector.data(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_vector_add() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = Vector::new(vec![4.0, 5.0, 6.0]);
        let result = a.add(&b);
        assert_eq!(result.data(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_scalar_multiply() {
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let result = vector.scalar_multiply(2.0);
        assert_eq!(result.data(), &[2.0, 4.0, 6.0]);
    }
}
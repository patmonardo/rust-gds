//! Scalar tensor - translated from Scalar.java
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This Scalar wraps a TensorData (composition) to share storage and methods.
//! This matches Java's inheritance: Scalar extends Tensor<Scalar>
//!
//! - TensorData provides: data, dimensions, aggregate_sum(), map(), etc.
//! - Scalar adds: value() convenience accessor
//! - Scalar delegates shared operations to inner TensorData

use super::tensor::Tensor;
use super::tensor_data::TensorData;
use crate::ml::core::dimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct Scalar {
    tensor: TensorData, // COMPOSITION: wraps shared storage/methods
}

impl Scalar {
    // ========================================================================
    // Constructors
    // ========================================================================

    pub fn new(value: f64) -> Self {
        let tensor = TensorData::new(vec![value], dimensions::scalar());
        Self { tensor }
    }

    // ========================================================================
    // Scalar-specific accessors
    // ========================================================================

    /// DELEGATION: Get scalar value (convenience wrapper).
    pub fn value(&self) -> f64 {
        self.tensor.data_at(0)
    }

    // ========================================================================
    // Delegation to TensorData (shared methods)
    // ========================================================================

    /// DELEGATION: Get raw data slice.
    pub fn data(&self) -> &[f64] {
        self.tensor.data()
    }
}

// ============================================================================
// Tensor Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Most methods delegate to the inner TensorData.
// Scalar-specific logic wraps the result in a new Scalar.

impl Tensor for Scalar {
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
        Box::new(Scalar::new(0.0))
    }

    // DELEGATION: Use TensorData.add(), wrap result
    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        let result_tensor = self.tensor.add(&other_scalar.tensor);
        Box::new(Scalar {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.add_inplace()
    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        self.tensor.add_inplace(&other_scalar.tensor);
    }

    // DELEGATION: Use TensorData.scalar_multiply(), wrap result
    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let result_tensor = self.tensor.scalar_multiply(scalar);
        Box::new(Scalar {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.scalar_multiply_mutate()
    fn scalar_multiply_mutate(&mut self, scalar: f64) {
        self.tensor.scalar_multiply_mutate(scalar);
    }

    // DELEGATION: Use TensorData.elementwise_product(), wrap result
    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        let result_tensor = self.tensor.elementwise_product(&other_scalar.tensor);
        Box::new(Scalar {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.elementwise_product_mutate()
    fn elementwise_product_mutate(&mut self, other: &dyn Tensor) {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        self.tensor.elementwise_product_mutate(&other_scalar.tensor);
    }

    // DELEGATION: Use TensorData.map(), wrap result
    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let result_tensor = self.tensor.map(f);
        Box::new(Scalar {
            tensor: result_tensor,
        })
    }

    // DELEGATION: Forward to TensorData.map_inplace()
    fn map_inplace(&mut self, f: fn(f64) -> f64) {
        self.tensor.map_inplace(f);
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Scalar::new(1.0))
    }

    // DELEGATION: Forward to TensorData.equals()
    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_scalar) = other.as_any().downcast_ref::<Scalar>() {
            self.tensor.equals(&other_scalar.tensor, tolerance)
        } else {
            false
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scalar: {}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_new_scalar() {
        let lhs = Scalar::new(1.5);
        let rhs = Scalar::new(2.5);

        let result = lhs.add(&rhs);
        assert_eq!(result.dimensions(), &[1, 1]);
        assert_eq!(result.data(), &[4.0]);
    }

    #[test]
    fn scalar_multiply_mutate_scales_in_place() {
        let mut scalar = Scalar::new(3.0);
        scalar.scalar_multiply_mutate(2.0);

        assert_eq!(scalar.value(), 6.0);
    }

    #[test]
    fn ones_like_produces_unit_scalar() {
        let scalar = Scalar::new(42.0);
        let one = scalar.ones_like();
        assert_eq!(one.dimensions(), &[1, 1]);
        assert_eq!(one.data(), &[1.0]);
    }
}

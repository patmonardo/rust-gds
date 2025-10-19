//! Scalar tensor - translated from Scalar.java
//!
//! This directly mirrors Java's `Scalar extends Tensor<Scalar>` pattern.
//! Contains data and dimensions directly, not wrapped in TensorData.

use super::tensor::{Tensor, AsAny};
use crate::ml::core::dimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct Scalar {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl Scalar {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create scalar from value.
    /// Java: `public Scalar(double value)`
    pub fn new(value: f64) -> Self {
        let data = vec![value];
        let dimensions = dimensions::scalar();
        Self { data, dimensions }
    }

    // ========================================================================
    // Scalar-specific accessors
    // ========================================================================

    /// Get scalar value.
    /// Java: `public double value()`
    pub fn value(&self) -> f64 {
        self.data[0]
    }

    /// Set scalar value.
    /// Java: `public void setValue(double value)`
    pub fn set_value(&mut self, value: f64) {
        self.data[0] = value;
    }

    /// Calculate size in bytes for scalar.
    /// Java: `public static long sizeInBytes()`
    pub fn size_in_bytes() -> usize {
        crate::ml::core::tensor::size_in_bytes(&dimensions::scalar())
    }
}

// ============================================================================
// Tensor Trait Implementation
// ============================================================================

impl Tensor for Scalar {
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
        Box::new(Scalar::new(0.0))
    }

    fn clone_box(&self) -> Box<dyn Tensor> {
        Box::new(self.clone())
    }

    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        Box::new(Scalar::new(self.value() + other_scalar.value()))
    }

    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        Box::new(Scalar::new(f(self.value())))
    }

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        Box::new(Scalar::new(self.value() * scalar))
    }

    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        Box::new(Scalar::new(self.value() * other_scalar.value()))
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Scalar::new(1.0))
    }

    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_scalar = other.as_any().downcast_ref::<Scalar>().unwrap();
        self.data[0] += other_scalar.value();
    }

    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_scalar) = other.as_any().downcast_ref::<Scalar>() {
            (self.value() - other_scalar.value()).abs() <= tolerance
        } else {
            false
        }
    }

    fn short_description(&self) -> String {
        "Scalar".to_string()
    }
}

impl AsAny for Scalar {
    fn as_any(&self) -> &dyn std::any::Any {
        self
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
    fn test_scalar_creation() {
        let scalar = Scalar::new(42.0);
        assert_eq!(scalar.value(), 42.0);
        assert_eq!(scalar.data(), &[42.0]);
    }

    #[test]
    fn test_scalar_add() {
        let a = Scalar::new(1.5);
        let b = Scalar::new(2.5);
        let result = a.add(&b);
        let result_scalar = result.as_any().downcast_ref::<Scalar>().expect("Expected Scalar");
        assert_eq!(result_scalar.value(), 4.0);
    }

    #[test]
    fn test_scalar_multiply() {
        let scalar = Scalar::new(3.0);
        let result = scalar.scalar_multiply(2.0);
        let result_scalar = result.as_any().downcast_ref::<Scalar>().expect("Expected Scalar");
        assert_eq!(result_scalar.value(), 6.0);
    }
}
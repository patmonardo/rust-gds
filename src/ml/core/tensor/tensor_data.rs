//! TensorData - shared storage and operations for all tensor types.
//!
//! This struct replaces Java's abstract Tensor<SELF> base class.
//! It contains the storage (data, dimensions) and all shared methods
//! that were inherited in Java's class hierarchy.
//!
//! Design Pattern: Composition + Delegation
//! - Matrix/Vector/Scalar WRAP a TensorData (composition)
//! - They delegate shared operations to their inner TensorData
//! - They implement only type-specific operations themselves
//!
//! This achieves the same goal as Java's inheritance but using Rust idioms.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Shared storage and operations for all tensor types.
///
/// This is the "base class" equivalent from Java's Tensor<SELF>.
/// Contains the protected fields (data, dimensions) and all concrete methods
/// that were inherited by Matrix, Vector, and Scalar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TensorData {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl TensorData {
    /// Create new TensorData with given data and dimensions.
    pub fn new(data: Vec<f64>, dimensions: Vec<usize>) -> Self {
        // Validate dimensions match data length
        let total_size: usize = dimensions.iter().product();
        assert_eq!(
            data.len(),
            total_size,
            "Data length {} does not match dimensions {:?} (total size {})",
            data.len(),
            dimensions,
            total_size
        );
        Self { data, dimensions }
    }

    /// Create TensorData filled with zeros.
    pub fn zeros(dimensions: Vec<usize>) -> Self {
        let total_size: usize = dimensions.iter().product();
        Self {
            data: vec![0.0; total_size],
            dimensions,
        }
    }

    /// Create TensorData filled with a constant value.
    pub fn filled(value: f64, dimensions: Vec<usize>) -> Self {
        let total_size: usize = dimensions.iter().product();
        Self {
            data: vec![value; total_size],
            dimensions,
        }
    }

    // ========================================================================
    // Accessors - match Java's protected fields
    // ========================================================================

    /// Get immutable reference to data array.
    /// Corresponds to Java's `protected double[] data`
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Get mutable reference to data array.
    /// Corresponds to Java's `protected double[] data`
    pub fn data_mut(&mut self) -> &mut [f64] {
        &mut self.data
    }

    /// Get reference to dimensions.
    /// Corresponds to Java's `protected int[] dimensions`
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    // ========================================================================
    // Indexed Access - from Java Tensor.java
    // ========================================================================

    /// Get value at flat index.
    /// Java: `public double dataAt(int idx)`
    pub fn data_at(&self, idx: usize) -> f64 {
        self.data[idx]
    }

    /// Set value at flat index.
    /// Java: `public void setDataAt(int idx, double newValue)`
    pub fn set_data_at(&mut self, idx: usize, value: f64) {
        self.data[idx] = value;
    }

    /// Add to value at flat index.
    /// Java: `public void addDataAt(int idx, double newValue)`
    pub fn add_data_at(&mut self, idx: usize, value: f64) {
        self.data[idx] += value;
    }

    // ========================================================================
    // Dimension Operations - from Java Tensor.java
    // ========================================================================

    /// Get a specific dimension by index.
    /// Java: `public int dimension(int dimensionIndex)`
    pub fn dimension(&self, dimension_index: usize) -> usize {
        self.dimensions[dimension_index]
    }

    /// Calculate total size (product of all dimensions).
    /// Java: `public int totalSize()`
    pub fn total_size(&self) -> usize {
        self.dimensions.iter().product()
    }

    // ========================================================================
    // Aggregation - from Java Tensor.java
    // ========================================================================

    /// Sum all elements in the tensor.
    /// Java: `public double aggregateSum()`
    pub fn aggregate_sum(&self) -> f64 {
        self.data.iter().sum()
    }

    // ========================================================================
    // Functional Operations - from Java Tensor.java
    // ========================================================================

    /// Apply function to each element, return new TensorData.
    /// Java: `public SELF map(DoubleUnaryOperator f)`
    pub fn map(&self, f: fn(f64) -> f64) -> Self {
        let new_data: Vec<f64> = self.data.iter().map(|&x| f(x)).collect();
        Self {
            data: new_data,
            dimensions: self.dimensions.clone(),
        }
    }

    /// Apply function to each element in-place.
    /// Java: `public Tensor<SELF> mapInPlace(DoubleUnaryOperator f)`
    pub fn map_inplace(&mut self, f: fn(f64) -> f64) {
        for x in &mut self.data {
            *x = f(*x);
        }
    }

    // ========================================================================
    // Element-wise Arithmetic - from Java Tensor.java
    // ========================================================================

    /// Add another tensor's data element-wise (in-place).
    /// Java: `public void addInPlace(Tensor<?> other)`
    pub fn add_inplace(&mut self, other: &TensorData) {
        assert_eq!(
            self.dimensions, other.dimensions,
            "Dimensions must match for add_inplace"
        );
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }

    /// Add another tensor's data element-wise (returns new).
    /// Helper for add() implementations.
    pub fn add(&self, other: &TensorData) -> Self {
        assert_eq!(
            self.dimensions, other.dimensions,
            "Dimensions must match for add"
        );
        let new_data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self {
            data: new_data,
            dimensions: self.dimensions.clone(),
        }
    }

    /// Multiply by scalar (immutable).
    /// Java: `public SELF scalarMultiply(double scalar)`
    pub fn scalar_multiply(&self, scalar: f64) -> Self {
        let new_data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Self {
            data: new_data,
            dimensions: self.dimensions.clone(),
        }
    }

    /// Multiply by scalar (in-place).
    /// Java: `public Tensor<SELF> scalarMultiplyMutate(double scalar)`
    pub fn scalar_multiply_mutate(&mut self, scalar: f64) {
        for x in &mut self.data {
            *x *= scalar;
        }
    }

    /// Element-wise product (immutable).
    /// Java: `public SELF elementwiseProduct(Tensor<?> other)`
    pub fn elementwise_product(&self, other: &TensorData) -> Self {
        assert_eq!(
            self.dimensions, other.dimensions,
            "Dimensions must match for elementwise_product"
        );
        let new_data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        Self {
            data: new_data,
            dimensions: self.dimensions.clone(),
        }
    }

    /// Element-wise product (in-place).
    /// Java: `public Tensor<SELF> elementwiseProductMutate(Tensor<?> other)`
    pub fn elementwise_product_mutate(&mut self, other: &TensorData) {
        assert_eq!(
            self.dimensions, other.dimensions,
            "Dimensions must match for elementwise_product_mutate"
        );
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a *= *b;
        }
    }

    // ========================================================================
    // Equality - from Java Tensor.java
    // ========================================================================

    /// Check equality with tolerance.
    /// Java: `public boolean equals(Tensor<?> other, double tolerance)`
    pub fn equals(&self, other: &TensorData, tolerance: f64) -> bool {
        if self.dimensions != other.dimensions {
            return false;
        }

        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(a, b)| (a - b).abs() <= tolerance)
    }
}

impl fmt::Display for TensorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TensorData{:?}: {:?}", self.dimensions, self.data)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_data_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let td = TensorData::new(data.clone(), vec![2, 2]);
        assert_eq!(td.data(), &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(td.dimensions(), &[2, 2]);
    }

    #[test]
    fn test_tensor_data_zeros() {
        let td = TensorData::zeros(vec![3, 2]);
        assert_eq!(td.data(), &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(td.total_size(), 6);
    }

    #[test]
    fn test_tensor_data_filled() {
        let td = TensorData::filled(5.0, vec![2, 3]);
        assert_eq!(td.data(), &[5.0, 5.0, 5.0, 5.0, 5.0, 5.0]);
    }

    #[test]
    fn test_indexed_access() {
        let mut td = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        assert_eq!(td.data_at(0), 1.0);
        assert_eq!(td.data_at(2), 3.0);

        td.set_data_at(1, 10.0);
        assert_eq!(td.data_at(1), 10.0);

        td.add_data_at(0, 5.0);
        assert_eq!(td.data_at(0), 6.0);
    }

    #[test]
    fn test_aggregate_sum() {
        let td = TensorData::new(vec![1.0, 2.0, 3.0, 4.0], vec![4]);
        assert_eq!(td.aggregate_sum(), 10.0);
    }

    #[test]
    fn test_map() {
        let td = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let mapped = td.map(|x| x * 2.0);
        assert_eq!(mapped.data(), &[2.0, 4.0, 6.0]);
        // Original unchanged
        assert_eq!(td.data(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_map_inplace() {
        let mut td = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        td.map_inplace(|x| x * 2.0);
        assert_eq!(td.data(), &[2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_add() {
        let td1 = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let td2 = TensorData::new(vec![4.0, 5.0, 6.0], vec![3]);
        let sum = td1.add(&td2);
        assert_eq!(sum.data(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_add_inplace() {
        let mut td1 = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let td2 = TensorData::new(vec![4.0, 5.0, 6.0], vec![3]);
        td1.add_inplace(&td2);
        assert_eq!(td1.data(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_scalar_multiply() {
        let td = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let scaled = td.scalar_multiply(2.0);
        assert_eq!(scaled.data(), &[2.0, 4.0, 6.0]);
        // Original unchanged
        assert_eq!(td.data(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_scalar_multiply_mutate() {
        let mut td = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        td.scalar_multiply_mutate(3.0);
        assert_eq!(td.data(), &[3.0, 6.0, 9.0]);
    }

    #[test]
    fn test_elementwise_product() {
        let td1 = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let td2 = TensorData::new(vec![2.0, 3.0, 4.0], vec![3]);
        let prod = td1.elementwise_product(&td2);
        assert_eq!(prod.data(), &[2.0, 6.0, 12.0]);
    }

    #[test]
    fn test_elementwise_product_mutate() {
        let mut td1 = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let td2 = TensorData::new(vec![2.0, 3.0, 4.0], vec![3]);
        td1.elementwise_product_mutate(&td2);
        assert_eq!(td1.data(), &[2.0, 6.0, 12.0]);
    }

    #[test]
    fn test_equals() {
        let td1 = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let td2 = TensorData::new(vec![1.0, 2.0, 3.0], vec![3]);
        let td3 = TensorData::new(vec![1.1, 2.0, 3.0], vec![3]);

        assert!(td1.equals(&td2, 1e-10));
        assert!(!td1.equals(&td3, 1e-10));
        assert!(td1.equals(&td3, 0.2)); // With larger tolerance
    }
}

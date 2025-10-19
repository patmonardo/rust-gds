//! Matrix tensor - translated from Matrix.java
//!
//! This directly mirrors Java's `Matrix extends Tensor<Matrix>` pattern.
//! Contains data and dimensions directly, not wrapped in TensorData.

use super::tensor::{Tensor, AsAny};
use super::vector::Vector;
use super::scalar::Scalar;
use crate::ml::core::dimensions;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Matrix {
    data: Vec<f64>,
    dimensions: Vec<usize>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    // ========================================================================
    // Constructors - match Java's constructor pattern
    // ========================================================================

    /// Create matrix from data array and dimensions.
    /// Java: `public Matrix(double[] data, int rows, int cols)`
    pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        let dimensions = dimensions::matrix(rows, cols);
        assert_eq!(data.len(), rows * cols, "Data length must match dimensions");
        Self {
            data,
            dimensions,
            rows,
            cols,
        }
    }

    /// Create matrix with given dimensions, filled with zeros.
    /// Java: `public Matrix(int rows, int cols)`
    pub fn with_dimensions(rows: usize, cols: usize) -> Self {
        let data = vec![0.0; rows * cols];
        Self::new(data, rows, cols)
    }

    /// Create matrix filled with a constant value.
    /// Java: `public static Matrix create(double v, int rows, int cols)`
    pub fn create(value: f64, rows: usize, cols: usize) -> Self {
        let data = vec![value; rows * cols];
        Self::new(data, rows, cols)
    }

    /// Create matrix filled with zeros.
    /// Java: `public static Matrix zeros(int rows, int cols)`
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::with_dimensions(rows, cols)
    }

    /// Calculate size in bytes for matrix.
    /// Java: `public static long sizeInBytes(int rows, int cols)`
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        crate::ml::core::tensor::size_in_bytes(&[rows, cols])
    }

    // ========================================================================
    // Matrix-specific accessors
    // ========================================================================

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get value at (row, col) position.
    /// Java: `public double dataAt(int row, int col)`
    pub fn data_at(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    /// Set value at (row, col) position.
    /// Java: `public void setDataAt(int row, int column, double newValue)`
    pub fn set_data_at(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    /// Set value at flat index position.
    /// Java: `public void setDataAtFlat(int index, double newValue)`
    pub fn set_data_at_flat(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }

    /// Get value at flat index position.
    /// Java: `public double dataAtFlat(int index)`
    pub fn data_at_flat(&self, index: usize) -> f64 {
        self.data[index]
    }

    /// Add to value at (row, col) position.
    /// Java: `public void addDataAt(int row, int column, double newValue)`
    pub fn add_data_at(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
    }

    /// Set a row from values array.
    /// Java: `public void setRow(int row, double[] values)`
    pub fn set_row(&mut self, row: usize, values: &[f64]) {
        if values.len() != self.cols {
            panic!(
                "Input vector dimension {} is unequal to column count {}",
                values.len(), self.cols
            );
        }
        let start = row * self.cols;
        self.data[start..start + self.cols].copy_from_slice(values);
    }

    /// Get a row as a slice.
    /// Java: `public double[] getRow(int rowIdx)`
    pub fn get_row(&self, row_idx: usize) -> &[f64] {
        let start = row_idx * self.cols;
        &self.data[start..start + self.cols]
    }

    /// Get a row as a slice (alias for get_row).
    /// Java: `public double[] getRow(int rowIdx)`
    pub fn row(&self, row_idx: usize) -> &[f64] {
        self.get_row(row_idx)
    }

    /// Update data at (row, col) using a function.
    /// Java: `public void updateDataAt(int row, int column, DoubleUnaryOperator updater)`
    pub fn update_data_at<F>(&mut self, row: usize, col: usize, updater: F)
    where
        F: FnOnce(f64) -> f64,
    {
        let idx = row * self.cols + col;
        self.data[idx] = updater(self.data[idx]);
    }

    // ========================================================================
    // Matrix-specific operations
    // ========================================================================

    /// Standard matrix multiplication: C = self × other
    /// Java: `public Matrix multiply(Matrix other)`
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(
            self.cols, other.rows,
            "Matrix dimensions must match! Got dimensions ({}, {}) + ({}, {})",
            self.rows, self.cols, other.rows, other.cols
        );
        
        let mut result = Matrix::with_dimensions(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data_at(i, k) * other.data_at(k, j);
                }
                result.set_data_at(i, j, sum);
            }
        }
        result
    }

    /// Matrix multiplication with second operand transposed: C = self × other^T
    /// Java: `public Matrix multiplyTransB(Matrix other)`
    pub fn multiply_trans_b(&self, other: &Matrix) -> Matrix {
        assert_eq!(
            self.cols, other.cols,
            "Cannot multiply matrix having dimensions ({}, {}) with transposed matrix of dimensions ({}, {})",
            self.rows, self.cols, other.rows, other.cols
        );
        
        let mut result = Matrix::with_dimensions(self.rows, other.rows);
        for i in 0..self.rows {
            for j in 0..other.rows {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data_at(i, k) * other.data_at(j, k);
                }
                result.set_data_at(i, j, sum);
            }
        }
        result
    }

    /// Matrix multiplication with first operand transposed: C = self^T × other
    /// Java: `public Matrix multiplyTransA(Matrix other)`
    pub fn multiply_trans_a(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::with_dimensions(self.cols, other.cols);
        for i in 0..self.cols {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.rows {
                    sum += self.data_at(k, i) * other.data_at(k, j);
                }
                result.set_data_at(i, j, sum);
            }
        }
        result
    }

    /// Add vector to each row of matrix (broadcast column-wise).
    /// Java: `public Matrix sumBroadcastColumnWise(Vector vector)`
    pub fn sum_broadcast_column_wise(&self, vector: &Vector) -> Matrix {
        let mut result = self.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                result.data[idx] += vector.data()[col];
            }
        }
        result
    }

    /// Sum each column to create a vector.
    /// Java: `public Vector sumPerColumn()`
    pub fn sum_per_column(&self) -> Vector {
        let mut column_sums = vec![0.0; self.cols];
        for col in 0..self.cols {
            for row in 0..self.rows {
                column_sums[col] += self.data_at(row, col);
            }
        }
        Vector::new(column_sums)
    }

    /// Copy a row from source matrix into this matrix.
    /// Java: `public void setRow(int rowIdx, Matrix input, int inputRowIdx)`
    pub fn set_row_from_matrix(&mut self, target_row: usize, source: &Matrix, source_row_idx: usize) {
        if source.cols != self.cols {
            panic!(
                "Input matrix must have the same number of columns. Expected {}, but got {}.",
                self.cols, source.cols
            );
        }
        let target_start = target_row * self.cols;
        let source_start = source_row_idx * source.cols;
        self.data[target_start..target_start + self.cols]
            .copy_from_slice(&source.data[source_start..source_start + source.cols]);
    }

    /// Check if this matrix is actually a vector.
    /// Java: `public boolean isVector()`
    pub fn is_vector(&self) -> bool {
        dimensions::is_vector(&self.dimensions)
    }
}

// ============================================================================
// Tensor Trait Implementation
// ============================================================================

impl Tensor for Matrix {
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

    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_matrix) = other.as_any().downcast_ref::<Matrix>() {
            if self.dimensions != other_matrix.dimensions {
                return false;
            }
            self.data.iter()
                .zip(other_matrix.data.iter())
                .all(|(a, b)| (a - b).abs() <= tolerance)
        } else {
            false
        }
    }

    fn short_description(&self) -> String {
        format!("Matrix({}, {})", self.rows, self.cols)
    }

    fn clone_box(&self) -> Box<dyn Tensor> {
        Box::new(self.clone())
    }

    fn create_with_same_dimensions(&self) -> Box<dyn Tensor> {
        Box::new(Matrix::with_dimensions(self.rows, self.cols))
    }

    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        assert_eq!(
            self.rows, other_matrix.rows,
            "Matrix dimensions must match! Got dimensions ({}, {}) + ({}, {})",
            self.rows, self.cols, other_matrix.rows, other_matrix.cols
        );
        assert_eq!(
            self.cols, other_matrix.cols,
            "Matrix dimensions must match! Got dimensions ({}, {}) + ({}, {})",
            self.rows, self.cols, other_matrix.rows, other_matrix.cols
        );
        
        let mut result = Matrix::with_dimensions(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + other_matrix.data[i];
        }
        Box::new(result)
    }

    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let new_data: Vec<f64> = self.data.iter().map(|&x| f(x)).collect();
        Box::new(Matrix {
            data: new_data,
            dimensions: self.dimensions.clone(),
            rows: self.rows,
            cols: self.cols,
        })
    }

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let new_data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Box::new(Matrix {
            data: new_data,
            dimensions: self.dimensions.clone(),
            rows: self.rows,
            cols: self.cols,
        })
    }

    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        let mut result = Matrix::with_dimensions(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] * other_matrix.data[i];
        }
        Box::new(result)
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Matrix::create(1.0, self.rows, self.cols))
    }

    fn add_inplace(&mut self, other: &dyn Tensor) {
        if let Some(other_matrix) = other.as_any().downcast_ref::<Matrix>() {
            // Both are matrices - element-wise addition
            for i in 0..self.data.len() {
                self.data[i] += other_matrix.data[i];
            }
        } else if let Some(other_scalar) = other.as_any().downcast_ref::<Scalar>() {
            // Other is a scalar - add scalar to each element
            let scalar_value = other_scalar.value();
            for i in 0..self.data.len() {
                self.data[i] += scalar_value;
            }
        } else {
            panic!("Cannot add tensor of type {} to Matrix", std::any::type_name_of_val(other));
        }
    }
}

impl AsAny for Matrix {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.short_description(), self.data)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.data[row * self.cols + col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 2);
        assert_eq!(matrix.data(), &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let b = Matrix::new(vec![5.0, 6.0, 7.0, 8.0], 2, 2);
        let result = a.multiply(&b);
        assert_eq!(result.data(), &[19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn test_sum_per_column() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2);
        let result = matrix.sum_per_column();
        assert_eq!(result.data(), &[9.0, 12.0]);
    }
}
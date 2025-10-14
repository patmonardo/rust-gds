//! Matrix tensor - translated from Matrix.java
//!
//! ## Design Pattern: Composition + Delegation
//!
//! This Matrix wraps a TensorData (composition) to share storage and methods.
//! This matches Java's inheritance: Matrix extends Tensor<Matrix>
//!
//! - TensorData provides: data, dimensions, aggregate_sum(), map(), etc.
//! - Matrix adds: rows, cols, multiply(), data_at(row, col), etc.
//! - Matrix delegates shared operations to inner TensorData

use super::tensor::Tensor;
use super::tensor_data::TensorData;
use super::vector::Vector;
use crate::ml::core::dimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    tensor: TensorData, // COMPOSITION: wraps shared storage/methods
    rows: usize,
    cols: usize,
}

impl Matrix {
    // ========================================================================
    // Constructors
    // ========================================================================

    pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        let tensor = TensorData::new(data, dimensions::matrix(rows, cols));
        Self { tensor, rows, cols }
    }

    pub fn with_dimensions(rows: usize, cols: usize) -> Self {
        let tensor = TensorData::zeros(dimensions::matrix(rows, cols));
        Self { tensor, rows, cols }
    }

    pub fn create(value: f64, rows: usize, cols: usize) -> Self {
        let tensor = TensorData::filled(value, dimensions::matrix(rows, cols));
        Self { tensor, rows, cols }
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
    /// Matrix-specific: uses 2D indexing.
    /// Java: `public double dataAt(int row, int col)`
    pub fn data_at(&self, row: usize, col: usize) -> f64 {
        self.tensor.data_at(row * self.cols + col)
    }

    /// Set value at (row, col) position.
    /// Matrix-specific: uses 2D indexing.
    /// Java: `public void setDataAt(int row, int column, double newValue)`
    pub fn set_data_at(&mut self, row: usize, col: usize, value: f64) {
        self.tensor.set_data_at(row * self.cols + col, value);
    }

    /// Add to value at (row, col) position.
    /// Matrix-specific: uses 2D indexing.
    pub fn add_data_at(&mut self, row: usize, col: usize, value: f64) {
        self.tensor.add_data_at(row * self.cols + col, value);
    }

    // ========================================================================
    // Delegation to TensorData (shared methods)
    // ========================================================================

    /// DELEGATION: Get raw data slice.
    /// Forwards to TensorData.data()
    pub fn data(&self) -> &[f64] {
        self.tensor.data()
    }

    /// DELEGATION: Get mutable data slice.
    /// Forwards to TensorData.data_mut()
    pub fn data_mut(&mut self) -> &mut [f64] {
        self.tensor.data_mut()
    }

    /// DELEGATION: Set value at flat index.
    /// Forwards to TensorData.set_data_at()
    /// Use `set_data_at(row, col, value)` for 2D access.
    pub fn set_data_at_flat(&mut self, index: usize, value: f64) {
        self.tensor.set_data_at(index, value);
    }

    /// DELEGATION: Get value at flat index.
    /// Forwards to TensorData.data_at()
    /// Use `data_at(row, col)` for 2D access.
    pub fn data_at_flat(&self, index: usize) -> f64 {
        self.tensor.data_at(index)
    }

    // ========================================================================
    // Matrix-specific operations (NOT delegated)
    // ========================================================================

    /// Standard matrix multiplication: C = self × other
    /// Dimensions: (m×n) × (n×p) -> (m×p)
    pub fn multiply(&self, other: &Matrix) -> Box<dyn Tensor> {
        assert_eq!(
            self.cols, other.rows,
            "Matrix multiplication dimension mismatch: ({}, {}) × ({}, {})",
            self.rows, self.cols, other.rows, other.cols
        );

        let m = self.rows;
        let n = self.cols;
        let p = other.cols;

        let mut result = vec![0.0; m * p];

        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += self.data_at(i, k) * other.data_at(k, j);
                }
                result[i * p + j] = sum;
            }
        }

        Box::new(Matrix::new(result, m, p))
    }

    /// Matrix multiplication with first operand transposed: C = self^T × other
    /// Dimensions: (n×m)^T × (n×p) -> (m×p)
    pub fn multiply_trans_a(&self, other: &Matrix) -> Box<dyn Tensor> {
        assert_eq!(
            self.rows, other.rows,
            "Matrix multiply_trans_a dimension mismatch: ({}, {})^T × ({}, {})",
            self.rows, self.cols, other.rows, other.cols
        );

        let m = self.cols;
        let n = self.rows;
        let p = other.cols;

        let mut result = vec![0.0; m * p];

        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    // self transposed: self[k][i] instead of self[i][k]
                    sum += self.data_at(k, i) * other.data_at(k, j);
                }
                result[i * p + j] = sum;
            }
        }

        Box::new(Matrix::new(result, m, p))
    }

    /// Matrix multiplication with second operand transposed: C = self × other^T
    /// Dimensions: (m×n) × (p×n)^T -> (m×p)
    pub fn multiply_trans_b(&self, other: &Matrix) -> Box<dyn Tensor> {
        assert_eq!(
            self.cols, other.cols,
            "Matrix multiply_trans_b dimension mismatch: ({}, {}) × ({}, {})^T",
            self.rows, self.cols, other.rows, other.cols
        );

        let m = self.rows;
        let n = self.cols;
        let p = other.rows;

        let mut result = vec![0.0; m * p];

        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    // other transposed: other[j][k] instead of other[k][j]
                    sum += self.data_at(i, k) * other.data_at(j, k);
                }
                result[i * p + j] = sum;
            }
        }

        Box::new(Matrix::new(result, m, p))
    }

    /// Sum each column to create a vector.
    /// Matrix (m×n) -> Vector (n) where each element is the sum of one column.
    pub fn sum_per_column(&self) -> Box<dyn Tensor> {
        let mut column_sums = vec![0.0; self.cols];

        for col in 0..self.cols {
            let mut sum = 0.0;
            for row in 0..self.rows {
                sum += self.data_at(row, col);
            }
            column_sums[col] = sum;
        }

        Box::new(Vector::new(column_sums))
    }

    /// Add vector to each row of matrix (broadcast column-wise).
    /// Matrix (m×n) + Vector (n) -> Matrix (m×n)
    /// Each element result[i][j] = self[i][j] + vector[j]
    pub fn sum_broadcast_column_wise(&self, vector: &Vector) -> Box<dyn Tensor> {
        assert_eq!(
            self.cols,
            vector.length(),
            "Broadcast dimension mismatch: matrix has {} cols, vector has {} elements",
            self.cols,
            vector.length()
        );

        let mut result = self.tensor.data().to_vec();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                result[idx] += vector.data()[col];
            }
        }

        Box::new(Matrix::new(result, self.rows, self.cols))
    }

    /// Copy a row from source matrix into this matrix.
    /// Sets self[target_row] = source[source_row_idx]
    pub fn set_row(&mut self, target_row: usize, source: &Matrix, source_row_idx: usize) {
        assert!(target_row < self.rows, "Target row out of bounds");
        assert!(source_row_idx < source.rows, "Source row out of bounds");
        assert_eq!(
            self.cols, source.cols,
            "Column count mismatch: self has {}, source has {}",
            self.cols, source.cols
        );

        for col in 0..self.cols {
            let target_idx = target_row * self.cols + col;
            let source_idx = source_row_idx * source.cols + col;
            self.tensor
                .set_data_at(target_idx, source.tensor.data_at(source_idx));
        }
    }
}

// ============================================================================
// Tensor Trait Implementation - DELEGATION Pattern
// ============================================================================
//
// Most methods delegate to the inner TensorData.
// Matrix-specific logic wraps the result in a new Matrix with correct dimensions.

impl Tensor for Matrix {
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
        Box::new(Matrix::with_dimensions(self.rows, self.cols))
    }

    // DELEGATION: Use TensorData.add(), wrap result
    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        assert_eq!(self.rows, other_matrix.rows);
        assert_eq!(self.cols, other_matrix.cols);

        let result_tensor = self.tensor.add(&other_matrix.tensor);
        Box::new(Matrix {
            tensor: result_tensor,
            rows: self.rows,
            cols: self.cols,
        })
    }

    // DELEGATION: Forward to TensorData.add_inplace()
    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        self.tensor.add_inplace(&other_matrix.tensor);
    }

    // DELEGATION: Use TensorData.scalar_multiply(), wrap result
    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let result_tensor = self.tensor.scalar_multiply(scalar);
        Box::new(Matrix {
            tensor: result_tensor,
            rows: self.rows,
            cols: self.cols,
        })
    }

    // DELEGATION: Forward to TensorData.scalar_multiply_mutate()
    fn scalar_multiply_mutate(&mut self, scalar: f64) {
        self.tensor.scalar_multiply_mutate(scalar);
    }

    // DELEGATION: Use TensorData.elementwise_product(), wrap result
    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        let result_tensor = self.tensor.elementwise_product(&other_matrix.tensor);
        Box::new(Matrix {
            tensor: result_tensor,
            rows: self.rows,
            cols: self.cols,
        })
    }

    // DELEGATION: Forward to TensorData.elementwise_product_mutate()
    fn elementwise_product_mutate(&mut self, other: &dyn Tensor) {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        self.tensor.elementwise_product_mutate(&other_matrix.tensor);
    }

    // DELEGATION: Use TensorData.map(), wrap result
    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let result_tensor = self.tensor.map(f);
        Box::new(Matrix {
            tensor: result_tensor,
            rows: self.rows,
            cols: self.cols,
        })
    }

    // DELEGATION: Forward to TensorData.map_inplace()
    fn map_inplace(&mut self, f: fn(f64) -> f64) {
        self.tensor.map_inplace(f);
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Matrix::create(1.0, self.rows, self.cols))
    }

    // DELEGATION: Forward to TensorData.equals()
    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_matrix) = other.as_any().downcast_ref::<Matrix>() {
            if self.rows != other_matrix.rows || self.cols != other_matrix.cols {
                return false;
            }
            self.tensor.equals(&other_matrix.tensor, tolerance)
        } else {
            false
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix({}, {})", self.rows, self.cols)
    }
}

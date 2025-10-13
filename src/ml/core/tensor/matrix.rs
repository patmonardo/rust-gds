//! Matrix tensor - translated from Matrix.java

use super::tensor::Tensor;
use crate::ml::core::dimensions;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
    dimensions: Vec<usize>,
}

impl Matrix {
    pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self {
            data,
            rows,
            cols,
            dimensions: dimensions::matrix(rows, cols),
        }
    }

    pub fn with_dimensions(rows: usize, cols: usize) -> Self {
        Self::new(vec![0.0; rows * cols], rows, cols)
    }

    pub fn create(value: f64, rows: usize, cols: usize) -> Self {
        Self::new(vec![value; rows * cols], rows, cols)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data_at(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set_data_at_rc(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    /// Get raw data slice (for operations module).
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Set value at flat index (for operations module).
    pub fn set_data_at(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }
}

impl Tensor for Matrix {
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
        Box::new(Matrix::with_dimensions(self.rows, self.cols))
    }

    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        assert_eq!(self.rows, other_matrix.rows);
        assert_eq!(self.cols, other_matrix.cols);

        let result_data: Vec<f64> = self
            .data
            .iter()
            .zip(other_matrix.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Box::new(Matrix::new(result_data, self.rows, self.cols))
    }

    fn add_inplace(&mut self, other: &dyn Tensor) {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        for (a, b) in self.data.iter_mut().zip(other_matrix.data.iter()) {
            *a += b;
        }
    }

    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor> {
        let result_data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Box::new(Matrix::new(result_data, self.rows, self.cols))
    }

    fn scalar_multiply_mutate(&mut self, scalar: f64) {
        for x in &mut self.data {
            *x *= scalar;
        }
    }

    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor> {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        let result_data: Vec<f64> = self
            .data
            .iter()
            .zip(other_matrix.data.iter())
            .map(|(a, b)| a * b)
            .collect();
        Box::new(Matrix::new(result_data, self.rows, self.cols))
    }

    fn elementwise_product_mutate(&mut self, other: &dyn Tensor) {
        let other_matrix = other.as_any().downcast_ref::<Matrix>().unwrap();
        for (a, b) in self.data.iter_mut().zip(other_matrix.data.iter()) {
            *a *= b;
        }
    }

    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor> {
        let result_data: Vec<f64> = self.data.iter().map(|&x| f(x)).collect();
        Box::new(Matrix::new(result_data, self.rows, self.cols))
    }

    fn map_inplace(&mut self, f: fn(f64) -> f64) {
        for x in &mut self.data {
            *x = f(*x);
        }
    }

    fn ones_like(&self) -> Box<dyn Tensor> {
        Box::new(Matrix::create(1.0, self.rows, self.cols))
    }

    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool {
        if let Some(other_matrix) = other.as_any().downcast_ref::<Matrix>() {
            if self.rows != other_matrix.rows || self.cols != other_matrix.cols {
                return false;
            }
            self.data
                .iter()
                .zip(other_matrix.data.iter())
                .all(|(a, b)| (a - b).abs() <= tolerance)
        } else {
            false
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix({}, {})", self.rows, self.cols)
    }
}

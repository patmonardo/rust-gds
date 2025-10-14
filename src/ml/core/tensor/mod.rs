//! Tensor module for ML in GDS.
//!
//! Translated from Java GDS ml-core tensor package.

pub mod matrix;
pub mod operations;
pub mod scalar;
pub mod tensor;
pub mod tensor_data;
pub mod vector;

pub use matrix::Matrix;
pub use scalar::Scalar;
pub use tensor::{size_in_bytes, Tensor};
pub use tensor_data::TensorData;
pub use vector::Vector;

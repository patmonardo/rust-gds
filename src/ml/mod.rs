//! Machine Learning module for GDS.
//!
//! Contains both core ML primitives (tensors, variables, functions)
//! and algorithm implementations (decision trees, etc.).

pub mod algo;
pub mod core;
pub mod training_method;

pub use algo::*;
pub use core::*;
pub use training_method::*;

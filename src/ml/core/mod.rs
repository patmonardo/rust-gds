//! ML Core module for GDS.
//!
//! This module contains core machine learning interfaces and utilities.
//!
//! Translated from Java GDS ml-core package.

pub mod abstract_variable;
pub mod computation_context;
pub mod dimensions;
pub mod features;
pub mod tensor;
pub mod variable;

pub use abstract_variable::{AbstractVariable, NotAFunctionException};
pub use computation_context::ComputationContext;
pub use dimensions::*;
pub use tensor::Tensor;
pub use variable::Variable;

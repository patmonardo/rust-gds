//! Functions module for ML in GDS.
//!
//! Translated from Java GDS ml-core functions package.
//! This is a literal 1:1 translation following repository translation policy.

pub mod abstract_variable;
pub mod constant_scale;
pub mod single_parent_variable;

pub use abstract_variable::AbstractVariable;
pub use constant_scale::ConstantScale;
pub use single_parent_variable::{SingleParentGradient, SingleParentVariable};

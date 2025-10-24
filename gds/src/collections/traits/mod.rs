//! Collections Traits: Core Collections Interface
//!
//! This module defines the core traits that all Collections implementations
//! must implement, providing a unified interface across all backends.

pub mod collections;
pub mod compute_kernels;

pub use collections::*;
pub use compute_kernels::*;

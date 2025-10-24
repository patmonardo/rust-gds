//! Collections Composition: Composition Implementations
//!
//! This module provides composition implementations for Collections,
//! including hybrid, layered, and adaptive collections.

pub mod hybrid;
pub mod layered;
pub mod adaptive;

pub use hybrid::*;
pub use layered::*;
pub use adaptive::*;
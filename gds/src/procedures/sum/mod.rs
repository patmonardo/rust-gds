//! Sum Aggregation Algorithm
//!
//! This module implements the Sum aggregation algorithm.
//!
//! ## Architecture
//!
//! - `spec.rs` - SumAlgorithmSpec (implements AlgorithmSpec trait)
//! - `storage.rs` - Storage runtime (PropertyValues access - Gross pole)
//! - `computation.rs` - Computation runtime (accumulation - Subtle pole)
//!
//! ## Functor Machinery
//!
//! The Sum algorithm demonstrates the Storage ↔ Computation mapping:
//!
//! ```text
//! PropertyValues (Gross/Storage) ← Functor → GdsValue (Subtle/Computation)
//!                                  ↓
//!              SumStorageRuntime ↔ SumComputationRuntime
//! ```

pub mod computation;
pub mod spec;
pub mod storage;

// Re-export public types
pub use computation::SumComputationRuntime;
pub use spec::{SumAlgorithmSpec, SumConfig};
pub use storage::SumStorageRuntime;

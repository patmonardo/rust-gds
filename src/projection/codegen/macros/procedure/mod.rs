//! Procedure Macro Infrastructure
//!
//! Macros for generating algorithm infrastructure:
//! - `algorithm_config!` - Generate config struct + builder + validation
//! - `define_algorithm!` - Generate AlgorithmSpec impl + execution modes
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rust_gds::projection::codegen::macros::procedure::*;
//!
//! // 1. Define configuration
//! algorithm_config! {
//!     pub struct PageRankConfig {
//!         pub damping_factor: f64,
//!         pub max_iterations: usize,
//!     }
//! }
//!
//! // 2. Define algorithm (future)
//! define_algorithm! {
//!     name: PageRank,
//!     config: PageRankConfig,
//!     // ... more fields
//! }
//! ```

pub mod algorithm;
pub mod config;

// Re-exports
pub use algorithm::*;
// algorithm_config! macro // define_algorithm! macro

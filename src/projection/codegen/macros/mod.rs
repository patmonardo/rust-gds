//! Code Generation Macros
//!
//! This module contains the declarative macros that eliminate boilerplate
//! in the rust-gds codebase:
//!
//! - `eval_macro` - value_type_table! macro (master projector for property types)
//! - `config` - Lightweight config builder macro
//! - `procedure/` - Algorithm infrastructure macros (algorithm_config!, define_algorithm!)
//!
//! ## Organization
//!
//! Macros are organized by what they generate:
//! - **Property types**: `eval_macro.rs` - projects types into storage + runtime
//! - **Configurations**: `config.rs` - generates config structs + builders
//! - **Algorithms**: `procedure/` - generates AlgorithmSpec impls + execution modes
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rust_gds::projection::codegen::macros::*;
//!
//! // Use value_type_table! to define property types
//! value_type_table! {
//!     Long { id: 1, value_type: ValueType::Long, ... },
//!     Double { id: 2, value_type: ValueType::Double, ... },
//! }
//!
//! // Use algorithm_config! to define algorithm configurations
//! algorithm_config! {
//!     pub struct PageRankConfig {
//!         pub damping_factor: f64,
//!         pub max_iterations: usize,
//!     }
//! }
//! ```

// Declare macro modules (#[macro_use] makes macros available)
#[macro_use]
pub mod eval_macro;

pub mod config;
pub mod procedure;

// Re-export macros (they're already at crate root due to #[macro_export])
// This module serves as documentation and organization

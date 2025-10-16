//! Procedure Code Generation - Algorithm Infrastructure Macros
//!
//! This module provides declarative macros for generating algorithm infrastructure:
//!
//! - `algorithm_config!` - Generate config struct + builder + validation
//! - `define_algorithm!` - Generate AlgorithmSpec impl + execution modes + catalog
//!
//! ## Design Philosophy
//!
//! In GDS, adding an algorithm requires ~450 lines of repetitive boilerplate:
//! - Configuration struct (~80 lines)
//! - Builder with validation (~80 lines)
//! - AlgorithmSpec implementation (~150 lines)
//! - Execution mode wrappers (~200 lines)
//!
//! **Our approach**: Declare WHAT the algorithm is, generate HOW it integrates.
//!
//! ## Example
//!
//! ```rust,ignore
//! use rust_gds::algorithm_config;
//!
//! // Configuration: 15 lines → generates 80+ lines
//! algorithm_config! {
//!     pub struct PageRankConfig {
//!         #[default(0.85)]
//!         #[range(0.0..1.0)]
//!         pub damping_factor: f64,
//!
//!         #[default(1e-7)]
//!         #[min(0.0)]
//!         pub tolerance: f64,
//!     }
//! }
//! ```
//!
//! **Result**: 35 lines → 430+ lines generated = **92% reduction**
//!
//! ## Location Rationale
//!
//! This module lives in `projection/codegen/procedure/` because:
//! - Part of the GDSL Runtime codegen infrastructure
//! - Parallel to `projection/codegen/ml/` (ML pipeline codegen)
//! - Generates code that integrates with `projection/eval/procedure/` (executor)
//!
//! ## See Also
//!
//! - `doc/ALGORITHM_MACRO_DESIGN.md` - Detailed design documentation
//! - `doc/PROCEDURE_SUBSYSTEM_GUIDE.md` - How to use the macros
//! - `src/projection/eval/procedure/` - Algorithm executor (uses generated code)

pub mod algorithm_macro;
pub mod config_macro;

// Re-export macros at crate root (macros exported with #[macro_export] appear at crate root)
// Users can access via: use rust_gds::algorithm_config;

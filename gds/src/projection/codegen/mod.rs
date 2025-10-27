
//! ## Organization 
//!
//! - `macros/` - Tools that PROJECT (eval_macro, config, procedure macros)
//! - `descriptors/` - IDENTITY/Science (property, computation, storage, pipeline)
//! - `runtimes/` - DIFFERENCE/Manifestation (Computer, StorageRuntime, etc.)
//! - `algorithm/` - Projection MAPPINGS (TypeProjector, Functors) - the genetic constituents
//! - `consequence/` - LOGICAL ENTAILMENT (rules determining runtime from membership)
//! - `values/` - Primitive Values Macro System (GdsValue codegen, Arrow compat)
//! - `registry/` - OMNISCIENCE (analyze descriptors → extract schema)
//! - `catalog/` - OMNIPOTENCE (manifest runtimes from schema)
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Get descriptors
//! use gds::projection::codegen::descriptors::*;
//!
//! // Get runtime contracts
//! use gds::projection::codegen::runtimes::*;
//!
//! // Get omniscience (analyze descriptors)
//! use gds::projection::codegen::registry::*;
//!
//! // Get omnipotence (manifest runtimes)
//! use gds::projection::codegen::catalog::*;
//!
//! // Apply the concept
//! let schema = registry_analyzer.analyze(&descriptor)?;
//! let runtime = runtime_catalog.create(&schema)?;
//! ```

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

// Algorithm module - provides define_algorithm_spec and related macros
#[macro_use]
pub mod algorithm;
#[macro_use]
pub mod config;
#[macro_use]
pub mod eval;
#[macro_use]
pub mod property;
#[macro_use]
pub mod collections;
#[macro_use]
pub mod values;

// ============================================================================
// RE-EXPORTS
// ============================================================================

// Algorithm macros (re-exported at crate root via #[macro_export])
pub use algorithm::*;

// Config macros (re-exported at crate root via #[macro_export])
pub use config::*;

// Eval macros (re-exported at crate root via #[macro_export])

// Property macros (re-exported at crate root via #[macro_export])
pub use property::*;

// Collections macros (re-exported at crate root via #[macro_export])
// Note: Collections module is available but not yet actively used
// pub use collections::*;

// Values macros (re-exported at crate root via #[macro_export])
// Note: ValueType table is now the Master Controller for all property value generation

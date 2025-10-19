//! PROJECTION CODEGEN: The Ground Concept (Five-Fold Synthesis)
//!
//! This module is the ABSOLUTE GROUND from which all recursive descents flow.
//! It is the FIRST APPEARANCE of @reality in rust-gds.
//! It demonstrates the Genetic Method: how one concept unfolds from pure thought
//! into determinate being through its own internal necessity.
//!
//! ╔════════════════════════════════════════════════════════════════════════╗
//! ║ PROJECTION: THE FIVE-FOLD SYNTHESIS (GROUND CONCEPT)                  ║
//! ╠════════════════════════════════════════════════════════════════════════╣
//! ║                                                                        ║
//! ║ This is the root principle. All modules in src/projection/ are       ║
//! ║ RECURSIVE DESCENTS of this concept, applied to specific domains.     ║
//! ║                                                                        ║
//! ║ 1. TRANSFORM (Absolute Ground / Brahma Principle)                     ║
//! ║    The undifferentiated unity from which all determination flows       ║
//! ║    Descriptor ≡ Runtime through Transform                             ║
//! ║                                                                        ║
//! ║ 2. DESCRIPTOR (Identity Pole / Sat / Static Being)                    ║
//! ║    What the concept IS in-itself (timeless, unchanging)               ║
//! ║    - ComputationDescriptor, PropertyDescriptor, StorageDescriptor      ║
//! ║                                                                        ║
//! ║ 3. MEMBERSHIP (First Division / Chit / Inherence)                     ║
//! ║    What is inherent in each descriptor's being?                       ║
//! ║    Constraints linking each extreme to all others                     ║
//! ║                                                                        ║
//! ║ 4. RUNTIME (Difference Pole / Ananda / Dynamic Manifestation)         ║
//! ║    How the descriptor manifests in time, in execution                 ║
//! ║    - Computer, PropertyValues, StorageRuntime, ProcedureFacade        ║
//! ║                                                                        ║
//! ║ 5. CONSEQUENCE (Second Division / Sat-Chit-Ananda / Entailment)       ║
//! ║    What MUST follow from Descriptor + Membership?                     ║
//! ║    Logical rules that determine runtime requirements                  ║
//! ║                                                                        ║
//! ╠════════════════════════════════════════════════════════════════════════╣
//! ║ RECURSIVE DESCENT: registry and catalog                              ║
//! ╠════════════════════════════════════════════════════════════════════════╣
//! ║                                                                        ║
//! ║ registry = Projection PROJECTED INTO Computation Domain               ║
//! ║   (first recursive descent, see ../registry/mod.rs)                   ║
//! ║   Direction: Descriptor → Analyze → Schema                           ║
//! ║   "What can we KNOW about computation?"                               ║
//! ║   Inherits Five-Fold and specializes it to computation.               ║
//! ║                                                                        ║
//! ║ catalog = Projection PROJECTED INTO Storage Domain                    ║
//! ║   (second recursive descent, see ../catalog/mod.rs)                   ║
//! ║   Direction: Schema → Create Consequences → Runtime                   ║
//! ║   "What shall we CREATE in storage being?"                            ║
//! ║   Inherits Five-Fold and specializes it to storage.                   ║
//! ║                                                                        ║
//! ║ UNIFICATION: registry ∘ catalog = Complete Projection Manifest        ║
//! ║              Knowledge + Power = Omniscience + Omnipotence             ║
//! ║              registry ∘ catalog = Pipeline                            ║
//! ║                                                                        ║
//! ╚════════════════════════════════════════════════════════════════════════╝
//!
//! ## Organization (The Five-Fold in Code)
//!
//! - `macros/` - Tools that PROJECT (eval_macro, config, procedure macros)
//! - `descriptors/` - IDENTITY/Science (property, computation, storage, pipeline)
//! - `runtimes/` - DIFFERENCE/Manifestation (Computer, StorageRuntime, etc.)
//! - `algorithm/` - Projection MAPPINGS (TypeProjector, Functors) - the genetic constituents
//! - `consequence/` - LOGICAL ENTAILMENT (rules determining runtime from membership)
//! - `registry/` - OMNISCIENCE (analyze descriptors → extract schema)
//! - `catalog/` - OMNIPOTENCE (manifest runtimes from schema)
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Get descriptors
//! use rust_gds::projection::codegen::descriptors::*;
//!
//! // Get runtime contracts
//! use rust_gds::projection::codegen::runtimes::*;
//!
//! // Get omniscience (analyze descriptors)
//! use rust_gds::projection::codegen::registry::*;
//!
//! // Get omnipotence (manifest runtimes)
//! use rust_gds::projection::codegen::catalog::*;
//!
//! // Apply the concept
//! let schema = registry_analyzer.analyze(&descriptor)?;
//! let runtime = runtime_catalog.create(&schema)?;
//! ```

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

#[macro_use]
pub mod macros;
pub mod algorithm;
pub mod catalog;
pub mod consequence;
pub mod descriptors;
pub mod inherence;
pub mod membership;
pub mod registry;
pub mod runtimes;

// ============================================================================
// RE-EXPORTS
// ============================================================================

// Macros (re-exported at crate root via #[macro_export])
pub use macros::*;

// Descriptors
pub use descriptors::{
    ComputationDescriptor, ComputationPattern, ComputationSpecies, FieldDescriptor,
    PropertyDescriptor, PropertyId, StorageHint, StructDescriptor, StructId,
};

// ML Pipeline is THE pipeline (re-export for convenience)
pub use descriptors::PipelineDescriptor;

// Runtimes
pub use runtimes::{
    instantiate_computer_from_descriptor, register_computer_factory, AccessMode, ComputeContext,
    ComputeError, ComputeStep, Computer, ComputerFactory, Messages, StorageAccessor,
    StorageContext, StorageError, StorageRuntime, StorageRuntimeFactory, StorageValue,
};

// Algorithm (genetic constituents)
pub use algorithm::{
    AdaptiveProjector, ArrowProjector, HugeArrayProjector, PregelProjector, ProjectionError,
    TypeProjector, TypeValidator, ValidationError,
};

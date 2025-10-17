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
//! ║ RECURSIVE DESCENT: eval and factory                                  ║
//! ╠════════════════════════════════════════════════════════════════════════╣
//! ║                                                                        ║
//! ║ eval = Projection PROJECTED INTO Computation Domain                   ║
//! ║   (first recursive descent, see ../eval/mod.rs)                       ║
//! ║   Direction: Descriptor → Analyze → Schema                           ║
//! ║   "What can we KNOW about computation?"                               ║
//! ║   Inherits Five-Fold and specializes it to computation.               ║
//! ║                                                                        ║
//! ║ factory = Projection PROJECTED INTO Storage Domain                    ║
//! ║   (second recursive descent, see ../factory/mod.rs)                   ║
//! ║   Direction: Schema → Create Consequences → Runtime                   ║
//! ║   "What shall we CREATE in storage being?"                            ║
//! ║   Inherits Five-Fold and specializes it to storage.                   ║
//! ║                                                                        ║
//! ║ UNIFICATION: eval ∘ factory = Complete Projection Manifest            ║
//! ║              Knowledge + Power = Omniscience + Omnipotence             ║
//! ║              eval ∘ factory = Pipeline                                ║
//! ║                                                                        ║
//! ╚════════════════════════════════════════════════════════════════════════╝
//!
//! ## Organization (The Five-Fold in Code)
//!
//! - `macros/` - Tools that PROJECT (eval_macro, config, procedure macros)
//! - `descriptors/` - IDENTITY/Science (property, computation, storage, pipeline)
//! - `runtimes/` - DIFFERENCE/Manifestation (Computer, StorageRuntime, etc.)
//! - `transforms/` - Projection MAPPINGS (TypeProjector, Functors)
//! - `consequence/` - LOGICAL ENTAILMENT (rules determining runtime from membership)
//! - `eval/` - OMNISCIENCE (analyze descriptors → extract schema)
//! - `factory/` - OMNIPOTENCE (manifest runtimes from schema)
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
//! use rust_gds::projection::codegen::eval::*;
//!
//! // Get omnipotence (manifest runtimes)
//! use rust_gds::projection::codegen::factory::*;
//!
//! // Apply the concept
//! let schema = eval_analyzer.analyze(&descriptor)?;
//! let runtime = runtime_factory.create(&schema)?;
//! ```

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

#[macro_use]
pub mod macros;
pub mod consequence;
pub mod descriptors;
pub mod eval;
pub mod factory;
pub mod runtimes;
pub mod transforms;

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

// Transforms
pub use transforms::{
    AdaptiveProjector, ArrowProjector, HugeArrayProjector, PregelProjector, ProjectionError,
    TypeProjector, TypeValidator, ValidationError,
};

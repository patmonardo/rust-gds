//! Runtime Execution Contracts
//!
//! This module contains trait definitions for HOW things EXECUTE
//! (Difference/Manifestation pole of the Five-Fold Brahmachakra).
//!
//! ## Organization
//!
//! - `computation.rs` - Computer, ComputeStep, ComputeContext (how computation executes)
//! - `storage.rs` - StorageRuntime, StorageAccessor, StorageContext (how storage executes)
//!
//! ## The Dialectic
//!
//! While `descriptors/` defines WHAT things ARE (Identity/Science),
//! `runtimes/` defines HOW things EXECUTE (Difference/Manifestation).
//!
//! Together they form the complete Five-Fold system:
//! 1. PropertyDescriptor (Unity) - WHAT the property IS
//! 2. ComputationDescriptor (Identity) - WHAT the computation IS
//! 3. **ComputationRuntime (Difference)** - HOW it EXECUTES ← THIS MODULE
//! 4. StorageDescriptor (Identity) - WHAT the storage IS
//! 5. **StorageRuntime (Difference)** - HOW it EXECUTES ← THIS MODULE
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::runtimes::*;
//!
//! // Implement Computer trait for BSP execution
//! impl Computer for BspComputer {
//!     fn init(&mut self, ctx: &mut ComputeContext) -> Result<(), ComputeError> {
//!         // Initialization logic
//!     }
//!     // ...
//! }
//! ```

pub mod computation;
pub mod storage;

// Re-exports
pub use computation::{
    instantiate_computer_from_descriptor, register_computer_factory, ComputeContext, ComputeError,
    ComputeStep, Computer, ComputerFactory, Messages,
};
pub use storage::{
    AccessMode, StorageAccessor, StorageContext, StorageError, StorageRuntime,
    StorageRuntimeFactory, StorageValue,
};

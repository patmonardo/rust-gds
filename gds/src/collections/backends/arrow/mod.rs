//! Arrow Collections Backend
//!
//! This module hosts Arrow-backed collection implementations and their
//! supporting utilities. Each submodule focuses on a separate concern so the
//! zero-copy pathway stays understandable and composable.

pub mod array;
pub mod bitmap;
pub mod builder;
pub mod chunk;
pub mod descriptor;
pub mod error;
pub mod factory;
pub mod interop;
pub mod kernel;

pub use array::{
    ArrowArrayBehavior, ArrowDoubleArray, ArrowFloatArray, ArrowIntArray, ArrowLongArray,
    ArrowPrimitiveArray,
};
pub use bitmap::ArrowBitmap;
pub use builder::{ArrowChunkBuilder, ArrowPrimitiveBuilder};
pub use chunk::ArrowChunk;
pub use descriptor::ArrowBackendDescriptor;
pub use error::{ArrowBackendError, ArrowKernelError, ArrowResult};
pub use factory::ArrowCollectionsFactory;
pub use interop::{ArrowInterOp, ArrowTableView};
pub use kernel::KernelExtensions;

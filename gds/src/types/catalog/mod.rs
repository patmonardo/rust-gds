//! Graph Catalog - typed registry of named graph stores
//!
//! This module provides a minimal, types-only catalog API and a default
//! in-memory implementation for managing named graph projections.
//!
//! Design choices:
//! - The API is trait-based and lives entirely under `types/`.
//! - The default implementation is also provided here for convenience.
//! - The catalog stores `Arc<DefaultGraphStore>` to align with current
//!   executor and specs which operate on `DefaultGraphStore` directly.

pub mod service;
pub mod in_memory;

pub use service::*;
pub use in_memory::*;



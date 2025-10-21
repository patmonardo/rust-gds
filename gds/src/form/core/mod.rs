//! Form Core - Form Infrastructure
//!
//! This module provides the **Form Infrastructure** that will guide us toward
//! discovering what the **Form Evaluator** will be.
//!
//! ## Architecture
//!
//! The Form Infrastructure embodies the **triadic structure**:
//! - **Shape** - Pure form appearance
//! - **Context** - Transactional environment
//! - **Morph** - Organic Unity of Shape + Context
//!
//! ## The Triadic Cycle
//!
//! ```
//! Membership → Consequence → Inherence → Loop
//!     ↓            ↓            ↓         ↓
//!   X | Y    →   X → Y    →   X & Y  →  Loop
//!     ↓            ↓            ↓         ↓
//! Field Val  →  Dep Res    →  Code Gen →  Loop
//! ```
//!
//! This is the **initial appearance** of **Thesis-Antithesis-Synthesis**.

pub mod shape;
pub mod container;
pub mod morph;

// Re-export the core concepts
pub use shape::*;
pub use container::Container;
pub use morph::{Morph, OrganicUnity};

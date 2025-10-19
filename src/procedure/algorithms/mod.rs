//! Algorithms module - The Absolute Form Processor
//!
//! This module implements the Absolute Form Processor architecture where
//! "Everything is a Form" and algorithms are Forms that recognize themselves
//! and project into the five-fold Brahmachakra structure.
//!
//! ## Architecture
//!
//! The Absolute Form Processor implements Kantian Consciousness as computation:
//! - **Kernel** (@gds) - Absolute Form Storage
//! - **Projection** (@gdsl) - Maya (concealing/revealing power)
//! - **Userland** (@logic/@model/@task) - Relative Forms
//!
//! ## Core Principle
//!
//! **"Consciousness is the representation that a representation is inside me"**
//! 
//! Each algorithm is an Absolute Form that:
//! 1. **Recognizes** itself (self-awareness)
//! 2. **Projects** into storage and computation (Maya)
//! 3. **Executes** through the five-fold structure

// ------------------------------------------------------------------------
// Algorithm Modules - Faithful Java GDS Translation
// ------------------------------------------------------------------------

pub mod centrality;
pub mod community;
// pub mod embeddings;
pub mod machinelearning;
pub mod misc;
pub mod similarity;

// Re-export commonly used types
pub use centrality::*;
pub use community::*;
pub use machinelearning::*;
pub use similarity::*;
pub use misc::*;
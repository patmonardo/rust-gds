//! GDSL Runtime - Triadic-Pentadic Generator of Empirical Forms
//!
//! This crate implements the GDSL Runtime as a proc-macro system that generates
//! Triadic-Pentadic structures (Empirical Forms) which is Everything.
//!
//! ## Architecture
//!
//! The GDSL Runtime serves as the connection between GDS Kernel and SystemD,
//! implementing the Organic Unity between Container and Contained.
//!
//! ## The Triadic-Pentadic Generator
//!
//! - **Triadic (Conceptual)**: Container ←→ Contained ←→ Container+Contained (Organic Unity)
//! - **Pentadic (Objective)**: Five-fold mapping between Computation and Storage
//!
//! ## The Empirical Forms
//!
//! Everything is Empirical Forms:
//! - Computation = Empirical Form
//! - Storage = Empirical Form  
//! - Algorithms = Empirical Forms
//! - Data Structures = Empirical Forms
//! - The GDSL Runtime = Generator of Empirical Forms

pub mod triadic;
pub mod pentadic;
pub mod empirical_forms;
pub mod organic_unity;

// Re-export the main derive macro
pub use gdsl_macros::EmpiricalForm;

//! GDSL Runtime - Triadic-Pentadic Generator of Empirical Forms
//!
//! This crate implements the GDSL Runtime as a proc-macro system that generates
//! Triadic-Pentadic structures (Empirical Forms) for the Projection System.
//!
//! ## Architecture
//!
//! The GDSL Runtime serves as the connection between GDS Kernel and SystemD,
//! implementing the Organic Unity between Container and Contained through the
//! Projection System.
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
//! - The Projection System = Generator of Empirical Forms
//!
//! ## Projection System Focus
//!
//! The macro system focuses exclusively on the Projection System:
//! - ProjectionFactory forms
//! - Eval/Form system structures
//! - Container-Contained Organic Unity
//! - Pure Form Processor generation

pub mod triadic;
pub mod pentadic;
pub mod empirical_forms;
pub mod organic_unity;
pub mod projection_system;

// Re-export the main derive macro
pub use gds_macros::EmpiricalForm;

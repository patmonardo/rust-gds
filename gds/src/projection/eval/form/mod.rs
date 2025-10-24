//! Form Evaluator - Fixed Singularity
//!
//! This module implements the **Form Evaluator** as a **fixed singularity** that executes
//! the **Form infrastructure**. It's the **third ISA** consisting of the **Triads of Hegel**.
//!
//! ## Architecture
//!
//! The Form Evaluator executes the **Form infrastructure**:
//! - **Thesis** = Procedure (Immediate)
//! - **Antithesis** = ML (Mediate)
//! - **Synthesis** = Form (Sublates both)
//!
//! ## The Three ISA
//!
//! ```
//! eval/procedure (Computation ISA)  ← AlgorithmSpec implementations
//! eval/ml (ML ISA)                 ← Pipeline implementations  
//! eval/form (Form ISA)             ← FormSpec implementations
//! ```

// pub mod form_spec;
pub mod executor;
pub mod triadic_cycle;

// Re-export the core concepts
pub use form_spec::*;
pub use executor::*;
pub use triadic_cycle::*;

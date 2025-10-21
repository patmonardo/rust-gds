//! Eval - Evaluation System
//!
//! This module provides the evaluation system for the GDS platform.
//! It contains the core evaluation logic and interfaces.
//!
//! ## The Three ISA Architecture
//!
//! ```
//! eval/procedure (Computation ISA)  ← AlgorithmSpec implementations
//! eval/ml (ML ISA)                 ← Pipeline implementations  
//! eval/form (Form ISA)             ← FormSpec implementations
//! ```

// Form Processor - speculative, future work
// pub mod form;
// pub mod form_processor;

// ML Pipeline - Java GDS translation (active)
// pub mod ml;
// pub mod native_factory;

// Procedure - Raising src/procedure infrastructure into consciousness
pub mod procedure;

// Form - The third ISA (Triads of Hegel)
pub mod form;

// pub use form::*;
// pub use form_processor::*;
// pub use ml::*;
// pub use native_factory::*;
pub use procedure::*;
pub use form::{FormSpec, FormExecutor, FormResult, FormError, TriadicCycle, Thesis, Antithesis, Synthesis};

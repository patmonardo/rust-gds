//! Form - Form Infrastructure
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
//! ## The Three Evaluators
//!
//! ```
//! ┌─────────────────────────────────────┐
//! │  THREE EVALUATORS                    │
//! │  • Procedure Evaluator              │ ← Known (projection/eval/procedure/)
//! │  • ML Evaluator                     │ ← Known (projection/eval/ml/)
//! │  • Form Evaluator                   │ ← UNKNOWN (what we're discovering!)
//! └─────────────────────────────────────┘
//! ```
//!
//! The **Form Evaluator** is **unknown** - but by building the **Form Infrastructure**,
//! we're **discovering** what it will be.

pub mod core;

// Re-export the core concepts
pub use core::*;

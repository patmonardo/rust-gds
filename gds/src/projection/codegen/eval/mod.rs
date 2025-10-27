//! Evaluation Code Generation
//!
//! This module contains macros for generating evaluation infrastructure:
//! - `value_type_table!` - Master projector for property types
//! - Property descriptor generation
//! - Functor implementations (Gross ↔ Subtle)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::eval::*;
//!
//! // Define property types
//! value_type_table! {
//!     Long { id: 1, value_type: ValueType::Long, ... },
//!     Double { id: 2, value_type: ValueType::Double, ... },
//! }
//! ```

// Import the macro from the eval_macro module
#[macro_use]
mod eval_macro;

// Note: The main value_type_table! macro is now in codegen/values
// and is available via projection::codegen::value_type_table!

//! Config - Pure FormShape Configuration System
//!
//! This module implements the Container-Level FormShape configuration system
//! that defines Pure Forms in anticipation of transcendent proc-macro evolution.
//!
//! ## Architecture
//!
//! The Config system operates at the Container Level, defining FormShapes that
//! represent the essential structure of configuration objects. These FormShapes
//! are Pure Forms that can be projected into various concrete implementations.
//!
//! ## Pure FormShapes
//!
//! A FormShape is the essential structure of a configuration:
//! - **Container**: The configuration struct itself
//! - **Contained**: The individual fields and their types
//! - **Container+Contained**: The builder pattern and validation
//!
//! This represents the Triadic structure at the heart of the Organic Unity.

pub mod define_config;
pub mod form_shape;
pub mod container;
pub mod validation;

// The define_config macro is exported at crate root via #[macro_export]
// so we don't need to re-export it here

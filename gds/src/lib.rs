//! Rust GDS - Graph Data Science library
//!
//! A modular graph data structure and algorithms library.
#![allow(ambiguous_wide_pointer_comparisons)]

// Real algorithm implementations are in procedures/ module
// (Previously had speculative stubs here - all moved to procedures/)

pub mod collections;
pub mod concurrency;
pub mod config;
pub mod core;
pub mod errors;
// pub mod form;
pub mod mem;
pub mod ml;
pub mod pregel;
pub mod procedures;
pub mod projection;
pub mod types;
pub mod values;

// pub use core::*;
// pub use ml::*;
// pub use procedures::*;
// pub use projection::*;
pub use types::*;
pub use values::*;

// Re-export procedure macros for procedures module
#[cfg(feature = "procedures")]
pub use projection::codegen::algorithm::*;  
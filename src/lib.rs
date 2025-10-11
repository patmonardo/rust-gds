//! Rust GDS - Graph Data Science library
//!
//! A modular graph data structure and algorithms library.

pub mod collections;
pub mod concurrency;
// Ensure projection (codegen macros) is compiled before config so macro_rules! defs
// inside projection::codegen are available to config modules that invoke them.
pub mod config;
pub mod core;
pub mod mem;
pub mod pregel;
pub mod projection;
pub mod termination;
pub mod types;
pub mod values;
pub use core::*;

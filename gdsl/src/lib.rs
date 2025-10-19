//! GDSL Runtime - SystemD Shell Language
//!
//! This crate implements the GDSL Runtime as a TS-JSON messaging language
//! that connects the Kernel (Pure Form Processor) to UserLand (Given Form Processor).
//!
//! ## Architecture
//!
//! ```
//! Kernel (Pure Form Processor) <- GDSL -> UserLand (Given Form Processor)
//! ```
//!
//! - **Kernel**: Pure Form Processor (GDS crate)
//! - **GDSL**: TS-JSON messaging language (this crate)
//! - **UserLand**: Given Form Processor (Logic package)
//!
//! ## The GDSL Messaging Language
//!
//! GDSL is the shell language that:
//! - Translates between Pure Forms and Given Forms
//! - Provides TS-JSON interface for UserLand
//! - Manages communication between Kernel and UserLand
//! - Implements the Container-Contained messaging protocol
//!
//! ## TS-JSON Interface
//!
//! The GDSL Runtime provides:
//! - TS-JSON serialization/deserialization
//! - Message passing between Kernel and UserLand
//! - Form translation and transformation
//! - UserLand API surface

pub mod messaging;
pub mod ts_json;
pub mod shell_language;
pub mod userland_interface;

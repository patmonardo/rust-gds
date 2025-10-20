//! GDSL Runtime - Graph DSL Shell Language
//!
//! This crate implements the GDSL Runtime as a TS-JSON messaging language
//! that connects the Kernel (Pure Form Processor) to the Model (SDSL) to UserLand (Given Form Processor).
//!
//! ## Architecture
//!
//! The GDSL Runtime acts as a messaging bridge:
//! 
//! Kernel (Pure Form Processor) ← GDSL → Model (SDSL) → TaskProcessor → UserLand (Given Form Processor)
//!
//! - **Kernel**: Pure Form Processor (GDS crate)
//! - **GDSL**: TS-JSON messaging language (this crate)
//! - **Model**: SDSL (Species-Specific DSL) - Middleware communication language for View/Representations
//! - **TaskProcessor**: The unity of Kernel Pure Form Processor and UserLand Given Form Processor
//! - **UserLand**: Given Form Processor (Logic package)
//!
//! ## The GDSL Messaging Language
//!
//! GDSL is the shell language that:
//! - Translates between Pure Forms and SDSL representations
//! - Provides TS-JSON interface for Model communication
//! - Manages communication between Kernel and Model
//! - Implements the Container-Contained messaging protocol
//!
//! ## TS-JSON Interface
//!
//! The GDSL Runtime provides:
//! - TS-JSON serialization/deserialization
//! - Message passing between Kernel and Model
//! - Form translation and transformation
//! - SDSL representation interface

pub mod messaging;
pub mod ts_json;
pub mod shell_language;
pub mod model_interface;
pub mod sdsl_interface;

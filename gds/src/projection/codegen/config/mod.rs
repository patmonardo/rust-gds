//! Config - Configuration System
//!
//! This module provides the `define_config!` macro for generating configuration
//! structs with builders, validation, and JSON parsing.
//!
//! ## Usage
//!
//! ```rust
//! define_config! {
//!     name: MyConfig,
//!     fields: {
//!         value: f64 = 1.0,
//!         enabled: bool = true,
//!     },
//!     validation: |cfg| {
//!         if cfg.value <= 0.0 {
//!             return Err(ConfigError::FieldValidation { 
//!                 field: "value".to_string(), 
//!                 message: "Must be positive".to_string() 
//!             });
//!         }
//!         Ok(())
//!     }
//! }
//! ```

pub mod config;
pub mod define_config;
pub mod validation;

// Re-export the macros
pub use crate::generate_config;

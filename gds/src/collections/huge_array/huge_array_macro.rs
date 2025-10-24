//! Declarative macro for generating HugeArray types
//!
//! **Note**: Implementation macros have been moved to `projection::codegen::collections`
//! for better organization and aesthetic barrel imports.
//!
//! Use: `use gds::projection::codegen::huge_primitive_array!;`

// Re-export macro from codegen for backward compatibility
pub use crate::projection::codegen::collections::huge_primitive_array;

//! Monadic PropertyStore: Collections First Base Implementation
//!
//! The monadic system provides a standalone property store that works
//! directly with Collections, independent of graph topology. This is a
//! proof-of-concept demonstrating the Collections First architecture.
//!
//! # Architecture
//!
//! ```text
//! MonadicPropertyStore (HashMap<String, MonadicProperty>)
//!     ↓
//! MonadicProperty (PropertySchema + PropertyValues)
//!     ↓
//! MonadicPropertyValues<C: Collections<T>>
//!     ↓
//! Collections Backend (VecLong, HugeLongArray, etc.)
//! ```
//!
//! # Key Features
//!
//! - **Standalone**: Works independently of GraphStore/Node/Relationship
//! - **Generic**: Works with ANY Collections backend (Vec, Huge, Arrow)
//! - **Type Safe**: Strong typing through Collections trait
//! - **Simple**: No complex inheritance or factory patterns
//! - **Scalable**: From tiny datasets to billions of elements
//!
//! # Examples
//!
//! See `examples/monadic_property_store_demo.rs` for comprehensive examples.

pub mod property;
pub mod property_store;
pub mod property_values;

// Macro system for generating property values types
#[macro_use]
pub mod macros;

// Re-export main types for convenience
pub use property::MonadicProperty;
pub use property_store::{MonadicPropertyStore, MonadicPropertyStoreBuilder};

// Re-export all generated property values types
pub use property_values::{
    // Primitives
    MonadicLongPropertyValues,
    MonadicDoublePropertyValues,
    MonadicIntPropertyValues,
    MonadicFloatPropertyValues,
    MonadicShortPropertyValues,
    MonadicBytePropertyValues,
    MonadicBooleanPropertyValues,
    MonadicCharPropertyValues,
    MonadicStringPropertyValues,
    // Arrays
    MonadicLongArrayPropertyValues,
    MonadicDoubleArrayPropertyValues,
    MonadicIntArrayPropertyValues,
    MonadicFloatArrayPropertyValues,
    MonadicShortArrayPropertyValues,
    MonadicByteArrayPropertyValues,
    MonadicBooleanArrayPropertyValues,
    MonadicCharArrayPropertyValues,
    MonadicStringArrayPropertyValues,
};

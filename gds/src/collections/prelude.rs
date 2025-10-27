//! Collections Prelude
//!
//! This module provides curated exports for the most commonly used
//! types and traits from the collections module. Import with:
//!
//! ```rust
//! use crate::collections::prelude::*;
//! ```

// Core traits - define the universal interface
pub use super::traits::{
    AggregationSupport,
    CachingSupport,
    Collections,
    CollectionsFactory,
    CompressionSupport,
    NullabilitySupport,
    ParallelSupport,
    PropertyValuesAdapter,
};

// Huge backend - paged arrays for large-scale data
pub use super::backends::huge::{
    HugeAtomicDoubleArray,
    HugeAtomicLongArray,
    HugeBooleanArray,
    HugeByteArray,
    HugeCharArray,
    HugeDoubleArray,
    HugeFloatArray,
    HugeIntArray,
    HugeLongArray,
    HugeObjectArray,
    HugeShortArray,
};

// Vec backend - enhanced standard library vectors
pub use super::backends::vec::{
    EnhancedVec,
    VecBoolean,
    VecByte,
    VecChar,
    VecDouble,
    VecDoubleArray,
    VecFloat,
    VecInt,
    VecLong,
    VecLongArray,
    VecShort,
};

// Arrow backend - Apache Arrow arrays
#[cfg(feature = "arrow")]
pub use super::backends::arrow::{
    ArrowIntArray,
    ArrowArray,
};

// Universal adapter - trait-based abstraction
pub use super::adapter::{
    CollectionFactory,
    UniversalPropertyValues,
};

// Backend selection
pub use super::CollectionsBackend;

// Legacy types for backward compatibility
pub use super::{
    BitSet,
    HugeSparseLongArray,
};


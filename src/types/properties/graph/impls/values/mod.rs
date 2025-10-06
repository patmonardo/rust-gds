mod double;
mod double_array;
mod float_array;
/// Individual value type implementations for graph properties.
/// Each file contains a focused implementation of a specific ValueType,
/// making it easy to swap backends (Vec -> Arrow2/Polars) per type.
///
/// Graph properties are simpler than node properties - they use iterator-based
/// access and don't require Option wrapping for arrays.
mod long;
mod long_array;

pub use double::DefaultDoubleGraphPropertyValues;
pub use double_array::DefaultDoubleArrayGraphPropertyValues;
pub use float_array::DefaultFloatArrayGraphPropertyValues;
pub use long::DefaultLongGraphPropertyValues;
pub use long_array::DefaultLongArrayGraphPropertyValues;

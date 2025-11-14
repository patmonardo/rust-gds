mod boolean;
mod double;
mod float;
mod int;
mod long;
mod primitive;
mod utf8;

pub use boolean::ArrowBooleanArray;
pub use double::ArrowDoubleArray;
pub use float::ArrowFloatArray;
pub use int::ArrowIntArray;
pub use long::ArrowLongArray;
pub use primitive::ArrowPrimitiveArray;
pub use utf8::ArrowUtf8Array;

/// Common behavior exposed by Arrow array wrappers.
pub trait ArrowArrayBehavior {
    fn len(&self) -> usize;
    fn null_count(&self) -> usize;
    fn is_null(&self, index: usize) -> bool;
}

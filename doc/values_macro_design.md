# Values System: Macro Design Document

**Date**: 2025-10-06  
**Status**: Design Phase  
**Related**: ADR 0005 - Values System Architecture

## Overview

The Second Macro System generates **individual value implementations** for the Projection/Graph layer. While the First Macro System (`property_values_impl!`) generates columnar storage, the Second Macro System generates individual value accessors.

## Current State: Hand-Written Implementations

### Pattern Analysis

All implementations follow the same structure:

```rust
// Pattern 1: Direct storage (no conversion)
pub struct DefaultLongArray {
    data: Arc<Vec<i64>>,  // Storage
}
impl DefaultLongArray {
    pub fn new(data: Vec<i64>) -> Self {
        Self { data: Arc::new(data) }
    }
}
impl IntegralArray for DefaultLongArray {
    fn long_value(&self, idx: usize) -> i64 {
        self.data[idx]  // Direct access
    }
    fn long_array_value(&self) -> Vec<i64> {
        (*self.data).clone()  // Clone for owned return
    }
}
impl Array for DefaultLongArray {
    fn length(&self) -> usize {
        self.data.len()
    }
}
impl GdsValue for DefaultLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_object(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}

// Pattern 2: With conversion (widening)
pub struct DefaultIntLongArray {
    data: Arc<Vec<i32>>,  // Smaller storage
}
impl IntegralArray for DefaultIntLongArray {
    fn long_value(&self, idx: usize) -> i64 {
        self.data[idx] as i64  // Widen on access
    }
    fn long_array_value(&self) -> Vec<i64> {
        self.data.iter().map(|v| *v as i64).collect()  // Convert
    }
}
// ... rest same as Pattern 1
```

### Key Observations

1. **Storage**: Always `Arc<Vec<T>>`
2. **Constructor**: Always `new(data: Vec<T>)`
3. **Trait Implementations**: Fixed pattern based on storage vs accessor type
4. **Conversions**: Widening conversions (i8→i64, i16→i64, i32→i64, f32→f64)
5. **ValueType**: Maps to enum variant

## Macro Design: `gds_value_impl!`

### Goals

1. Generate struct + constructor
2. Generate trait implementations
3. Handle type conversions automatically
4. Support both scalar and array types
5. Generate appropriate `as_object()` JSON serialization

### Macro Variants

```rust
// Variant 1: Direct storage (no conversion)
gds_value_impl!(array_direct, DefaultLongArray, i64, Long, IntegralArray);

// Variant 2: Widening conversion
gds_value_impl!(array_convert, DefaultIntLongArray, i32, i64, Long, IntegralArray);

// Variant 3: Scalar value
gds_value_impl!(scalar, DefaultLongValue, i64, Long, IntegralValue);

// Variant 4: Floating point array (direct)
gds_value_impl!(array_direct, DefaultDoubleArray, f64, Double, FloatingPointArray);

// Variant 5: Floating point array (convert)
gds_value_impl!(array_convert, DefaultFloatArray, f32, f64, Float, FloatingPointArray);
```

### Macro Expansion Examples

**Input**:

```rust
gds_value_impl!(array_direct, DefaultLongArray, i64, LongArray, IntegralArray);
```

**Expands to**:

```rust
#[derive(Clone)]
pub struct DefaultLongArray {
    data: Arc<Vec<i64>>,
}

impl DefaultLongArray {
    pub fn new(data: Vec<i64>) -> Self {
        Self { data: Arc::new(data) }
    }
}

impl IntegralArray for DefaultLongArray {
    fn long_value(&self, idx: usize) -> i64 {
        self.data[idx]
    }
    fn long_array_value(&self) -> Vec<i64> {
        (*self.data).clone()
    }
}

impl Array for DefaultLongArray {
    fn length(&self) -> usize {
        self.data.len()
    }
}

impl GdsValue for DefaultLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_object(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}
```

**Input with conversion**:

```rust
gds_value_impl!(array_convert, DefaultIntLongArray, i32, i64, LongArray, IntegralArray);
```

**Expands to**:

```rust
#[derive(Clone)]
pub struct DefaultIntLongArray {
    data: Arc<Vec<i32>>,  // Storage type
}

impl DefaultIntLongArray {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data: Arc::new(data) }
    }
}

impl IntegralArray for DefaultIntLongArray {
    fn long_value(&self, idx: usize) -> i64 {
        self.data[idx] as i64  // Convert storage → accessor
    }
    fn long_array_value(&self) -> Vec<i64> {
        self.data.iter().map(|v| *v as i64).collect()
    }
}

impl Array for DefaultIntLongArray {
    fn length(&self) -> usize {
        self.data.len()
    }
}

impl GdsValue for DefaultIntLongArray {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }
    fn as_object(&self) -> JsonValue {
        JsonValue::from(self.long_array_value())
    }
}
```

## Implementation Plan

### Phase 1: Core Macro (Current Priority)

File: `src/values/macros.rs`

```rust
/// Generate GdsValue implementation for array types with direct storage
#[macro_export]
macro_rules! gds_value_array_impl {
    ($name:ident, $storage_ty:ty, $value_type:ident, $array_trait:ident) => {
        // Generate struct, constructor, trait impls
    };
}

/// Generate GdsValue implementation for array types with conversion
#[macro_export]
macro_rules! gds_value_array_convert_impl {
    ($name:ident, $storage_ty:ty, $accessor_ty:ty, $value_type:ident, $array_trait:ident) => {
        // Generate struct, constructor, trait impls with conversion
    };
}

/// Generate GdsValue implementation for scalar types
#[macro_export]
macro_rules! gds_value_scalar_impl {
    ($name:ident, $ty:ty, $value_type:ident, $scalar_trait:ident) => {
        // Generate scalar struct and impls
    };
}
```

### Phase 2: PrimitiveValues Factory Enhancement

```rust
/// Generate PrimitiveValues::of() match arms for a type
#[macro_export]
macro_rules! primitive_value_factory {
    ($(
        $variant:ident => $impl_type:ty
    ),* $(,)?) => {
        // Generate match arms in PrimitiveValues::of()
    };
}
```

Usage:

```rust
primitive_value_factory! {
    Long => DefaultLongValue,
    Double => DefaultFloatingPointValue,
    LongArray => DefaultLongArray,
    DoubleArray => DefaultDoubleArray,
    // ... etc for all 45 variants
}
```

### Phase 3: Integration with PropertyValues

Bridge macro to extract GdsValue from PropertyValues column:

```rust
/// Extract individual GdsValue from PropertyValues at index
pub trait PropertyValuesExt {
    fn value_at(&self, index: usize) -> Option<Arc<dyn GdsValue>>;
}

// Generated for each PropertyValues implementation
```

## Type Mapping Table

| ValueType Variant | Storage Type | Accessor Type | Implementation            | Status  |
| ----------------- | ------------ | ------------- | ------------------------- | ------- |
| Long              | i64          | i64           | DefaultLongValue          | ✅ Done |
| Double            | f64          | f64           | DefaultFloatingPointValue | ✅ Done |
| LongArray         | Vec<i64>     | i64/Vec<i64>  | DefaultLongArray          | ✅ Done |
| LongArray         | Vec<i32>     | i64/Vec<i64>  | DefaultIntLongArray       | ✅ Done |
| LongArray         | Vec<i16>     | i64/Vec<i64>  | DefaultShortLongArray     | ✅ Done |
| LongArray         | Vec<u8>      | i64/Vec<i64>  | DefaultByteLongArray      | ✅ Done |
| DoubleArray       | Vec<f64>     | f64/Vec<f64>  | DefaultDoubleArray        | ✅ Done |
| FloatArray        | Vec<f32>     | f64/Vec<f64>  | DefaultFloatArray         | ✅ Done |
| String            | String       | String        | DefaultStringValue        | ⏳ TODO |
| StringArray       | Vec<String>  | String/Vec    | DefaultStringArray        | ⏳ TODO |
| LongMap           | HashMap      | -             | DefaultLongMap            | ⏳ TODO |
| ... (37 more)     | -            | -             | -                         | ⏳ TODO |

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_default_long_array() {
    let arr = DefaultLongArray::new(vec![1, 2, 3]);
    assert_eq!(arr.length(), 3);
    assert_eq!(arr.long_value(0), 1);
    assert_eq!(arr.value_type(), ValueType::LongArray);
}

#[test]
fn test_conversion_int_to_long() {
    let arr = DefaultIntLongArray::new(vec![1i32, 2, 3]);
    assert_eq!(arr.long_value(0), 1i64);
    // Verify widening works correctly
}
```

### Integration Tests

```rust
#[test]
fn test_primitive_values_factory() {
    let json = json!([1, 2, 3]);
    let value = PrimitiveValues::of(&json).unwrap();
    assert_eq!(value.value_type(), ValueType::LongArray);
}
```

## Future Enhancements

1. **Arrow2 Backing**: Support `Arc<PrimitiveArray<T>>` as storage
2. **Null Support**: Optional<T> and nullable arrays
3. **String Types**: UTF-8 string handling
4. **Map Types**: Nested key-value structures
5. **Custom Conversions**: User-defined type widening
6. **Lazy Conversion**: Delay conversion until accessor called

## References

- First Macro System: `src/types/properties/property_values.rs`
- ValueType enum: `src/types/value_type.rs`
- TS GDS Values: TypeScript value hierarchy
- Rust macro_rules: https://doc.rust-lang.org/book/ch19-06-macros.html

---

**Next Action**: Implement `src/values/macros.rs` with initial macro variants for array types.

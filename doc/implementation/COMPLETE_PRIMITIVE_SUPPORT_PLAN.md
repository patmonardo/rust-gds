# Complete Primitive Support: Vec and Huge Backends

**Date**: October 2024  
**Goal**: Support all 9 primitive types in both Vec and Huge backends  
**Scope**: RAM only (Vec + Huge), Arrow deferred to future

---

## The 9 Primitive Types

Based on Arrow/Java GDS/Rust standards:

| Type | Rust Type | Size | Arrow Equivalent |
|------|-----------|------|------------------|
| **Byte** | i8 | 1 byte | Int8 |
| **Short** | i16 | 2 bytes | Int16 |
| **Int** | i32 | 4 bytes | Int32 |
| **Long** | i64 | 8 bytes | Int64 |
| **Float** | f32 | 4 bytes | Float32 |
| **Double** | f64 | 8 bytes | Float64 |
| **Boolean** | bool | 1 byte | Boolean |
| **Char** | char | 4 bytes | UInt32 (Unicode) |
| **String** | String | heap | Utf8/LargeUtf8 |

**Why these 9?**
- Match Arrow primitive types
- Match Java GDS primitive types
- Standard numeric/boolean/text types
- Direct storage (no boxing)

---

## Current Status Matrix

### Vec Backend

| Type | Struct | Collections Impl | Status |
|------|--------|------------------|--------|
| i8 | VecByte | ✅ | DONE |
| i16 | VecShort | ✅ | DONE |
| i32 | VecInt | ✅ | DONE |
| i64 | VecLong | ✅ | DONE |
| f32 | VecFloat | ✅ | DONE |
| f64 | VecDouble | ✅ | DONE |
| bool | VecBoolean | ✅ | DONE |
| char | VecChar | ✅ | DONE |
| String | VecString | ❌ | **MISSING** |

**Status**: 8/9 ✅ (only String missing!)

### Huge Backend

| Type | Struct | Collections Impl | Status |
|------|--------|------------------|--------|
| i8 | HugeByteArray | ❌ | **MISSING** |
| i16 | HugeShortArray | ❌ | **MISSING** |
| i32 | HugeIntArray | ✅ | DONE |
| i64 | HugeLongArray | ✅ | DONE |
| f32 | HugeFloatArray | ❌ | **MISSING** |
| f64 | HugeDoubleArray | ✅ | DONE |
| bool | HugeBooleanArray | ❌ | **MISSING** |
| char | HugeCharArray | ❌ | **MISSING** |
| String | HugeStringArray | ❌ | **MISSING** or use HugeObjectArray<String>? |

**Status**: 3/9 ✅ (need 6 more!)

---

## What Exists But Needs Collections Impl

### Files That Exist

All Huge types already have their array implementations:

```
✅ gds/src/collections/backends/huge/huge_byte_array.rs     (673 lines)
✅ gds/src/collections/backends/huge/huge_short_array.rs    (693 lines)
✅ gds/src/collections/backends/huge/huge_int_array.rs      (1004 lines)
✅ gds/src/collections/backends/huge/huge_long_array.rs     (similar)
✅ gds/src/collections/backends/huge/huge_float_array.rs    (902 lines)
✅ gds/src/collections/backends/huge/huge_double_array.rs   (902 lines)
✅ gds/src/collections/backends/huge/huge_boolean_array.rs  (693 lines)
✅ gds/src/collections/backends/huge/huge_char_array.rs     (693 lines)
✅ gds/src/collections/backends/huge/huge_object_array.rs   (446 lines)
```

**They just need Collections<T> impls!**

---

## Implementation Strategy

### Phase 1: Add Collections Impls to Huge (Easy!)

We have a macro that makes this trivial. Look at what's already done in `huge/mod.rs`:

```rust
// EXISTING (lines 38-186):
impl Collections<i32> for HugeIntArray { ... }
impl Collections<i64> for HugeLongArray { ... }  
impl Collections<f64> for HugeDoubleArray { ... }
```

**Just add 6 more!**

```rust
// ADD THESE:
impl Collections<f32> for HugeFloatArray { ... }
impl Collections<i16> for HugeShortArray { ... }
impl Collections<i8> for HugeByteArray { ... }
impl Collections<bool> for HugeBooleanArray { ... }
impl Collections<char> for HugeCharArray { ... }
```

### Phase 2: Add VecString (New File)

Create `gds/src/collections/backends/vec/vec_string.rs`:

```rust
#[derive(Debug, Clone)]
pub struct VecString {
    pub data: Vec<String>,
}

impl Collections<String> for VecString {
    // Similar to VecLong but for String
}
```

### Phase 3: String Decision for Huge

**Option A**: Create HugeStringArray (specialized)
- Pros: Consistent with other primitives
- Cons: String is heap-allocated, not really "huge" benefit

**Option B**: Use HugeObjectArray<String>
- Pros: Reuse existing code, String already on heap
- Cons: Less consistent naming

**Recommendation**: Start with HugeObjectArray<String>, add HugeStringArray if needed

---

## Detailed Implementation Steps

### Step 1: Add Huge Float Collections Impl

**File**: `gds/src/collections/backends/huge/mod.rs`

**Add after HugeDoubleArray impl** (around line 476):

```rust
// ============================================================================
// Collections<f32> for HugeFloatArray
// ============================================================================

impl Collections<f32> for HugeFloatArray {
    fn get(&self, index: usize) -> Option<f32> {
        Some(self.get(index))
    }
    
    fn set(&mut self, index: usize, value: f32) {
        self.set(index, value);
    }
    
    fn len(&self) -> usize {
        self.size()
    }
    
    fn sum(&self) -> Option<f32> where f32: Sum {
        let mut sum = 0.0f32;
        for i in 0..self.size() {
            sum += self.get(i);
        }
        Some(sum)
    }
    
    fn mean(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            let sum: f32 = self.sum()?;
            Some(sum as f64 / self.size() as f64)
        }
    }
    
    fn min(&self) -> Option<f32> where f32: Ord {
        if self.size() == 0 {
            return None;
        }
        let mut min = self.get(0);
        for i in 1..self.size() {
            let val = self.get(i);
            if val < min {
                min = val;
            }
        }
        Some(min)
    }
    
    fn max(&self) -> Option<f32> where f32: Ord {
        if self.size() == 0 {
            return None;
        }
        let mut max = self.get(0);
        for i in 1..self.size() {
            let val = self.get(i);
            if val > max {
                max = val;
            }
        }
        Some(max)
    }
    
    // ... std_dev, variance, median, percentile similar to HugeDoubleArray
    
    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> f32 { 0.0 }
    fn backend(&self) -> CollectionsBackend { CollectionsBackend::Huge }
    fn features(&self) -> &[Extension] { &[] }
    fn extensions(&self) -> &[Extension] { &[] }
    fn value_type(&self) -> ValueType { ValueType::Float }
    fn with_capacity(capacity: usize) -> Self { Self::new(capacity) }
    fn with_defaults(count: usize, default_value: f32) -> Self {
        let mut arr = Self::new(count);
        arr.fill(default_value);
        arr
    }
    
    // ... implement remaining methods
}
```

**Repeat for**: i16, i8, bool, char

### Step 2: Create VecString

**File**: `gds/src/collections/backends/vec/vec_string.rs`

```rust
//! VecString: Vec-based String Collections implementation

use crate::collections::traits::Collections;
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;

/// Vec-based String Collections implementation
#[derive(Debug, Clone)]
pub struct VecString {
    pub data: Vec<String>,
}

impl VecString {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: String) {
        self.data.push(value);
    }
}

impl From<Vec<String>> for VecString {
    fn from(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl Collections<String> for VecString {
    fn get(&self, index: usize) -> Option<String> {
        self.data.get(index).cloned()
    }

    fn set(&mut self, index: usize, value: String) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, String::new());
            self.data[index] = value;
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    // No arithmetic ops for String
    fn sum(&self) -> Option<String> { None }
    fn mean(&self) -> Option<f64> { None }
    fn min(&self) -> Option<String> { None }
    fn max(&self) -> Option<String> { None }
    fn std_dev(&self) -> Option<f64> { None }
    fn variance(&self) -> Option<f64> { None }
    fn median(&self) -> Option<String> { None }
    fn percentile(&self, _p: f64) -> Option<String> { None }
    
    fn binary_search(&self, key: &String) -> Result<usize, usize> {
        self.data.binary_search(key)
    }
    
    fn sort(&mut self) {
        self.data.sort();
    }
    
    fn to_vec(self) -> Vec<String> {
        self.data
    }
    
    fn as_slice(&self) -> &[String] {
        &self.data
    }

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> String { String::new() }
    fn backend(&self) -> CollectionsBackend { CollectionsBackend::Vec }
    fn features(&self) -> &[Extension] { &[] }
    fn extensions(&self) -> &[Extension] { &[] }
    fn value_type(&self) -> ValueType { ValueType::String }
    
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    
    fn with_defaults(count: usize, default_value: String) -> Self {
        Self { data: vec![default_value; count] }
    }
}
```

**Add to** `gds/src/collections/backends/vec/mod.rs`:
```rust
pub mod vec_string;
pub use vec_string::VecString;
```

### Step 3: Generate PropertyValues Types

Once Collections impls are done, use the `monadic_property_values!` macro:

```rust
// In monadic/property_values.rs, ADD:

monadic_property_values!(MonadicBytePropertyValues => i8, ValueType::Byte);
monadic_property_values!(MonadicShortPropertyValues => i16, ValueType::Short);
// (already have Int, Long, Float, Double, Boolean, Char, String)
```

**Easy expansion!**

---

## Testing Strategy

### Test Each Type

For each new Collections impl, add test:

```rust
#[test]
fn test_huge_float_collections() {
    let mut arr = HugeFloatArray::new(1000);
    arr.set(0, 3.14);
    arr.set(999, 2.71);
    
    assert_eq!(arr.get(0), 3.14);
    assert_eq!(arr.get(999), 2.71);
    assert_eq!(arr.len(), 1000);
}
```

### Test PropertyValues

```rust
#[test]
fn monadic_float_property_values_with_huge() {
    let huge = HugeFloatArray::new(100);
    let values = MonadicFloatPropertyValues::new(huge, 0.0);
    
    assert_eq!(values.value_type(), ValueType::Float);
    assert_eq!(values.element_count(), 100);
}
```

---

## Work Estimates

| Task | Effort | Priority |
|------|--------|----------|
| HugeFloatArray Collections | 15 min | HIGH |
| HugeShortArray Collections | 15 min | HIGH |
| HugeByteArray Collections | 15 min | HIGH |
| HugeBooleanArray Collections | 10 min | HIGH |
| HugeCharArray Collections | 10 min | MEDIUM |
| VecString | 20 min | MEDIUM |
| Tests for all | 30 min | HIGH |
| PropertyValues generation | 5 min | LOW (macro!) |

**Total**: ~2 hours to complete all 9 primitives!

---

## Summary

**Goal**: Support all 9 Arrow-compatible primitives in Vec and Huge

**Current**:
- Vec: 8/9 (just need String)
- Huge: 3/9 (need Float, Short, Byte, Boolean, Char, String)

**Strategy**:
1. Add Collections impls to existing Huge arrays (90% code exists!)
2. Create VecString
3. Use macros to generate PropertyValues
4. Test everything

**Result**: Complete primitive support in RAM backends! ✅

Then we can tackle HugeObjectArray<T> for complex types! 🚀

---

## Notes

- **Leave Arrow out** - focus on RAM (Vec/Huge) first
- **String is special** - heap-allocated, maybe use HugeObjectArray<String>?
- **Char is weird** - Rust char is 4 bytes (Unicode), not ASCII
- **Macros help** - once Collections works, PropertyValues is automatic

**Let's complete this foundation!** 🎯


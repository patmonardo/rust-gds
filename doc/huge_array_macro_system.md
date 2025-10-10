# HugeArray Macro System

## Overview

The `huge_primitive_array!` declarative macro generates complete, type-safe HugeArray implementations for all primitive types. This eliminates code duplication while maintaining performance and type safety.

## Architecture

Each generated HugeArray consists of three types:

```
HugeFooArray (enum)
├── Single(SingleHugeFooArray)  // ≤ 268M elements
└── Paged(PagedHugeFooArray)    // > 268M elements
```

The macro automatically generates:

- ✅ All core methods (get, set, fill, set_all, copy_of, etc.)
- ✅ Cursor support via HugeCursorSupport trait
- ✅ Binary search (for PartialOrd types)
- ✅ Memory tracking (size_of)
- ✅ Conversion to/from Vec

## Usage

### Basic Usage

```rust
use crate::huge_primitive_array;

huge_primitive_array! {
    HugeIntArray,              // Main enum name
    SingleHugeIntArray,        // Single-page struct name
    PagedHugeIntArray,         // Multi-page struct name
    i32,                       // Element type
    "Int",                     // Display name (for docs)
    "A long-indexable i32 array supporting billions of elements."
}
```

### Generated API

```rust
// Creation
let array = HugeIntArray::new(1_000_000);
let array = HugeIntArray::from_vec(vec![1, 2, 3]);

// Access
array.get(index) -> i32
array.set(index, value)

// Bulk operations
array.fill(42);
array.set_all(|i| i as i32);
array.copy_to(&mut dest, length);
let copy = array.copy_of(new_size);

// Metadata
array.size() -> usize
array.size_of() -> usize  // Memory in bytes

// Search (for PartialOrd types)
array.binary_search(value) -> isize
array.binary_search_by(|v| v.cmp(&target))

// Iteration via cursors
let mut cursor = array.new_cursor();
init_cursor(&array, &mut cursor);
while cursor.next() { ... }
```

## Implementation Plan

### Phase 1: Core Numeric Types (High Priority)

Generate these to match ValueType enum requirements:

```rust
// Signed integers
huge_primitive_array!(HugeByteArray, SingleHugeByteArray, PagedHugeByteArray,
    i8, "Byte", "8-bit signed integer array");

huge_primitive_array!(HugeShortArray, SingleHugeShortArray, PagedHugeShortArray,
    i16, "Short", "16-bit signed integer array");

huge_primitive_array!(HugeIntArray, SingleHugeIntArray, PagedHugeIntArray,
    i32, "Int", "32-bit signed integer array");

// Already exists: HugeLongArray (i64)

// Floating point
huge_primitive_array!(HugeFloatArray, SingleHugeFloatArray, PagedHugeFloatArray,
    f32, "Float", "32-bit floating point array");

// Already exists: HugeDoubleArray (f64)
```

### Phase 2: Boolean and Character Types

```rust
huge_primitive_array!(HugeBooleanArray, SingleHugeBooleanArray, PagedHugeBooleanArray,
    bool, "Boolean", "Boolean value array");

huge_primitive_array!(HugeCharArray, SingleHugeCharArray, PagedHugeCharArray,
    char, "Char", "Unicode character array");
```

### Phase 3: Unsigned Types (If Needed)

```rust
huge_primitive_array!(HugeUByteArray, SingleHugeUByteArray, PagedHugeUByteArray,
    u8, "UByte", "8-bit unsigned integer array");

huge_primitive_array!(HugeUShortArray, SingleHugeUShortArray, PagedHugeUShortArray,
    u16, "UShort", "16-bit unsigned integer array");

huge_primitive_array!(HugeUIntArray, SingleHugeUIntArray, PagedHugeUIntArray,
    u32, "UInt", "32-bit unsigned integer array");

huge_primitive_array!(HugeULongArray, SingleHugeULongArray, PagedHugeULongArray,
    u64, "ULong", "64-bit unsigned integer array");
```

## File Organization

```
src/collections/huge_array/
├── mod.rs                      // Re-exports all types
├── huge_array_macro.rs         // Macro definition
├── huge_byte_array.rs          // i8
├── huge_short_array.rs         // i16
├── huge_int_array.rs           // i32  ← Example implementation
├── huge_long_array.rs          // i64 (existing, to be migrated)
├── huge_float_array.rs         // f32
├── huge_double_array.rs        // f64 (existing, to be migrated)
├── huge_boolean_array.rs       // bool
├── huge_char_array.rs          // char
└── huge_object_array.rs        // Generic (existing, keep as-is)
```

## Migration Strategy

### Option A: Gradual Migration

1. Add macro and HugeIntArray (new)
2. Test thoroughly
3. Migrate existing HugeLongArray to use macro
4. Migrate existing HugeDoubleArray to use macro
5. Generate remaining types

### Option B: Fresh Start

1. Add macro
2. Generate all primitive types fresh
3. Keep existing implementations for compatibility
4. Deprecate old implementations
5. Remove after one release cycle

**Recommendation**: Option A (gradual migration) for safety.

## Testing Strategy

Each generated array should have:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() { ... }

    #[test]
    fn test_get_set() { ... }

    #[test]
    fn test_fill() { ... }

    #[test]
    fn test_set_all() { ... }

    #[test]
    fn test_from_vec() { ... }

    #[test]
    fn test_copy_of() { ... }

    #[test]
    fn test_binary_search() { ... }

    #[test]
    fn test_large_array() { ... }  // Test paging

    #[test]
    fn test_cursor_iteration() { ... }
}
```

## Benefits of Macro Approach

1. **Single Source of Truth**: All HugeArrays share the same logic
2. **Easy to Extend**: New types require just 6 lines
3. **Consistent API**: All types have identical interfaces
4. **Maintainability**: Bug fixes apply to all types
5. **Type Safety**: Generated code is fully type-checked
6. **Performance**: Zero runtime overhead vs hand-written

## Limitations

1. **Debugging**: Macro-generated code can be harder to debug
   - **Mitigation**: Use `cargo expand` to see generated code
2. **Custom Behavior**: Harder to add type-specific optimizations
   - **Mitigation**: Override methods after macro invocation if needed
3. **Learning Curve**: Team needs to understand macro syntax
   - **Mitigation**: Good documentation (this file!)

## Next Steps

1. ✅ Create `huge_array_macro.rs` with macro definition
2. ✅ Create example `huge_int_array.rs` using macro
3. ⏳ Test HugeIntArray thoroughly
4. ⏳ Update `mod.rs` to export HugeIntArray
5. ⏳ Generate remaining primitive types
6. ⏳ Consider migrating existing HugeLongArray/HugeDoubleArray
7. ⏳ Document in main README

## References

- Java GDS HugeArrays: Reference implementation
- Rust declarative macros: https://doc.rust-lang.org/book/ch19-06-macros.html
- `cargo expand`: Tool for debugging macros
- Copilot instructions: See `.github/copilot-instructions.md`

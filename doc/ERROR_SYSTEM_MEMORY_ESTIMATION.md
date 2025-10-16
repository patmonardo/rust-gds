# Error System Addition - MemoryEstimationError

**Date**: October 16, 2025  
**Translation**: `MemoryEstimationNotImplementedException.java` → `MemoryEstimationError::NotImplemented`  
**Status**: ✅ Complete and tested

---

## What Was Added

### New Error Type

```rust
/// **Translation**: `MemoryEstimationNotImplementedException.java`
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum MemoryEstimationError {
    #[error("Memory estimation is not implemented for this algorithm.")]
    NotImplemented,
}
```

**Characteristics**:

- ✅ `Clone + Copy` - Lightweight, can be duplicated
- ✅ `PartialEq + Eq` - Can be compared for equality
- ✅ `Error` trait via `thiserror::Error`
- ✅ Clear error message via `#[error]` attribute

### Module Exposure

Added `pub mod errors;` to `src/lib.rs` to make error types publicly available.

### Tests Added

```rust
#[test]
fn test_memory_estimation_error_message() {
    let err = MemoryEstimationError::NotImplemented;
    assert_eq!(
        err.to_string(),
        "Memory estimation is not implemented for this algorithm."
    );
}

#[test]
fn test_memory_estimation_error_is_cloneable() {
    let err1 = MemoryEstimationError::NotImplemented;
    let err2 = err1; // Copy
    assert_eq!(err1, err2);
}

#[test]
fn test_api_error_messages() {
    // Tests for existing ApiError variants
}
```

**All 3 tests passing** ✅

---

## Design Rationale

### Java Exception Model

```java
public class MemoryEstimationNotImplementedException
    extends IllegalArgumentException {
    public MemoryEstimationNotImplementedException() {
        super("Memory estimation is not implemented for this algorithm.");
    }
}
```

**Issues**:

- Extends `IllegalArgumentException` (misleading - not about arguments!)
- Runtime exception (unchecked) - easy to forget handling
- No type-safe error handling
- Stack unwinding overhead

### Rust Error Model

```rust
pub enum MemoryEstimationError {
    #[error("Memory estimation is not implemented for this algorithm.")]
    NotImplemented,
}
```

**Benefits**:

- Dedicated type with clear semantics
- Compile-time checked (`Result<T, MemoryEstimationError>`)
- Zero-cost (just an enum variant)
- Composable with `?` operator
- Pattern matchable for exhaustive handling

---

## Usage Example

```rust
use rust_gds::errors::MemoryEstimationError;

trait Algorithm {
    fn memory_estimation(&self) -> Result<MemoryEstimation, MemoryEstimationError> {
        // Default: not implemented
        Err(MemoryEstimationError::NotImplemented)
    }
}

struct PageRank;

impl Algorithm for PageRank {
    fn memory_estimation(&self) -> Result<MemoryEstimation, MemoryEstimationError> {
        // Can override with actual implementation
        Ok(MemoryEstimation::new(/* ... */))
    }
}

// Usage
fn check_memory() -> Result<(), MemoryEstimationError> {
    let algo = PageRank;
    let est = algo.memory_estimation()?; // Propagate error with ?
    println!("Estimated: {} bytes", est.bytes());
    Ok(())
}
```

---

## Integration with Existing Errors

### Current Error Types

1. **MemoryEstimationError** (NEW)

   - Purpose: Memory estimation not available
   - Variants: `NotImplemented`

2. **ApiError** (Existing)
   - Purpose: General API-level errors
   - Variants: `NotFound`, `InvalidInput`, `InternalError`

### Future Error Conversions

When crossing API boundaries:

```rust
impl From<MemoryEstimationError> for ApiError {
    fn from(err: MemoryEstimationError) -> Self {
        match err {
            MemoryEstimationError::NotImplemented => {
                ApiError::InvalidInput(
                    "Memory estimation not available for this algorithm".to_string()
                )
            }
        }
    }
}
```

---

## Documentation Added

Created `doc/ERROR_HANDLING_PHILOSOPHY.md` with:

- Error handling philosophy (Rust vs Java)
- Design patterns for error types
- Best practices for library errors
- Translation strategy from Java exceptions
- Testing approaches
- Future error types to add

---

## Files Modified

1. `src/errors.rs`

   - Added `MemoryEstimationError` enum
   - Added tests for error messages
   - Added documentation

2. `src/lib.rs`

   - Added `pub mod errors;` to expose error types

3. `doc/ERROR_HANDLING_PHILOSOPHY.md` (NEW)
   - Comprehensive guide to error handling in rust-gds
   - Translation patterns from Java exceptions
   - Best practices and examples

---

## Test Results

```
running 3 tests
test errors::tests::test_memory_estimation_error_is_cloneable ... ok
test errors::tests::test_memory_estimation_error_message ... ok
test errors::tests::test_api_error_messages ... ok

test result: ok. 3 passed; 0 failed
```

---

## Why This Matters

This "little specimen" demonstrates the **fundamental difference** between Java and Rust error handling:

### Java Philosophy

- Exceptions as control flow
- Runtime errors (hope you catch them!)
- Inheritance hierarchies
- Stack unwinding overhead

### Rust Philosophy

- Errors as values
- Compile-time guarantees (must handle!)
- Enum-based composition
- Zero-cost abstractions

**This is post-Java design in action.** 🦀

---

## Next Steps

As we translate more Java code, add error types for:

1. **Algorithm errors**

   - `AlgorithmNotSupportedException`
   - `InvalidConfigurationException`

2. **Graph errors**

   - `NodeNotFoundException`
   - `PropertyNotFoundException`

3. **Execution errors**
   - `TerminatedException`
   - `TimeoutException`

Each will follow the same pattern: **specific enum type with clear semantics**, not generic string errors.

---

## Conclusion

✅ `MemoryEstimationError` is now officially part of rust-gds  
✅ All tests passing  
✅ Documentation complete  
✅ Pattern established for future error types

**The specimen is hanging around officially!** 🎉

---

_"Errors are not exceptional. Make them first-class citizens."_

# Core Utils ID Node Property Values - Completion Notes

## Status: PENDING FIXES

Both `MappedIdNodePropertyValues` and `OriginalIdNodePropertyValues` have been created but need systematic fixes before tests will pass.

## Issues to Fix

### 1. Error Handling Pattern

**Problem**: Using `.into()` on string literals doesn't work with `PropertyValuesError`

**Solution**: Use `PropertyValuesError::unsupported_operation(message)`

**Files**: Both `mapped_id_node_property_values.rs` and `original_id_node_property_values.rs`

**Pattern to replace**:

```rust
Err("Message".into())
```

**With**:

```rust
Err(PropertyValuesError::unsupported_operation("Message"))
```

### 2. Test Module Imports

**Problem**: Trait methods not in scope in test modules

**Solution**: Add trait import to test modules

**Files**: Both files need this in their `#[cfg(test)] mod tests` section:

```rust
use crate::types::properties::node::NodePropertyValues;
```

### 3. OriginalIdNodePropertyValues Type Mismatch

**Problem**: `to_original_node_id()` returns `Option<i64>` not `Option<u64>`

**Solution**: Change function signature and Vec type:

```rust
// Change from:
to_original_node_id: Box<dyn Fn(u64) -> Option<u64> + Send + Sync>,
let original_ids: Vec<Option<u64>> = ...

// To:
to_original_node_id: Box<dyn Fn(u64) -> Option<i64> + Send + Sync>,
let original_ids: Vec<Option<i64>> = ...
```

## Files Created

1. **src/core/utils/mapped_id_node_property_values.rs** (~260 lines)

   - Implements `LongNodePropertyValues` where property value = mapped node ID
   - Zero-overhead ID tracking
   - 16 comprehensive tests
   - **Status**: Needs error handling + test import fixes

2. **src/core/utils/original_id_node_property_values.rs** (~350 lines)
   - Implements `LongNodePropertyValues` where property value = original node ID
   - Captures ID mapping at creation time to avoid lifetime issues
   - Supports custom mapping functions
   - 20 comprehensive tests including database ID scenarios
   - **Status**: Needs error handling + test import + type fixes

## Integration Points

- Uses `IdMap::node_count()` and `IdMap::to_original_node_id()`
- Implements `PropertyValues`, `NodePropertyValues`, `LongNodePropertyValues` traits
- Added to `src/core/utils/mod.rs` module exports
- Part of core/utils final modules for IdMap integration

## Next Steps

1. Fix error handling in both files (use `PropertyValuesError::unsupported_operation`)
2. Add `use NodePropertyValues` to test modules
3. Fix OriginalIdNodePropertyValues types (i64 not u64)
4. Run `cargo test --lib mapped_id_node_property_values --features core`
5. Run `cargo test --lib original_id_node_property_values --features core`
6. Verify 36 total tests pass (16 + 20)
7. Create completion document

## Context

These are the **final two core/utils modules** bridging IdMap and the property system. Critical for:

- Export scenarios requiring original IDs
- Algorithm results that reference source graph IDs
- Testing and debugging with node identity tracking
- MetaMacro awareness for ID mapping coordination

## Related Files

- `src/types/graph/id_map/id_map.rs` - IdMap trait definition
- `src/types/properties/node/node_property_values.rs` - NodePropertyValues traits
- `src/types/properties/property_values.rs` - PropertyValuesError helper methods
- `doc/adr0005_values_system_architecture.md` - Values System architecture

## Test Coverage When Complete

- **MappedIdNodePropertyValues**: 16 tests
  - Basic functionality (new, value retrieval)
  - Trait implementations (PropertyValues, NodePropertyValues, LongNodePropertyValues)
  - Edge cases (zero nodes, large node counts)
  - Unsupported operations (doubles, arrays)
- **OriginalIdNodePropertyValues**: 20 tests
  - Identity, offset, and custom mappings
  - Sparse mappings (partial ID coverage)
  - Database ID scenarios (non-sequential IDs)
  - Edge cases and error handling

Total: 36 tests covering ID → property value mappings

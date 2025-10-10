# Eval Macro: Priority Action Items

**Date**: October 10, 2025  
**Status**: System built, migration needed  
**Urgency**: High (affects portability and GDSL integration)

---

## 🚨 Critical Issue: Unsafe u64→usize Casts

### The Problem

**30+ locations** in codebase use **unchecked casts** that will **silently corrupt data** on 32-bit targets:

```rust
// UNSAFE: Throughout PropertyValues implementations
fn long_value(&self, node_id: u64) -> i64 {
    self.values[node_id as usize]  // ← WRAPS on 32-bit if node_id > u32::MAX!
}
```

**Failure Scenario**:

```rust
let node_id: u64 = 5_000_000_000;     // 5 billion nodes (large graph)
let idx = node_id as usize;            // On 32-bit: wraps to 705_032_704
self.values[idx]                       // WRONG NODE! Silent data corruption!
```

### The Solution

**Form Processor** provides safe, checked conversion:

```rust
use crate::projection::form_processor;

fn long_value(&self, node_id: u64) -> Result<i64, FormProcessorError> {
    let idx = form_processor::checked_u64_to_usize(node_id)?;
    Ok(self.values[idx])  // Safe: panics if overflow, no silent corruption
}
```

---

## 📋 Migration Checklist

### Phase 1: Audit (1 hour)

```bash
# Find all unsafe casts
git grep "node_id as usize" src/types/properties/
git grep "node_id as usize" src/pregel/
git grep "id as usize" src/types/graph/

# Expected: 30+ matches across:
# - src/types/properties/node/impls/*.rs
# - src/pregel/node_value.rs
# - src/pregel/context/*.rs
# - src/types/graph/default_graph.rs
```

**Create tracking issue**:

````markdown
## Migrate unsafe u64→usize casts to form_processor

**Files affected** (estimated):

- [ ] src/types/properties/node/impls/default_node_property_values.rs (10 casts)
- [ ] src/types/properties/node/impls/values/\*.rs (15 casts)
- [ ] src/pregel/node_value.rs (5 casts)
- [ ] src/pregel/context/\*.rs (3 casts)
- [ ] src/types/graph/default_graph.rs (2 casts)

**Pattern**:

```rust
// Before:
self.values[node_id as usize]

// After:
let idx = form_processor::checked_u64_to_usize(node_id)?;
self.values[idx]
```
````

````

### Phase 2: Add Safe Methods (2-3 hours)

**For each PropertyValues impl**:

1. **Keep unchecked for hot paths** (document assumptions):
   ```rust
   /// UNSAFE: Assumes node_id < usize::MAX
   /// Only call after validation in checked methods
   #[inline]
   fn long_value_unchecked(&self, node_id: u64) -> i64 {
       debug_assert!(node_id <= usize::MAX as u64, "node_id overflow");
       self.values[node_id as usize]
   }
````

2. **Add checked public method**:

   ```rust
   /// Safe: Returns error if node_id > usize::MAX on this platform
   fn long_value(&self, node_id: u64) -> Result<i64, FormProcessorError> {
       let idx = form_processor::checked_u64_to_usize(node_id)?;
       Ok(self.values[idx])
   }
   ```

3. **Add convenience wrapper**:
   ```rust
   /// Checked access with default on error
   fn long_value_or_default(&self, node_id: u64, default: i64) -> i64 {
       self.long_value(node_id).unwrap_or(default)
   }
   ```

### Phase 3: Update Callers (1-2 days)

**Pregel context** (high priority):

```rust
// src/pregel/context/node_centric_context.rs
pub(crate) fn double_node_value(&self, key: &str) -> f64 {
    let node_value = self.node_value.read();
    // Before: node_value.double_value(key, self.node_id as usize)
    // After:
    form_processor::checked_u64_to_usize(self.node_id)
        .ok()
        .and_then(|idx| node_value.double_value(key, idx).ok())
        .unwrap_or(0.0)  // Or return Result
}
```

**PropertyStore initialization** (medium priority):

```rust
// src/pregel/executor.rs - initialize_from_property_store()
for node_id in 0..graph.node_count() {
    // Before: direct usize conversion
    // After:
    let idx = form_processor::checked_u64_to_usize(node_id)?;
    if let Some(value) = DefaultValue::from_property(&*props, node_id) {
        // ...
    }
}
```

### Phase 4: Tests (1 hour)

**Add overflow protection tests**:

```rust
// tests/form_processor_overflow.rs
use rust_gds::projection::form_processor::*;

#[test]
fn test_checked_conversion_succeeds_within_range() {
    let id: u64 = 1000;
    let result = checked_u64_to_usize(id);
    assert_eq!(result.unwrap(), 1000);
}

#[test]
fn test_checked_conversion_fails_on_overflow() {
    // This would overflow on 32-bit targets
    let huge_id: u64 = (u32::MAX as u64) + 1;
    let result = checked_u64_to_usize(huge_id);

    // On 64-bit: succeeds
    // On 32-bit: fails with IndexOverflow error
    if cfg!(target_pointer_width = "32") {
        assert!(result.is_err());
    } else {
        assert!(result.is_ok());
    }
}

#[test]
fn test_property_values_safe_access() {
    let values = DefaultLongNodePropertyValues::new(vec![100, 200, 300], 3);

    // Safe access within range
    assert_eq!(values.long_value(0).unwrap(), 100);
    assert_eq!(values.long_value(2).unwrap(), 300);

    // Safe failure on overflow (simulated)
    // Real test would require 32-bit target
}
```

### Phase 5: Documentation (30 min)

**Update inline docs**:

```rust
/// # Safety Notes
///
/// This method uses `checked_u64_to_usize()` to ensure safe indexing across
/// all platforms (32-bit and 64-bit). Returns `FormProcessorError::IndexOverflow`
/// if node_id exceeds `usize::MAX` on the current platform.
///
/// For hot-path code where node_id is guaranteed valid (e.g., after validation
/// in a loop), use `long_value_unchecked()` with appropriate assertions.
```

**Update ADR**:

- Add migration completion date to ADR 0006
- Document performance impact (if any)

---

## 🎯 Priority Order

### Week 1 (Immediate)

1. **Audit** all unsafe casts (1 hour)

   - Create tracking issue with file list
   - Estimate effort per file

2. **Add safe methods** to NodePropertyValues trait (2 hours)

   - Update trait definition
   - Add default implementations

3. **Migrate Pregel context** (3 hours)
   - Highest risk (user-facing compute)
   - Most visible (example demonstrates it)

### Week 2 (Follow-up)

4. **Migrate PropertyStore implementations** (1 day)

   - Update all DefaultXxxNodePropertyValues impls
   - Add tests per implementation

5. **Migrate Graph/IdMap** (2 hours)

   - Update DefaultGraph degree methods
   - Update SimpleIdMap

6. **Add comprehensive tests** (3 hours)
   - Overflow protection
   - Error handling
   - Edge cases (MAX values, zero, etc.)

### Week 3 (Polish)

7. **Performance validation** (2 hours)

   - Benchmark checked vs unchecked
   - Profile hot paths
   - Optimize if needed

8. **Documentation pass** (1 hour)
   - Update API docs
   - Add examples to ADR 0006
   - Create migration guide

---

## 🔍 Verification Steps

### After Migration

1. **No unsafe casts remain**:

   ```bash
   # Should return 0 matches
   git grep "node_id as usize" src/types/properties/
   git grep "node_id as usize" src/pregel/
   ```

2. **All tests pass**:

   ```bash
   cargo test
   cargo test --test form_processor_overflow
   ```

3. **Example still works**:

   ```bash
   cargo run --example pregel_propertystore_integration
   # Should show 100.0 → 1600.0 values loading
   ```

4. **Clippy happy**:

   ```bash
   cargo clippy --all-targets
   # No warnings about as conversions
   ```

5. **32-bit simulation** (optional):
   ```bash
   # Cross-compile to 32-bit target
   rustup target add i686-unknown-linux-gnu
   cargo build --target i686-unknown-linux-gnu
   cargo test --target i686-unknown-linux-gnu
   ```

---

## 📊 Impact Assessment

### Benefits

✅ **Portability**: Works correctly on 32-bit ARM, WASM32  
✅ **Safety**: Fail-fast on overflow instead of silent corruption  
✅ **Auditability**: All conversions in one place (form_processor)  
✅ **GDSL Ready**: Safe boundary for TypeScript → Rust calls  
✅ **Documentation**: Clear policy for future contributors

### Costs

⚠️ **Performance**: Checked conversion adds branch (mitigated by keeping unchecked for hot paths)  
⚠️ **API Changes**: Methods return `Result` instead of bare values (breaking change)  
⚠️ **Migration Effort**: 30+ files to update (~2-3 days)

### Risk Mitigation

- Keep unchecked methods for validated hot paths
- Add inline `#[inline]` to checked_u64_to_usize
- Use `debug_assert!` in unchecked methods (catch in testing)
- Phased rollout (Pregel first, then PropertyStore)

---

## 🤝 Related Work

### PropertyProjection Trait

**Relationship**: Complementary, not conflicting

- **PropertyProjection**: Handles type conversion (Long → Double, Array → Scalar)
- **Form Processor**: Handles boundary safety (u64 → usize, overflow protection)
- **Both used together**: PropertyProjection calls form_processor for safe indexing

**Example**:

```rust
impl PropertyProjection for DefaultValue {
    fn from_property(props: &dyn NodePropertyValues, node_id: u64) -> Option<Self> {
        // Form Processor ensures safe indexing
        let idx = form_processor::checked_u64_to_usize(node_id).ok()?;

        // PropertyProjection handles type conversion
        match props.value_type() {
            ValueType::Long => Some(DefaultValue::Long(props.long_value_unchecked(node_id))),
            ValueType::Double => Some(DefaultValue::Double(props.double_value_unchecked(node_id))),
            // ...
        }
    }
}
```

---

## 📚 Key References

1. **EVAL_MACRO_STRATEGIC_ROLE.md**: Complete strategic documentation
2. **ADR 0006**: Architectural decision (projection as GDSL)
3. **form_processor.rs**: Implementation and helpers
4. **QUALITY_CONTROL_ROADMAP.md**: Section 3 (Critical priority)

---

## ✅ Done Criteria

**Phase 1 Complete** when:

- [ ] All unsafe casts audited and documented
- [ ] Tracking issue created with file list
- [ ] Safe methods added to NodePropertyValues trait

**Phase 2 Complete** when:

- [ ] Pregel context migrated (context/\*.rs)
- [ ] Example runs successfully with checked conversions
- [ ] Tests added for overflow protection

**Phase 3 Complete** when:

- [ ] All PropertyValues impls migrated
- [ ] All Graph/IdMap impls migrated
- [ ] Zero unsafe casts remain in public API

**Migration Complete** when:

- [ ] All tests passing (including new overflow tests)
- [ ] Clippy warnings addressed
- [ ] Documentation updated
- [ ] Performance validated (no regression)
- [ ] 32-bit target tested (if possible)

---

**Bottom Line**: This is **not optional**. The current code has **silent data corruption risk** on 32-bit targets. The fix is straightforward (use form_processor helpers), well-designed (already implemented), and testable. Priority: **HIGH** for correctness and portability.

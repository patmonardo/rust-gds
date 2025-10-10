# The PropertyValues Storage Problem

**Date**: October 10, 2025  
**Issue**: PropertyValues currently use Vec<T>, not HugeArrays  
**Impact**: Cannot handle billions of properties efficiently

---

## Current State (The Problem)

### How Properties Are Stored Now

```rust
// src/types/properties/node/impls/values/long.rs
pub struct DefaultLongNodePropertyValues {
    values: Vec<i64>,        // ← Problem: Vec limited to isize::MAX
    node_count: usize,
}

// src/types/properties/node/impls/values/double.rs
pub struct DefaultDoubleNodePropertyValues {
    values: Vec<f64>,        // ← Same problem
    node_count: usize,
}
```

**Limitations**:

- ❌ Vec limited to ~2 billion elements (isize::MAX on 64-bit)
- ❌ Cannot handle graphs with billions of nodes
- ❌ No automatic paging
- ❌ Cannot memory-map or use cursor iteration

### Java GDS Solution

**Java GDS ALWAYS uses HugeArrays**:

```java
// org.neo4j.gds.core.loading.ArrayIdMap
public class ArrayIdMap implements IdMap {
    private final HugeLongArray graphIds;  // ← Always HugeArray!
    private final long nodeCount;
}

// org.neo4j.gds.core.loading.nodeproperties.DoubleNodePropertiesBuilder
public class DoubleNodePropertiesBuilder {
    private final HugeDoubleArray values;  // ← Always HugeArray!
}
```

**Key insight**: Java GDS **never** uses plain arrays for graph data. HugeArrays are **mandatory**, not optional.

---

## The Architectural Issue

### Why Vec Is Wrong

```rust
// This WILL fail for large graphs:
let node_count = 10_000_000_000;  // 10 billion nodes
let values = vec![0i64; node_count];  // ❌ Panic! Vec too large
```

### Why We Need HugeArrays

```rust
// This WILL work:
let node_count = 10_000_000_000;  // 10 billion nodes
let values = HugeLongArray::new(node_count);  // ✅ Automatic paging
```

**Benefits of HugeArrays**:

1. ✅ Support billions of elements (paged storage)
2. ✅ Cursor-based iteration (zero-copy)
3. ✅ Memory-efficient (can use mmap)
4. ✅ Consistent with Java GDS architecture
5. ✅ Future: Can parallelize operations over pages

---

## The Solution: Make HugeArrays Mandatory

### Refactor PropertyValues to Use HugeArrays

**Before (current)**:

```rust
pub struct DefaultLongNodePropertyValues {
    values: Vec<i64>,          // ← Plain Vec
    node_count: usize,
}

impl DefaultLongNodePropertyValues {
    pub fn new(values: Vec<i64>, node_count: usize) -> Self {
        Self { values, node_count }
    }

    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.values[node_id as usize]  // Direct Vec indexing
    }
}
```

**After (proposed)**:

```rust
use crate::collections::HugeLongArray;

pub struct DefaultLongNodePropertyValues {
    values: HugeLongArray,     // ← HugeArray!
    node_count: usize,
}

impl DefaultLongNodePropertyValues {
    pub fn new(values: HugeLongArray, node_count: usize) -> Self {
        Self { values, node_count }
    }

    // Convenience constructor from Vec (for small graphs)
    pub fn from_vec(values: Vec<i64>, node_count: usize) -> Self {
        Self {
            values: HugeLongArray::from_vec(values),
            node_count,
        }
    }

    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.values.get(node_id as usize)  // HugeArray indexing
    }
}
```

---

## Random Generation Impact

### Current Random Generation (Inefficient)

```rust
// Generate random properties
let mut values = Vec::with_capacity(node_count);  // ❌ Won't work for billions
for i in 0..node_count {
    values.push(rng.gen_range(0..100));
}
let property_values = DefaultLongNodePropertyValues::new(values, node_count);
```

**Problems**:

1. ❌ Vec allocation fails for large graphs
2. ❌ Sequential generation (slow)
3. ❌ Cannot parallelize
4. ❌ High memory pressure

### Proposed Random Generation (Efficient)

```rust
// Generate random properties directly into HugeArray
let values = HugeLongArray::random_uniform(node_count, 0, 100)
    .seed(42)
    .parallel()  // ← Fill pages in parallel!
    .build();

let property_values = DefaultLongNodePropertyValues::new(values, node_count);
```

**Benefits**:

1. ✅ Works for billions of nodes
2. ✅ Parallel generation across pages
3. ✅ No intermediate Vec allocation
4. ✅ Integrates with paging from the start

---

## Implementation Plan

### Phase 1: Refactor Existing PropertyValues (Priority)

**Files to modify**:

1. `src/types/properties/node/impls/values/long.rs`
2. `src/types/properties/node/impls/values/double.rs`
3. `src/types/properties/node/impls/values/long_array.rs`
4. `src/types/properties/node/impls/values/double_array.rs`
5. `src/types/properties/node/impls/values/float_array.rs`

**Changes**:

```rust
// OLD:
pub struct DefaultLongNodePropertyValues {
    values: Vec<i64>,
    node_count: usize,
}

// NEW:
use crate::collections::HugeLongArray;

pub struct DefaultLongNodePropertyValues {
    values: HugeLongArray,
    node_count: usize,
}

impl DefaultLongNodePropertyValues {
    // Primary constructor
    pub fn new(values: HugeLongArray, node_count: usize) -> Self {
        Self { values, node_count }
    }

    // Convenience for small graphs / tests
    pub fn from_vec(values: Vec<i64>, node_count: usize) -> Self {
        Self::new(HugeLongArray::from_vec(values), node_count)
    }
}
```

**Impact**:

- Tests need updating (use `from_vec()` for small test data)
- API stays mostly compatible (add `from_vec()` convenience)
- Performance improves for large graphs

---

### Phase 2: Add RandomFillable to HugeArrays

**Once PropertyValues use HugeArrays**, random generation becomes trivial:

```rust
// In RandomGraphConfig implementation:
fn generate_node_properties<R: Rng>(
    config: &RandomGraphConfig,
    rng: &mut R,
) -> HashMap<String, Arc<dyn NodePropertyValues>> {

    let mut properties = HashMap::new();

    for prop_config in &config.node_properties {
        match prop_config.value_type {
            ValueType::Long => {
                // Generate random HugeLongArray directly
                let values = HugeLongArray::fill_with_rng(
                    config.node_count,
                    |rng| rng.gen_range(0..100)  // Custom distribution
                );

                let property_values = DefaultLongNodePropertyValues::new(
                    values,
                    config.node_count
                );

                properties.insert(
                    prop_config.name.clone(),
                    Arc::new(property_values) as Arc<dyn NodePropertyValues>
                );
            }
            ValueType::Double => {
                // Same pattern for Double
                let values = HugeDoubleArray::fill_with_rng(
                    config.node_count,
                    |rng| rng.gen_range(0.0..1.0)
                );

                let property_values = DefaultDoubleNodePropertyValues::new(
                    values,
                    config.node_count
                );

                properties.insert(
                    prop_config.name.clone(),
                    Arc::new(property_values) as Arc<dyn NodePropertyValues>
                );
            }
            _ => unimplemented!("Type not yet supported"),
        }
    }

    properties
}
```

---

### Phase 3: Add Distribution Support

**Once basic random generation works**, add distributions:

```rust
impl HugeLongArray {
    /// Fill with uniform random values
    pub fn random_uniform(size: usize, min: i64, max: i64) -> Self {
        Self::random_uniform_seeded(size, min, max, None)
    }

    pub fn random_uniform_seeded(
        size: usize,
        min: i64,
        max: i64,
        seed: Option<u64>
    ) -> Self {
        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };

        Self::fill_with_rng(size, |rng| rng.gen_range(min..=max))
    }

    /// Fill with normal distribution (convert to i64)
    pub fn random_normal(size: usize, mean: f64, stddev: f64) -> Self {
        use rand_distr::Normal;
        let dist = Normal::new(mean, stddev).unwrap();

        Self::fill_with_dist(size, dist)
            .map(|f| f.round() as i64)
    }

    /// Fill with power-law distribution
    pub fn random_power_law(size: usize, exponent: f64) -> Self {
        use rand_distr::Exp;
        // Power-law via exponential transformation
        let dist = Exp::new(exponent).unwrap();

        Self::fill_with_dist(size, dist)
            .map(|f| f.round() as i64)
    }
}
```

---

## Breaking Changes and Migration

### API Compatibility Strategy

**Option A: Hard break (clean but disruptive)**

```rust
// Remove old constructor entirely
impl DefaultLongNodePropertyValues {
    // OLD: pub fn new(values: Vec<i64>, node_count: usize) -> Self
    // NEW:
    pub fn new(values: HugeLongArray, node_count: usize) -> Self { ... }
}
```

**Option B: Gradual migration (safer)**

```rust
impl DefaultLongNodePropertyValues {
    // Primary constructor (HugeArray)
    pub fn new(values: HugeLongArray, node_count: usize) -> Self { ... }

    // Deprecated convenience (Vec)
    #[deprecated(since = "0.2.0", note = "Use from_vec() instead")]
    pub fn from_values(values: Vec<i64>, node_count: usize) -> Self {
        Self::from_vec(values, node_count)
    }

    // Recommended for small graphs
    pub fn from_vec(values: Vec<i64>, node_count: usize) -> Self {
        Self::new(HugeLongArray::from_vec(values), node_count)
    }
}
```

**Recommendation**: **Option B** (gradual migration)

- Less disruptive to existing code
- Tests can use `from_vec()` for small data
- Production code uses `new()` with HugeArrays
- Clear upgrade path

---

## Testing Strategy

### Test Data Generation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Small tests use from_vec (convenience)
    #[test]
    fn test_small_graph() {
        let values = DefaultLongNodePropertyValues::from_vec(
            vec![1, 2, 3, 4, 5],
            5
        );
        assert_eq!(values.long_value(0).unwrap(), 1);
    }

    // Large tests use HugeArray directly
    #[test]
    fn test_large_graph() {
        let huge = HugeLongArray::random_uniform(1_000_000, 0, 100);
        let values = DefaultLongNodePropertyValues::new(huge, 1_000_000);
        assert_eq!(values.node_count(), 1_000_000);
    }

    // Deterministic tests use seeded generation
    #[test]
    fn test_deterministic_generation() {
        let huge = HugeLongArray::random_uniform_seeded(10_000, 0, 100, Some(42));
        let values = DefaultLongNodePropertyValues::new(huge, 10_000);

        // Reproducible results
        assert_eq!(values.long_value(0).unwrap(), /* expected from seed 42 */);
    }
}
```

---

## Decision Points

### 1. Do we refactor PropertyValues now or defer?

**Recommendation**: **Refactor now**

**Rationale**:

- This is a **foundational architectural issue**
- The longer we wait, the more code depends on Vec
- Random generation **requires** HugeArrays to work properly
- Java GDS proves this is the right architecture

### 2. Breaking change or gradual migration?

**Recommendation**: **Gradual migration** (Option B above)

**Rationale**:

- Less disruptive to existing tests/examples
- Clear upgrade path for users
- Can deprecate old APIs in next release

### 3. Should HugeArrays support random generation or separate trait?

**Recommendation**: **Integrate into HugeArray** (methods on the type)

**Rationale**:

- Arrays know their paging structure
- Can optimize generation per page
- Simpler API (no extra trait to import)
- Matches builder pattern we use elsewhere

---

## Action Items (This Morning)

### Immediate (Priority 1):

1. ✅ Document the issue (this file)
2. ⏳ **Refactor DefaultLongNodePropertyValues to use HugeLongArray**
3. ⏳ **Refactor DefaultDoubleNodePropertyValues to use HugeDoubleArray**
4. ⏳ **Update all tests to use from_vec() for small data**

### Short-term (Priority 2):

5. ⏳ Add `fill_with_rng()` method to HugeLongArray
6. ⏳ Add `fill_with_rng()` method to HugeDoubleArray
7. ⏳ Update RandomGraphConfig to generate properties
8. ⏳ Test with billion-node random graphs

### Medium-term (Priority 3):

9. Add distribution support (uniform, normal, power-law)
10. Add parallel generation (Rayon over pages)
11. Refactor array property values (LongArray, DoubleArray, FloatArray)
12. Consider Arrow2 integration for columnar storage

---

## The Core Recognition

**You were right**: Using Vec for PropertyValues is **wrong**.

**Java GDS is right**: HugeArrays should be **mandatory**, not optional.

**The issue**: We ported the **API surface** from Java GDS but used Rust's **Vec** thinking it was "simpler."

**The fix**: Use HugeArrays everywhere, like Java GDS does. Add convenience methods (`from_vec()`) for small graphs/tests, but the primary API uses HugeArrays.

**Random generation forces the issue**: You can't generate random properties for billion-node graphs with Vec. HugeArrays are **necessary**.

---

**Ready to refactor?** Should we:

- A. Start with DefaultLongNodePropertyValues refactor
- B. Add random generation to HugeArrays first
- C. Do both in parallel

My vote: **Option A** (refactor PropertyValues first, then add random generation)

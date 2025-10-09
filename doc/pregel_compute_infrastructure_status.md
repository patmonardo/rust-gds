# Pregel Compute Infrastructure Status

**Date**: 2025-10-09  
**Status**: ✅ Partition COMPLETE | ⚠️ ComputeStep BLOCKED | ✅ Element COMPLETE

## Summary

Implemented `Partition` work batch representation for Pregel parallel execution. Identified fundamental design challenge with `ComputeStep` trait translation from Java to Rust due to type erasure differences. `Element` was already fully implemented.

---

## ✅ COMPLETE: Partition

**File**: `src/pregel/partition.rs` (182 lines)  
**Tests**: 6/6 passing ✅  
**Total test count**: 831 tests passing

### Purpose

Represents a contiguous batch of node IDs to be processed by a single worker thread in Pregel computation. Essential for work distribution across parallel workers.

### API

```rust
pub struct Partition {
    start_node: u64,
    node_count: usize,
}

impl Partition {
    /// Create a new partition with the given start node and count
    pub fn new(start_node: u64, node_count: usize) -> Self

    /// Get the first node ID in this partition
    pub fn start_node(&self) -> u64

    /// Get the number of nodes in this partition
    pub fn node_count(&self) -> usize

    /// Get the range of node IDs in this partition [start, end)
    pub fn range(&self) -> Range<u64>

    /// Execute a function for each node in this partition
    pub fn consume<F>(&self, f: F) where F: FnMut(u64)
}

// Conversions from ranges
impl From<Range<u64>> for Partition
impl From<Range<usize>> for Partition
```

### Key Pattern: `consume` method

```rust
let partition = Partition::new(0, 1000);

partition.consume(|node_id| {
    // Process each node in the partition
    println!("Processing node {}", node_id);
});
```

This is the core iteration pattern used by ComputeStep to process batches.

### Tests

1. ✅ `test_new_partition` - Constructor and getters
2. ✅ `test_partition_range` - Range generation
3. ✅ `test_consume` - Iteration over nodes
4. ✅ `test_consume_empty` - Empty partition handling
5. ✅ `test_from_range_u64` - Conversion from u64 range
6. ✅ `test_from_range_usize` - Conversion from usize range

### Integration

- **Module**: Added to `src/pregel/mod.rs`
- **Export**: `pub use partition::Partition;`
- **Usage**: Will be used by parallel executor to distribute work across threads

### Example Usage (from executor perspective)

```rust
use rust_gds::pregel::Partition;
use rust_gds::concurrency::Concurrency;

let node_count = 10_000;
let concurrency = Concurrency::of(4);
let batch_size = node_count / concurrency.value();

// Create partitions for parallel processing
let partitions: Vec<Partition> = (0..concurrency.value())
    .map(|i| {
        let start = i * batch_size;
        let count = if i == concurrency.value() - 1 {
            node_count - start  // Last partition gets remainder
        } else {
            batch_size
        };
        Partition::new(start as u64, count)
    })
    .collect();

// Process partitions in parallel (simplified)
partitions.into_par_iter().for_each(|partition| {
    partition.consume(|node_id| {
        // ComputeStep processes this node
    });
});
```

---

## ✅ COMPLETE: Element

**File**: `src/pregel/schema.rs` (already implemented)  
**Tests**: 5/5 passing ✅

### Purpose

Represents a schema element (property definition) in the Pregel computation framework. Defines which properties are stored for each node and their types.

### API

```rust
pub struct Element {
    pub property_key: String,
    pub property_type: ValueType,
    pub visibility: Visibility,
    pub default_value: Option<DefaultValue>,
}

impl Element {
    /// Create a new element with the given key, type and visibility
    pub fn new(
        property_key: impl Into<String>,
        property_type: ValueType,
        visibility: Visibility,
    ) -> Self

    /// Create a new element with a default value
    pub fn with_default(
        property_key: impl Into<String>,
        default_value: DefaultValue,
        visibility: Visibility,
    ) -> Self
}

pub enum Visibility {
    Public,   // Accessible from outside
    Private,  // Internal use only
}

pub enum DefaultValue {
    Long(i64),
    Double(f64),
    LongArray(Vec<i64>),
    DoubleArray(Vec<f64>),
}
```

### Example Usage

```rust
use rust_gds::pregel::{Element, Visibility, DefaultValue};
use rust_gds::types::ValueType;

// Simple property
let rank_element = Element::new("rank", ValueType::Double, Visibility::Public);

// Property with default value
let temp_element = Element::with_default(
    "temp",
    DefaultValue::Long(0),
    Visibility::Private
);

// Used in schema builder
let schema = PregelSchema::builder()
    .add("rank", ValueType::Double, Visibility::Public)
    .add("temp", ValueType::Long, Visibility::Private)
    .build();
```

### Tests

1. ✅ `test_schema_builder` - Schema construction
2. ✅ `test_properties_map` - Property key mapping
3. ✅ `test_property_type` - Type lookup
4. ✅ `test_visibility` - Visibility handling
5. ✅ `test_element_with_default` - Default values

### Integration

- **Module**: Already in `src/pregel/schema.rs`
- **Export**: `pub use schema::Element;`
- **Usage**: Used by PregelSchema to define node property layout

---

## ⚠️ BLOCKED: ComputeStep

**Status**: Design challenge identified - requires architectural decision

### The Problem: Type Erasure Mismatch

Java GDS's `ComputeStep` interface uses generics that are erased at runtime:

```java
public interface ComputeStep<
    CONFIG extends PregelConfig,
    ITERATOR extends Messages.MessageIterator,
    INIT_CONTEXT extends InitContext<CONFIG>,
    COMPUTE_CONTEXT extends ComputeContext<CONFIG>
> {
    HugeAtomicBitSet voteBits();
    InitFunction<CONFIG, INIT_CONTEXT> initFunction();
    ComputeFunction<CONFIG, COMPUTE_CONTEXT> computeFunction();
    // ... other methods

    default void computeBatch() {
        // Can call generic methods dynamically
    }
}
```

**Rust doesn't support this pattern** because:

1. **No trait objects with generic methods**: `&dyn Trait<T>` cannot have methods with generic type parameters
2. **No type erasure**: Rust needs to know all types at compile time for monomorphization
3. **No dynamic dispatch on generic parameters**: Can't call `fn foo<T>(&self)` through a trait object

### Attempted Solutions (All Failed)

#### Attempt 1: Direct trait translation

```rust
pub trait ComputeStep<CONFIG, ITER, INIT_CTX, COMPUTE_CTX> {
    fn compute_batch(&mut self);  // ❌ Can't store this as `Box<dyn ComputeStep<...>>`
}
```

**Issue**: Would need to know all 4 type parameters at every call site.

#### Attempt 2: Generic methods in trait

```rust
pub trait ComputeStep {
    fn init_context<C: PregelConfig>(&mut self) -> &mut InitContext<C>;
    // ❌ Not dyn-compatible
}
```

**Issue**: Rust error: "method has generic type parameters... not dyn compatible"

#### Attempt 3: Associated types

```rust
pub trait ComputeStep {
    type Config: PregelConfig;
    type Iterator: MessageIterator;
    fn init_context(&mut self) -> &mut InitContext<Self::Config>;
}
```

**Issue**: Still can't store heterogeneous `Box<dyn ComputeStep<Config=A>>` and `Box<dyn ComputeStep<Config=B>>` in same collection.

### Root Cause

Java's approach works because:

- Type parameters exist only at compile time
- Runtime uses raw types with casts
- Virtual dispatch doesn't need to know generic types

Rust's approach requires:

- All generic parameters known at compile time for monomorphization
- No type erasure - every `T` creates a new function
- Trait objects can't have generic methods

### Possible Solutions (Not Implemented Yet)

#### Option A: Concrete Types (Simplest)

Remove generics entirely, use concrete types:

```rust
pub trait ComputeStep {
    fn vote_bits(&self) -> &HugeAtomicBitSet;
    fn init_context(&mut self) -> &mut InitContext;  // No generic
    fn compute_context(&mut self) -> &mut ComputeContext;  // No generic
    fn compute_batch(&mut self);
}
```

**Pros**: Works with trait objects, simple  
**Cons**: Loses type safety, InitContext/ComputeContext can't be generic

#### Option B: Enum Dispatch

Use an enum to represent all possible Config types:

```rust
pub enum AnyConfig {
    PageRank(PageRankConfig),
    SSSP(SSSPConfig),
    // ... all algorithm configs
}

pub trait ComputeStep {
    fn config(&self) -> &AnyConfig;
    fn compute_batch(&mut self);
}
```

**Pros**: Type-safe, works with trait objects  
**Cons**: Closed set of algorithms, verbose matching

#### Option C: Macro-Generated Concrete Implementations

Generate a concrete `ComputeStep` implementation for each algorithm:

```rust
macro_rules! impl_compute_step {
    ($config:ty, $iter:ty) => {
        pub struct ConcreteComputeStep<$config, $iter> {
            // fields
        }

        impl ComputeStep for ConcreteComputeStep<$config, $iter> {
            // concrete implementation
        }
    };
}
```

**Pros**: Type-safe, flexible  
**Cons**: More boilerplate, still can't mix in collections

#### Option D: Redesign Around Closures

Instead of trait, use function pointers or closures:

```rust
pub struct ComputeStep {
    vote_bits: HugeAtomicBitSet,
    compute_fn: Box<dyn Fn(&mut Context, &Messages)>,
    // ...
}
```

**Pros**: Maximum flexibility  
**Cons**: Loses structure, harder to test/mock

### Recommendation

**For Now**: Skip `ComputeStep` trait and implement concrete executor directly.

**Reasons**:

1. The executor is the only consumer of ComputeStep
2. We can inline the logic directly in the executor
3. Avoids premature abstraction that Rust doesn't support well
4. Can refactor later if needed

**Next Steps**:

1. Implement Pregel executor with concrete types
2. Use PageRank as first algorithm to validate design
3. Once executor works, evaluate if abstraction is needed
4. If yes, choose Option A or B based on actual requirements

---

## Integration Status

### What's Working

- ✅ **Partition**: Full API, 6 tests passing
- ✅ **Element & PregelSchema**: Already complete, 5 tests passing
- ✅ **HugeAtomicBitSet**: 16 tests passing (vote tracking ready)
- ✅ **Message system**: Already implemented
- ✅ **Contexts**: Stub implementations exist

### What's Blocked

- ⚠️ **ComputeStep**: Architectural decision needed
- ⚠️ **Executor**: Waiting on ComputeStep resolution

### Total Test Count

**831 tests passing** (6 new Partition tests added)

---

## Next Actions

### Immediate (Don't Block On ComputeStep)

1. **Implement Pregel Executor** without ComputeStep abstraction

   - Use concrete types directly
   - Inline compute batch logic
   - Focus on BSP loop and synchronization

2. **Implement PageRank Algorithm**

   - First test of executor
   - Validates message passing
   - Validates vote-to-halt

3. **Context Wiring**
   - Fill out InitContext stubs
   - Fill out ComputeContext stubs
   - Connect to NodeValue and Graph

### Future (After Executor Works)

4. **Evaluate ComputeStep Need**

   - Is abstraction actually needed?
   - What's the real use case?
   - Choose appropriate solution (A, B, C, or D)

5. **Refactor if Beneficial**
   - Only if multiple algorithms need the abstraction
   - Only if the complexity pays off

---

## Files Modified/Created

### Created

- `src/pregel/partition.rs` (182 lines, 6 tests)

### Modified

- `src/pregel/mod.rs` - Added partition module and export

### Attempted (Not Committed)

- `src/pregel/compute_step.rs` - Multiple attempts, all blocked by Rust's type system

---

## Design Lessons

1. **Java's type erasure ≠ Rust's generics**
   - Java can abstract over runtime-erased types
   - Rust requires compile-time knowledge of all types
2. **Trait objects have limits**
   - Can't have generic methods
   - Can't mix different associated type values
3. **When in doubt, go concrete**
   - Premature abstraction in Rust can be a trap
   - Concrete implementations are clearer and compile
4. **Rust favors composition over inheritance**
   - Instead of trait hierarchies, use function composition
   - Instead of virtual dispatch, use closures

---

## References

- **Java Source**: `org.neo4j.gds.beta.pregel.ComputeStep`
- **TypeScript Source**: `organon/gds/src/pregel/ComputeStep.ts`
- **Rust Partition**: `src/pregel/partition.rs`
- **Rust Element**: `src/pregel/schema.rs` (Element struct)
- **Design Discussion**: This document

---

## Conclusion

✅ **Partition is production-ready** - Clean API, well-tested, ready for executor  
✅ **Element is production-ready** - Already complete, part of PregelSchema  
⚠️ **ComputeStep is blocked** - Architectural decision needed, not critical path

**Recommendation**: **Skip ComputeStep abstraction and implement concrete executor directly.**

This unblocks Pregel development and lets us validate the design with a working algorithm before committing to an abstraction pattern.

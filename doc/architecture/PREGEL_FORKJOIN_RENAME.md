# Architectural Alignment: ComputeStep → ForkJoinComputeStep

## Issue Identified

The Rust implementation had **flattened** the Java/TypeScript architecture by naming the work-stealing task generically as `ComputeStep` instead of `ForkJoinComputeStep`.

## Architecture Comparison

### Java/TypeScript (Original)

```
PregelComputer (interface/trait)
    ↑
    └── ForkJoinComputer (concrete implementation)
            └── creates → ForkJoinComputeStep (work-stealing task)
```

### Rust (Before Fix)

```
PregelComputer (trait)
    ↑
    └── ForkJoinComputer (concrete implementation)
            └── creates → ComputeStep (generic name - doesn't indicate fork-join!)
```

### Rust (After Fix)

```
PregelComputer (trait)
    ↑
    └── ForkJoinComputer (concrete implementation)
            └── creates → ForkJoinComputeStep (matches Java/TS naming!)
```

## What Changed

### Files Modified

1. **`src/pregel/compute_step.rs`**

   - `struct ComputeStep` → `struct ForkJoinComputeStep`
   - Updated documentation to reference Java's `ForkJoinComputeStep` and `CountedCompleter`
   - Clarified that Rust uses Rayon's work-stealing instead of manual task tree management

2. **`src/pregel/computer.rs`**

   - Import: `ComputeStep` → `ForkJoinComputeStep`
   - Field type: `Option<ComputeStep<C, I>>` → `Option<ForkJoinComputeStep<C, I>>`
   - Constructor call: `ComputeStep::new()` → `ForkJoinComputeStep::new()`

3. **`src/pregel/mod.rs`**
   - Export: `pub use compute_step::{..., ComputeStep, ...}` → `pub use compute_step::{..., ForkJoinComputeStep, ...}`

## Why This Matters

### 1. **Architectural Clarity**

The name `ForkJoinComputeStep` clearly indicates:

- This is a **concrete implementation** (not an abstract interface)
- It uses **fork-join parallelism** (not sequential or other strategy)
- It's part of the **ForkJoinComputer** family

### 2. **Future Extensibility**

If we add other parallelism strategies (e.g., GPU, distributed), we'd have:

- `ForkJoinComputeStep` (current, using Rayon)
- `SequentialComputeStep` (single-threaded for debugging)
- `DistributedComputeStep` (for cluster execution)

A generic `ComputeStep` name would conflict with these.

### 3. **Cross-Language Consistency**

Java, TypeScript, and Rust now use the same names:

- `ForkJoinComputer`
- `ForkJoinComputeStep`

This makes the codebase easier to understand when comparing implementations.

## Implementation Differences (Rust vs Java/TS)

### Java/TypeScript Approach

```typescript
class ForkJoinComputeStep {
  private parent?: ForkJoinComputeStep;
  private pendingSubtasks = 0;

  compute() {
    if (shouldSubdivide) {
      const leftTask = new ForkJoinComputeStep(..., this); // Pass parent
      this.pendingSubtasks++;
      TaskPool.submit(leftTask);
      this.compute(); // Right half
    } else {
      this.computeBatch();
      this.tryComplete(); // Manual completion tracking
    }
  }

  onChildCompleted() {
    this.pendingSubtasks--;
    this.tryComplete();
  }
}
```

**Manual task tree management**: Explicit parent/child tracking, completion callbacks.

### Rust Approach (Current)

```rust
impl ForkJoinComputeStep {
    pub fn compute(mut self) {
        if self.node_batch.node_count() >= SEQUENTIAL_THRESHOLD {
            let (left_batch, right_batch) = self.split_batch();
            let left_step = ForkJoinComputeStep { /* ... left */ };
            self.node_batch = right_batch;

            rayon::join(
                || left_step.compute(),  // Left half
                || self.compute()         // Right half
            );
        } else {
            self.compute_batch(); // Base case
        }
    }
}
```

**Rayon's work-stealing**: No manual parent/child tracking - Rayon handles it.

### Trade-offs

| Aspect          | Java/TS Manual           | Rust Rayon                 |
| --------------- | ------------------------ | -------------------------- |
| **Complexity**  | Higher (manual tree)     | Lower (Rayon abstracts it) |
| **Control**     | Fine-grained             | Coarse-grained             |
| **Performance** | Tunable                  | Optimized by Rayon         |
| **Debugging**   | Harder (task tree state) | Easier (less state)        |
| **Portability** | Works anywhere           | Requires Rayon             |

**Conclusion**: Rust's approach is **simpler and idiomatic**. We get the same fork-join semantics without manually implementing task coordination.

## Status

✅ **Compiles successfully**
✅ **Names now match Java/TypeScript**
✅ **Architecture clearly expressed**

No breaking changes to public API (this is internal implementation).

## Next Steps

If we want to match Java/TS even more closely:

1. Add explicit parent/child tracking (probably unnecessary - Rayon handles it)
2. Add completion callbacks (useful for fine-grained monitoring)
3. Implement custom task pool (only needed for non-Rayon backends)

**Recommendation**: Keep current Rayon-based approach. It's cleaner and more Rust-idiomatic while preserving the same fork-join semantics.

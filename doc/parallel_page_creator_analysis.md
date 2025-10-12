# Parallel Page Creator Analysis

## Overview

Analysis of four TypeScript parallel page creator modules from Java GDS to understand parallelization patterns and translation strategy for Rust implementation using the existing `virtual_threads` infrastructure.

## Source Modules (TypeScript/Java)

### 1. ParallelBytePageCreator

- **Purpose:** Parallel allocation of `Uint8Array` pages
- **Use Cases:** Byte buffers, binary data, compressed storage, I/O buffers
- **Pattern:** Simple allocation without custom initialization (zero-filled)
- **Parallelization:** Uses `ParallelUtil.parallelStreamConsume`

### 2. ParallelDoublePageCreator (Float64)

- **Purpose:** Parallel allocation with **custom value generation**
- **Use Cases:** Edge weights, node embeddings, mathematical sequences, scientific data
- **Pattern:** Factory methods: `of(generator)`, `identity()`, `passThrough()`
- **Parallelization:** Parallel page creation + parallel value generation per page

### 3. ParallelIntPageCreator

- **Purpose:** Parallel allocation of integer arrays
- **Use Cases:** Graph properties, counters, index arrays, buffer pools
- **Pattern:** Simple allocation without custom initialization (zero-filled)
- **Parallelization:** Uses `ParallelUtil.parallelStreamConsume`

### 4. ParallelLongPageCreator

- **Purpose:** Parallel allocation with **custom value generation**
- **Use Cases:** Node IDs, index mappings, computed sequences, lookup tables
- **Pattern:** Factory methods: `of(generator)`, `identity()`, `passThrough()`
- **Parallelization:** Parallel page creation + parallel value generation per page

## Common Patterns Observed

### 1. Two-Phase Allocation Strategy

```typescript
// Phase 1: Allocate all pages except last in parallel
ParallelUtil.parallelStreamConsume(
  range(0, lastPageIndex),
  concurrency,
  termination,
  (pageIndex) => createPage(pages, pageIndex, pageSize)
);

// Phase 2: Handle last page separately (potentially partial)
createPage(pages, lastPageIndex, lastPageSize);
```

**Rationale:** Last page may have different size, so handle it sequentially to avoid edge cases.

### 2. Value Generation Patterns

#### A. Pass-Through (No Initialization)

```typescript
fillPage(page: T[], base: number): void {
  // NO-OP - pages allocated with default zeros
}
```

#### B. Identity Mapping

```typescript
generator = (i) => i; // array[i] = i
```

#### C. Custom Generator

```typescript
generator = (i) => Math.random() * 100;
generator = (i) => Math.sin(i * 0.001);
generator = (i) => fibonacci(i);
```

### 3. Base Index Calculation

```typescript
const base = pageIndex << pageShift; // Efficient: pageIndex * pageSize
```

Uses bit shifting for fast multiplication by power-of-2 page sizes.

## Rust Translation Strategy

### Current Infrastructure Available

#### 1. Concurrency Primitives

- ✅ `Concurrency` - Thread pool sizing
- ✅ `TerminationFlag` - Cancellation support
- ✅ `virtual_threads::Executor` - High-level parallel executor
- ✅ `virtual_threads::Scope` - Synchronization boundaries

#### 2. Parallel Execution Methods

```rust
// From src/concurrency/virtual_threads/scope.rs
scope.spawn_many(count, |i| { /* parallel task */ });
scope.spawn_range(start, end, |i| { /* parallel task */ });

// From src/concurrency/parallel_util/parallel_executor.rs
parallel_for_each_node(node_count, concurrency, termination, |node_id| { /* task */ });
read_parallel(start, end, concurrency, batch_size, termination, |range| { /* task */ });
```

#### 3. Existing Page Infrastructure

- ✅ `PageUtil` - Page size calculations
- ✅ `HugeArrays` - Page indexing constants
- ✅ Paged array implementations already allocate pages sequentially

### Proposed Rust Implementation

#### Option 1: Factory Methods on Existing Arrays

Add parallel initialization methods to existing `HugeLongArray`, `HugeDoubleArray`, etc.

```rust
// Simple parallel allocation (already exists via new())
let array = HugeLongArray::new(1_000_000_000);

// Parallel allocation with value generation
let array = HugeLongArray::with_generator(
    1_000_000_000,
    Concurrency::of(8),
    |index| index as i64  // identity mapping
);

// Parallel allocation with custom function
let weights = HugeDoubleArray::with_generator(
    edge_count,
    Concurrency::of(8),
    |i| rand::random::<f64>() * 100.0
);
```

#### Option 2: Separate PageCreator Trait System

Create dedicated page creator infrastructure (closer to Java GDS).

```rust
pub trait PageCreator<T> {
    fn fill(&self, pages: &mut [Vec<T>], last_page_size: usize, page_shift: u32);
    fn fill_page(&self, page: &mut [T], base: usize);
}

pub struct ParallelLongPageCreator {
    concurrency: Concurrency,
    generator: Option<Box<dyn Fn(usize) -> i64 + Send + Sync>>,
}

impl ParallelLongPageCreator {
    pub fn of<F>(concurrency: Concurrency, generator: F) -> Self
    where F: Fn(usize) -> i64 + Send + Sync + 'static
    {
        Self {
            concurrency,
            generator: Some(Box::new(generator)),
        }
    }

    pub fn identity(concurrency: Concurrency) -> Self {
        Self::of(concurrency, |i| i as i64)
    }

    pub fn pass_through(concurrency: Concurrency) -> Self {
        Self { concurrency, generator: None }
    }
}
```

### Parallelization Implementation

Using `virtual_threads::Scope`:

```rust
impl ParallelLongPageCreator {
    pub fn fill(&self, pages: &mut [Vec<i64>], last_page_size: usize, page_shift: u32) {
        let page_size = 1 << page_shift;
        let last_page_index = pages.len() - 1;
        let termination = TerminationFlag::running_true();

        let executor = Executor::new(self.concurrency);

        // Phase 1: Allocate all pages except last in parallel
        executor.scope(&termination, |scope| {
            scope.spawn_range(0, last_page_index, |page_index| {
                self.create_and_fill_page(
                    &mut pages[page_index],
                    page_index,
                    page_size,
                    page_shift
                );
            });
        });

        // Phase 2: Handle last page separately
        self.create_and_fill_page(
            &mut pages[last_page_index],
            last_page_index,
            last_page_size,
            page_shift
        );
    }

    fn create_and_fill_page(
        &self,
        page: &mut Vec<i64>,
        page_index: usize,
        page_size: usize,
        page_shift: u32
    ) {
        page.resize(page_size, 0);

        if let Some(ref gen) = self.generator {
            let base = page_index << page_shift;
            for i in 0..page_size {
                page[i] = gen(base + i);
            }
        }
    }
}
```

### Key Rust Considerations

#### 1. Mutable Access Challenge

**Problem:** Rust borrow checker prevents multiple mutable references to pages array.

**Solutions:**

- ✅ Use `split_at_mut()` to prove disjoint access
- ✅ Use `Arc<Mutex<Vec<Vec<T>>>>` for true concurrent writes
- ✅ Pre-allocate pages, then fill in parallel with only individual page mutability

```rust
// Pre-allocate structure
let mut pages: Vec<Vec<i64>> = (0..num_pages)
    .map(|i| {
        let size = if i == num_pages - 1 { last_page_size } else { page_size };
        Vec::with_capacity(size)
    })
    .collect();

// Now fill pages in parallel (each worker gets &mut to its own page)
pages.par_iter_mut().enumerate().for_each(|(page_index, page)| {
    // Safe: each thread has exclusive access to its page
    self.fill_page(page, page_index << page_shift);
});
```

#### 2. Generator Ownership

**Challenge:** Generators may not be `Clone`, need `Fn` (not `FnMut`) for parallel access.

**Solution:** Use `Arc<dyn Fn>` or require `Fn + Send + Sync`:

```rust
pub struct ParallelDoublePageCreator {
    concurrency: Concurrency,
    generator: Option<Arc<dyn Fn(usize) -> f64 + Send + Sync>>,
}
```

#### 3. Async vs Sync

**Decision:** Keep synchronous (no async/await) since:

- Rayon handles parallelism via work-stealing threads
- Page allocation is CPU-bound, not I/O-bound
- Simpler API matches Java GDS patterns

## Performance Characteristics

### Expected Speedup

For `N` pages on `C` cores:

- **Sequential:** `O(N * page_size)`
- **Parallel:** `O(N / C * page_size)` with near-linear speedup

### Cache Efficiency

Each worker fills entire page sequentially:

- ✅ Excellent spatial locality
- ✅ Minimal false sharing (each worker owns its page)
- ✅ Efficient memory bandwidth utilization

### Benchmarking Targets

1. **1 million elements** (61 pages @ 16K/page) - Baseline
2. **100 million elements** (6,104 pages) - Medium scale
3. **1 billion elements** (61,036 pages) - Large scale
4. **10 billion elements** (610,352 pages) - Massive scale

Compare:

- Sequential allocation vs parallel allocation
- Identity generation vs pass-through
- Different concurrency levels (1, 2, 4, 8, 16)

## Implementation Priority

### Phase 1: Add to Existing Arrays (Recommended)

**Why:** Simpler, idiomatic Rust, integrates with current codebase.

1. Add `::with_generator()` to `HugeLongArray`
2. Add `::with_generator()` to `HugeDoubleArray`
3. Implement using `rayon::par_iter_mut()` over pages
4. Write tests and benchmarks

**Files to modify:**

- `src/collections/huge_array/huge_long_array.rs`
- `src/collections/huge_array/huge_double_array.rs`

### Phase 2: Separate PageCreator Trait (Optional)

**Why:** Matches Java GDS architecture, more flexible for complex initialization patterns.

1. Create `src/core/utils/paged/page_creator.rs` with trait
2. Implement for Long, Double, Int, Byte types
3. Add factory methods (`of`, `identity`, `pass_through`)
4. Write comprehensive tests

**New files:**

- `src/core/utils/paged/page_creator.rs`
- `src/core/utils/paged/parallel_long_page_creator.rs`
- `src/core/utils/paged/parallel_double_page_creator.rs`
- `src/core/utils/paged/parallel_int_page_creator.rs`
- `src/core/utils/paged/parallel_byte_page_creator.rs`

## Use Case Examples

### 1. Graph Node IDs (Identity Mapping)

```rust
// Allocate 1 billion node IDs in parallel
let node_ids = HugeLongArray::with_generator(
    1_000_000_000,
    Concurrency::of(8),
    |i| i as i64
);
// Result: [0, 1, 2, ..., 999_999_999]
```

### 2. Random Edge Weights

```rust
use rand::thread_rng;
use rand::Rng;

let edge_weights = HugeDoubleArray::with_generator(
    edge_count,
    Concurrency::of(8),
    |_| thread_rng().gen_range(0.0..100.0)
);
```

### 3. Pre-computed Sequences

```rust
// Generate powers of 2
let powers = HugeLongArray::with_generator(
    64,
    Concurrency::of(4),
    |i| 1i64 << i  // 2^i
);
// Result: [1, 2, 4, 8, 16, 32, ...]
```

### 4. Byte Buffers for Compression

```rust
// Allocate 1GB byte buffer (no initialization needed)
let buffer_pages = vec![Vec::<u8>::new(); num_pages];
// Fill pages in parallel with zero-init
executor.scope(&termination, |scope| {
    scope.spawn_many(buffer_pages.len(), |i| {
        buffer_pages[i] = vec![0u8; page_size];
    });
});
```

## Testing Strategy

### Unit Tests

1. **Correctness:**

   - Identity mapping produces [0, 1, 2, ...]
   - Custom generator produces expected values
   - Last page has correct size (boundary condition)

2. **Concurrency:**

   - Results identical for sequential vs parallel
   - No data races (use Miri or loom)
   - Termination flag respected

3. **Edge Cases:**
   - Empty arrays
   - Single page arrays
   - Last page exactly fills
   - Last page is partial

### Integration Tests

1. Create huge arrays with parallel creators
2. Use in actual algorithms (PageRank, BFS)
3. Verify correctness matches sequential creation

### Performance Tests

```rust
#[bench]
fn bench_parallel_allocation_1b_elements(b: &mut Bencher) {
    b.iter(|| {
        let array = HugeLongArray::with_generator(
            1_000_000_000,
            Concurrency::of(8),
            |i| i as i64
        );
        black_box(array);
    });
}
```

## API Design Comparison

### Java GDS (Original)

```java
ParallelLongPageCreator creator = ParallelLongPageCreator.of(
    concurrency,
    i -> i  // identity
);

long[][] pages = new long[numPages][];
creator.fill(pages, lastPageSize, pageShift);
```

### TypeScript (Organon)

```typescript
const creator = ParallelLongPageCreator.identity(new Concurrency(8));
const pages: number[][] = new Array(numPages);

await creator.fill(pages, lastPageSize, pageShift);
```

### Rust (Proposed - Option 1: Integrated)

```rust
let array = HugeLongArray::with_generator(
    total_size,
    Concurrency::of(8),
    |i| i as i64
);
// Pages created and filled automatically
```

### Rust (Proposed - Option 2: Separate Creator)

```rust
let creator = ParallelLongPageCreator::identity(Concurrency::of(8));
let mut pages = vec![Vec::new(); num_pages];

creator.fill(&mut pages, last_page_size, page_shift);
```

## Recommendation

**Start with Option 1 (Integrated):**

1. Simpler for users
2. More idiomatic Rust
3. Avoids complex trait hierarchies
4. Easy to add later if needed

**Defer Option 2 until:**

- Need more complex initialization patterns
- Want exact Java GDS API parity
- Performance requires finer control

## Next Steps

1. ✅ **Analysis Complete** - This document
2. ⏳ **Prototype** - Add `::with_generator()` to `HugeLongArray`
3. ⏳ **Benchmark** - Measure sequential vs parallel speedup
4. ⏳ **Decide** - Commit to Option 1 or explore Option 2
5. ⏳ **Implement** - Full implementation across array types
6. ⏳ **Document** - Usage examples and performance characteristics

## Files for Reference

### Existing Infrastructure

- `src/concurrency/virtual_threads/executor.rs` - Parallel executor
- `src/concurrency/virtual_threads/scope.rs` - Synchronization scopes
- `src/concurrency/parallel_util/parallel_executor.rs` - Batch utilities
- `src/collections/huge_array/huge_long_array.rs` - Current paged implementation
- `src/mem/page_util.rs` - Page calculation utilities

### TypeScript Sources

- `ParallelBytePageCreator.ts` - Simple byte allocation
- `ParallelDoublePageCreator.ts` - Float64 with generators
- `ParallelIntPageCreator.ts` - Integer allocation
- `ParallelLongPageCreator.ts` - Long with generators

---

**Status:** Ready for prototyping phase.
**Priority:** Medium (performance optimization, not core functionality)
**Complexity:** Low-Medium (existing infrastructure simplifies implementation)

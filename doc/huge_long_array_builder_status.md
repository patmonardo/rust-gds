# HugeLongArrayBuilder Implementation - Status and Next Steps

## Summary

Implemented the foundational `HugeLongArrayBuilder` infrastructure for concurrent array construction with the following components:

### Completed

1. **HugeLongArrayBuilder** (Core Builder)

   - Location: `src/core/utils/paged/huge_long_array_builder.rs`
   - Atomic page array with acquire/release semantics
   - Thread-safe growth operations
   - Lock-free reads after allocation
   - Integration with `HugeLongArray::of()` method

2. **Architecture**

   - Atomic pointer to page array (`AtomicPtr<Vec<Vec<i64>>>`)
   - Fine-grained mutex locking for growth
   - Memory ordering guarantees (Acquire/Release/SeqCst)
   - Safe concurrent allocation from multiple threads

3. **API Surface**

   - `HugeLongArrayBuilder::new()` - Create builder
   - `allocate(start, length, allocator)` - Allocate range
   - `build(size)` - Construct final `HugeLongArray`

4. **HugeLongArray Extension**
   - Added `HugeLongArray::of(pages, size)` factory method
   - Handles both single-page and multi-page construction
   - Integrates with existing `SingleHugeLongArray` and `PagedHugeLongArray`

### Current Issue

The `Allocator` struct has an architectural challenge:

**Problem**: The allocator works on a cloned copy of pages, not the builder's original pages. When data is written via the allocator, the builder's pages remain unchanged.

**Root Cause**:

```rust
// In HugeLongArrayBuilder::allocate()
let pages = self.get_pages_acquire();  // Clone of pages
allocator.reset(start, end, pages);     // Allocator gets clone

// In Allocator::insert()
self.pages[i][j] = value;  // Modifies clone, not builder's pages!
```

**Why This Happens**:

- Rust's ownership rules prevent shared mutable access
- Builder owns pages; allocator needs mutable access
- Can't have multiple mutable references to same data

### Architectural Solutions

#### Option 1: Interior Mutability with UnsafeCell (Java-like)

Pros:

- Closest to Java implementation
- Zero runtime overhead
- True concurrent access

Cons:

- Requires unsafe code
- Complex lifetime management
- Easy to introduce bugs

```rust
struct HugeLongArrayBuilder {
    pages: AtomicPtr<Vec<UnsafeCell<Vec<i64>>>>,
    // ...
}
```

#### Option 2: RefCell/Mutex Interior Mutability

Pros:

- Safe Rust
- Runtime borrow checking

Cons:

- Runtime overhead
- RefCell not thread-safe
- Mutex adds contention

```rust
struct HugeLongArrayBuilder {
    pages: Arc<Mutex<Vec<Vec<i64>>>>,
    // ...
}
```

#### Option 3: Merge Allocator into Builder

Pros:

- Simple, safe
- No lifetime issues
- Clear ownership

Cons:

- Different API from Java/TS
- Less flexible

```rust
impl HugeLongArrayBuilder {
    pub fn write_range(&mut self, start: usize, data: &[i64]) {
        // Direct writing to builder's pages
    }
}
```

#### Option 4: Arena/Bump Allocator Pattern

Pros:

- Safe, idiomatic Rust
- Good performance
- Clean separation

Cons:

- Significant rearchitecture
- Different from reference implementation

### Recommendation

**For production use**: Implement Option 3 (merge allocator into builder) first, then optionally add Option 1 (UnsafeCell) for maximum performance if benchmarks show it's needed.

**Rationale**:

1. Option 3 is safe, simple, and immediately useful
2. API can evolve later to support separate Allocator
3. Matches Rust idioms better than forcing Java patterns
4. Most graph loading code doesn't need the allocator separation

### Proposed Safe API

```rust
impl HugeLongArrayBuilder {
    /// Allocates and fills a range in one operation (safe, simple)
    pub fn fill_range(&self, start: usize, data: &[i64]) {
        // Thread-safe direct writing to pages
        let _lock = self.lock.lock().unwrap();
        // ... write data directly to self.pages ...
    }

    /// For advanced use: get mutable page slice
    pub fn get_page_mut(&self, page_index: usize) -> MutexGuard<Vec<i64>> {
        // Caller manages writing within page
    }
}
```

### Files Modified

**New**:

- `src/core/utils/paged/huge_long_array_builder.rs` (700+ lines)

**Modified**:

- `src/core/utils/paged/mod.rs` - Added exports
- `src/collections/huge_array/huge_long_array.rs` - Added `of()` method

### Testing Status

**Compiles**: ✅ Yes  
**Tests Pass**: ❌ No (architectural issue with Allocator)

**Test Results**:

- 2 tests pass (builder_creation, allocator_properties)
- 7 tests fail (all data-writing tests fail due to clone issue)

### Next Steps

1. **Immediate** (to get working code):

   - Implement Option 3: `fill_range()` method on builder
   - Update tests to use simpler API
   - Verify concurrent usage patterns

2. **Follow-up** (for feature parity):

   - Implement Option 1 with UnsafeCell if needed
   - Add cursor-based batch writing
   - Benchmark different approaches

3. **Integration**:
   - Use in graph loading pipelines
   - Add to examples/
   - Document concurrency patterns

### Example Usage (Proposed Safe API)

```rust
use rust_gds::core::utils::paged::HugeLongArrayBuilder;
use std::sync::Arc;
use std::thread;

let builder = Arc::new(HugeLongArrayBuilder::new());

// Concurrent filling from multiple threads
let handles: Vec<_> = (0..4).map(|thread_id| {
    let builder_clone = Arc::clone(&builder);
    thread::spawn(move || {
        let start = thread_id * 100_000;
        let data: Vec<i64> = (start..start + 100_000).map(|i| i as i64).collect();

        // Simple, safe API
        builder_clone.fill_range(start, &data);
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}

let array = builder.build(400_000);
```

### Conclusion

The `HugeLongArrayBuilder` implementation demonstrates the core concurrent building infrastructure but requires architectural refinement to handle Rust's ownership model. The recommended path is to implement a simpler safe API first (Option 3), then optionally add the more complex UnsafeCell-based Allocator pattern if performance benchmarks justify it.

**Key Achievement**: Successfully implemented thread-safe page growth with atomic operations and proper memory ordering, which is the most complex part of the builder.

**Remaining Work**: Resolve the Allocator data mutation pattern to match Rust's ownership rules while maintaining the concurrent building capability.

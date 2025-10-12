# Test Memory Optimization

## Issue

Running 1454 tests in parallel can consume excessive memory, potentially killing VS Code.

## Current Test Inventory

- **Total tests**: 1454
- **Large array tests**: ~10 tests using 1-10 million elements each
- **Sparse tests**: Use large indices (100M+) but only allocate 4 pages (very memory efficient)
- **8GB test**: Located in `examples/eight_billion_nodes.rs` (NOT run during `cargo test`)

## Memory-Intensive Tests

### Parallel Page Creators (1M elements each)

- `parallel_long_page_creator::test_large_array` - 1M i64s = 8MB
- `parallel_double_page_creator::test_large_array` - 1M f64s = 8MB
- `parallel_int_page_creator::test_large_array` - 1M i32s = 4MB
- `parallel_byte_page_creator::test_large_array` - 1M bytes = 1MB

### HugeArray Tests

- `huge_double_array::test_with_generator_million_elements` - 10M f64s = 80MB
- `two_arrays_sort::test_large_array` - 1000 elements = minimal

### Sparse Tests (Memory Efficient ✅)

- All sparse tests use large indices but minimal actual allocation
- Only 4 pages allocated despite 100M+ capacity

## Solutions

### Option 1: Reduce Test Parallelism (Recommended)

```bash
# Run tests with limited parallelism
cargo test --lib --features core -- --test-threads=4

# Or even more conservative
cargo test --lib --features core -- --test-threads=2
```

### Option 2: Ignore Large Tests by Default

Add `#[ignore]` to memory-intensive tests and run separately:

```rust
#[test]
#[ignore] // Run with: cargo test -- --ignored
fn test_with_generator_million_elements() {
    // ...
}
```

### Option 3: Reduce Test Data Size

Change 10M → 1M or 100K for tests:

```rust
// Before
let size = 10_000_000;  // 80MB

// After
let size = 1_000_000;   // 8MB
```

## Recommended Immediate Action

**Use test-threads to limit parallelism**:

```bash
# In .cargo/config.toml or as command flag
cargo test --lib --features core -- --test-threads=4
```

This will prevent memory spikes from running 1454 tests simultaneously.

## Memory Budget Estimate

- **Per large test**: ~8-80MB
- **10 large tests running in parallel**: ~400MB peak
- **1454 tests with default parallelism**: Could spike to several GB

**With --test-threads=4**: Maximum ~320MB (4 x 80MB) which is very safe.

## Future Consideration

Once we move to Projection mode tomorrow, we can:

1. Review which large tests are essential
2. Add `#[ignore]` to stress tests
3. Create a separate `cargo test --ignored` CI step for full validation

## Status

✅ **8GB test is NOT run during `cargo test`** (it's an example)
✅ **Most tests are small** (<1MB)
⚠️ **~10 large tests** (1-10M elements) could spike memory if all run simultaneously
✅ **Sparse tests are efficient** (only allocate what they need)

## Implementation - COMPLETED

### ✅ Large Tests Now Ignored

Added `#[ignore]` to 4 memory-intensive parallel page creator tests:

1. `parallel_long_page_creator::test_large_array`
2. `parallel_double_page_creator::test_large_array`
3. `parallel_int_page_creator::test_large_array`
4. `parallel_byte_page_creator::test_large_array`

Each test creates 1M elements with 8 parallel threads.

**Run large tests separately**:

```bash
cargo test -- --ignored --test-threads=2
```

### Immediate Actions

**Immediate**:

- ✅ Large tests isolated with `#[ignore]`
- ✅ Use `--test-threads=4` for regular test runs
- ✅ Use `test-safe.sh` script

**Tomorrow**: Review as part of Projection mode transition

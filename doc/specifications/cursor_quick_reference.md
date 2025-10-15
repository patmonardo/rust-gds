# Cursor API Quick Reference

## Imports

```rust
use rust_gds::collections::cursor::{
    HugeCursor,              // Core cursor trait
    HugeCursorSupport,       // Array integration trait
    SinglePageCursor,        // Single-page implementation
    PagedCursor,             // Multi-page implementation
    init_cursor,             // Full array helper
    init_cursor_range,       // Range helper
};
```

## Basic Pattern

```rust
// 1. Get cursor from array
let mut cursor = array.new_cursor();

// 2. Initialize for full array
init_cursor(&array, &mut cursor);

// 3. Iterate
while cursor.next() {
    let page = cursor.array().unwrap();
    for i in cursor.offset()..cursor.limit() {
        let global_index = cursor.base() + i;
        let value = page[i];
        // Process value...
    }
}
```

## Range-Based Pattern

```rust
// Iterate over specific range [start, end)
let mut cursor = array.new_cursor();
init_cursor_range(&array, &mut cursor, start, end);

while cursor.next() {
    // Process range...
}
```

## Parallel Pattern

```rust
use std::thread;

let total = array.size();
let chunks = 4;
let chunk_size = total / chunks;

let handles: Vec<_> = (0..chunks).map(|i| {
    let start = i * chunk_size;
    let end = if i == chunks - 1 { total } else { (i + 1) * chunk_size };

    thread::spawn(move || {
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, start, end);

        let mut local_sum = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for j in cursor.offset()..cursor.limit() {
                local_sum += page[j];
            }
        }
        local_sum
    })
}).collect();

let total_sum: i64 = handles.into_iter()
    .map(|h| h.join().unwrap())
    .sum();
```

## Reset Pattern

```rust
let mut cursor = array.new_cursor();
init_cursor(&array, &mut cursor);

// First pass
while cursor.next() {
    // Count elements...
}

// Reset and do second pass
cursor.reset();
while cursor.next() {
    // Process elements...
}
```

## Trait Methods

### HugeCursor<'a>

| Method                  | Returns           | Description                                          |
| ----------------------- | ----------------- | ---------------------------------------------------- |
| `next()`                | `bool`            | Advance to next page, returns true if data available |
| `base()`                | `usize`           | Global index of first element in current page        |
| `array()`               | `Option<&'a [T]>` | Reference to current page data                       |
| `offset()`              | `usize`           | First valid index within current page                |
| `limit()`               | `usize`           | First invalid index within current page (exclusive)  |
| `set_range(start, end)` | `()`              | Configure iteration range                            |
| `reset()`               | `()`              | Reset to iterate full array again                    |

### HugeCursorSupport<'a>

| Method         | Returns  | Description                       |
| -------------- | -------- | --------------------------------- |
| `size()`       | `usize`  | Total number of elements          |
| `new_cursor()` | `Cursor` | Create new cursor (uninitialized) |
| `capacity()`   | `usize`  | Capacity (may exceed size)        |

## Helper Functions

### init_cursor()

```rust
pub fn init_cursor<'a, S>(support: &S, cursor: &mut S::Cursor)
where
    S: HugeCursorSupport<'a>
```

Initialize cursor to iterate entire array.

### init_cursor_range()

```rust
pub fn init_cursor_range<'a, S>(
    support: &S,
    cursor: &mut S::Cursor,
    start: usize,
    end: usize,
) where
    S: HugeCursorSupport<'a>
```

Initialize cursor to iterate range `[start, end)`.

**Panics** if:

- `start > size`
- `end < start`
- `end > size`

## Cursor State Machine

```
┌─────────────┐
│  Created    │  new_cursor()
│ (invalid)   │
└──────┬──────┘
       │ init_cursor() or init_cursor_range()
       ↓
┌─────────────┐
│ Initialized │
│ (invalid)   │
└──────┬──────┘
       │ next() → true
       ↓
┌─────────────┐
│   Valid     │ ←─────┐
│  (reading)  │       │ next() → true
└──────┬──────┘       │
       │              │
       ├──────────────┘
       │ next() → false
       ↓
┌─────────────┐
│  Exhausted  │
│  (invalid)  │
└──────┬──────┘
       │ reset()
       └──────→ back to Initialized
```

## Common Patterns

### Sum All Elements

```rust
let mut cursor = array.new_cursor();
init_cursor(&array, &mut cursor);

let mut sum = 0i64;
while cursor.next() {
    let page = cursor.array().unwrap();
    for i in cursor.offset()..cursor.limit() {
        sum += page[i];
    }
}
```

### Find Maximum

```rust
let mut cursor = array.new_cursor();
init_cursor(&array, &mut cursor);

let mut max = i64::MIN;
while cursor.next() {
    let page = cursor.array().unwrap();
    for i in cursor.offset()..cursor.limit() {
        max = max.max(page[i]);
    }
}
```

### Count Non-Zero

```rust
let mut cursor = array.new_cursor();
init_cursor(&array, &mut cursor);

let mut count = 0;
while cursor.next() {
    let page = cursor.array().unwrap();
    for i in cursor.offset()..cursor.limit() {
        if page[i] != 0 {
            count += 1;
        }
    }
}
```

### Copy to Another Array

```rust
let mut src_cursor = src_array.new_cursor();
init_cursor(&src_array, &mut src_cursor);

let mut dst_index = 0;
while src_cursor.next() {
    let page = src_cursor.array().unwrap();
    for i in src_cursor.offset()..src_cursor.limit() {
        dst_array.set(dst_index, page[i]);
        dst_index += 1;
    }
}
```

## Performance Tips

### ✅ Do

- Use cursors for sequential access patterns
- Process full pages when possible
- Reuse cursors via `reset()` for multiple passes
- Use range iteration for parallel processing
- Access page data via slice operations

### ❌ Don't

- Create cursors for single-element access (use `get()`)
- Nest cursor iterations (prefer indices)
- Call `array()` multiple times per page
- Use cursors with atomic arrays (incompatible)
- Forget to call `next()` before accessing data

## Supported Types

| Array Type              | Cursor Support | Element Type |
| ----------------------- | -------------- | ------------ |
| `HugeLongArray`         | ✅ Yes         | `i64`        |
| `HugeDoubleArray`       | ✅ Yes         | `f64`        |
| `HugeAtomicLongArray`   | ❌ No          | `AtomicI64`  |
| `HugeAtomicDoubleArray` | ❌ No          | `AtomicU64`  |
| `HugeSparseLongArray`   | ⏳ Future      | `i64`        |
| `HugeSparseDoubleArray` | ⏳ Future      | `f64`        |

## Compile-Time Guarantees

### Lifetime Safety

```rust
let cursor = {
    let array = HugeLongArray::new(100);
    array.new_cursor() // ❌ Won't compile!
}; // array dropped, cursor lifetime is tied to it
```

### Zero-Copy

```rust
let page = cursor.array().unwrap();
// `page` is &[i64], not Vec<i64>
// No allocation occurred!
```

### Type Safety

```rust
let mut cursor: HugeLongArrayCursor = array.new_cursor();
// Cursor type matches array type
// Can't mix Long and Double cursors
```

## Error Handling

Cursors use **panic for contract violations**:

```rust
// ❌ Panics: start > size
init_cursor_range(&array, &mut cursor, 1000, 500);

// ❌ Panics: end < start
init_cursor_range(&array, &mut cursor, 100, 50);

// ✅ OK: empty range
init_cursor_range(&array, &mut cursor, 50, 50);
```

## Testing Patterns

```rust
#[test]
fn test_cursor_iteration() {
    use crate::collections::cursor::init_cursor;

    let mut array = HugeLongArray::new(100);
    array.set_all(|i| i as i64);

    let mut cursor = array.new_cursor();
    init_cursor(&array, &mut cursor);

    let mut visited = 0;
    while cursor.next() {
        let page = cursor.array().unwrap();
        for i in cursor.offset()..cursor.limit() {
            assert_eq!(page[i], (cursor.base() + i) as i64);
            visited += 1;
        }
    }

    assert_eq!(visited, 100);
}
```

## Further Reading

- `doc/cursor_implementation_summary.md` - Implementation details
- `doc/CURSOR_CELEBRATION.md` - Achievement summary
- `src/collections/cursor/huge_cursor.rs` - Core trait definitions
- `src/collections/cursor/huge_cursor_support.rs` - Support trait and helpers

---

**Version**: 1.0.0  
**Status**: ✅ Stable  
**Tests**: 29 passing

# 🔥 Concurrency Package: Java → TypeScript → Rust Translation

**Date**: October 9, 2025  
**Mission**: Build a Java GDS-compatible Concurrency package in Rust  
**Philosophy**: Rust-GDS is the Kernel, TS is UserLand

---

## 🎯 The Three Worlds of Concurrency

### Java GDS Concurrency (The Original)

- **Threading Model**: JVM threads, ForkJoinPool
- **Language Features**: `synchronized`, `volatile`, `java.util.concurrent.*`
- **Simplicity**: Java handles a lot automatically
- **Problem**: Still has race conditions, deadlocks possible

### TypeScript Concurrency (The UserLand)

- **Threading Model**: Single-threaded event loop (Node.js)
- **Parallelism Options**: Worker threads, child processes
- **Simplicity**: Very simple (mostly no threads!)
- **Problem**: **TS threading won't hack it for compute-heavy graph algorithms**

### Rust Concurrency (The Kernel)

- **Threading Model**: OS threads, Rayon work-stealing
- **Language Features**: Ownership system prevents data races **AT COMPILE TIME**
- **Simplicity**: More complex than Java, but **SAFE BY DEFAULT**
- **Advantage**: **Zero-cost abstractions + fearless concurrency**

---

## 🔍 Why Java Concurrency is "Simpler" Than TS

Let's compare the **SAME OPERATION** in Java vs TS:

### Java: Concurrent Counter

```java
// Java: Built-in atomic types
import java.util.concurrent.atomic.AtomicInteger;

public class Counter {
    private AtomicInteger count = new AtomicInteger(0);

    public void increment() {
        count.incrementAndGet();  // Thread-safe!
    }

    public int get() {
        return count.get();
    }
}

// Use from multiple threads - just works
ExecutorService executor = Executors.newFixedThreadPool(4);
Counter counter = new Counter();

for (int i = 0; i < 1000; i++) {
    executor.submit(() -> counter.increment());
}
```

### TypeScript: Concurrent Counter (oof!)

```typescript
// TypeScript: Need Worker threads + SharedArrayBuffer
import { Worker } from "worker_threads";

// Main thread
const sharedBuffer = new SharedArrayBuffer(4);
const sharedArray = new Int32Array(sharedBuffer);

// Create workers
for (let i = 0; i < 4; i++) {
  const worker = new Worker("./worker.js", {
    workerData: { sharedBuffer },
  });
}

// worker.js
const { workerData } = require("worker_threads");
const sharedArray = new Int32Array(workerData.sharedBuffer);

// Atomic increment requires Atomics API
for (let i = 0; i < 250; i++) {
  Atomics.add(sharedArray, 0, 1); // Index 0, add 1
}
```

**Why Java is "Simpler"**:

1. ✅ Built-in thread pools (`ExecutorService`)
2. ✅ Rich `java.util.concurrent` library
3. ✅ Automatic thread lifecycle management
4. ✅ Lambda-friendly parallel streams
5. ✅ Shared memory is the DEFAULT model

**Why TS is Complex**:

1. ❌ Worker threads are heavyweight
2. ❌ Message passing is the DEFAULT (cloning overhead)
3. ❌ `SharedArrayBuffer` requires explicit setup
4. ❌ `Atomics` API is limited (only integers)
5. ❌ No thread pools out of the box
6. ❌ **Can't share complex objects between workers**

---

## 🚀 Rust: Best of Both Worlds

### Rust: Concurrent Counter (Simple + Safe)

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// Thread-safe counter
let counter = Arc::new(AtomicUsize::new(0));

// Spawn 4 threads
let handles: Vec<_> = (0..4).map(|_| {
    let counter_clone = Arc::clone(&counter);
    thread::spawn(move || {
        for _ in 0..250 {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }
    })
}).collect();

// Wait for all threads
for handle in handles {
    handle.join().unwrap();
}

assert_eq!(counter.load(Ordering::SeqCst), 1000);
```

**Rust Advantages**:

1. ✅ **Compile-time safety** - can't have data races
2. ✅ **Explicit sharing** via `Arc` - clear ownership
3. ✅ **Zero-cost** - compiles to same assembly as C
4. ✅ **Rich ecosystem** - Rayon, Crossbeam, Tokio
5. ✅ **Lock-free** - atomics built-in
6. ✅ **Scoped threads** - borrow stack data safely

---

## 📦 The Concurrency Package Design

### Core Principle: **Wrapper Around Rust Concurrency**

The Java GDS Concurrency package is relatively simple:

- `Concurrency` - represents thread count
- `ParallelUtil` - work distribution utilities
- `Partition` - work partitioning
- `ProgressTracker` - progress monitoring
- `TerminationFlag` - cancellation support

**Our Rust Package Should**:

1. **Mirror Java GDS API** for familiarity
2. **Leverage Rayon** for work-stealing parallelism
3. **Use Rust idioms** (traits, ownership, lifetimes)
4. **Integrate with Cursors** for zero-copy parallel iteration
5. **Expose to TS via NAPI** when needed

---

## 🏗️ Package Structure

```
src/concurrency/
├── mod.rs                    # Public API
├── concurrency.rs            # Concurrency type (thread count)
├── parallel_util.rs          # Work distribution utilities
├── partitioner.rs            # Work partitioning
├── progress_tracker.rs       # Progress monitoring
├── termination_flag.rs       # Cancellation support
├── atomic_aggregators.rs     # Lock-free aggregation
└── rayon_integration.rs      # Cursor + Rayon helpers
```

---

## 📋 Component Design

### 1. Concurrency Type (Thread Count)

**Java GDS**:

```java
public class Concurrency {
    private final int value;

    public Concurrency(int value) {
        if (value < 1) {
            throw new IllegalArgumentException("...");
        }
        this.value = value;
    }

    public int value() { return value; }
    public long squared() { return (long) value * value; }
}
```

**TypeScript** (your implementation):

```typescript
export class Concurrency {
    private readonly _value: number;

    constructor(value: number) {
        if (value < 1) {
            throw new Error(`Valid values for Concurrency are int[1..]...`);
        }
        this._value = value;
    }

    public value(): number { return this._value; }
    public squared(): number { return this._value * this._value; }

    public static availableCores(defaultValue: number = 4): Concurrency {
        const cpuCount = navigator.hardwareConcurrency || ...;
        return new Concurrency(cpuCount);
    }

    public static singleThreaded(): Concurrency {
        return new Concurrency(1);
    }
}
```

**Rust Design**:

```rust
use std::fmt;
use std::num::NonZeroUsize;

/// Represents a concurrency level (number of threads/workers) for parallel processing.
///
/// Guaranteed to be at least 1 by using NonZeroUsize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Concurrency {
    value: NonZeroUsize,
}

impl Concurrency {
    /// Creates a new Concurrency with the specified value.
    ///
    /// Returns None if value is 0.
    pub fn new(value: usize) -> Option<Self> {
        NonZeroUsize::new(value).map(|v| Self { value: v })
    }

    /// Creates a new Concurrency, panicking if value is 0.
    pub fn new_unchecked(value: usize) -> Self {
        Self {
            value: NonZeroUsize::new(value)
                .expect("Concurrency value must be at least 1"),
        }
    }

    /// Returns the concurrency level.
    pub fn value(&self) -> usize {
        self.value.get()
    }

    /// Returns the square of the concurrency level.
    pub fn squared(&self) -> usize {
        let v = self.value.get();
        v * v
    }

    /// Creates a concurrency level based on available CPU cores.
    pub fn available_cores() -> Self {
        let cpus = num_cpus::get().max(1);
        Self::new_unchecked(cpus)
    }

    /// Creates a single-threaded concurrency level.
    pub const fn single_threaded() -> Self {
        // SAFETY: 1 is never zero
        Self {
            value: unsafe { NonZeroUsize::new_unchecked(1) },
        }
    }

    /// Converts from usize, clamping to minimum of 1.
    pub fn from_usize(value: usize) -> Self {
        Self::new(value.max(1)).unwrap()
    }
}

impl Default for Concurrency {
    fn default() -> Self {
        Self::single_threaded()
    }
}

impl fmt::Display for Concurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Concurrency({})", self.value())
    }
}

impl From<NonZeroUsize> for Concurrency {
    fn from(value: NonZeroUsize) -> Self {
        Self { value }
    }
}
```

**Why This Design**:

- ✅ **`NonZeroUsize`** guarantees value ≥ 1 at compile time
- ✅ **No runtime checks** in hot paths
- ✅ **Copy + Eq** - can use as map keys
- ✅ **`available_cores()`** mirrors Java/TS API
- ✅ **`single_threaded()`** is const - compile-time constant

---

### 2. Partitioner (Work Distribution)

**Purpose**: Divide work into chunks for parallel processing.

**Rust Design**:

```rust
use std::ops::Range;

/// Divides a range of work into partitions for parallel processing.
#[derive(Debug, Clone)]
pub struct Partitioner {
    total_work: usize,
    num_partitions: usize,
}

impl Partitioner {
    /// Creates a new partitioner for the given amount of work and partition count.
    pub fn new(total_work: usize, num_partitions: usize) -> Self {
        let num_partitions = num_partitions.max(1);
        Self {
            total_work,
            num_partitions,
        }
    }

    /// Creates a partitioner based on available CPU cores.
    pub fn for_cores(total_work: usize) -> Self {
        let cores = Concurrency::available_cores().value();
        Self::new(total_work, cores)
    }

    /// Returns the range of work for a specific partition.
    ///
    /// # Panics
    /// Panics if partition_id >= num_partitions.
    pub fn partition(&self, partition_id: usize) -> Range<usize> {
        assert!(partition_id < self.num_partitions,
                "Partition ID {} out of range (max {})",
                partition_id, self.num_partitions);

        let chunk_size = self.total_work / self.num_partitions;
        let remainder = self.total_work % self.num_partitions;

        let start = partition_id * chunk_size + partition_id.min(remainder);
        let extra = if partition_id < remainder { 1 } else { 0 };
        let end = start + chunk_size + extra;

        start..end
    }

    /// Returns the number of partitions.
    pub fn partition_count(&self) -> usize {
        self.num_partitions
    }

    /// Returns the total amount of work.
    pub fn total_work(&self) -> usize {
        self.total_work
    }

    /// Iterates over all partition ranges.
    pub fn iter(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        (0..self.num_partitions).map(move |id| self.partition(id))
    }
}
```

**Usage with Cursors**:

```rust
use rayon::prelude::*;

// Parallel cursor iteration
let partitioner = Partitioner::for_cores(array.size());

(0..partitioner.partition_count())
    .into_par_iter()
    .map(|partition_id| {
        let range = partitioner.partition(partition_id);

        // Each thread gets its own cursor
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, range.start, range.end);

        let mut local_sum = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                local_sum += page[i];
            }
        }
        local_sum
    })
    .sum::<i64>()
```

---

### 3. Progress Tracker

**Purpose**: Monitor long-running operations and estimate completion.

**Rust Design**:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Tracks progress of a long-running operation.
#[derive(Debug)]
pub struct ProgressTracker {
    total_tasks: usize,
    completed_tasks: Arc<AtomicUsize>,
    start_time: Instant,
}

impl ProgressTracker {
    /// Creates a new progress tracker for the given number of tasks.
    pub fn new(total_tasks: usize) -> Self {
        Self {
            total_tasks,
            completed_tasks: Arc::new(AtomicUsize::new(0)),
            start_time: Instant::now(),
        }
    }

    /// Increments the completed task count.
    /// Returns the new count.
    pub fn increment(&self) -> usize {
        self.completed_tasks.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Adds to the completed task count.
    pub fn add(&self, count: usize) {
        self.completed_tasks.fetch_add(count, Ordering::Relaxed);
    }

    /// Returns the number of completed tasks.
    pub fn completed(&self) -> usize {
        self.completed_tasks.load(Ordering::Relaxed)
    }

    /// Returns the total number of tasks.
    pub fn total(&self) -> usize {
        self.total_tasks
    }

    /// Returns the progress as a fraction (0.0 to 1.0).
    pub fn progress(&self) -> f64 {
        if self.total_tasks == 0 {
            return 1.0;
        }
        self.completed() as f64 / self.total_tasks as f64
    }

    /// Returns elapsed time since start.
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Estimates time until completion based on current progress.
    pub fn estimated_remaining(&self) -> Option<Duration> {
        let completed = self.completed();
        if completed == 0 {
            return None;
        }

        let elapsed = self.elapsed();
        let rate = completed as f64 / elapsed.as_secs_f64();
        let remaining = self.total_tasks - completed;

        Some(Duration::from_secs_f64(remaining as f64 / rate))
    }

    /// Logs progress to stdout.
    pub fn log_progress(&self) {
        println!(
            "Progress: {}/{} ({:.1}%) - Elapsed: {:?} - Remaining: {:?}",
            self.completed(),
            self.total_tasks,
            self.progress() * 100.0,
            self.elapsed(),
            self.estimated_remaining()
                .map(|d| format!("{:?}", d))
                .unwrap_or_else(|| "Unknown".to_string())
        );
    }

    /// Creates a handle that can be shared across threads.
    pub fn handle(&self) -> ProgressHandle {
        ProgressHandle {
            completed_tasks: Arc::clone(&self.completed_tasks),
        }
    }
}

/// Thread-safe handle for incrementing progress.
#[derive(Debug, Clone)]
pub struct ProgressHandle {
    completed_tasks: Arc<AtomicUsize>,
}

impl ProgressHandle {
    pub fn increment(&self) {
        self.completed_tasks.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add(&self, count: usize) {
        self.completed_tasks.fetch_add(count, Ordering::Relaxed);
    }
}
```

**Usage**:

```rust
let tracker = ProgressTracker::new(1_000_000);

// Spawn progress logger
let tracker_clone = tracker.handle();
thread::spawn(move || {
    loop {
        thread::sleep(Duration::from_secs(1));
        if tracker_clone.completed() >= 1_000_000 {
            break;
        }
        // Could log here
    }
});

// Parallel work with progress
(0..1_000_000).into_par_iter().for_each(|i| {
    // Do work...
    tracker.handle().increment();
});
```

---

### 4. Termination Flag (Cancellation)

**Purpose**: Allow graceful cancellation of long-running operations.

**Rust Design**:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Allows cancellation of long-running operations.
#[derive(Debug, Clone)]
pub struct TerminationFlag {
    cancelled: Arc<AtomicBool>,
}

impl TerminationFlag {
    /// Creates a new termination flag.
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Checks if cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Checks cancellation and returns an error if cancelled.
    pub fn check(&self) -> Result<(), Cancelled> {
        if self.is_cancelled() {
            Err(Cancelled)
        } else {
            Ok(())
        }
    }
}

impl Default for TerminationFlag {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for cancelled operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cancelled;

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation was cancelled")
    }
}

impl std::error::Error for Cancelled {}
```

**Usage**:

```rust
let flag = TerminationFlag::new();
let flag_clone = flag.clone();

// Spawn cancellable work
let handle = thread::spawn(move || {
    for i in 0..1_000_000 {
        // Check every 1000 iterations
        if i % 1000 == 0 {
            flag_clone.check()?;
        }

        // Do work...
    }
    Ok(())
});

// Cancel from another thread
thread::sleep(Duration::from_millis(100));
flag.cancel();

// Work should stop gracefully
match handle.join().unwrap() {
    Ok(_) => println!("Completed"),
    Err(Cancelled) => println!("Cancelled as expected"),
}
```

---

### 5. Atomic Aggregators

**Purpose**: Lock-free aggregation for parallel algorithms.

**Rust Design**:

```rust
use std::sync::atomic::{AtomicU64, Ordering};

/// Lock-free double adder using atomic operations.
///
/// Stores f64 as u64 via bit-casting.
#[derive(Debug)]
pub struct AtomicDoubleAdder {
    bits: AtomicU64,
}

impl AtomicDoubleAdder {
    pub fn new(initial: f64) -> Self {
        Self {
            bits: AtomicU64::new(initial.to_bits()),
        }
    }

    /// Atomically adds a value using compare-and-swap.
    pub fn add(&self, value: f64) {
        let mut current = self.bits.load(Ordering::Relaxed);
        loop {
            let current_f64 = f64::from_bits(current);
            let new_f64 = current_f64 + value;
            let new_bits = new_f64.to_bits();

            match self.bits.compare_exchange_weak(
                current,
                new_bits,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(actual) => current = actual,
            }
        }
    }

    /// Returns the current sum.
    pub fn sum(&self) -> f64 {
        f64::from_bits(self.bits.load(Ordering::Relaxed))
    }
}

/// Lock-free maximum tracker.
#[derive(Debug)]
pub struct AtomicMax {
    value: AtomicU64,
}

impl AtomicMax {
    pub fn new(initial: i64) -> Self {
        Self {
            value: AtomicU64::new(initial as u64),
        }
    }

    /// Updates the maximum if value is greater.
    pub fn update(&self, value: i64) {
        let value_bits = value as u64;
        let mut current = self.value.load(Ordering::Relaxed);

        while (current as i64) < value {
            match self.value.compare_exchange_weak(
                current,
                value_bits,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(actual) => current = actual,
            }
        }
    }

    /// Returns the current maximum.
    pub fn get(&self) -> i64 {
        self.value.load(Ordering::Relaxed) as i64
    }
}
```

---

## 🎯 Integration Points

### 1. Cursors + Rayon

**Helper for parallel cursor iteration**:

```rust
use rayon::prelude::*;

pub fn parallel_cursor_map<'a, T, F, R>(
    array: &'a impl HugeCursorSupport<'a>,
    concurrency: Concurrency,
    f: F,
) -> Vec<R>
where
    T: 'a,
    F: Fn(&[T]) -> R + Send + Sync,
    R: Send,
{
    let partitioner = Partitioner::new(array.size(), concurrency.value());

    (0..concurrency.value())
        .into_par_iter()
        .map(|partition_id| {
            let range = partitioner.partition(partition_id);
            let mut cursor = array.new_cursor();
            init_cursor_range(array, &mut cursor, range.start, range.end);

            let mut results = Vec::new();
            while cursor.next() {
                let page = cursor.array().unwrap();
                results.push(f(page));
            }
            results
        })
        .flatten()
        .collect()
}
```

### 2. With Progress Tracking

```rust
pub fn parallel_cursor_with_progress<'a, T, F>(
    array: &'a impl HugeCursorSupport<'a>,
    concurrency: Concurrency,
    tracker: &ProgressTracker,
    f: F,
)
where
    T: 'a,
    F: Fn(&[T]) + Send + Sync,
{
    let partitioner = Partitioner::new(array.size(), concurrency.value());

    (0..concurrency.value())
        .into_par_iter()
        .for_each(|partition_id| {
            let range = partitioner.partition(partition_id);
            let mut cursor = array.new_cursor();
            init_cursor_range(array, &mut cursor, range.start, range.end);

            let handle = tracker.handle();
            while cursor.next() {
                let page = cursor.array().unwrap();
                f(page);
                handle.add(page.len());
            }
        });
}
```

---

## 🌍 The Future: Language of Futures

You mentioned **Futures** - this is BRILLIANT because:

### Rust Has Two Concurrency Models:

1. **Thread-Based** (what we're building now)

   - `std::thread`
   - Rayon work-stealing
   - Good for CPU-bound algorithms

2. **Async/Await + Futures** (for I/O-bound work)
   - `async fn`, `.await`
   - Tokio runtime
   - Good for network, disk I/O

### Why Futures Matter:

**TypeScript uses Promises** (which are Futures):

```typescript
async function loadGraph(): Promise<Graph> {
  const data = await fetch("/graph-data");
  return parseGraph(data);
}
```

**Rust has the SAME model**:

```rust
async fn load_graph() -> Result<Graph, Error> {
    let data = fetch_data("/graph-data").await?;
    parse_graph(data)
}
```

**We can expose Rust async functions to TS via NAPI!**

```rust
#[napi]
pub async fn load_graph(path: String) -> Result<Graph> {
    // Rust async code
    let data = tokio::fs::read(path).await?;
    parse_graph(data)
}
```

**This means**:

- ✅ TS can `await` Rust computations
- ✅ Non-blocking I/O in both layers
- ✅ Rust does heavy compute, TS orchestrates
- ✅ **Language of Futures unifies the stack!**

---

## 🎊 Action Plan for Today

### Phase 1: Basic Concurrency Types (Morning)

- [ ] `Concurrency` struct with `NonZeroUsize`
- [ ] `Partitioner` for work distribution
- [ ] Tests for both

### Phase 2: Advanced Types (Midday)

- [ ] `ProgressTracker` with atomic counters
- [ ] `TerminationFlag` for cancellation
- [ ] `AtomicDoubleAdder` for lock-free aggregation
- [ ] Tests for all

### Phase 3: Integration (Afternoon)

- [ ] Cursor + Rayon helper functions
- [ ] Example: Parallel sum with cursors
- [ ] Example: Parallel PageRank sketch
- [ ] Benchmarks

### Phase 4: Documentation (Evening)

- [ ] API documentation
- [ ] Usage examples
- [ ] Performance notes
- [ ] Future directions (async/await)

---

## 🔥 Why This is Exciting

With this Concurrency package:

1. ✅ **Java GDS-compatible API** - familiar to existing users
2. ✅ **Rust safety guarantees** - no data races possible
3. ✅ **Zero-cost abstractions** - as fast as manual threading
4. ✅ **Cursor integration** - zero-copy parallel iteration
5. ✅ **Future-ready** - can add async/await later
6. ✅ **TS interop** - NAPI can expose these patterns

This is the **compute kernel** that makes graph algorithms **SCREAM**! 🚀

---

**Status**: 📋 READY TO BUILD  
**Complexity**: 🌶️🌶️ MODERATE (Rust concurrency, but with clear patterns)  
**Impact**: 🔥🔥🔥 MASSIVE (enables all parallel algorithms)  
**Fun Factor**: 🎉🎉🎉 MAXIMUM (this is where Rust shines!)

_Let's build a concurrency package that shows why Rust is the future of systems programming!_ ⚡

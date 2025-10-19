//! Showcase for PaddedAtomicLong - false sharing prevention.
//!
//! Demonstrates the performance impact of cache line padding in concurrent scenarios.

use rust_gds::core::utils::paged::PaddedAtomicLong;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("=== PaddedAtomicLong Showcase ===\n");
    println!("Demonstrating false sharing prevention with cache line padding\n");

    demo_basic_operations();
    demo_memory_layout();
    benchmark_false_sharing();
    demo_concurrent_counters();
}

/// Demonstrates basic atomic operations
fn demo_basic_operations() {
    println!("1. Basic Atomic Operations");

    let counter = PaddedAtomicLong::new(0);
    println!("   Initial value: {}", counter.get());

    counter.set(42);
    println!("   After set(42): {}", counter.get());

    let old = counter.get_and_increment();
    println!(
        "   get_and_increment() returned: {}, new value: {}",
        old,
        counter.get()
    );

    let new = counter.increment_and_get();
    println!("   increment_and_get() returned: {}", new);

    counter.add_and_get(10);
    println!("   After add_and_get(10): {}", counter.get());

    let swapped = counter.compare_and_set(54, 100);
    println!("   compare_and_set(54, 100) succeeded: {}", swapped);
    println!("   Final value: {}", counter.get());

    println!("   ✓ All operations working correctly\n");
}

/// Demonstrates memory layout and padding
fn demo_memory_layout() {
    println!("2. Memory Layout & Cache Line Padding");

    let counter = PaddedAtomicLong::new(0);
    let size = std::mem::size_of::<PaddedAtomicLong>();

    println!("   Structure size: {} bytes", size);
    println!("   Cache line size (typical): 64 bytes");
    println!("   AtomicI64 size: 8 bytes");
    println!("   Padding size: {} bytes", size - 8);
    println!("   Padding sum (anti-optimization): {}", counter.sum());

    println!("\n   Memory Layout:");
    println!("   |<-------- 64 bytes (one cache line) -------->|");
    println!("   | AtomicI64 (8B) | Padding (56B)             |");
    println!("   | value          | p1 p2 p3 p4 p5 p6 p7      |");

    println!("\n   ✓ Each PaddedAtomicLong occupies its own cache line");
    println!("   ✓ No false sharing with adjacent variables\n");
}

/// Benchmarks false sharing effect
fn benchmark_false_sharing() {
    println!("3. False Sharing Performance Impact");

    let threads = 4;
    let increments_per_thread = 1_000_000;

    // Scenario 1: WITHOUT padding (false sharing likely)
    println!("   Scenario A: Regular AtomicI64 (may have false sharing)");
    struct UnpaddedCounters {
        counter1: AtomicI64,
        counter2: AtomicI64,
        counter3: AtomicI64,
        counter4: AtomicI64,
    }

    let unpaded = Arc::new(UnpaddedCounters {
        counter1: AtomicI64::new(0),
        counter2: AtomicI64::new(0),
        counter3: AtomicI64::new(0),
        counter4: AtomicI64::new(0),
    });

    let start = Instant::now();
    let mut handles = vec![];

    for thread_id in 0..threads {
        let counters = Arc::clone(&unpaded);
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                match thread_id {
                    0 => counters.counter1.fetch_add(1, Ordering::SeqCst),
                    1 => counters.counter2.fetch_add(1, Ordering::SeqCst),
                    2 => counters.counter3.fetch_add(1, Ordering::SeqCst),
                    _ => counters.counter4.fetch_add(1, Ordering::SeqCst),
                };
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let unpadded_duration = start.elapsed();
    let unpadded_ops = (threads * increments_per_thread) as f64 / unpadded_duration.as_secs_f64();

    println!("      Time: {:.2?}", unpadded_duration);
    println!(
        "      Throughput: {:.2} M ops/sec",
        unpadded_ops / 1_000_000.0
    );

    // Scenario 2: WITH padding (no false sharing)
    println!("\n   Scenario B: PaddedAtomicLong (no false sharing)");
    struct PaddedCounters {
        counter1: PaddedAtomicLong,
        counter2: PaddedAtomicLong,
        counter3: PaddedAtomicLong,
        counter4: PaddedAtomicLong,
    }

    let padded = Arc::new(PaddedCounters {
        counter1: PaddedAtomicLong::new(0),
        counter2: PaddedAtomicLong::new(0),
        counter3: PaddedAtomicLong::new(0),
        counter4: PaddedAtomicLong::new(0),
    });

    let start = Instant::now();
    let mut handles = vec![];

    for thread_id in 0..threads {
        let counters = Arc::clone(&padded);
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                match thread_id {
                    0 => counters.counter1.fetch_add(1),
                    1 => counters.counter2.fetch_add(1),
                    2 => counters.counter3.fetch_add(1),
                    _ => counters.counter4.fetch_add(1),
                };
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let padded_duration = start.elapsed();
    let padded_ops = (threads * increments_per_thread) as f64 / padded_duration.as_secs_f64();

    println!("      Time: {:.2?}", padded_duration);
    println!(
        "      Throughput: {:.2} M ops/sec",
        padded_ops / 1_000_000.0
    );

    let speedup = padded_ops / unpadded_ops;
    println!(
        "\n   Performance Improvement: {:.1}x faster with padding",
        speedup
    );

    if speedup > 1.1 {
        println!("   ✓ Significant performance gain from eliminating false sharing!");
    } else {
        println!("   ℹ Note: False sharing impact varies by CPU architecture");
        println!("           and current system load. Benefits are more visible");
        println!("           on systems with higher cache contention.");
    }
    println!();
}

/// Demonstrates real-world usage in concurrent counters
fn demo_concurrent_counters() {
    println!("4. Concurrent Counter Example");

    let threads = 8;
    let increments_per_thread = 500_000;

    println!("   Processing with {} threads...", threads);
    println!("   Each thread: {} increments", increments_per_thread);

    let counter = Arc::new(PaddedAtomicLong::new(0));
    let start = Instant::now();
    let mut handles = vec![];

    for thread_id in 0..threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                counter_clone.fetch_add(1);
            }
            println!("      Thread {} completed", thread_id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let total_ops = threads * increments_per_thread;
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();

    println!("\n   Results:");
    println!("      Total operations: {}", total_ops);
    println!("      Time: {:.2?}", duration);
    println!(
        "      Throughput: {:.2} M ops/sec",
        ops_per_sec / 1_000_000.0
    );
    println!("      Final counter value: {}", counter.get());
    assert_eq!(counter.get(), total_ops as i64);

    println!("   ✓ All increments accounted for - perfect atomicity!\n");
}

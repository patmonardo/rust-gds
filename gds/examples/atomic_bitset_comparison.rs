//! Showcase of HugeAtomicBitSet and HugeAtomicGrowingBitSet
//!
//! Demonstrates the difference between fixed-size and growing atomic bitsets
//! for concurrent graph algorithm scenarios.

use gds::core::utils::paged::{HugeAtomicBitSet, HugeAtomicGrowingBitSet};
use std::sync::Arc;
use std::time::Instant;

fn main() {
    println!("===========================================");
    println!("Atomic Bitset Comparison");
    println!("===========================================\n");

    // 1. Fixed-size bitset (best when size is known)
    demo_fixed_bitset();

    // 2. Growing bitset (best when size is unknown)
    demo_growing_bitset();

    // 3. Concurrent performance comparison
    demo_concurrent_performance();
}

fn demo_fixed_bitset() {
    println!("1. HugeAtomicBitSet (Fixed Size)");
    println!("----------------------------------");

    let size = 10_000_000;
    let bitset = Arc::new(HugeAtomicBitSet::new(size));

    println!("Created bitset with {} bits", size);
    println!(
        "Memory: ~{} MB\n",
        HugeAtomicBitSet::memory_estimation(size) / 1_000_000
    );

    // Simulate parallel BFS visited tracking
    let start = Instant::now();
    std::thread::scope(|s| {
        for worker in 0..4 {
            let bitset = Arc::clone(&bitset);
            s.spawn(move || {
                let base = worker * 2_500_000;
                for i in base..(base + 2_500_000) {
                    if !bitset.get_and_set(i) {
                        // First visit - process node (no-op for demo)
                    }
                }
            });
        }
    });
    let elapsed = start.elapsed();

    println!("4 workers marked {} nodes as visited", size);
    println!("Time: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M ops/sec",
        size as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
    println!(
        "Cardinality: {} (all should be set)\n",
        bitset.cardinality()
    );
}

fn demo_growing_bitset() {
    println!("2. HugeAtomicGrowingBitSet (Dynamic Growth)");
    println!("--------------------------------------------");

    // Start with small capacity
    let initial_size = 1000;
    let bitset = Arc::new(HugeAtomicGrowingBitSet::create(initial_size));

    println!("Created growing bitset with initial {} bits", initial_size);
    println!("Initial capacity: {} bits\n", bitset.capacity());

    // Simulate streaming graph discovery - workers find nodes dynamically
    let start = Instant::now();
    std::thread::scope(|s| {
        for worker in 0..4 {
            let bitset = Arc::clone(&bitset);
            s.spawn(move || {
                // Each worker discovers different ranges (simulating graph expansion)
                let ranges = match worker {
                    0 => vec![0..1000, 10_000..11_000],
                    1 => vec![1000..2000, 50_000..51_000],
                    2 => vec![2000..3000, 100_000..101_000],
                    3 => vec![3000..4000, 500_000..501_000],
                    _ => unreachable!(),
                };

                for range in ranges {
                    for i in range {
                        if !bitset.get_and_set(i) {
                            // First discovery - process node
                        }
                    }
                }
            });
        }
    });
    let elapsed = start.elapsed();

    println!("4 workers discovered nodes across dynamic ranges");
    println!(
        "Final capacity: {} bits (grew from {})",
        bitset.capacity(),
        initial_size
    );
    println!("Time: {:?}", elapsed);
    println!("Set bits: {}", bitset.cardinality());
    println!(
        "Capacity utilization: {:.2}%\n",
        bitset.cardinality() as f64 / bitset.capacity() as f64 * 100.0
    );
}

fn demo_concurrent_performance() {
    println!("3. Concurrent Performance Comparison");
    println!("-------------------------------------");

    let size = 1_000_000;
    let iterations = 3;

    // Fixed bitset benchmark
    let mut fixed_times = Vec::new();
    for _ in 0..iterations {
        let bitset = Arc::new(HugeAtomicBitSet::new(size));
        let start = Instant::now();

        std::thread::scope(|s| {
            for worker in 0..4 {
                let bitset = Arc::clone(&bitset);
                s.spawn(move || {
                    for i in (worker..size).step_by(4) {
                        bitset.set(i);
                    }
                });
            }
        });

        fixed_times.push(start.elapsed());
    }
    let fixed_avg = fixed_times.iter().sum::<std::time::Duration>() / iterations as u32;

    // Growing bitset benchmark
    let mut growing_times = Vec::new();
    for _ in 0..iterations {
        let bitset = Arc::new(HugeAtomicGrowingBitSet::create(size));
        let start = Instant::now();

        std::thread::scope(|s| {
            for worker in 0..4 {
                let bitset = Arc::clone(&bitset);
                s.spawn(move || {
                    for i in (worker..size).step_by(4) {
                        bitset.set(i);
                    }
                });
            }
        });

        growing_times.push(start.elapsed());
    }
    let growing_avg = growing_times.iter().sum::<std::time::Duration>() / iterations as u32;

    println!(
        "Setting {} bits with 4 threads (averaged over {} runs):",
        size, iterations
    );
    println!("  HugeAtomicBitSet:        {:?}", fixed_avg);
    println!("  HugeAtomicGrowingBitSet: {:?}", growing_avg);
    println!(
        "  Overhead: {:.2}%\n",
        (growing_avg.as_secs_f64() / fixed_avg.as_secs_f64() - 1.0) * 100.0
    );

    println!("Use Cases:");
    println!("  - Fixed:   BFS/DFS visited tracking (known graph size)");
    println!("  - Growing: Streaming algorithms (unknown final size)");
    println!("  - Growing: Online community detection (dynamic expansion)");
    println!("  - Growing: Real-time graph analysis (discover-as-you-go)");
}

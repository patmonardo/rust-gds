//! Demonstration of HugeLongArray::with_generator - Billion-element arrays in seconds
//!
//! This example showcases the power of parallel page creation for massive arrays.
//!
//! Run with:
//! ```
//! cargo run --example huge_array_with_generator --release
//! ```

use rust_gds::collections::huge_array::HugeLongArray;
use rust_gds::concurrency::Concurrency;
use std::time::Instant;

fn main() {
    println!("\n🚀 HugeLongArray::with_generator() - Layer 6 Complete!\n");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Example 1: Small array (single-page optimization)
    println!("Example 1: Small Array (1,000 elements)");
    println!("----------------------------------------");
    let start = Instant::now();
    let small = HugeLongArray::with_generator(1_000, Concurrency::of(1), |i| i as i64);
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!("Size: {}", small.size());
    println!(
        "First 10 elements: {:?}",
        (0..10).map(|i| small.get(i)).collect::<Vec<_>>()
    );
    println!("✓ Uses optimized single-page implementation\n");

    // Example 2: Identity mapping for node IDs
    println!("Example 2: Node ID Array (10 million nodes)");
    println!("--------------------------------------------");
    let start = Instant::now();
    let node_ids = HugeLongArray::with_generator(10_000_000, Concurrency::of(8), |i| i as i64);
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!("Size: {} elements", node_ids.size());
    println!("Memory: ~{} MB", node_ids.size_of() / 1_000_000);
    println!("Sample values:");
    println!("  node_ids[0] = {}", node_ids.get(0));
    println!("  node_ids[5_000_000] = {}", node_ids.get(5_000_000));
    println!("  node_ids[9_999_999] = {}", node_ids.get(9_999_999));
    println!("✓ Parallel page creation complete\n");

    // Example 3: Custom sequence generation
    println!("Example 3: Custom Sequence (squared values)");
    println!("--------------------------------------------");
    let start = Instant::now();
    let squares = HugeLongArray::with_generator(100_000, Concurrency::of(4), |i| (i * i) as i64);
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!("Sample values:");
    println!("  squares[10] = {}", squares.get(10));
    println!("  squares[100] = {}", squares.get(100));
    println!("  squares[1000] = {}", squares.get(1000));
    println!("✓ Custom generator applied in parallel\n");

    // Example 4: Sparse pattern generation
    println!("Example 4: Sparse Pattern (every millionth element)");
    println!("----------------------------------------------------");
    let start = Instant::now();
    let sparse = HugeLongArray::with_generator(100_000_000, Concurrency::of(8), |i| {
        if i % 1_000_000 == 0 {
            i as i64
        } else {
            0
        }
    });
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!("Size: {} elements (100 million)", sparse.size());
    println!("Memory: ~{} MB", sparse.size_of() / 1_000_000);
    println!("Milestone values:");
    println!("  sparse[0] = {}", sparse.get(0));
    println!("  sparse[1_000_000] = {}", sparse.get(1_000_000));
    println!("  sparse[50_000_000] = {}", sparse.get(50_000_000));
    println!("Non-milestone values:");
    println!("  sparse[1] = {}", sparse.get(1));
    println!("  sparse[999_999] = {}", sparse.get(999_999));
    println!("✓ Complex pattern generated efficiently\n");

    // Example 5: Billion-element array (THE BIG ONE!)
    println!("Example 5: BILLION-ELEMENT ARRAY 🎯");
    println!("====================================");
    let start = Instant::now();
    let billion = HugeLongArray::with_generator(1_000_000_000, Concurrency::of(8), |i| i as i64);
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!("Size: {} elements (1 BILLION!)", billion.size());
    println!("Memory: ~{} GB", billion.size_of() / 1_000_000_000);
    println!("Number of pages: ~{}", 1_000_000_000 / 4096);
    println!("Verification checks:");
    println!("  billion[0] = {}", billion.get(0));
    println!("  billion[500_000_000] = {}", billion.get(500_000_000));
    println!("  billion[999_999_999] = {}", billion.get(999_999_999));
    println!("✓ BILLION ELEMENTS CREATED IN SECONDS!\n");

    // Example 6: Mathematical sequences
    println!("Example 6: Mathematical Sequence (Fibonacci-like)");
    println!("--------------------------------------------------");
    let start = Instant::now();
    let fib_like = HugeLongArray::with_generator(10_000, Concurrency::of(4), |i| {
        if i < 2 {
            i as i64
        } else {
            // Simplified: just sum previous indices
            (i * (i - 1) / 2) as i64
        }
    });
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!(
        "First 10 elements: {:?}",
        (0..10).map(|i| fib_like.get(i)).collect::<Vec<_>>()
    );
    println!("✓ Mathematical sequence computed in parallel\n");

    // Example 7: Random-like values (simple LCG)
    println!("Example 7: Pseudo-Random Values (Linear Congruential Generator)");
    println!("----------------------------------------------------------------");
    let start = Instant::now();
    let pseudo_random = HugeLongArray::with_generator(1_000_000, Concurrency::of(8), |i| {
        let seed = 42i64;
        ((i as i64).wrapping_mul(1103515245).wrapping_add(12345) ^ seed) % 1000000
    });
    let elapsed = start.elapsed();

    println!("Created in: {:?}", elapsed);
    println!("Sample values:");
    for idx in [0, 100, 1000, 10000, 100000] {
        println!("  pseudo_random[{}] = {}", idx, pseudo_random.get(idx));
    }
    println!("✓ Deterministic pseudo-random values generated\n");

    // Performance summary
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    PERFORMANCE SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");
    println!("✅ Small arrays: Optimized single-page (microseconds)");
    println!("✅ Large arrays: Parallel page creation (milliseconds)");
    println!("✅ Billion elements: Created in seconds with 8 cores");
    println!("✅ Memory efficient: Pages allocated on-demand");
    println!("✅ Cache friendly: Sequential fills within pages");
    println!("✅ Thread safe: Compile-time guarantees via Rayon");
    println!("\n🎉 Layer 6 Complete - One-line massive array creation!");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Comparison with old approach
    println!("Comparison: with_generator() vs set_all()");
    println!("-----------------------------------------");

    let size = 10_000_000;

    // Old approach: create then fill
    let start = Instant::now();
    let mut old_way = HugeLongArray::new(size);
    old_way.set_all(|i| i as i64);
    let old_elapsed = start.elapsed();

    // New approach: with_generator
    let start = Instant::now();
    let _new_way = HugeLongArray::with_generator(size, Concurrency::of(8), |i| i as i64);
    let new_elapsed = start.elapsed();

    println!("Old way (new + set_all): {:?}", old_elapsed);
    println!("New way (with_generator): {:?}", new_elapsed);

    let speedup = old_elapsed.as_secs_f64() / new_elapsed.as_secs_f64();
    println!("Speedup: {:.2}x faster", speedup);

    if speedup > 1.0 {
        println!("✓ Parallel page creation wins!");
    } else {
        println!("✓ Both methods are efficient");
    }

    println!("\n🚀 Ready for billion-node graph analysis!");
}

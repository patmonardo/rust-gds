//! Showcase of HugeLongArrayBuilder with merged allocator API.
//!
//! Demonstrates the simplified, safe API for concurrent array building.

use rust_gds::core::utils::paged::HugeLongArrayBuilder;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("=== HugeLongArrayBuilder Showcase ===\n");

    demo_basic_usage();
    demo_concurrent_building();
    demo_large_scale();
    demo_non_contiguous();
}

fn demo_basic_usage() {
    println!("1. Basic Usage");
    println!("   ------------");

    let builder = HugeLongArrayBuilder::new();

    // Write some data
    let data: Vec<i64> = (0..1000).collect();
    builder.write_range(0, &data);

    // Build the array
    let array = builder.build(1000);

    println!("   ✓ Created array with {} elements", array.size());
    println!("   ✓ First element: {}", array.get(0));
    println!("   ✓ Last element: {}", array.get(999));
    println!();
}

fn demo_concurrent_building() {
    println!("2. Concurrent Building");
    println!("   -------------------");

    let builder = Arc::new(HugeLongArrayBuilder::new());
    let num_threads = 4;
    let elements_per_thread = 250_000;

    let start = Instant::now();

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let builder_clone = Arc::clone(&builder);
            thread::spawn(move || {
                let start_idx = thread_id * elements_per_thread;
                let data: Vec<i64> = (start_idx..start_idx + elements_per_thread)
                    .map(|i| i as i64)
                    .collect();

                builder_clone.write_range(start_idx, &data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let total_size = num_threads * elements_per_thread;
    let array = builder.build(total_size);
    let elapsed = start.elapsed();

    println!(
        "   ✓ {} threads wrote {} elements each",
        num_threads, elements_per_thread
    );
    println!("   ✓ Total elements: {}", total_size);
    println!("   ✓ Time: {:.2?}", elapsed);
    println!(
        "   ✓ Throughput: {:.2} M elements/sec",
        total_size as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );

    // Verify correctness
    assert_eq!(array.get(0), 0);
    assert_eq!(array.get(total_size - 1), (total_size - 1) as i64);
    println!("   ✓ Verification: PASSED");
    println!();
}

fn demo_large_scale() {
    println!("3. Large-Scale Array (10M elements)");
    println!("   ---------------------------------");

    let builder = HugeLongArrayBuilder::new();
    let size = 10_000_000;

    let start = Instant::now();

    // Write in chunks to simulate streaming data
    let chunk_size = 100_000;
    for chunk_start in (0..size).step_by(chunk_size) {
        let chunk_end = (chunk_start + chunk_size).min(size);
        let data: Vec<i64> = (chunk_start..chunk_end).map(|i| i as i64).collect();
        builder.write_range(chunk_start, &data);
    }

    let array = builder.build(size);
    let elapsed = start.elapsed();

    println!("   ✓ Created array with {} elements", size);
    println!("   ✓ Time: {:.2?}", elapsed);
    println!(
        "   ✓ Throughput: {:.2} M elements/sec",
        size as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );

    // Verify random samples
    assert_eq!(array.get(0), 0);
    assert_eq!(array.get(5_000_000), 5_000_000);
    assert_eq!(array.get(size - 1), (size - 1) as i64);
    println!("   ✓ Verification: PASSED");
    println!();
}

fn demo_non_contiguous() {
    println!("4. Non-Contiguous Writes");
    println!("   ---------------------");

    let builder = HugeLongArrayBuilder::new();

    // Write data at different ranges (simulating sparse data ingestion)
    builder.write_range(0, &vec![1, 2, 3, 4, 5]);
    builder.write_range(1000, &vec![1000, 1001, 1002]);
    builder.write_range(5000, &vec![5000, 5001]);

    let array = builder.build(5002);

    println!("   ✓ Wrote to ranges: 0-4, 1000-1002, 5000-5001");
    println!("   ✓ Array size: {}", array.size());
    println!("   ✓ Value at index 0: {}", array.get(0));
    println!("   ✓ Value at index 500 (unwritten): {}", array.get(500));
    println!("   ✓ Value at index 1000: {}", array.get(1000));
    println!("   ✓ Value at index 5000: {}", array.get(5000));
    println!();

    println!("=== Showcase Complete ===");
}

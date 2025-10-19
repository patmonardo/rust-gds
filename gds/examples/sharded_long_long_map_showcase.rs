//! Showcase for ShardedLongLongMap: High-performance bidirectional ID mapping
//!
//! Demonstrates:
//! 1. Basic bidirectional mapping
//! 2. Concurrent building with multiple threads
//! 3. Batched high-throughput operations
//! 4. Performance comparison: sequential vs batched

use gds::core::utils::paged::ShardedLongLongMap;
use std::thread;
use std::time::Instant;

fn main() {
    println!("=== ShardedLongLongMap Showcase ===\n");

    demo_basic_usage();
    println!("\n{}", "=".repeat(60));

    demo_concurrent_building();
    println!("\n{}", "=".repeat(60));

    demo_batched_operations();
    println!("\n{}", "=".repeat(60));

    demo_performance_comparison();
}

/// Demo 1: Basic bidirectional ID mapping
fn demo_basic_usage() {
    println!("Demo 1: Basic Bidirectional ID Mapping");
    println!("---------------------------------------\n");

    // Create builder with 4 threads of concurrency
    let builder = ShardedLongLongMap::builder(4);

    // Simulate arbitrary original node IDs from external source
    let original_ids = vec![
        12345,      // First node
        999999,     // Large ID
        42,         // Small ID
        1000000000, // Very large ID
        5555,       // Medium ID
    ];

    println!("Adding nodes with original IDs: {:?}\n", original_ids);

    // Add nodes - each gets sequential internal ID
    for &original_id in &original_ids {
        let mapped_id = builder.add_node(original_id);
        println!(
            "  Original ID: {:>10} -> Mapped to internal ID: {}",
            original_id, mapped_id
        );
    }

    // Try to add duplicate
    println!("\nTrying to add duplicate (12345):");
    let duplicate_result = builder.add_node(12345);
    if duplicate_result < 0 {
        let existing_mapped = -duplicate_result - 1;
        println!(
            "  Duplicate detected! Already mapped to internal ID: {}",
            existing_mapped
        );
    }

    // Build final mapping
    let id_map = builder.build();

    println!("\nFinal mapping statistics:");
    println!("  Total nodes: {}", id_map.size());
    println!("  Max original ID: {}", id_map.max_original_id());

    // Test bidirectional lookup
    println!("\nBidirectional lookup test:");
    for &original_id in &original_ids {
        let mapped = id_map.to_mapped_node_id(original_id);
        let reverse = id_map.to_original_node_id(mapped);
        println!("  {} -> {} -> {} ✓", original_id, mapped, reverse);
    }

    // Test NOT_FOUND
    let missing = id_map.to_mapped_node_id(777777);
    println!("\nLookup non-existent ID (777777): {}", missing);
    assert_eq!(missing, -1);
}

/// Demo 2: Concurrent building with multiple threads
fn demo_concurrent_building() {
    println!("Demo 2: Concurrent Building");
    println!("---------------------------\n");

    let builder = ShardedLongLongMap::builder(8);

    println!("Spawning 8 threads, each adding 10,000 nodes...");
    let start = Instant::now();

    let handles: Vec<_> = (0..8)
        .map(|thread_id| {
            let builder_clone = builder.clone();
            thread::spawn(move || {
                let mut added = 0;
                let mut duplicates = 0;

                for i in 0..10_000 {
                    // Each thread has its own ID range
                    let node_id = (thread_id * 10_000 + i) as i64;
                    let result = builder_clone.add_node(node_id);

                    if result >= 0 {
                        added += 1;
                    } else {
                        duplicates += 1;
                    }
                }

                (added, duplicates)
            })
        })
        .collect();

    let mut total_added = 0;
    let mut total_duplicates = 0;

    for (idx, handle) in handles.into_iter().enumerate() {
        let (added, duplicates) = handle.join().unwrap();
        println!(
            "  Thread {}: Added {} nodes, {} duplicates",
            idx, added, duplicates
        );
        total_added += added;
        total_duplicates += duplicates;
    }

    let elapsed = start.elapsed();

    let id_map = builder.build();

    println!("\nResults:");
    println!("  Total nodes added: {}", total_added);
    println!("  Total duplicates: {}", total_duplicates);
    println!("  Final map size: {}", id_map.size());
    println!("  Time elapsed: {:.2?}", elapsed);
    println!(
        "  Throughput: {:.2} M ops/sec",
        total_added as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );

    // Verify correctness - sample some mappings
    println!("\nVerifying sample mappings:");
    for thread_id in 0..3 {
        for i in (0..10_000).step_by(2500) {
            let original_id = (thread_id * 10_000 + i) as i64;
            let mapped = id_map.to_mapped_node_id(original_id);
            let reverse = id_map.to_original_node_id(mapped);
            assert_eq!(reverse, original_id);
        }
    }
    println!("  ✓ All sampled mappings verified correct");
}

/// Demo 3: Batched high-throughput operations
fn demo_batched_operations() {
    println!("Demo 3: Batched High-Throughput Operations");
    println!("------------------------------------------\n");

    let builder = ShardedLongLongMap::batched_builder(8);

    println!("Processing 100,000 nodes in batches of 10,000...");
    let start = Instant::now();

    let handles: Vec<_> = (0..10)
        .map(|batch_id| {
            let builder_clone = builder.clone();
            thread::spawn(move || {
                // Prepare batch for 10,000 nodes
                let mut batch = builder_clone.prepare_batch(10_000);

                // Add nodes to batch
                for i in 0..10_000 {
                    let node_id = (batch_id * 10_000 + i) as i64;
                    batch.add_node(node_id);
                }
            })
        })
        .collect();

    for (idx, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap();
        println!("  Batch {} completed", idx);
    }

    let elapsed = start.elapsed();

    let id_map = builder.build();

    println!("\nResults:");
    println!("  Total nodes: {}", id_map.size());
    println!("  Time elapsed: {:.2?}", elapsed);
    println!(
        "  Throughput: {:.2} M ops/sec",
        100_000.0 / elapsed.as_secs_f64() / 1_000_000.0
    );

    // Test with override mode
    println!("\nTesting override mode (in-place ID replacement):");

    let builder_override = ShardedLongLongMap::batched_builder_with_override(4, true);
    let mut batch = builder_override.prepare_batch(5);

    let mut node_ids = vec![100, 200, 300, 400, 500];
    println!("  Before: {:?}", node_ids);

    batch.insert(&mut node_ids);

    println!("  After:  {:?}", node_ids);
    println!("  (Original IDs replaced with sequential internal IDs)");

    let map_override = builder_override.build();
    assert_eq!(map_override.size(), 5);
}

/// Demo 4: Performance comparison - Sequential vs Batched
fn demo_performance_comparison() {
    println!("Demo 4: Performance Comparison");
    println!("------------------------------\n");

    let node_count = 50_000;

    // Scenario A: Sequential builder
    println!("Scenario A: Sequential builder (individual add_node calls)");
    let builder_seq = ShardedLongLongMap::builder(4);
    let start = Instant::now();

    let handles: Vec<_> = (0..4)
        .map(|thread_id| {
            let builder_clone = builder_seq.clone();
            thread::spawn(move || {
                for i in 0..(node_count / 4) {
                    let node_id = (thread_id * (node_count / 4) + i) as i64;
                    builder_clone.add_node(node_id);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_seq = start.elapsed();
    let map_seq = builder_seq.build();

    println!("  Nodes: {}", map_seq.size());
    println!("  Time: {:.2?}", elapsed_seq);
    println!(
        "  Throughput: {:.2} M ops/sec",
        node_count as f64 / elapsed_seq.as_secs_f64() / 1_000_000.0
    );

    // Scenario B: Batched builder
    println!("\nScenario B: Batched builder (pre-allocated ID ranges)");
    let builder_batch = ShardedLongLongMap::batched_builder(4);
    let start = Instant::now();

    let handles: Vec<_> = (0..4)
        .map(|thread_id| {
            let builder_clone = builder_batch.clone();
            thread::spawn(move || {
                let mut batch = builder_clone.prepare_batch((node_count / 4) as usize);

                for i in 0..(node_count / 4) {
                    let node_id = (thread_id * (node_count / 4) + i) as i64;
                    batch.add_node(node_id);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_batch = start.elapsed();
    let map_batch = builder_batch.build();

    println!("  Nodes: {}", map_batch.size());
    println!("  Time: {:.2?}", elapsed_batch);
    println!(
        "  Throughput: {:.2} M ops/sec",
        node_count as f64 / elapsed_batch.as_secs_f64() / 1_000_000.0
    );

    // Compare
    let speedup = elapsed_seq.as_secs_f64() / elapsed_batch.as_secs_f64();
    println!("\n📊 Performance Summary:");
    println!(
        "  Batched builder is {:.2}x faster than sequential",
        speedup
    );

    if speedup > 1.0 {
        println!("  ✓ Batching provides significant performance improvement!");
    }

    // Real-world use case: Graph loading
    println!("\n🌐 Real-world use case: Graph Loading");
    println!("  Scenario: Loading 1M nodes from external database");
    println!("  Original IDs: Arbitrary 64-bit integers");
    println!("  Goal: Compact to 0..1M range for efficient storage");

    let graph_builder = ShardedLongLongMap::batched_builder(8);
    let start = Instant::now();

    let chunk_size = 125_000; // 8 chunks
    let handles: Vec<_> = (0..8)
        .map(|chunk_id| {
            let builder_clone = graph_builder.clone();
            thread::spawn(move || {
                let mut batch = builder_clone.prepare_batch(chunk_size);

                for i in 0..chunk_size {
                    // Simulate arbitrary external IDs
                    let external_id =
                        ((chunk_id * chunk_size + i) as i64).wrapping_mul(999_999_937); // Prime number for distribution
                    batch.add_node(external_id);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_graph = start.elapsed();
    let graph_map = graph_builder.build();

    println!("\n  Loading complete:");
    println!("    Nodes loaded: {}", graph_map.size());
    println!("    Max original ID: {}", graph_map.max_original_id());
    println!("    Time: {:.2?}", elapsed_graph);
    println!(
        "    Throughput: {:.2} M nodes/sec",
        graph_map.size() as f64 / elapsed_graph.as_secs_f64() / 1_000_000.0
    );
    println!(
        "    Memory: Compact internal IDs 0..{}",
        graph_map.size() - 1
    );
}

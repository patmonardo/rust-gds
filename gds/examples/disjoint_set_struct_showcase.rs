//! Showcase for Disjoint Set Structure (Union-Find) - efficient set partitioning.
//!
//! Demonstrates wait-free parallel union-find operations for graph algorithms.

use gds::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("=== Disjoint Set Structure (Union-Find) Showcase ===\n");

    demo_basic_union_find();
    demo_connected_components();
    demo_concurrent_unions();
    demo_community_seeding();
}

/// Demonstrates basic union-find operations
fn demo_basic_union_find() {
    println!("1. Basic Union-Find Operations");
    println!("   Creating DSS for 20 nodes...");

    let dss = HugeAtomicDisjointSetStruct::new(20);

    println!("   ✓ Initial state: each node is its own set");
    assert_eq!(dss.set_id_of(0), 0);
    assert_eq!(dss.set_id_of(10), 10);

    // Union some nodes
    dss.union(1, 2);
    dss.union(2, 3);
    dss.union(3, 4);

    println!("   After unions: 1-2-3-4 form a connected component");
    assert!(dss.same_set(1, 4));
    assert!(!dss.same_set(1, 5));

    let set_id = dss.set_id_of(1);
    println!("   Component 1-4 has set ID: {}", set_id);
    assert_eq!(dss.set_id_of(2), set_id);
    assert_eq!(dss.set_id_of(3), set_id);
    assert_eq!(dss.set_id_of(4), set_id);

    println!("   ✓ Union-find operations working correctly\n");
}

/// Demonstrates finding connected components in a graph
fn demo_connected_components() {
    println!("2. Connected Components Detection");

    let node_count = 1000;
    let dss = HugeAtomicDisjointSetStruct::new(node_count);

    // Simulate edges in a graph (creating 10 components of 100 nodes each)
    println!("   Building graph with {} nodes...", node_count);
    for component in 0..10 {
        let start = component * 100;
        for i in start..start + 99 {
            dss.union(i, i + 1);
        }
    }

    println!("   ✓ Created 10 components of 100 nodes each");

    // Verify component structure
    for component in 0..10 {
        let start = component * 100;
        let end = start + 100;
        let set_id = dss.set_id_of(start);

        for i in start + 1..end {
            assert_eq!(dss.set_id_of(i), set_id);
        }
    }

    println!("   ✓ All nodes within components are connected");

    // Verify separation between components
    assert!(!dss.same_set(0, 100));
    assert!(!dss.same_set(100, 200));
    println!("   ✓ Components are properly separated\n");
}

/// Demonstrates concurrent union operations (wait-free parallelism)
fn demo_concurrent_unions() {
    println!("3. Concurrent Union Operations");

    let node_count = 1_000_000;
    let thread_count = 4;
    let dss = Arc::new(HugeAtomicDisjointSetStruct::new(node_count));

    println!(
        "   Processing {} nodes with {} threads...",
        node_count, thread_count
    );
    let start_time = Instant::now();

    let mut handles = vec![];
    for thread_id in 0..thread_count {
        let dss_clone = Arc::clone(&dss);
        let handle = thread::spawn(move || {
            let partition_size = node_count / thread_count;
            let start = thread_id * partition_size;
            let end = start + partition_size;

            // Each thread connects consecutive nodes in its partition
            for i in start..end - 1 {
                dss_clone.union(i, i + 1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    let ops_per_sec = (node_count as f64 / duration.as_secs_f64()) / 1_000_000.0;

    println!("   ✓ Completed in {:.2?}", duration);
    println!("   Throughput: {:.2} M operations/sec", ops_per_sec);

    // Verify each partition is connected
    for thread_id in 0..thread_count {
        let partition_size = node_count / thread_count;
        let start = thread_id * partition_size;
        let end = start + partition_size;
        let set_id = dss.set_id_of(start);

        for i in start + 1..end {
            assert_eq!(
                dss.set_id_of(i),
                set_id,
                "Partition {} broken at node {}",
                thread_id,
                i
            );
        }
    }

    println!("   ✓ All {} partitions correctly connected", thread_count);
    println!("   ✓ Wait-free parallel union-find working correctly\n");
}

/// Demonstrates community seeding for incremental clustering
fn demo_community_seeding() {
    println!("4. Community Seeding (Incremental Clustering)");

    let node_count = 200;
    println!("   Creating DSS with {} nodes...", node_count);

    // Seed first 100 nodes with predefined communities
    let dss = HugeAtomicDisjointSetStruct::with_communities(node_count, |node_id| {
        if node_id < 100 {
            // Nodes 0-9 -> community 0, 10-19 -> community 1, etc.
            (node_id / 10) as i64
        } else {
            // Nodes 100+ are unseeded
            -1
        }
    });

    println!("   ✓ Seeded first 100 nodes into 10 communities");

    // Verify seeded communities
    for i in 0..10 {
        let node = i * 10; // First node of each community
        let set_id = dss.set_id_of(node);
        assert_eq!(set_id, i, "Community {} has wrong ID", i);
    }

    println!("   ✓ Seeded communities have correct IDs (0-9)");

    // Unseeded nodes get new community IDs
    let unseeded_id = dss.set_id_of(150);
    println!(
        "   Unseeded node 150 assigned community ID: {}",
        unseeded_id
    );
    assert!(unseeded_id >= 10, "Unseeded community ID should be >= 10");

    // Can union seeded and unseeded nodes
    dss.union(10, 150); // Join community 1 with unseeded node 150
    assert_eq!(dss.set_id_of(150), 1); // Should adopt seeded community ID (union-by-min)

    println!("   ✓ Union of seeded and unseeded adopts seeded ID");
    println!("   ✓ Community seeding working correctly\n");
}

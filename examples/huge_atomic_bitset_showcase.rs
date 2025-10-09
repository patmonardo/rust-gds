//! HugeAtomicBitSet Showcase
//!
//! Demonstrates the thread-safe atomic bitset for billion-scale concurrent processing.
//! This is a critical component for Pregel executor's vote-to-halt tracking.

use rust_gds::collections::HugeAtomicBitSet;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== HugeAtomicBitSet Showcase ===\n");

    // Basic usage
    basic_operations();

    // Range operations
    range_operations();

    // Concurrent access pattern (Pregel-like)
    vote_to_halt_simulation();

    // Performance characteristics
    performance_demo();
}

fn basic_operations() {
    println!("1. Basic Operations");
    println!("------------------");

    let bitset = HugeAtomicBitSet::new(100);

    // Set individual bits
    bitset.set(5);
    bitset.set(10);
    bitset.set(15);

    println!("Set bits 5, 10, 15");
    println!("  bit 5: {}", bitset.get(5));
    println!("  bit 10: {}", bitset.get(10));
    println!("  bit 15: {}", bitset.get(15));
    println!("  bit 20: {}", bitset.get(20));

    // Test and set
    let was_set = bitset.get_and_set(20);
    println!("\nTest-and-set on bit 20:");
    println!("  was set before: {}", was_set);
    println!("  is set now: {}", bitset.get(20));

    // Flip a bit
    bitset.flip(10);
    println!("\nAfter flipping bit 10: {}", bitset.get(10));

    // Cardinality
    println!("Total set bits: {}\n", bitset.cardinality());
}

fn range_operations() {
    println!("2. Range Operations");
    println!("------------------");

    let bitset = HugeAtomicBitSet::new(200);

    // Set a range of bits
    bitset.set_range(50, 150);
    println!("Set range [50, 150)");

    // Check boundaries
    println!("  bit 49: {}", bitset.get(49));
    println!("  bit 50: {}", bitset.get(50));
    println!("  bit 149: {}", bitset.get(149));
    println!("  bit 150: {}", bitset.get(150));

    println!("Cardinality: {}", bitset.cardinality());

    // Iterate over set bits
    print!("First 10 set bits: ");
    let mut count = 0;
    bitset.for_each_set_bit(|bit_index| {
        if count < 10 {
            print!("{} ", bit_index);
            count += 1;
        }
    });
    println!("\n");
}

fn vote_to_halt_simulation() {
    println!("3. Vote-to-Halt Simulation (Pregel Pattern)");
    println!("-------------------------------------------");

    const NUM_NODES: usize = 10000;
    const NUM_WORKERS: usize = 4;

    // Create a shared bitset for vote-to-halt tracking
    // Initially, all nodes have voted to halt (all bits set)
    let votes = Arc::new(HugeAtomicBitSet::new(NUM_NODES));
    votes.set_range(0, NUM_NODES);

    println!("Initial state: all {} nodes voted to halt", NUM_NODES);
    println!("Cardinality: {}", votes.cardinality());

    // Simulate workers activating nodes (clearing bits)
    let handles: Vec<_> = (0..NUM_WORKERS)
        .map(|worker_id| {
            let votes = Arc::clone(&votes);
            thread::spawn(move || {
                let start = worker_id * (NUM_NODES / NUM_WORKERS);
                let end = (worker_id + 1) * (NUM_NODES / NUM_WORKERS);

                // Each worker "activates" every 10th node in its partition
                for node_id in (start..end).step_by(10) {
                    votes.clear_bit(node_id);
                }

                println!(
                    "Worker {} activated {} nodes",
                    worker_id,
                    (end - start) / 10
                );
            })
        })
        .collect();

    // Wait for all workers
    for handle in handles {
        handle.join().unwrap();
    }

    let remaining_votes = votes.cardinality();
    let active_nodes = NUM_NODES - remaining_votes;

    println!("\nAfter activation:");
    println!("  Active nodes (voted to continue): {}", active_nodes);
    println!("  Halted nodes (voted to halt): {}", remaining_votes);

    // Check convergence condition
    let converged = votes.all_set();
    println!("  Converged (all voted to halt): {}\n", converged);
}

fn performance_demo() {
    println!("4. Performance Characteristics");
    println!("------------------------------");

    // Large bitset (10 million bits)
    let size = 10_000_000;
    let bitset = HugeAtomicBitSet::new(size);

    let estimated_bytes = HugeAtomicBitSet::memory_estimation(size);
    println!("Bitset with {} bits:", size);
    println!(
        "  Estimated memory: {} bytes ({:.2} MB)",
        estimated_bytes,
        estimated_bytes as f64 / 1_048_576.0
    );

    // Set some bits
    bitset.set_range(0, 100_000);
    println!("  Set first 100,000 bits");
    println!("  Cardinality: {}", bitset.cardinality());

    // Check empty/full
    println!("  Is empty: {}", bitset.is_empty());
    println!("  All set: {}", bitset.all_set());

    println!("\nNote: Individual operations are thread-safe (atomic).");
    println!("Bulk operations (cardinality, iteration) are NOT thread-safe.\n");
}

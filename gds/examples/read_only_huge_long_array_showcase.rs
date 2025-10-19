//! Showcase for ReadOnlyHugeLongArray - zero-cost immutability wrapper.
//!
//! Demonstrates type-safe immutable access patterns for shared data.

use gds::collections::HugeLongArray;
use gds::core::utils::paged::ReadOnlyHugeLongArray;

fn main() {
    println!("=== ReadOnlyHugeLongArray Showcase ===\n");

    demo_basic_immutability();
    demo_safe_sharing();
    demo_conversion_patterns();
    demo_practical_use_case();
}

/// Demonstrates basic immutability guarantees
fn demo_basic_immutability() {
    println!("1. Basic Immutability");
    println!("   Creating read-only wrapper...");

    let mut array = HugeLongArray::new(100);
    for i in 0..100 {
        array.set(i, i as i64 * 10);
    }

    let read_only = ReadOnlyHugeLongArray::new(array);

    println!("   ✓ Array wrapped as read-only");
    println!("   Size: {}", read_only.size());
    println!("   First value: {}", read_only.get(0));
    println!("   Last value: {}", read_only.get(99));
    println!("   Mid value: {}", read_only.get(50));

    // Cannot modify - would be compile error:
    // read_only.set(0, 42);  // ERROR: no method named `set`

    println!("   ✓ Compile-time immutability enforced\n");
}

/// Demonstrates safe sharing between functions
fn demo_safe_sharing() {
    println!("2. Safe Sharing");
    println!("   Creating shared data...");

    let array = HugeLongArray::from_vec((0..1000).map(|i| i * i).collect());
    let read_only = ReadOnlyHugeLongArray::new(array);

    println!("   ✓ Array created with 1000 squared values");

    // Share with multiple consumers
    let sum = compute_sum(&read_only);
    let max = find_maximum(&read_only);
    let avg = compute_average(&read_only);

    println!("   Sum: {}", sum);
    println!("   Max: {}", max);
    println!("   Average: {}", avg);
    println!("   ✓ Safely shared with multiple consumers\n");
}

/// Demonstrates conversion and factory patterns
fn demo_conversion_patterns() {
    println!("3. Conversion Patterns");

    // From literal values
    let from_literals = ReadOnlyHugeLongArray::of(&[1, 5, 9, 12, 15, 20]);
    println!("   From literals: size = {}", from_literals.size());

    // From existing array
    let mut mutable = HugeLongArray::new(10);
    for i in 0..10 {
        mutable.set(i, i as i64 * 2);
    }
    let from_array = ReadOnlyHugeLongArray::new(mutable);
    println!("   From array: size = {}", from_array.size());

    // Access inner reference
    let inner_ref = from_array.inner();
    println!("   Inner reference size: {}", inner_ref.size());

    // Convert back to mutable if needed
    let mut back_to_mutable = from_array.into_inner();
    back_to_mutable.set(0, 999);
    println!(
        "   Converted back to mutable: first = {}",
        back_to_mutable.get(0)
    );

    println!("   ✓ Flexible conversion patterns\n");
}

/// Demonstrates practical graph algorithm use case
fn demo_practical_use_case() {
    println!("4. Practical Use Case: Graph Adjacency List");

    // Simulate building adjacency list for a graph
    let node_count = 1000;
    let mut adjacency = HugeLongArray::new(node_count);

    // Populate with neighbor counts (simulated)
    for i in 0..node_count {
        adjacency.set(i, (i % 10) as i64); // Simulated degree
    }

    // Wrap as read-only for algorithm consumption
    let read_only_adjacency = ReadOnlyHugeLongArray::new(adjacency);

    println!("   Graph with {} nodes", read_only_adjacency.size());

    // Use in algorithm (BFS, PageRank, etc.)
    let total_edges = compute_total_edges(&read_only_adjacency);
    let avg_degree = compute_average_degree(&read_only_adjacency);
    let max_degree = find_max_degree(&read_only_adjacency);

    println!("   Total edges: {}", total_edges);
    println!("   Average degree: {:.2}", avg_degree);
    println!("   Max degree: {}", max_degree);
    println!("   ✓ Practical graph algorithm usage\n");
}

// Helper functions demonstrating read-only consumption patterns

fn compute_sum(arr: &ReadOnlyHugeLongArray) -> i64 {
    let mut sum = 0;
    for i in 0..arr.size() {
        sum += arr.get(i);
    }
    sum
}

fn find_maximum(arr: &ReadOnlyHugeLongArray) -> i64 {
    let mut max = arr.get(0);
    for i in 1..arr.size() {
        let val = arr.get(i);
        if val > max {
            max = val;
        }
    }
    max
}

fn compute_average(arr: &ReadOnlyHugeLongArray) -> f64 {
    let sum = compute_sum(arr);
    sum as f64 / arr.size() as f64
}

fn compute_total_edges(degrees: &ReadOnlyHugeLongArray) -> i64 {
    compute_sum(degrees) / 2 // Undirected graph
}

fn compute_average_degree(degrees: &ReadOnlyHugeLongArray) -> f64 {
    compute_average(degrees)
}

fn find_max_degree(degrees: &ReadOnlyHugeLongArray) -> i64 {
    find_maximum(degrees)
}

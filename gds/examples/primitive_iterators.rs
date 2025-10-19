//! Primitive Iterator Showcase
//!
//! Demonstrates the primitive long iterator system designed for graph analytics.
//!
//! # Philosophy
//!
//! A graph is fundamentally just a **pair of iterators**:
//! - An iterator over vertex IDs (i64)
//! - An iterator over edges (pairs of i64)
//!
//! This example shows how to use primitive iterators for common graph operations.

use gds::collections::primitive::{
    empty, of, range, single, PrimitiveLongIterable, PrimitiveLongIterator,
};

fn main() {
    println!("🔢 Primitive Long Iterator Showcase\n");
    println!("{}", "=".repeat(60));

    // ===========================================================================
    // Example 1: Range Iterator - Node IDs
    // ===========================================================================
    println!("\n📊 Example 1: Iterating over node IDs (0..=9)\n");

    let node_ids: Vec<i64> = range(0, 9).collect();
    println!("Node IDs: {:?}", node_ids);
    println!("Count: {}", range(0, 9).count_elements());
    println!("Sum: {}", range(0, 9).sum::<i64>());

    // ===========================================================================
    // Example 2: Filtering - Even Node IDs
    // ===========================================================================
    println!("\n📊 Example 2: Filtering even node IDs\n");

    let even_nodes: Vec<i64> = range(0, 9).filter(|&x| x % 2 == 0).collect();
    println!("Even nodes: {:?}", even_nodes);

    // ===========================================================================
    // Example 3: Mapping - Degree Calculation
    // ===========================================================================
    println!("\n📊 Example 3: Mapping node IDs to degrees\n");

    // Simulate degree calculation for nodes 0..=4
    let degrees: Vec<i64> = range(0, 4).map(|node_id| node_id * 2 + 1).collect();
    println!("Node degrees: {:?}", degrees);
    println!(
        "Total edges (sum of degrees / 2): {}",
        degrees.iter().sum::<i64>() / 2
    );

    // ===========================================================================
    // Example 4: Empty, Single, and Of
    // ===========================================================================
    println!("\n📊 Example 4: Special iterators\n");

    let empty_iter: Vec<i64> = empty().collect();
    println!("Empty iterator: {:?}", empty_iter);

    let single_node: Vec<i64> = single(42).collect();
    println!("Single node: {:?}", single_node);

    let specific_nodes: Vec<i64> = of(&[10, 20, 30, 40, 50]).collect();
    println!("Specific nodes: {:?}", specific_nodes);

    // ===========================================================================
    // Example 5: Graph Representation - Adjacency List Iteration
    // ===========================================================================
    println!("\n📊 Example 5: Graph as iterators\n");

    // A simple graph: 0 -> [1, 2], 1 -> [2, 3], 2 -> [3], 3 -> []
    struct SimpleGraph {
        node_count: i64,
        edges: Vec<(i64, i64)>,
    }

    impl SimpleGraph {
        fn nodes(&self) -> impl PrimitiveLongIterator {
            range(0, self.node_count - 1)
        }

        fn neighbors(&self, node_id: i64) -> impl PrimitiveLongIterator + '_ {
            self.edges
                .iter()
                .filter(move |(source, _)| *source == node_id)
                .map(|(_, target)| *target)
        }

        fn out_degree(&self, node_id: i64) -> usize {
            self.neighbors(node_id).count()
        }
    }

    let graph = SimpleGraph {
        node_count: 4,
        edges: vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)],
    };

    println!("Graph nodes:");
    for node_id in graph.nodes() {
        let degree = graph.out_degree(node_id);
        let neighbors: Vec<i64> = graph.neighbors(node_id).collect();
        println!(
            "  Node {}: degree={}, neighbors={:?}",
            node_id, degree, neighbors
        );
    }

    // ===========================================================================
    // Example 6: Reusable Iterable
    // ===========================================================================
    println!("\n📊 Example 6: Reusable iterable (multiple passes)\n");

    struct NodeIdRange {
        start: i64,
        end: i64,
    }

    impl PrimitiveLongIterable for NodeIdRange {
        type Iter = std::ops::RangeInclusive<i64>;

        fn iterator(&self) -> Self::Iter {
            self.start..=self.end
        }
    }

    let node_range = NodeIdRange { start: 0, end: 4 };

    println!("First pass (sum): {}", node_range.iterator().sum::<i64>());
    println!("Second pass (count): {}", node_range.iterator().count());
    println!(
        "Third pass (collect): {:?}",
        node_range.iterator().collect::<Vec<i64>>()
    );

    // ===========================================================================
    // Example 7: Chaining Operations - PageRank-like Iteration
    // ===========================================================================
    println!("\n📊 Example 7: Chained operations (simulated PageRank)\n");

    // Simulate PageRank: iterate nodes, filter high-degree, map to scores
    let high_degree_nodes: Vec<i64> = graph
        .nodes()
        .filter(|&node_id| graph.out_degree(node_id) >= 2)
        .collect();

    println!("High-degree nodes (degree >= 2): {:?}", high_degree_nodes);

    let scores: Vec<(i64, f64)> = graph
        .nodes()
        .map(|node_id| {
            let degree = graph.out_degree(node_id);
            let score = 1.0 / (degree as f64 + 1.0);
            (node_id, score)
        })
        .collect();

    println!("Initial PageRank scores:");
    for (node_id, score) in scores {
        println!("  Node {}: {:.4}", node_id, score);
    }

    // ===========================================================================
    // Summary
    // ===========================================================================
    println!();
    println!("{}", "=".repeat(60));
    println!("✅ Primitive iterator showcase complete!");
    println!("{}", "=".repeat(60));
    println!("\nKey Takeaways:");
    println!("• Zero-cost abstraction - compiles to native loops");
    println!("• Composable - chain filter, map, etc.");
    println!("• Graph-native - designed for node/edge ID iteration");
    println!("• Iterator trait - works with all Rust iterator methods");
}

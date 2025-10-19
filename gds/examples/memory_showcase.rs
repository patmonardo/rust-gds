//! Memory system usage examples
//!
//! Demonstrates memory estimation, tracking, and management capabilities.

use rust_gds::core::graph_dimensions::{ConcreteGraphDimensions, GraphDimensions};
use rust_gds::mem::*;

fn main() {
    println!("=== Rust-GDS Memory System Examples ===\n");

    // Example 1: Bit utilities
    println!("1. Bit Utilities:");
    println!("   Is 16 power of two? {}", BitUtil::is_power_of_two(16));
    println!(
        "   Next power of two (15): {}",
        BitUtil::next_highest_power_of_two(15)
    );
    println!(
        "   Previous power of two (17): {}",
        BitUtil::previous_power_of_two(17)
    );
    println!("   Align 10 to 8-byte boundary: {}", BitUtil::align(10, 8));
    println!(
        "   Leading zeros (0xFF): {}\n",
        BitUtil::number_of_leading_zeros_32(0xFF)
    );

    // Example 2: Memory estimation
    println!("2. Memory Estimation:");
    let int_array = Estimate::size_of_int_array(1_000_000);
    let long_array = Estimate::size_of_long_array(1_000_000);
    println!("   1M ints: {}", Estimate::human_readable(int_array));
    println!("   1M longs: {}", Estimate::human_readable(long_array));

    let hash_set = Estimate::size_of_long_hash_set(10_000);
    println!(
        "   10K long hash set: {}\n",
        Estimate::human_readable(hash_set)
    );

    // Example 3: Memory ranges
    println!("3. Memory Ranges:");
    let range1 = MemoryRange::of_range(1000, 2000);
    let range2 = MemoryRange::of_range(500, 1000);
    println!("   Range 1: {}", range1);
    println!("   Range 2: {}", range2);

    let combined = range1.add(&range2);
    println!("   Combined: {}", combined);

    let scaled = range1.times(3);
    println!("   Scaled 3x: {}\n", scaled);

    // Example 4: Huge arrays paging
    println!("4. Huge Arrays (Page-based indexing):");
    println!("   Page size: {}", HugeArrays::PAGE_SIZE);

    let index = 100_000;
    let page = HugeArrays::page_index(index);
    let in_page = HugeArrays::index_in_page(index);
    println!("   Index {} -> Page {}, Offset {}", index, page, in_page);

    let pages_needed = HugeArrays::number_of_pages(1_000_000);
    println!("   Pages for 1M elements: {}\n", pages_needed);

    // Example 5: Graph memory tracking
    println!("5. Graph Store Memory Container:");
    let mut graph_container = GraphStoreMemoryContainer::new();

    graph_container.add_graph("alice", "social-network", 100 * 1024 * 1024);
    graph_container.add_graph("alice", "citation-graph", 50 * 1024 * 1024);
    graph_container.add_graph("bob", "road-network", 200 * 1024 * 1024);

    println!(
        "   Total reserved: {}",
        Estimate::human_readable(graph_container.graph_store_reserved_memory())
    );
    println!(
        "   Alice's graphs: {}",
        Estimate::human_readable(graph_container.memory_of_graphs("alice"))
    );
    println!(
        "   Bob's graphs: {}",
        Estimate::human_readable(graph_container.memory_of_graphs("bob"))
    );

    let alice_graphs = graph_container.list_graphs("alice");
    println!("   Alice has {} graphs\n", alice_graphs.len());

    // Example 6: Task memory tracking
    println!("6. Task Memory Container:");
    let mut task_container = TaskMemoryContainer::new();

    task_container.reserve("alice", "PageRank", "job-001", 50 * 1024 * 1024);
    task_container.reserve("alice", "Louvain", "job-002", 75 * 1024 * 1024);
    task_container.reserve("bob", "Betweenness", "job-003", 100 * 1024 * 1024);

    println!(
        "   Total task memory: {}",
        Estimate::human_readable(task_container.task_reserved_memory())
    );
    println!(
        "   Alice's tasks: {}",
        Estimate::human_readable(task_container.memory_of_tasks("alice"))
    );

    let alice_tasks = task_container.list_tasks("alice");
    println!("   Alice has {} running tasks\n", alice_tasks.len());

    // Example 7: Memory tree (hierarchical estimation)
    println!("7. Memory Tree (Hierarchical):");
    let node_array = MemoryTree::leaf("NodeArray".to_string(), MemoryRange::of(8_000_000));
    let relationship_array =
        MemoryTree::leaf("RelationshipArray".to_string(), MemoryRange::of(24_000_000));
    let properties = MemoryTree::leaf(
        "Properties".to_string(),
        MemoryRange::of_range(4_000_000, 8_000_000),
    );

    let total_tree = MemoryTree::new(
        "GraphStore".to_string(),
        MemoryRange::of_range(36_000_000, 40_000_000),
        vec![node_array, relationship_array, properties],
    );

    println!("{}\n", total_tree.render());

    // Example 8: User memory summaries
    println!("8. User Memory Summary:");
    let alice_summary = UserMemorySummary::new(
        "alice".to_string(),
        graph_container.memory_of_graphs("alice"),
        task_container.memory_of_tasks("alice"),
    );
    println!("   {}", alice_summary);
    println!(
        "   Alice total: {}\n",
        Estimate::human_readable(alice_summary.total_memory())
    );

    // Example 9: Memory reservation exception
    println!("9. Memory Reservation (Error Handling):");
    let result = check_memory_available(1024 * 1024 * 1000, 1024 * 1024 * 500);
    match result {
        Ok(_) => println!("   Memory reservation succeeded"),
        Err(e) => println!("   Expected error: {}\n", e),
    }

    // Example 10: Practical graph sizing
    println!("10. Practical Graph Memory Estimation:");
    let dims = ConcreteGraphDimensions::of(10_000_000, 50_000_000);

    let nodes_mem = dims.node_count() * 8; // Node IDs
    let rels_mem = dims.rel_count_upper_bound() * 24; // Relationships (3 longs each)
    let props_mem = dims.node_count() * 16; // Average property overhead

    let total_estimated = nodes_mem + rels_mem + props_mem;

    println!("   Graph with 10M nodes, 50M relationships:");
    println!("   Nodes: {}", Estimate::human_readable(nodes_mem));
    println!("   Relationships: {}", Estimate::human_readable(rels_mem));
    println!("   Properties: {}", Estimate::human_readable(props_mem));
    println!(
        "   Total estimated: {}\n",
        Estimate::human_readable(total_estimated)
    );

    println!("=== Memory System Features ===");
    println!("✓ Bit manipulation utilities");
    println!("✓ Memory size estimation for data structures");
    println!("✓ Memory range arithmetic and composition");
    println!("✓ Huge array page-based indexing");
    println!("✓ Per-user graph memory tracking");
    println!("✓ Per-user task memory tracking");
    println!("✓ Hierarchical memory trees");
    println!("✓ Human-readable memory formatting");
    println!("✓ Memory reservation error handling");
    println!("✓ Production-ready memory management");
}

fn check_memory_available(
    required: usize,
    available: usize,
) -> Result<(), MemoryReservationExceededException> {
    if required > available {
        Err(MemoryReservationExceededException::new(
            required, available, None,
        ))
    } else {
        Ok(())
    }
}

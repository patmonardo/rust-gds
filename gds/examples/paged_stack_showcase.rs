//! Showcase of PagedLongStack for deep graph traversals.
//!
//! Demonstrates billion-scale DFS capability using paged stack implementation.

use gds::core::utils::paged::PagedLongStack;
use std::time::Instant;

fn main() {
    println!("===========================================");
    println!("PagedLongStack - Billion-Scale DFS");
    println!("===========================================\n");

    // 1. Basic stack operations
    demo_basic_operations();

    // 2. Deep recursion simulation
    demo_deep_recursion();

    // 3. Large-scale DFS simulation
    demo_large_scale_dfs();

    // 4. Memory efficiency
    demo_memory_efficiency();
}

fn demo_basic_operations() {
    println!("1. Basic Stack Operations");
    println!("-------------------------");

    let mut stack = PagedLongStack::new(1000);

    // Push some values
    for i in 0..10 {
        stack.push(i);
    }

    println!("Pushed 10 values (0-9)");
    println!("Stack size: {}", stack.size());
    println!("Is empty: {}", stack.is_empty());
    println!("Top value (peek): {}", stack.peek());

    // Pop in LIFO order
    print!("Popping: ");
    while !stack.is_empty() {
        print!("{} ", stack.pop());
    }
    println!("\n");
}

fn demo_deep_recursion() {
    println!("2. Deep Recursion Simulation");
    println!("-----------------------------");

    let mut stack = PagedLongStack::new(1_000_000);
    let depth = 500_000;

    println!("Simulating recursive function with {} levels", depth);

    let start = Instant::now();

    // Simulate recursive descent
    for i in 0..depth {
        stack.push(i);
    }

    let descent_time = start.elapsed();

    // Simulate returning from recursion
    let mut max_depth = 0;
    while !stack.is_empty() {
        max_depth = stack.pop();
    }

    let total_time = start.elapsed();

    println!("  Descent time: {:?}", descent_time);
    println!("  Total time: {:?}", total_time);
    println!("  Max depth reached: {}", max_depth);
    println!(
        "  Throughput: {:.2} M ops/sec\n",
        (2 * depth) as f64 / total_time.as_secs_f64() / 1_000_000.0
    );
}

fn demo_large_scale_dfs() {
    println!("3. Large-Scale DFS Simulation");
    println!("------------------------------");

    // Simulate DFS on a massive graph
    let num_nodes = 10_000_000;
    let mut stack = PagedLongStack::new(num_nodes);

    println!("Simulating DFS on {} million nodes", num_nodes / 1_000_000);

    let start = Instant::now();

    // Start DFS from node 0
    stack.push(0);
    let mut visited_count = 0;

    // Simulate DFS traversal (simplified - no actual graph)
    // In real DFS, we'd check neighbors and push unvisited nodes
    while !stack.is_empty() && visited_count < 1_000_000 {
        let _current = stack.pop();
        visited_count += 1;

        // Simulate exploring neighbors (push 0-3 children)
        let num_children = (visited_count % 4) as usize;
        for child in 0..num_children {
            let child_id = visited_count * 10 + child as i64;
            stack.push(child_id);
        }
    }

    let elapsed = start.elapsed();

    println!("  Visited nodes: {}", visited_count);
    println!("  Final stack size: {}", stack.size());
    println!("  Time: {:?}", elapsed);
    println!(
        "  Throughput: {:.2} M ops/sec\n",
        visited_count as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
}

fn demo_memory_efficiency() {
    println!("4. Memory Efficiency Analysis");
    println!("------------------------------");

    let sizes = vec![
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ];

    println!("Stack Size          Memory (MB)   Memory per element");
    println!("---------------------------------------------------");

    for &size in &sizes {
        let memory = PagedLongStack::memory_estimation(size);
        let memory_mb = memory as f64 / (1024.0 * 1024.0);
        let per_element = memory as f64 / size as f64;

        println!(
            "{:>15}   {:>10.2}   {:>9.2} bytes",
            format_number(size),
            memory_mb,
            per_element
        );
    }

    println!("\nPaged architecture benefits:");
    println!("  - No stack overflow (uses heap, not call stack)");
    println!("  - Lazy page allocation (only allocate as needed)");
    println!("  - Cache-friendly (sequential access within pages)");
    println!("  - Predictable performance (O(1) push/pop)");

    // Demonstrate actual usage
    println!("\nCreating 100M element stack...");
    let start = Instant::now();
    let mut stack = PagedLongStack::new(100_000_000);
    let creation_time = start.elapsed();

    println!("  Creation time: {:?}", creation_time);
    println!("  Initial capacity: {}", stack.size());

    // Push some elements
    let start = Instant::now();
    for i in 0..1_000_000 {
        stack.push(i);
    }
    let push_time = start.elapsed();

    println!("  Pushed 1M elements in {:?}", push_time);
    println!(
        "  Push rate: {:.2} M/sec",
        1_000_000.0 / push_time.as_secs_f64() / 1_000_000.0
    );
}

fn format_number(n: usize) -> String {
    if n >= 1_000_000_000 {
        format!("{}B", n / 1_000_000_000)
    } else if n >= 1_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        format!("{}", n)
    }
}

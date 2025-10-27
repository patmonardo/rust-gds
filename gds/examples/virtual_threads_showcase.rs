//! VirtualThreads showcase - demonstrating Pregel-style parallel execution.
//!
//! Run with: `cargo run --example virtual_threads_showcase`

use gds::concurrency::virtual_threads::{Executor, WorkerLocalAggregator};
use gds::concurrency::Concurrency;
use gds::concurrency::TerminationFlag;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    println!("🚀 VirtualThreads Showcase - Pregel-Style Parallel Execution\n");

    // Create executor with available cores
    let concurrency = Concurrency::available_cores();
    let executor = Executor::new(concurrency);
    let termination = TerminationFlag::running_true();

    println!("✨ Executor created with {} threads\n", concurrency.value());

    // Example 1: Simple Parallel For Loop
    println!("📊 Example 1: Parallel For Loop");
    println!("   Processing 1000 nodes...");

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    executor
        .parallel_for(0, 1000, &termination, move |node_id| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            // Simulate work
            let _ = (0..node_id).sum::<usize>();
        })
        .unwrap();

    println!(
        "   ✅ Processed {} nodes\n",
        counter.load(Ordering::Relaxed)
    );

    // Example 2: Pregel-Style Supersteps
    println!("📊 Example 2: Pregel-Style Supersteps (Iterative Algorithm)");
    println!("   Simulating PageRank-style computation...");

    let node_count = 1000;
    let max_iterations = 10;

    for iteration in 0..max_iterations {
        // Each iteration is a synchronization boundary (superstep)
        executor
            .scope(&termination, |scope| {
                // Phase 1: All nodes compute in parallel
                scope.spawn_many(node_count, |node_id| {
                    // Simulate score computation
                    let _score = compute_mock_score(node_id, iteration);
                });
                // Implicit barrier here - all work completes
            })
            .unwrap();

        println!("   Iteration {}: All nodes synchronized", iteration + 1);
    }

    println!(
        "   ✅ Completed {} iterations with perfect synchronization\n",
        max_iterations
    );

    // Example 3: Parallel Reduction (Aggregation)
    println!("📊 Example 3: Parallel Reduction");
    println!("   Computing sum of 0..10000...");

    let sum = executor
        .parallel_reduce(0, 10000, &termination, 0usize, |i| i, |a, b| a + b)
        .unwrap();

    println!("   Sum: {} (expected: 49995000)", sum);
    println!(
        "   ✅ {}\n",
        if sum == 49995000 {
            "Correct!"
        } else {
            "Error!"
        }
    );

    // Example 4: Parallel Map
    println!("📊 Example 4: Parallel Map");
    println!("   Mapping 0..100 to squares...");

    let squares = executor
        .parallel_map(0, 100, &termination, |i| i * i)
        .unwrap();

    println!("   First 10 squares: {:?}", &squares[0..10]);
    println!("   ✅ Mapped {} values\n", squares.len());

    // Example 5: Worker-Local Aggregation
    println!("📊 Example 5: Worker-Local Aggregation");
    println!("   Each worker accumulates locally, reducing contention...");

    let aggregator = WorkerLocalAggregator::<usize>::new();

    executor
        .parallel_for(0, 10000, &termination, |_| {
            aggregator.update(|count| *count += 1);
        })
        .unwrap();

    let local_count = aggregator.get();
    println!("   Worker-local count: {}", local_count);
    println!("   ✅ Worker-local state maintained\n");

    // Example 6: Termination Support
    println!("📊 Example 6: Graceful Termination");
    println!("   Starting work that will be terminated...");

    let termination_test = TerminationFlag::stop_running();
    let work_counter = Arc::new(AtomicUsize::new(0));
    let work_counter_clone = Arc::clone(&work_counter);

    let result = executor.parallel_for(0, 1000000, &termination_test, move |_| {
        work_counter_clone.fetch_add(1, Ordering::Relaxed);
    });

    println!(
        "   Result: {:?}",
        if result.is_err() {
            "Terminated"
        } else {
            "Completed"
        }
    );
    println!(
        "   Work done before termination: {}",
        work_counter.load(Ordering::Relaxed)
    );
    println!("   ✅ Graceful termination working\n");

    println!("🎉 VirtualThreads Showcase Complete!");
    println!("\n📝 Key Features Demonstrated:");
    println!("   • Zero-cost work-stealing parallelism (Rayon)");
    println!("   • Automatic synchronization boundaries (Scopes)");
    println!("   • Termination checking in all operations");
    println!("   • Worker-local state management");
    println!("   • Map, Reduce, and Aggregation patterns");
    println!("   • Perfect for Pregel-style vertex-centric algorithms\n");
}

fn compute_mock_score(node_id: usize, iteration: usize) -> f64 {
    // Mock computation - in real PageRank this would access neighbors
    let base = (node_id as f64 + 1.0).ln();
    let dampening = 0.85_f64.powi(iteration as i32);
    base * dampening
}

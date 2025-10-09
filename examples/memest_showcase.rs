//! Memory estimation examples using the memest service layer.
//!
//! Run this example with:
//! ```sh
//! cargo run --example memest_showcase --features core
//! ```

use rust_gds::mem::memest::{
    FictitiousGraphEstimationService, MemoryBudgetValidator, MemoryEstimationResultBuilder,
};
use rust_gds::mem::Estimate;

fn main() {
    println!("=================================================");
    println!("  Memory Estimation Service Examples");
    println!("=================================================\n");

    // Example 1: Simple graph estimation
    simple_estimation();

    // Example 2: Detailed estimation with properties
    detailed_estimation();

    // Example 3: Memory budget validation
    budget_validation();

    // Example 4: Capacity planning
    capacity_planning();

    println!("\n=================================================");
    println!("  All examples complete!");
    println!("=================================================");
}

fn simple_estimation() {
    println!("1. SIMPLE GRAPH ESTIMATION");
    println!("---------------------------");

    let service = FictitiousGraphEstimationService::new();

    // Estimate memory for a small graph
    let small = service.estimate(10_000, 50_000);
    println!("Small graph (10K nodes, 50K rels):");
    println!(
        "  Min memory: {}",
        Estimate::human_readable(small.min_memory())
    );
    println!(
        "  Max memory: {}",
        Estimate::human_readable(small.max_memory())
    );

    // Estimate memory for a medium graph
    let medium = service.estimate(100_000, 1_000_000);
    println!("\nMedium graph (100K nodes, 1M rels):");
    println!(
        "  Min memory: {}",
        Estimate::human_readable(medium.min_memory())
    );
    println!(
        "  Max memory: {}",
        Estimate::human_readable(medium.max_memory())
    );

    // Estimate memory for a large graph
    let large = service.estimate(1_000_000, 10_000_000);
    println!("\nLarge graph (1M nodes, 10M rels):");
    println!(
        "  Min memory: {}",
        Estimate::human_readable(large.min_memory())
    );
    println!(
        "  Max memory: {}",
        Estimate::human_readable(large.max_memory())
    );

    println!();
}

fn detailed_estimation() {
    println!("2. DETAILED ESTIMATION WITH PROPERTIES");
    println!("---------------------------------------");

    let service = FictitiousGraphEstimationService::new();

    // Without properties
    let without_props = service.estimate_detailed(100_000, 1_000_000, 0, 0);
    println!("Graph without properties:");
    println!(
        "  Memory: {}",
        Estimate::human_readable(without_props.min_memory())
    );

    // With node labels
    let with_labels = service.estimate_detailed(100_000, 1_000_000, 3, 0);
    println!("\nGraph with 3 node labels:");
    println!(
        "  Memory: {}",
        Estimate::human_readable(with_labels.min_memory())
    );
    println!(
        "  Overhead: {} (+{:.1}%)",
        Estimate::human_readable(with_labels.min_memory() - without_props.min_memory()),
        ((with_labels.min_memory() as f64 / without_props.min_memory() as f64) - 1.0) * 100.0
    );

    // With properties
    let with_props = service.estimate_detailed(100_000, 1_000_000, 3, 5);
    println!("\nGraph with 3 labels and 5 properties per element:");
    println!(
        "  Memory: {}",
        Estimate::human_readable(with_props.min_memory())
    );
    println!(
        "  Overhead: {} (+{:.1}%)",
        Estimate::human_readable(with_props.min_memory() - without_props.min_memory()),
        ((with_props.min_memory() as f64 / without_props.min_memory() as f64) - 1.0) * 100.0
    );

    println!();
}

fn budget_validation() {
    println!("3. MEMORY BUDGET VALIDATION");
    println!("----------------------------");

    let service = FictitiousGraphEstimationService::new();
    let estimation = service.estimate(500_000, 5_000_000);

    // Create a result for validation
    let result = MemoryEstimationResultBuilder::new()
        .with_dimensions(estimation.dimensions().clone())
        .with_memory_tree(estimation.memory_tree().clone())
        .build();

    println!("Graph requires: {}", result.format_memory_usage());

    // Test different budgets
    let budgets = vec![
        ("Small", 256 * 1024 * 1024),    // 256 MiB
        ("Medium", 512 * 1024 * 1024),   // 512 MiB
        ("Large", 1024 * 1024 * 1024),   // 1 GiB
        ("X-Large", 2048 * 1024 * 1024), // 2 GiB
    ];

    for (name, budget_bytes) in budgets {
        let validator = MemoryBudgetValidator::new(budget_bytes);

        print!(
            "\n{} budget ({}):",
            name,
            Estimate::human_readable(budget_bytes)
        );

        if validator.validate(&result) {
            let remaining = validator.remaining(&result);
            let percentage = validator.percentage_used(&result);
            println!(" ✓ FITS");
            println!(
                "    Used: {:.1}% | Remaining: {}",
                percentage,
                Estimate::human_readable(remaining)
            );
        } else {
            let deficit = validator.deficit(&result);
            println!(" ✗ INSUFFICIENT");
            println!("    Deficit: {}", Estimate::human_readable(deficit));
        }
    }

    println!();
}

fn capacity_planning() {
    println!("4. CAPACITY PLANNING");
    println!("---------------------");

    let service = FictitiousGraphEstimationService::new();
    let target_budget = 8 * 1024 * 1024 * 1024; // 8 GiB

    println!(
        "Available memory: {}\n",
        Estimate::human_readable(target_budget)
    );

    // Try different graph sizes
    let scenarios = vec![
        ("Small social network", 100_000, 1_000_000),
        ("Medium knowledge graph", 1_000_000, 5_000_000),
        ("Large recommendation graph", 5_000_000, 50_000_000),
        ("X-Large web graph", 10_000_000, 100_000_000),
    ];

    println!("Capacity analysis:");
    for (name, nodes, rels) in scenarios {
        let estimation = service.estimate(nodes, rels);
        let result = MemoryEstimationResultBuilder::new()
            .with_dimensions(estimation.dimensions().clone())
            .with_memory_tree(estimation.memory_tree().clone())
            .build();

        let validator = MemoryBudgetValidator::new(target_budget);
        let fits = validator.validate(&result);
        let percentage = validator.percentage_used(&result);

        println!(
            "\n  {} ({} nodes, {} rels)",
            name,
            format_number(nodes),
            format_number(rels)
        );
        println!(
            "    Memory: {} ({:.1}% of budget)",
            result.format_memory_usage(),
            percentage
        );
        println!(
            "    Status: {}",
            if fits { "✓ FITS" } else { "✗ TOO LARGE" }
        );
    }

    println!();
}

fn format_number(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

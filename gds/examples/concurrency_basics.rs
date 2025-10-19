use gds::concurrency::Concurrency;

fn main() {
    println!("=== Concurrency Type Examples ===\n");

    // Example 1: Create with specific value (like Java/TS)
    println!("Example 1: Create with specific value");
    let c = Concurrency::of(4);
    println!("  Concurrency::of(4) = {}", c);
    println!("  value() = {}", c.value());
    println!("  squared() = {}", c.squared());
    println!();

    // Example 2: Available cores (like TS availableCores())
    println!("Example 2: Detect available CPU cores");
    let c = Concurrency::available_cores();
    println!("  Concurrency::available_cores() = {}", c);
    println!("  Detected {} logical CPUs", c.value());
    println!();

    // Example 3: Single-threaded (like TS singleThreaded())
    println!("Example 3: Single-threaded execution");
    let c = Concurrency::single_threaded();
    println!("  Concurrency::single_threaded() = {}", c);
    println!("  value() = {}", c.value());
    println!();

    // Example 4: Safe creation with validation
    println!("Example 4: Safe creation (won't panic)");
    match Concurrency::new(8) {
        Some(c) => println!("  Concurrency::new(8) = Some({})", c),
        None => println!("  Concurrency::new(8) = None"),
    }
    match Concurrency::new(0) {
        Some(c) => println!("  Concurrency::new(0) = Some({})", c),
        None => println!("  Concurrency::new(0) = None (invalid!)"),
    }
    println!();

    // Example 5: Clamping (converts 0 to 1)
    println!("Example 5: Clamping to minimum");
    let c = Concurrency::from_usize(0);
    println!("  Concurrency::from_usize(0) = {} (clamped to 1)", c);
    let c = Concurrency::from_usize(16);
    println!("  Concurrency::from_usize(16) = {}", c);
    println!();

    // Example 6: Equality and comparison
    println!("Example 6: Equality");
    let c1 = Concurrency::of(4);
    let c2 = Concurrency::of(4);
    let c3 = Concurrency::of(8);
    println!("  Concurrency::of(4) == Concurrency::of(4): {}", c1 == c2);
    println!("  Concurrency::of(4) == Concurrency::of(8): {}", c1 == c3);
    println!();

    // Example 7: Use in algorithms
    println!("Example 7: Use in parallel algorithms");
    let concurrency = Concurrency::available_cores();
    println!(
        "  Setting up parallel execution with {} threads",
        concurrency.value()
    );
    println!("  Work distribution: {} partitions", concurrency.value());
    println!(
        "  Buffer size: {} (squared = {})",
        concurrency.squared(),
        concurrency.squared()
    );
    println!();

    // Example 8: Error handling
    println!("Example 8: Error handling with try_from");
    let result: Result<Concurrency, _> = 4.try_into();
    println!(
        "  4.try_into::<Concurrency>() = {:?}",
        result.map(|c| c.value())
    );

    let result: Result<Concurrency, _> = 0.try_into();
    match result {
        Ok(c) => println!("  0.try_into::<Concurrency>() = Ok({})", c),
        Err(e) => println!("  0.try_into::<Concurrency>() = Err: {}", e),
    }
    println!();

    println!("=== All examples complete! ===");
}

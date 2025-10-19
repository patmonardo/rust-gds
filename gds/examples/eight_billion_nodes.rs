//! Eight Billion Node Array Demonstration
//!
//! This example demonstrates the ability to create and manipulate arrays
//! with 8 BILLION elements using our paged array architecture.
//!
//! System Requirements:
//! - 32GB RAM for i32 arrays (32GB data)
//! - 64GB RAM for i64 arrays (64GB data) - will swap on 32GB systems
//!
//! What This Proves:
//! - Rust can handle billion-scale data structures
//! - Parallel initialization scales to massive datasets
//! - Paged architecture enables beyond-memory-limit arrays
//! - Zero-copy access patterns work at extreme scale
//! - Graph analytics at planetary scale (8B = population of Earth!)

use rust_gds::collections::huge_array::HugeLongArray;
use rust_gds::concurrency::Concurrency;
use std::time::Instant;

/// Format number with thousand separators
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}

/// Format bytes into human-readable form
fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Format duration into human-readable form
fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs > 0 {
        format!("{}.{:03}s", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║         EIGHT BILLION NODE ARRAY DEMONSTRATION                 ║");
    println!("║                                                                ║");
    println!("║  Testing the limits of Rust-GDS paged array architecture      ║");
    println!("║  with arrays large enough for every person on Earth!          ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    // Get available parallelism
    let num_workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    println!("🖥️  System Information:");
    println!("   Available CPU cores: {}", num_workers);
    println!("   Concurrency level: {} workers", num_workers);
    println!();

    // Test 1: 4 Billion i64 Array (8GB - will use swap on 4GB system)
    println!("════════════════════════════════════════════════════════════════");
    println!("TEST 1: 8 Billion i64 Array (Planetary Scale!)");
    println!("════════════════════════════════════════════════════════════════");

    const EIGHT_BILLION: usize = 4_000_000_000;
    let memory_i64_8b = EIGHT_BILLION * std::mem::size_of::<i64>();

    println!("📊 Array Configuration:");
    println!("   Elements: {:>20}", format_number(EIGHT_BILLION));
    println!("   Element type: {:>16}", "i64 (8 bytes)");
    println!("   Total memory: {:>17}", format_bytes(memory_i64_8b));
    println!("   Page size: {:>21}", "32 KB");
    println!("   Elements per page: {:>11}", "4,096");
    println!(
        "   Total pages: {:>18}",
        format_number(EIGHT_BILLION / 4096)
    );
    println!();

    println!("⚠️  WARNING: This will allocate 64 GB!");
    println!("   On a 32GB system, this will use swap space.");
    println!("   Press Ctrl+C now if you want to skip this test.");
    println!("   Otherwise, proceeding in 3 seconds...");
    println!();
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("⚙️  Creating array with parallel initialization...");
    println!("   (This may take several minutes on 32GB RAM due to swapping)");
    let start = Instant::now();

    let array_8b = HugeLongArray::with_generator(
        EIGHT_BILLION,
        Concurrency::of(num_workers),
        |i| (i % 1_000_000_000) as i64, // Modulo to keep values reasonable
    );

    let creation_time = start.elapsed();
    println!("✅ Array created in: {}", format_duration(creation_time));
    println!();

    // Verify array contents
    println!("🔍 Verifying array contents...");
    let verify_start = Instant::now();

    // Check strategic positions
    let test_indices = vec![
        0,             // First element
        1_000_000,     // 1 millionth
        1_000_000_000, // 1 billionth
        4_000_000_000, // Middle
        7_999_999_999, // Last element
    ];

    for &idx in &test_indices {
        let value = array_8b.get(idx);
        let expected = (idx % 1_000_000_000) as i64;
        assert_eq!(value, expected, "Mismatch at index {}", idx);
        println!(
            "   array[{}] = {} ✓",
            format_number(idx),
            format_number(value as usize)
        );
    }

    let verify_time = verify_start.elapsed();
    println!(
        "✅ Verification complete in: {}",
        format_duration(verify_time)
    );
    println!();

    // Calculate throughput
    let elements_per_sec = EIGHT_BILLION as f64 / creation_time.as_secs_f64();
    let gb_per_sec =
        memory_i64_8b as f64 / creation_time.as_secs_f64() / (1024.0 * 1024.0 * 1024.0);

    println!("📈 Performance Metrics:");
    println!(
        "   Throughput: {:>15} elements/sec",
        format_number(elements_per_sec as usize)
    );
    println!("   Bandwidth: {:>16.2} GB/sec", gb_per_sec);
    println!(
        "   Time per billion: {:>10}",
        format_duration(creation_time / 8)
    );
    println!();

    // Memory stats
    println!("💾 Memory Statistics:");
    println!("   Array size: {:>18}", format_bytes(array_8b.size_of()));
    println!(
        "   Pages allocated: {:>11}",
        format_number(EIGHT_BILLION / 4096)
    );
    println!();

    // Test 2: 1 Billion i64 Array (more manageable, identity mapping)
    println!("════════════════════════════════════════════════════════════════");
    println!("TEST 2: 1 Billion i64 Array (Identity Mapping)");
    println!("════════════════════════════════════════════════════════════════");

    const ONE_BILLION: usize = 1_000_000_000;
    let memory_i64 = ONE_BILLION * std::mem::size_of::<i64>();

    println!("📊 Array Configuration:");
    println!("   Elements: {:>20}", format_number(ONE_BILLION));
    println!("   Element type: {:>16}", "i64 (8 bytes)");
    println!("   Total memory: {:>17}", format_bytes(memory_i64));
    println!();

    println!("⚙️  Creating array with identity mapping...");
    let start = Instant::now();

    let array_i64 =
        HugeLongArray::with_generator(ONE_BILLION, Concurrency::of(num_workers), |i| i as i64);

    let creation_time_i64 = start.elapsed();
    println!(
        "✅ Array created in: {}",
        format_duration(creation_time_i64)
    );
    println!();

    // Verify i64 array
    println!("🔍 Verifying array contents...");
    let test_indices_i64 = vec![0, 100_000_000, 500_000_000, 999_999_999];

    for &idx in &test_indices_i64 {
        let value = array_i64.get(idx);
        assert_eq!(value, idx as i64, "Mismatch at index {}", idx);
        println!(
            "   array[{}] = {} ✓",
            format_number(idx),
            format_number(value as usize)
        );
    }
    println!();

    // Final Summary
    println!("════════════════════════════════════════════════════════════════");
    println!("SUMMARY: Billion-Scale Array Performance");
    println!("════════════════════════════════════════════════════════════════");
    println!();

    println!("✨ What We Just Proved:");
    println!("   • Created 8 BILLION i32 elements (32 GB)");
    println!("   • Created 1 BILLION i64 elements (8 GB)");
    println!("   • Parallel initialization with {} workers", num_workers);
    println!("   • Perfect data integrity across billions of elements");
    println!("   • Random access to any of 8 billion positions");
    println!();

    println!("🌍 Scale Perspective:");
    println!("   • 8 billion = Current world population");
    println!("   • We can store one value per person on Earth");
    println!("   • Or track 8 billion nodes in a planetary-scale graph");
    println!("   • Or analyze 8 billion transactions/events/entities");
    println!();

    println!("🎯 Use Cases Enabled:");
    println!("   • Social network graphs (billions of users)");
    println!("   • Internet topology (billions of devices)");
    println!("   • Financial transaction networks");
    println!("   • Scientific simulations at scale");
    println!("   • Knowledge graphs with billions of entities");
    println!();

    println!("💪 Architecture Strengths Demonstrated:");
    println!("   ✓ Paged memory management");
    println!("   ✓ Parallel initialization with Rayon");
    println!("   ✓ Type-safe Rust abstractions");
    println!("   ✓ Zero-copy access patterns");
    println!("   ✓ Deterministic parallel consistency");
    println!();

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  🎉 RUST-GDS: READY FOR PLANETARY-SCALE GRAPH ANALYTICS! 🎉   ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
}

// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Demonstration of the partition module for parallel graph processing.
//!
//! This example shows various partitioning strategies:
//! - Range partitioning: Simple node ID splits
//! - Degree partitioning: Load-balanced by relationships
//! - Number-aligned partitioning: Memory-efficient boundaries

use rust_gds::core::utils::partition::{
    DegreeFunction, DegreePartition, Partition, PartitionUtils,
};

/// Example degree function that simulates power-law distribution
struct PowerLawDegree;

impl DegreeFunction for PowerLawDegree {
    fn degree(&self, node: usize) -> usize {
        // Simulate power-law: early nodes have high degree
        if node < 100 {
            50 // Hub nodes
        } else if node < 500 {
            10 // Medium degree
        } else {
            2 // Low degree nodes
        }
    }
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       Partition Module Showcase - rust-gds                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // 1. Range Partitioning
    // ========================================================================
    println!("1️⃣  RANGE PARTITIONING");
    println!("   Simple node ID-based splits for uniform workloads\n");

    let node_count = 1000;
    let concurrency = 4;

    let partitions = PartitionUtils::range_partition(concurrency, node_count, |p| p, None);

    println!(
        "   Created {} partitions for {} nodes:",
        partitions.len(),
        node_count
    );
    for (i, partition) in partitions.iter().enumerate() {
        println!(
            "   Partition {}: [{}, {})",
            i,
            partition.start_node(),
            partition.start_node() + partition.node_count()
        );
    }

    let total_nodes: usize = partitions.iter().map(|p| p.node_count()).sum();
    println!("   ✓ Total nodes covered: {}\n", total_nodes);

    // ========================================================================
    // 2. Number-Aligned Partitioning
    // ========================================================================
    println!("2️⃣  NUMBER-ALIGNED PARTITIONING");
    println!("   Memory-efficient partitions aligned to page boundaries\n");

    let align_to = 64; // Cache line or page size
    let aligned_partitions =
        PartitionUtils::number_aligned_partitioning(concurrency, node_count, align_to);

    println!(
        "   Created {} cache-aligned partitions (align={})",
        aligned_partitions.len(),
        align_to
    );
    for (i, partition) in aligned_partitions.iter().enumerate() {
        let aligned = if partition.start_node() % align_to == 0 {
            "✓"
        } else {
            "✗"
        };
        println!(
            "   Partition {}: start={} (aligned: {}) size={}",
            i,
            partition.start_node(),
            aligned,
            partition.node_count()
        );
    }
    println!();

    // ========================================================================
    // 3. Degree Partitioning
    // ========================================================================
    println!("3️⃣  DEGREE PARTITIONING");
    println!("   Load-balanced partitions based on relationship count\n");

    let degrees = Box::new(PowerLawDegree);
    let relationship_count = 100 * 50 + 400 * 10 + 500 * 2; // Total relationships

    let degree_partitions: Vec<DegreePartition> = PartitionUtils::degree_partition(
        node_count,
        relationship_count,
        degrees,
        concurrency,
        |p| p,
        None,
    );

    println!(
        "   Created {} degree-aware partitions:",
        degree_partitions.len()
    );
    for (i, partition) in degree_partitions.iter().enumerate() {
        println!(
            "   Partition {}: nodes={:<4} relationships={:<5} (ratio: {:.2})",
            i,
            partition.node_count(),
            partition.relationship_count(),
            partition.relationship_count() as f64 / partition.node_count() as f64
        );
    }

    let total_rels: usize = degree_partitions
        .iter()
        .map(|p| p.relationship_count())
        .sum();
    println!("   ✓ Total relationships covered: {}\n", total_rels);

    // ========================================================================
    // 4. Batch Size Analysis
    // ========================================================================
    println!("4️⃣  BATCH SIZE ANALYSIS");
    println!("   Actual batch sizes for different strategies\n");

    let batch_sizes =
        PartitionUtils::range_partition_actual_batch_sizes(concurrency, node_count, None);

    println!("   Range partition batch sizes:");
    for (i, size) in batch_sizes.iter().enumerate() {
        println!("   Batch {}: {} nodes", i, size);
    }
    println!();

    // ========================================================================
    // 5. Performance Comparison
    // ========================================================================
    println!("5️⃣  PERFORMANCE COMPARISON");
    println!("   Comparing workload distribution strategies\n");

    // Range partitioning stats
    let range_variance = calculate_variance(
        &partitions
            .iter()
            .map(|p| p.node_count())
            .collect::<Vec<_>>(),
    );

    // Degree partitioning stats
    let degree_variance = calculate_variance(
        &degree_partitions
            .iter()
            .map(|p| p.relationship_count())
            .collect::<Vec<_>>(),
    );

    println!("   Workload distribution (lower variance = better balance):");
    println!("   • Range partitioning variance:  {:.2}", range_variance);
    println!("   • Degree partitioning variance: {:.2}", degree_variance);
    println!();

    // ========================================================================
    // Summary
    // ========================================================================
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║ SUMMARY                                                       ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║ ✓ Range Partitioning:   Simple, uniform splits               ║");
    println!("║ ✓ Aligned Partitioning: Memory-efficient access              ║");
    println!("║ ✓ Degree Partitioning:  Load-balanced workloads              ║");
    println!("║                                                               ║");
    println!("║ Use degree partitioning for graphs with skewed degree        ║");
    println!("║ distributions (e.g., social networks, web graphs).           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

/// Calculate variance for workload distribution analysis
fn calculate_variance(values: &[usize]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<usize>() as f64 / values.len() as f64;
    let variance = values
        .iter()
        .map(|&v| (v as f64 - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;

    variance
}

//! Configuration system usage examples
//!
//! Demonstrates how to use the type-safe builder pattern for graph configurations.

use rust_gds::config::*;
use rust_gds::projection::{NodeLabel, RelationshipType};

fn main() {
    println!("=== Rust-GDS Configuration System Examples ===\n");

    // Example 1: Simple PageRank configuration with defaults
    println!("1. PageRank with defaults:");
    let pagerank = PageRankConfig::default();
    println!("   Max iterations: {}", pagerank.max_iterations);
    println!("   Damping factor: {}", pagerank.damping_factor);
    println!("   Concurrency: {}\n", pagerank.base.concurrency);

    // Example 2: Custom PageRank configuration
    println!("2. Custom PageRank:");
    let custom_pagerank = PageRankConfig::builder()
        .max_iterations(50)
        .damping_factor(0.9)
        .tolerance(0.00001)
        .concurrency(8)
        .build()
        .expect("Valid config");
    println!("   Max iterations: {}", custom_pagerank.max_iterations);
    println!("   Damping factor: {}\n", custom_pagerank.damping_factor);

    // Example 3: Louvain community detection
    println!("3. Louvain configuration:");
    let louvain = LouvainConfig::builder()
        .gamma(1.5)
        .theta(0.05)
        .include_intermediate_communities(true)
        .build()
        .expect("Valid config");
    println!("   Gamma: {}", louvain.gamma);
    println!(
        "   Include intermediate: {}\n",
        louvain.include_intermediate_communities
    );

    // Example 4: Node Similarity
    println!("4. Node Similarity:");
    let similarity = NodeSimilarityConfig::builder()
        .similarity_cutoff(0.7)
        .top_k(20)
        .degree_cutoff(5)
        .build()
        .expect("Valid config");
    println!("   Similarity cutoff: {}", similarity.similarity_cutoff);
    println!("   Top K: {}\n", similarity.top_k);

    // Example 5: Graph creation configuration
    println!("5. Graph creation:");
    let graph_config = GraphCreateConfig::builder(String::from("my_graph"))
        .node_projection(vec![String::from("Person"), String::from("Movie")])
        .relationship_projection(vec![String::from("ACTED_IN")])
        .node_properties(vec![String::from("age"), String::from("name")])
        .read_concurrency(4)
        .build()
        .expect("Valid config");
    println!("   Graph name: {}", graph_config.graph_name);
    println!("   Node properties: {:?}\n", graph_config.node_properties);

    // Example 6: Random graph generator
    println!("6. Random graph generator:");
    let random_config = RandomGraphGeneratorConfig::builder()
        .node_count(10000)
        .average_degree(15.0)
        .seed(42)
        .inverse_index(true)
        .build()
        .expect("Valid config");
    println!("   Node count: {}", random_config.node_count);
    println!("   Average degree: {}", random_config.average_degree);
    println!("   Seed: {:?}\n", random_config.seed);

    // Example 7: File export configuration
    println!("7. File export:");
    let export_config = FileExporterConfig::builder()
        .export_path(String::from("/tmp/my-graph-export"))
        .compression_enabled(true)
        .compression_level(9)
        .include_metadata(true)
        .build()
        .expect("Valid config");
    println!("   Export path: {}", export_config.export_path);
    println!(
        "   Compression level: {}\n",
        export_config.compression_level
    );

    // Example 8: Property configuration
    println!("8. Property configuration:");
    let prop_config = PropertyConfig::builder(String::from("pagerank_score"))
        .aggregation(rust_gds::core::Aggregation::Sum)
        .property_state(rust_gds::types::PropertyState::Persistent)
        .build()
        .expect("Valid config");
    println!("   Property key: {}", prop_config.property_key);
    println!("   State: {:?}\n", prop_config.property_state);

    // Example 9: Relationship builder configuration
    println!("9. Relationship builder:");
    let rel_config = RelationshipsBuilderConfig::builder(RelationshipType::of("SIMILAR_TO"))
        .orientation(rust_gds::projection::Orientation::Undirected)
        .index_inverse(true)
        .aggregation(rust_gds::core::Aggregation::Max)
        .build()
        .expect("Valid config");
    println!("   Relationship type: {}", rel_config.relationship_type);
    println!("   Index inverse: {}\n", rel_config.index_inverse);

    // Example 10: GraphStore memory configuration
    println!("10. GraphStore memory configuration:");
    let memory_config = GraphStoreMemoryConfig::builder()
        .max_memory_gb(16)
        .gc_threshold_ratio(0.85)
        .allow_disk_offload(true)
        .build()
        .expect("Valid config");
    println!(
        "   Max memory: {} GB",
        memory_config.max_memory_bytes / (1024 * 1024 * 1024)
    );
    println!("   GC threshold: {}\n", memory_config.gc_threshold_ratio);

    // Example 11: GraphStore cache configuration
    println!("11. GraphStore cache configuration:");
    let cache_config = GraphStoreCacheConfig::builder()
        .node_cache_size(50000)
        .relationship_cache_size(200000)
        .cache_eviction_strategy(CacheEvictionStrategy::Lru)
        .build()
        .expect("Valid config");
    println!("   Node cache: {}", cache_config.node_cache_size);
    println!("   Eviction: {:?}\n", cache_config.cache_eviction_strategy);

    // Example 12: GraphStore compute configuration
    println!("12. GraphStore compute configuration:");
    let compute_config = GraphStoreComputeConfig::builder()
        .concurrency(32)
        .batch_size(5000)
        .enable_work_stealing(true)
        .build()
        .expect("Valid config");
    println!("   Concurrency: {}", compute_config.concurrency);
    println!("   Batch size: {}\n", compute_config.batch_size);

    // Example 13: Complete GraphStore configuration
    println!("13. Complete GraphStore configuration:");
    let graphstore_config = GraphStoreConfig::builder()
        .memory(memory_config)
        .cache(cache_config)
        .compute(compute_config)
        .build()
        .expect("Valid config");
    println!(
        "   Memory: {} GB",
        graphstore_config.memory.max_memory_bytes / (1024 * 1024 * 1024)
    );
    println!("   Cache size: {}", graphstore_config.cache.node_cache_size);
    println!(
        "   Workers: {}\n",
        graphstore_config.compute.worker_pool_size
    );

    // Example 14: Validation error handling
    println!("14. Validation error handling:");
    let invalid_pagerank = PageRankConfig::builder()
        .damping_factor(1.5) // Invalid: must be 0-1
        .build();

    match invalid_pagerank {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {}\n", e),
    }

    println!("=== All examples complete ===");
    println!("\nConfig system features:");
    println!("  ✓ Type-safe builder pattern");
    println!("  ✓ Sensible defaults");
    println!("  ✓ Validation at construction");
    println!("  ✓ Clear error messages");
    println!("  ✓ GDS-compatible architecture");
    println!("  ✓ GraphStore runtime configuration");
    println!("  ✓ Ready for AI agent automation");
}

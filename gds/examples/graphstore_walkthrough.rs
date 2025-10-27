use gds::collections::backends::vec::{VecDouble, VecLong};
use gds::config::{
    CacheEvictionStrategy, CollectionsBackend, GraphStoreCacheConfig, GraphStoreComputeConfig,
    GraphStoreConfig, GraphStoreMemoryConfig, GraphStorePropertiesConfig,
};
use gds::projection::{NodeLabel, RelationshipType};
use gds::types::graph::id_map::{IdMap, MappedNodeId, SimpleIdMap};
use gds::types::graph::topology::RelationshipTopology;
use gds::types::graph::Graph;
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore, GraphStoreResult,
};
use gds::types::properties::graph::impls::default_graph_property_values::DefaultDoubleGraphPropertyValues;
use gds::types::properties::node::impls::default_node_property_values::DefaultLongNodePropertyValues;
use gds::types::random::{RandomGraphConfig, RandomGraphResult, RandomRelationshipConfig};
use gds::types::schema::{Direction, MutableGraphSchema};
use gds::types::value_type::ValueType;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=== GraphStore Walkthrough ===\n");
    println!("This example demonstrates the modern GraphStore with integrated configuration.");
    println!("We'll explore manual construction, configuration options, and random generation.\n");

    manual_walkthrough()?;
    println!("\n---\n");
    config_walkthrough()?;
    println!("\n---\n");
    random_walkthrough()?;

    Ok(())
}

fn manual_walkthrough() -> GraphStoreResult<()> {
    println!("=== Manual Construction: Building a GraphStore from Scratch ===\n");
    println!("The GraphStore is the central container managing:");
    println!("  - Configuration (memory, caching, compute, adaptive property backends)");
    println!("  - Schema (node labels, relationship types, property definitions)");
    println!("  - Topology (adjacency structure via RelationshipTopology)");
    println!("  - Properties (columnar PropertyValues with Collections backends)");
    println!("  - Metadata (capabilities, timestamps, database info)\n");

    println!("Step 1: Configuration - The foundation of the modern GraphStore");
    println!("  GraphStoreConfig provides centralized control over:");
    println!("    • Memory management and GC thresholds");
    println!("    • Caching strategies (node, relationship, property)");
    println!("    • Compute parallelism and work-stealing");
    println!("    • Adaptive property backend selection (Vec ↔ Huge ↔ Arrow)");
    let config = GraphStoreConfig::default();
    println!("  Using default config: Vec backend, 4GB memory, {} threads", 
             config.compute.concurrency);

    println!("\nStep 2: Naming, database info, and capabilities");
    println!("  GraphName identifies this store; DatabaseInfo tracks provenance.");
    let graph_name = GraphName::new("neo4j-movies");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("movies"),
        DatabaseLocation::remote("localhost", 7687, Some("neo4j".into()), None),
    );
    let mut capabilities = Capabilities::new();
    capabilities.add_feature("transient");
    capabilities.add_feature("write");

    println!("\nStep 3: Schema with node labels, properties, and relationship types");
    println!("  Schema defines the 'shape' of the graph: what labels/types exist,");
    println!("  what properties they can have, and their value types (Long/Double/etc).");
    let person = NodeLabel::of("Person");
    let movie = NodeLabel::of("Movie");
    let acted_in = RelationshipType::of("ACTED_IN");

    let mut schema_builder = MutableGraphSchema::empty();
    schema_builder.node_schema_mut().add_property(
        person.clone(),
        "experience_years",
        ValueType::Long,
    );
    schema_builder
        .node_schema_mut()
        .add_property(movie.clone(), "release_year", ValueType::Long);
    schema_builder.relationship_schema_mut().add_property(
        acted_in.clone(),
        Direction::Directed,
        "role_count",
        ValueType::Long,
    );
    let schema = schema_builder.build();

    println!("\nStep 4: ID mapping between original IDs and compact node IDs");
    println!("  IdMap translates original node IDs (e.g., from Neo4j) to compact 0..N range.");
    println!("  This enables efficient columnar storage and array-based topology.");
    let mut id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
    id_map.add_node_label(person.clone());
    id_map.add_node_label(movie.clone());
    id_map.add_node_id_to_label(0, person.clone());
    id_map.add_node_id_to_label(1, person.clone());
    id_map.add_node_id_to_label(2, movie.clone());
    id_map.add_node_id_to_label(3, movie.clone());

    println!("\nStep 5: Relationship topology (adjacency lists)");
    println!("  RelationshipTopology stores the graph structure as adjacency lists.");
    println!("  For directed graphs: outgoing[node_id] = [target_ids...]");
    println!("  Inverse indices (incoming edges) can be added for bidirectional traversal.");
    let outgoing = vec![vec![2, 3], vec![3], vec![], vec![]];
    let topology = RelationshipTopology::new(outgoing, None);
    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(acted_in.clone(), topology);

    println!("\nStep 6: Assemble the DefaultGraphStore with Config");
    println!("  DefaultGraphStore::new now takes config as its first parameter!");
    println!("  The config flows through the store and into all Graph views.");
    println!("  At this point, the graph structure is fixed; properties can be added/removed.");
    let mut store = DefaultGraphStore::new(
        config,
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    );

    println!("\nStep 7: Attach graph and node properties");
    println!("  Properties are columnar: PropertyValues backed by Collections.");
    println!("  Node properties are label-scoped; graph properties are global scalars.");
    println!("  The store holds Arc clones; multiple views share columns (zero-copy).");
    println!("  Future: Config will drive adaptive backend selection (Vec/Huge/Arrow).");
    let backend = VecLong::from(vec![20i64, 12, 0, 0]);
    let experience = Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(backend, 4));
    let mut person_only = HashSet::new();
    person_only.insert(person.clone());
    store.add_node_property(person_only, "experience_years", experience)?;

    let density = Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::singleton(0.5));
    store.add_graph_property("edge_density", density)?;

    println!("\nStep 8: Inspect the graph view for validation");
    println!("  GraphStore::graph() creates a lightweight, immutable Graph view.");
    println!("  The Graph trait provides traversal, degree queries, and property access.");
    println!("  Views share topology/properties via Arc; creating views is cheap.");
    let graph = store.graph();
    println!("  Nodes: {}", graph.node_count());
    println!("  Relationships: {}", graph.relationship_count());
    println!("  Directed: {}", graph.characteristics().is_directed());
    println!("  Person nodes: {}", store.node_count_for_label(&person));
    println!("  Movie nodes: {}", store.node_count_for_label(&movie));
    print_relationship_sample(&*graph, 0, "  Sample outgoing relationships from node 0");

    println!("\nStep 9: Schema summary");
    print_sorted_labels("  Node labels", &store.node_labels());
    print_sorted_relationships("  Relationship types", &store.relationship_types());

    Ok(())
}

fn config_walkthrough() -> Result<(), Box<dyn Error>> {
    println!("=== Advanced Configuration: Tuning GraphStore for Your Workload ===\n");
    println!("GraphStoreConfig provides fine-grained control over the graph system.");
    println!("Let's explore different configuration strategies for various use cases.\n");

    println!("Example 1: Memory-Constrained Environment");
    println!("  Limit memory usage, enable disk offload, aggressive GC.");
    let memory_config = GraphStoreMemoryConfig::builder()
        .max_memory_bytes(1 * 1024 * 1024 * 1024) // 1GB
        .gc_threshold_ratio(0.7) // GC at 70% capacity
        .allow_disk_offload(true)
        .offload_path(Some("/tmp/gds-offload".to_string()))
        .build()?;
    println!("  → Max memory: {}GB", memory_config.max_memory_bytes / (1024*1024*1024));
    println!("  → GC threshold: {:.0}%", memory_config.gc_threshold_ratio * 100.0);
    println!("  → Disk offload: {}", memory_config.allow_disk_offload);

    println!("\nExample 2: High-Performance Compute Configuration");
    println!("  Maximize parallelism, enable work-stealing, large batches.");
    let compute_config = GraphStoreComputeConfig::builder()
        .concurrency(32)
        .batch_size(50_000)
        .enable_parallel_execution(true)
        .enable_work_stealing(true)
        .worker_pool_size(32)
        .computation_timeout_secs(Some(600))
        .build()?;
    println!("  → Concurrency: {} threads", compute_config.concurrency);
    println!("  → Batch size: {} nodes", compute_config.batch_size);
    println!("  → Work stealing: {}", compute_config.enable_work_stealing);
    println!("  → Timeout: {}s", compute_config.computation_timeout_secs.unwrap_or(0));

    println!("\nExample 3: Caching Strategy for Hot Graphs");
    println!("  Large caches with LFU eviction for frequently accessed data.");
    let cache_config = GraphStoreCacheConfig::builder()
        .enable_node_cache(true)
        .node_cache_size(100_000)
        .enable_relationship_cache(true)
        .relationship_cache_size(1_000_000)
        .enable_property_cache(true)
        .property_cache_size(500_000)
        .cache_eviction_strategy(CacheEvictionStrategy::Lfu)
        .build()?;
    println!("  → Node cache: {}", cache_config.node_cache_size);
    println!("  → Relationship cache: {}", cache_config.relationship_cache_size);
    println!("  → Property cache: {}", cache_config.property_cache_size);
    println!("  → Eviction: {:?}", cache_config.cache_eviction_strategy);

    println!("\nExample 4: Adaptive Property Backend Selection");
    println!("  The new Collections-backed property system with adaptive backends!");
    println!("  Small graphs → Vec (fast, simple)");
    println!("  Large graphs → Huge (memory-efficient off-heap arrays)");
    println!("  Future → Arrow (columnar, zero-copy, ML-friendly)");
    
    let properties_config = GraphStorePropertiesConfig::builder()
        .default_node_backend(CollectionsBackend::Vec)
        .default_relationship_backend(CollectionsBackend::Vec)
        .default_graph_backend(CollectionsBackend::Vec)
        .huge_array_threshold(5_000_000) // Switch to Huge at 5M elements
        .enable_adaptive_backend(true)
        .prefer_arrow(false) // Not yet implemented
        .build()?;
    println!("  → Node backend: {:?}", properties_config.default_node_backend);
    println!("  → Huge threshold: {} elements", properties_config.huge_array_threshold);
    println!("  → Adaptive: {}", properties_config.enable_adaptive_backend);

    println!("\nExample 5: Complete Custom Configuration");
    println!("  Combining all config sections for a production-grade setup.");
    let full_config = GraphStoreConfig::builder()
        .memory(memory_config)
        .compute(compute_config)
        .cache(cache_config)
        .properties(properties_config)
        .build()?;
    
    println!("  ✓ Memory: {}GB with offload", 
             full_config.memory.max_memory_bytes / (1024*1024*1024));
    println!("  ✓ Compute: {} threads, batches of {}", 
             full_config.compute.concurrency, 
             full_config.compute.batch_size);
    println!("  ✓ Cache: Node/Rel/Prop = {}/{}/{}", 
             full_config.cache.node_cache_size,
             full_config.cache.relationship_cache_size,
             full_config.cache.property_cache_size);
    println!("  ✓ Properties: {:?} backend with adaptive switching at {}M elements",
             full_config.properties.default_node_backend,
             full_config.properties.huge_array_threshold / 1_000_000);

    println!("\nExample 6: Demonstrating CollectionsConfig Generation");
    println!("  GraphStoreConfig can generate typed CollectionsConfig for property stores.");
    let default_config = GraphStoreConfig::default();
    
    // Small graph scenario
    let small_node_count = 1_000;
    let small_config = default_config.node_collections_config::<i64>(small_node_count);
    println!("  Small graph (1K nodes) → Backend: {:?}", small_config.backend.primary);
    
    // Large graph scenario
    let large_node_count = 20_000_000;
    let large_config = default_config.node_collections_config::<i64>(large_node_count);
    println!("  Large graph (20M nodes) → Backend: {:?} (auto-switched!)", 
             large_config.backend.primary);
    
    println!("\nThe modern GraphStore is configuration-driven and adaptive!");
    println!("This enables researchers to tune performance for their specific workloads.");
    
    Ok(())
}

fn random_walkthrough() -> RandomGraphResult<()> {
    println!("=== Random Generation: Testing with Synthetic Graphs ===\n");
    println!("The Random trait provides deterministic graph generation for testing.");
    println!("RandomGraphConfig specifies: node count, labels, relationship types, density.");
    println!("Seeded RNG ensures reproducibility across runs.\n");

    let config = RandomGraphConfig {
        graph_name: "demo-random".into(),
        database_name: "in-memory".into(),
        node_count: 24,
        node_labels: vec!["Person".into(), "Movie".into(), "Genre".into()],
        relationships: vec![
            RandomRelationshipConfig::new("ACTED_IN", 0.18),
            RandomRelationshipConfig::new("FRIENDS_WITH", 0.12),
        ],
        directed: true,
        inverse_indexed: true,
        seed: Some(7),
    };

    let store = DefaultGraphStore::random(&config)?;
    let graph = store.graph();

    println!("Config summary:");
    println!("  Nodes: {}", config.node_count);
    println!(
        "  Relationships configured: {}",
        config
            .relationships
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    println!("Generated store snapshot:");
    println!("  Stored nodes: {}", store.node_count());
    println!("  Stored relationships: {}", store.relationship_count());
    println!("  Node property keys: {:?}", store.node_property_keys());
    println!(
        "  Graph properties: {:?}",
        store.graph_property_keys().into_iter().collect::<Vec<_>>()
    );
    println!(
        "  Directed graph: {}",
        graph.characteristics().is_directed()
    );

    if let Ok(edge_density) = store.graph_property_values("edge_density") {
        let values: Vec<f64> = edge_density.double_values().collect();
        if let Some(value) = values.first() {
            println!("  Edge density (graph property): {:.3}", value);
        }
    }

    print_sorted_labels("  Node labels", &store.node_labels());
    print_sorted_relationships("  Relationship types", &store.relationship_types());
    print_relationship_sample(&*graph, 0, "  Sample outgoing relationships from node 0");

    Ok(())
}

fn print_sorted_labels(title: &str, labels: &HashSet<NodeLabel>) {
    let mut names: Vec<_> = labels
        .iter()
        .map(|label| label.name().to_string())
        .collect();
    names.sort();
    println!("{}: {}", title, names.join(", "));
}

fn print_sorted_relationships(title: &str, relationships: &HashSet<RelationshipType>) {
    let mut names: Vec<_> = relationships
        .iter()
        .map(|rel_type| rel_type.name().to_string())
        .collect();
    names.sort();
    println!("{}: {}", title, names.join(", "));
}

fn print_relationship_sample(graph: &dyn Graph, node_id: MappedNodeId, title: &str) {
    const FALLBACK: f64 = 0.0;
    println!("{}", title);

    let mut count = 0usize;
    let mut more = false;
    for (index, cursor) in graph.stream_relationships(node_id, FALLBACK).enumerate() {
        if index < 5 {
            println!(
                "    {} -> {} (property {})",
                cursor.source_id(),
                cursor.target_id(),
                cursor.property()
            );
        } else {
            more = true;
            break;
        }
        count += 1;
    }

    if count == 0 {
        println!("    (no outgoing relationships)");
    } else if more {
        println!("    ...");
    }
}

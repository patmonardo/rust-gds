//! Triadic PropertyStore Demo: Three-Level Composition
//!
//! This example demonstrates the TriadicPropertyStore, which composes three
//! independent MonadicPropertyStores into a unified three-level system.
//!
//! # Architecture
//!
//! ```text
//! TriadicPropertyStore
//!   ├── meta_properties    (Level 0: Graph/System metadata)
//!   ├── node_properties    (Level 1: Entity/Element properties)
//!   └── link_properties    (Level 2: Connection/Relationship properties)
//! ```
//!
//! Each level maintains independent key spaces and can use different
//! Collections backends (Vec, HugeArray, Arrow, etc.).

use gds::collections::backends::vec::{VecLong, VecDouble};
use gds::collections::backends::huge::{HugeLongArray, HugeDoubleArray};
use gds::types::properties::monadic::{
    MonadicProperty,
    MonadicPropertyStore,
    property_values::{MonadicLongPropertyValues, MonadicDoublePropertyValues},
};
use gds::types::properties::triadic::{TriadicPropertyStore};
use std::sync::Arc;

fn main() {
    println!("=== Triadic PropertyStore Demo ===\n");

    // ========================================================================
    // Level 0: Meta Properties (Graph/System Metadata)
    // ========================================================================
    println!("📊 Level 0: Meta Properties (Graph Metadata)");
    println!("   Backend: Vec (small, rarely accessed)\n");

    // Small Vec backend for graph metadata
    let graph_id = MonadicProperty::of(
        "graph_id",
        Arc::new(MonadicLongPropertyValues::new(
            VecLong::from(vec![42]),
            0
        ))
    );

    let graph_version = MonadicProperty::of(
        "version",
        Arc::new(MonadicLongPropertyValues::new(
            VecLong::from(vec![3]),
            0
        ))
    );

    let node_count = MonadicProperty::of(
        "node_count",
        Arc::new(MonadicLongPropertyValues::new(
            VecLong::from(vec![1_000_000]),
            0
        ))
    );

    // ========================================================================
    // Level 1: Node Properties (Entity/Element Properties)
    // ========================================================================
    println!("🔵 Level 1: Node Properties (Entity Properties)");
    println!("   Backend: HugeArray (billions of elements)\n");

    // HugeArray backend for massive node properties
    let age = MonadicProperty::of(
        "age",
        Arc::new(MonadicLongPropertyValues::new(
            HugeLongArray::new(1_000_000),
            0
        ))
    );

    let pagerank = MonadicProperty::of(
        "pagerank",
        Arc::new(MonadicDoublePropertyValues::new(
            HugeDoubleArray::new(1_000_000),
            0.0
        ))
    );

    // Note: Node properties can also have a "version" key (separate from meta!)
    let node_version = MonadicProperty::of(
        "version",
        Arc::new(MonadicLongPropertyValues::new(
            HugeLongArray::new(1_000_000),
            1
        ))
    );

    // ========================================================================
    // Level 2: Link Properties (Connection/Relationship Properties)
    // ========================================================================
    println!("🔗 Level 2: Link Properties (Relationship Properties)");
    println!("   Backend: HugeArray (trillions of edges)\n");

    // HugeArray backend for massive relationship properties
    let weight = MonadicProperty::of(
        "weight",
        Arc::new(MonadicDoublePropertyValues::new(
            HugeDoubleArray::new(5_000_000),
            1.0
        ))
    );

    let timestamp = MonadicProperty::of(
        "timestamp",
        Arc::new(MonadicLongPropertyValues::new(
            HugeLongArray::new(5_000_000),
            0
        ))
    );

    // ========================================================================
    // Build the Triadic PropertyStore
    // ========================================================================
    println!("🏗️  Building TriadicPropertyStore...\n");

    let store = TriadicPropertyStore::builder()
        // Meta properties
        .put_meta("graph_id", graph_id)
        .put_meta("version", graph_version)
        .put_meta("node_count", node_count)
        // Node properties
        .put_node("age", age)
        .put_node("pagerank", pagerank)
        .put_node("version", node_version)  // Same key as meta, different level!
        // Link properties
        .put_link("weight", weight)
        .put_link("timestamp", timestamp)
        .build();

    // ========================================================================
    // Demonstrate Separate Key Spaces
    // ========================================================================
    println!("✨ Demonstrating Separate Key Spaces:");
    println!("   'version' exists at BOTH meta and node levels!\n");

    if let Some(meta_version) = store.get_meta_property_values("version") {
        println!("   Meta 'version': {} elements (graph version)",
                 meta_version.element_count());
    }

    if let Some(node_version) = store.get_node_property_values("version") {
        println!("   Node 'version': {} elements (per-node versions)",
                 node_version.element_count());
    }

    println!();

    // ========================================================================
    // Demonstrate Access Patterns
    // ========================================================================
    println!("🔍 Access Patterns:");
    
    println!("\n   Meta Properties:");
    for key in store.meta_property_keys() {
        if let Some(values) = store.get_meta_property_values(key) {
            println!("     - {}: {} elements", key, values.element_count());
        }
    }

    println!("\n   Node Properties:");
    for key in store.node_property_keys() {
        if let Some(values) = store.get_node_property_values(key) {
            println!("     - {}: {} elements", key, values.element_count());
        }
    }

    println!("\n   Link Properties:");
    for key in store.link_property_keys() {
        if let Some(values) = store.get_link_property_values(key) {
            println!("     - {}: {} elements", key, values.element_count());
        }
    }

    // ========================================================================
    // Summary Statistics
    // ========================================================================
    println!("\n📈 Summary Statistics:");
    println!("   Total Properties: {}", store.total_property_count());
    println!("   Meta Properties:  {}", store.meta_property_count());
    println!("   Node Properties:  {}", store.node_property_count());
    println!("   Link Properties:  {}", store.link_property_count());

    // ========================================================================
    // Demonstrate Composition Pattern
    // ========================================================================
    println!("\n🎯 Key Insight: Composition Over Inheritance");
    println!("   Each level is an independent MonadicPropertyStore");
    println!("   Each can use different Collections backends");
    println!("   Each maintains its own key space");
    println!("   The composition is just delegation!");

    // ========================================================================
    // Compare with Monadic
    // ========================================================================
    println!("\n🔄 Monadic vs Triadic:");
    println!("   Monadic  → Universal single-level store");
    println!("   Triadic  → Three monadic stores composed");
    println!("   GraphStore → Could use Triadic internally!");

    // ========================================================================
    // Demonstrate Modification via Builder
    // ========================================================================
    println!("\n🛠️  Modification Pattern:");
    
    let updated_store = store.to_builder()
        .put_meta("created_at", MonadicProperty::of(
            "created_at",
            Arc::new(MonadicLongPropertyValues::new(
                VecLong::from(vec![1640000000]),
                0
            ))
        ))
        .build();

    println!("   Original meta properties: {}", store.meta_property_count());
    println!("   Updated meta properties:  {}", updated_store.meta_property_count());

    // ========================================================================
    // Universal Pattern
    // ========================================================================
    println!("\n🌍 Universal Meta/Node/Link Pattern:");
    println!("   Not just for graphs!");
    println!("   - File Systems:  volume metadata / files / directories");
    println!("   - Databases:     database metadata / tables / foreign keys");
    println!("   - Networks:      system config / hosts / connections");
    println!("   - ML Pipelines:  model metadata / samples / batches");

    println!("\n✅ Triadic PropertyStore Demo Complete!");
}


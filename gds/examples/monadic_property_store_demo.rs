//! Monadic PropertyStore Demo
//!
//! Demonstrates the new Collections First architecture with a standalone
//! MonadicPropertyStore that works independently of GraphStore.
//!
//! This example shows how to:
//! - Create Collections-backed properties with VecLong and VecDouble
//! - Build a MonadicPropertyStore
//! - Use HugeArrays for billion-element scale
//! - Access property values through the unified Collections interface

use gds::collections::backends::huge::{HugeDoubleArray, HugeLongArray};
use gds::collections::backends::vec::{VecDouble, VecLong};
use gds::types::properties::monadic::{
    MonadicDoublePropertyValues, MonadicLongPropertyValues,
    MonadicProperty, MonadicPropertyStore,
};
use std::sync::Arc;

fn main() {
    println!("🚀 Collections First: Monadic PropertyStore Demo");
    println!("================================================\n");

    // Example 1: Vec-backed properties (for small to medium datasets)
    println!("📊 Example 1: Vec-backed Properties");
    println!("-----------------------------------");

    let vec_long = VecLong::from(vec![100, 200, 300, 400, 500]);
    let age_values = MonadicLongPropertyValues::new(vec_long, 0);
    let age_property = MonadicProperty::of("age", Arc::new(age_values));

    let vec_double = VecDouble::from(vec![1.5, 2.5, 3.5, 4.5]);
    let score_values = MonadicDoublePropertyValues::new(vec_double, 0.0);
    let score_property = MonadicProperty::of("score", Arc::new(score_values));

    let store = MonadicPropertyStore::builder()
        .put("age", age_property)
        .put("score", score_property)
        .build();

    println!("✅ Created property store with {} properties", store.len());
    println!("   Properties: {:?}", store.keys().collect::<Vec<_>>());

    if let Some(age_prop) = store.get("age") {
        println!("   Age property: {} elements", age_prop.values().element_count());
        println!("      Type: {:?}", age_prop.property_schema().value_type());
    }

    if let Some(score_prop) = store.get("score") {
        println!("   Score property: {} elements", score_prop.values().element_count());
        println!("      Type: {:?}", score_prop.property_schema().value_type());
    }
    println!();

    // Example 2: HugeArray-backed properties (for billion-element scale)
    println!("🔢 Example 2: HugeArray-backed Properties (Large Scale)");
    println!("--------------------------------------------------------");

    // Create a HugeLongArray for 1 million node IDs
    let mut huge_ids = HugeLongArray::new(1_000_000);
    for i in 0..1000 {
        huge_ids.set(i, (i * 100) as i64);
    }

    let id_values = MonadicLongPropertyValues::new(huge_ids, -1);
    let id_property = MonadicProperty::of("node_id", Arc::new(id_values));

    // Create a HugeDoubleArray for 1 million rankings
    let mut huge_ranks = HugeDoubleArray::new(1_000_000);
    for i in 0..1000 {
        huge_ranks.set(i, (i as f64) * 0.001);
    }

    let rank_values = MonadicDoublePropertyValues::new(huge_ranks, 0.0);
    let rank_property = MonadicProperty::of("page_rank", Arc::new(rank_values));

    let large_store = MonadicPropertyStore::builder()
        .put("node_id", id_property)
        .put("page_rank", rank_property)
        .build();

    println!("✅ Created large-scale property store with {} properties", large_store.len());
    
    if let Some(id_prop) = large_store.get("node_id") {
        println!("   Node ID property: {} elements (1M scale)", id_prop.values().element_count());
        println!("      Backend: HugeLongArray");
    }

    if let Some(rank_prop) = large_store.get("page_rank") {
        println!("   PageRank property: {} elements (1M scale)", rank_prop.values().element_count());
        println!("      Backend: HugeDoubleArray");
    }
    println!();

    // Example 3: Builder pattern with conditional properties
    println!("🏗️  Example 3: Builder Pattern with Conditional Logic");
    println!("-----------------------------------------------------");

    let mut builder = MonadicPropertyStore::builder();

    // Always add core properties
    let vec_core = VecLong::from(vec![1, 2, 3, 4, 5]);
    let core_values = MonadicLongPropertyValues::new(vec_core, 0);
    let core_property = MonadicProperty::of("core_metric", Arc::new(core_values));
    builder = builder.put("core_metric", core_property);

    // Conditionally add optional properties
    let include_optional = true;
    if include_optional {
        let vec_opt = VecDouble::from(vec![10.0, 20.0, 30.0]);
        let opt_values = MonadicDoublePropertyValues::new(vec_opt, 0.0);
        let opt_property = MonadicProperty::of("optional_metric", Arc::new(opt_values));
        builder = builder.put("optional_metric", opt_property);
    }

    let conditional_store = builder.build();

    println!("✅ Built conditional property store with {} properties", conditional_store.len());
    println!("   Contains 'core_metric': {}", conditional_store.contains_key("core_metric"));
    println!("   Contains 'optional_metric': {}", conditional_store.contains_key("optional_metric"));
    println!();

    // Summary
    println!("📝 Summary");
    println!("-----------");
    println!("✨ Collections First Architecture Benefits:");
    println!("   • Simple API: No complex inheritance hierarchies");
    println!("   • Unified Backend: Vec, HugeArray, Arrow all work the same");
    println!("   • Type Safe: Strong typing with Collections trait");
    println!("   • Scalable: From tiny datasets to billions of elements");
    println!("   • Standalone: Works independently of GraphStore");
    println!("   • Testable: Easy to test with mock Collections");
    println!();
    println!("🎯 This proves Collections can be the universal backend!");
}


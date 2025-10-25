//! Monadic PropertyStore with Collections Config
//!
//! Demonstrates how CollectionsConfig enables pluggable backends (Vec vs Huge)
//! for MonadicPropertyStore - the Collections First pattern in action!

use gds::collections::adapter::factory::CollectionFactory;
use gds::collections::backends::vec::VecLong;
use gds::collections::backends::huge::HugeLongArray;
use gds::collections::traits::Collections;
use gds::config::{CollectionsBackend, CollectionsConfigBuilder};
use gds::types::properties::monadic::{MonadicProperty, MonadicPropertyStore};
use gds::types::properties::monadic::property_values::*;
use gds::types::properties::PropertyValues;
use gds::types::ValueType;
use std::collections::HashMap;
use std::sync::Arc;

fn main() {
    println!("=== Monadic PropertyStore with Config Demo ===\n");

    // ========================================================================
    // Example 1: Vec Backend via Config and Factory
    // ========================================================================
    println!("1. Creating Collections with Vec backend via config and factory...");
    
    let vec_config = CollectionsConfigBuilder::<i64>::new()
        .with_backend(CollectionsBackend::Vec)
        .build();
    
    let vec_collection = CollectionFactory::create_long_collection(&vec_config);
    println!("   Created Collection via factory");
    println!("   Backend: {:?}", vec_collection.backend());
    println!("   Value type: {:?}\n", vec_collection.value_type());

    // ========================================================================
    // Example 2: Huge Backend via Config and Factory
    // ========================================================================
    println!("2. Creating Collections with Huge backend via config and factory...");
    
    let huge_config = CollectionsConfigBuilder::<i64>::new()
        .with_backend(CollectionsBackend::Huge)
        .build();
    
    let huge_collection = CollectionFactory::create_long_collection(&huge_config);
    println!("   Created Collection via factory");
    println!("   Backend: {:?}", huge_collection.backend());
    println!("   Value type: {:?}\n", huge_collection.value_type());

    // ========================================================================
    // Example 3: Direct Construction with Concrete Types
    // ========================================================================
    println!("3. Direct construction with concrete types (current approach)...");
    
    let vec_direct = VecLong::from(vec![10, 20, 30]);
    let vec_property_values = MonadicLongPropertyValues::new(vec_direct, 0);
    println!("   Vec PropertyValues: {} elements", vec_property_values.element_count());
    
    let mut huge_direct = HugeLongArray::new(5);
    huge_direct.set(0, 100);
    huge_direct.set(1, 200);
    let huge_property_values = MonadicLongPropertyValues::new(huge_direct, 0);
    println!("   Huge PropertyValues: {} elements\n", huge_property_values.element_count());

    // ========================================================================
    // Example 4: Building a Complete PropertyStore
    // ========================================================================
    println!("4. Building a complete MonadicPropertyStore...");
    
    let score_property = MonadicProperty::of(
        "score",
        Arc::new(MonadicLongPropertyValues::new(
            VecLong::from(vec![95, 87, 92, 88, 91]),
            0,
        ))
    );
    
    let mut properties = HashMap::new();
    properties.insert("score".to_string(), score_property);
    
    let store = MonadicPropertyStore::new(properties);
    println!("   Created PropertyStore with {} properties", store.len());
    println!("   Property 'score' exists: {}\n", store.contains_key("score"));

    // ========================================================================
    // Summary
    // ========================================================================
    println!("=== Summary ===");
    println!("✓ Collections factory creates Vec or Huge backends based on CollectionsConfig");
    println!("✓ Factory returns Box<dyn Collections<T>> for backend abstraction");
    println!("✓ MonadicPropertyValues work with concrete Vec/Huge backends");
    println!("✓ Next step: Enum wrapper or generic builder to connect factory → PropertyValues");
    println!("\nThe Collections First factory pattern is working! 🎉");
}


use rust_gds::types::properties::property::{Property, PropertyTrait};
use rust_gds::types::properties::relationship::impls::default_relationship_property_store::DefaultRelationshipPropertyStore;
use rust_gds::types::properties::relationship::relationship_property_store::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
use rust_gds::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use rust_gds::types::properties::relationship::DefaultRelationshipPropertyValues;
use rust_gds::types::property::PropertyState;
use std::sync::Arc;

fn main() {
    println!("\n=== Relationship Property Store Basics ===\n");

    let weight_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![1.2, 0.8, 1.5], 0.0, 3),
    );
    let capacity_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![100.0, 80.0, 120.0], 0.0, 3),
    );

    // Create properties using Property::of
    let weight_property = Property::of("weight", PropertyState::Normal, Arc::clone(&weight_values));
    let capacity_property = Property::of(
        "capacity",
        PropertyState::Normal,
        Arc::clone(&capacity_values),
    );

    let store = DefaultRelationshipPropertyStore::builder()
        .put("weight", weight_property)
        .put("capacity", capacity_property)
        .build();

    let keys: Vec<&str> = store
        .relationship_properties()
        .keys()
        .map(|s| s.as_str())
        .collect();
    println!("Property keys: {:?}", keys);
    println!("Length: {}\n", store.len());

    for (key, property) in store.relationship_properties() {
        println!("Property `{}`", key);
        println!("  key from property: {}", property.key());
        println!("  default value: {}", property.values().default_value());

        let count = property.values().relationship_count();
        for rel_index in 0..count {
            let value = property
                .values()
                .double_value(rel_index as u64)
                .unwrap_or(property.values().default_value());
            println!("  rel #{rel_index}: {value}");
        }
        println!();
    }

    println!(
        "Contains weight? {}  Contains capacity? {}",
        store.contains_key("weight"),
        store.contains_key("capacity")
    );
}

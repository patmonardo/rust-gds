use rust_gds::types::properties::relationship::{
    DefaultRelationshipPropertyValues, RelationshipPropertyStoreBuilder, RelationshipPropertyValues,
};
use rust_gds::types::schema::Aggregation;
use std::sync::Arc;

fn main() {
    println!("\n=== Relationship Property Store Basics ===\n");

    let weight_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![1.2, 0.8, 1.5], 0.0, 3),
    );
    let capacity_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![100.0, 80.0, 120.0], 0.0, 3),
    );

    let store = RelationshipPropertyStoreBuilder::new()
        .put_property("weight", Arc::clone(&weight_values))
        .put_property_with_aggregation("capacity", Arc::clone(&capacity_values), Aggregation::Max)
        .build();

    println!("Property keys: {:?}", store.key_set());
    println!("Length: {}\n", store.len());

    for property in store.values() {
        println!("Property `{}`", property.key());
        println!("  aggregation: {:?}", property.aggregation());
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

    let filtered = store.filter("capacity");
    println!(
        "Filtered store keys: {:?} (len = {})",
        filtered.key_set(),
        filtered.len()
    );
    println!(
        "Contains weight? {}  Contains capacity? {}",
        store.contains_key("weight"),
        store.contains_key("capacity")
    );
}

use gds::types::properties::relationship::relationship_property_of;
use gds::types::properties::relationship::RelationshipPropertyValues;
use gds::types::properties::relationship::impls::default_relationship_property_values::DefaultRelationshipPropertyValues;
use gds::types::properties::relationship::{
    DefaultRelationshipPropertyStore,
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
use gds::types::property_state::PropertyState;
use std::sync::Arc;

fn main() {
    println!("\n=== Relationship Property Store Basics ===\n");
    println!("This example demonstrates the PropertyStore builder pattern.");
    println!(
        "PropertyStore = HashMap<String, RelationshipProperty (wrapping Arc<dyn RelationshipPropertyValues>)>.\n"
    );
    println!("Key operations:");
    println!(
        "  1. relationship_property_of(key, state, values) constructs the RelationshipProperty wrapper"
    );
    println!("  2. builder().put(key, property).build() creates an immutable store");
    println!("  3. store.to_builder() clones the store for modification (copy-on-write)\n");
    println!(
        "RelationshipProperty bundles the shared column with schema metadata, including state and default value.\n"
    );

    let weight_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::with_values(vec![1.2, 0.8, 1.5], 0.0, 3),
    );
    let capacity_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::with_values(vec![100.0, 80.0, 120.0], 0.0, 3),
    );

    // Relationship stores expect a RelationshipProperty, which wraps values with schema metadata
    let weight_property =
        relationship_property_of("weight", PropertyState::Persistent, weight_values.clone());
    let capacity_property = relationship_property_of(
        "capacity",
        PropertyState::Persistent,
        capacity_values.clone(),
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

        let rel_values = property.values();

        println!("  default value: {}", rel_values.default_value());

        let count = rel_values.relationship_count();
        for rel_index in 0..count {
            let value = rel_values
                .double_value(rel_index as u64)
                .unwrap_or(rel_values.default_value());
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

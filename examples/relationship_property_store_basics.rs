use rust_gds::types::properties::property::DefaultProperty;
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
    println!("This example demonstrates the PropertyStore builder pattern.");
    println!("PropertyStore = HashMap<String, Property<Arc<dyn *PropertyValues>>>.\n");
    println!("Key operations:");
    println!("  1. Property::of(key, state, values) constructs a Property wrapper");
    println!("  2. builder().put(key, property).build() creates an immutable store");
    println!("  3. store.to_builder() clones the store for modification (copy-on-write)\n");
    println!("PropertyValues (the column) is shared via Arc; Property adds schema metadata.\n");

    let weight_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![1.2, 0.8, 1.5], 0.0, 3),
    );
    let capacity_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![100.0, 80.0, 120.0], 0.0, 3),
    );

    // Create properties using DefaultProperty::of
    // Note: We need to upcast to Arc<dyn PropertyValues>
    let weight_property = DefaultProperty::of(
        "weight",
        PropertyState::Normal,
        weight_values.clone()
            as Arc<dyn rust_gds::types::properties::property_values::PropertyValues>,
    );
    let capacity_property = DefaultProperty::of(
        "capacity",
        PropertyState::Normal,
        capacity_values.clone()
            as Arc<dyn rust_gds::types::properties::property_values::PropertyValues>,
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

        // Cast to domain-specific type to access relationship methods
        use rust_gds::types::properties::property::Property;
        let values_arc = property.values();
        let rel_values: Arc<dyn RelationshipPropertyValues> =
            unsafe { std::mem::transmute(values_arc) };

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

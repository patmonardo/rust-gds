use gds::collections::backends::vec::{VecDouble, VecLong};
use gds::types::properties::node::{
    node_property_of, DefaultDoubleNodePropertyValues, DefaultLongNodePropertyValues,
    DefaultNodePropertyStore, NodePropertyStore, NodePropertyStoreBuilder, NodePropertyValues,
};
use gds::types::properties::property_store::PropertyStore as _;
use gds::types::property_state::PropertyState;
use std::sync::Arc;

fn main() {
    println!("\n=== Node Property Store Basics ===\n");
    println!("This example mirrors the relationship store demo, but for nodes.");
    println!(
        "NodePropertyStore = HashMap<String, NodeProperty (wrapping Arc<dyn NodePropertyValues>)>.\n"
    );
    println!("Key operations:");
    println!("  1. node_property_of(key, state, values) constructs the NodeProperty wrapper");
    println!("  2. builder().put(key, property).build() creates an immutable store");
    println!(
        "  3. to_builder() enables copy-on-write updates; put_property(key, values) handles wrapping for you\n"
    );

    // Collections-backed property values: construct a Vec backend then adapt it
    let age_backend = VecLong::from(vec![29i64, 35, 41]);
    let age_values: Arc<dyn NodePropertyValues> = Arc::new(
        DefaultLongNodePropertyValues::from_collection(age_backend, 3),
    );

    let score_backend = VecDouble::from(vec![0.82f64, 0.91, 0.76]);
    let score_values: Arc<dyn NodePropertyValues> = Arc::new(
        DefaultDoubleNodePropertyValues::from_collection(score_backend, 3),
    );

    let age_property = node_property_of("age", PropertyState::Persistent, age_values.clone());
    let score_property = node_property_of("score", PropertyState::Persistent, score_values.clone());

    let store = DefaultNodePropertyStore::builder()
        .put("age", age_property)
        .put("score", score_property)
        .build();

    let depth_backend = VecLong::from(vec![1i64, 2, 3]);
    let depth_values: Arc<dyn NodePropertyValues> = Arc::new(
        DefaultLongNodePropertyValues::from_collection(depth_backend, 3),
    );

    let store = store
        .to_builder()
        .put_property("depth", depth_values.clone())
        .build();

    let mut keys = store.key_set();
    keys.sort();
    println!("Property keys: {:?}", keys);
    println!("Length: {}\n", store.len());

    for (key, property) in store.node_properties() {
        println!("Property `{}`", key);
        println!("  key from property: {}", property.key());
        println!(
            "  default value: {:?}",
            property.property_schema().default_value()
        );

        let values = property.values();
        let count = values.node_count();
        for node_id in 0..count {
            let display = values
                .double_value(node_id as u64)
                .map(|d| format!("{d:.2}"))
                .or_else(|_| values.long_value(node_id as u64).map(|l| l.to_string()))
                .unwrap_or_else(|_| "<missing>".to_string());
            println!("  node #{node_id}: {display}");
        }
        println!();
    }

    println!(
        "Contains age? {}  Contains score? {}  Contains depth? {}",
        store.contains_key("age"),
        store.contains_key("score"),
        store.contains_key("depth")
    );
}

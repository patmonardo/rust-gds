use rust_gds::types::random::{RandomGraphConfig, RandomRelationshipConfig};
use rust_gds::types::{random_graph_store, GraphStore};

#[test]
fn random_graph_store_wrapper_creates_store() {
    let cfg = RandomGraphConfig {
        graph_name: "random-test".into(),
        database_name: "unit-test".into(),
        node_count: 10,
        node_labels: vec!["A".into()],
        relationships: vec![RandomRelationshipConfig::new("R", 0.15)],
        directed: true,
        inverse_indexed: false,
        seed: Some(42),
    };

    let store = random_graph_store(&cfg).expect("random graph store created");
    assert_eq!(store.node_count(), cfg.node_count);
}

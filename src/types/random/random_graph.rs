use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::{RelationshipTopology, SimpleIdMap};
use crate::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStoreError,
};
use crate::types::properties::graph::DefaultDoubleGraphPropertyValues;
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::schema::{Direction, MutableGraphSchema};
use crate::types::value_type::ValueType;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;

// ...existing code...

impl Randomizable<RandomGraphConfig> for DefaultGraphStore {
    type Error = RandomGraphError;

    fn random_with_rng<R: Rng + ?Sized>(
        config: &RandomGraphConfig,
        rng: &mut R,
    ) -> RandomGraphResult<Self> {
        if config.node_count == 0 {
            return Err(RandomGraphError::EmptyGraph);
        }

        let node_labels = resolve_node_labels(config)?;

        // Build IdMap with random label assignments
        let original_ids: Vec<i64> = (0..config.node_count).map(|id| id as i64).collect();
        let mut id_map = SimpleIdMap::from_original_ids(original_ids);

        for label in &node_labels {
            id_map.add_node_label(label.clone());
        }

        for mapped_id in 0..config.node_count as u64 {
            let label_index = rng.gen_range(0..node_labels.len());
            let label = node_labels[label_index].clone();
            id_map.add_node_id_to_label(mapped_id, label);
        }

        let direction = if config.directed {
            Direction::Directed
        } else {
            Direction::Undirected
        };

        let mut schema_builder = MutableGraphSchema::empty();
        for label in &node_labels {
            schema_builder.node_schema_mut().add_label(label.clone());
            schema_builder.node_schema_mut().add_property(
                label.clone(),
                "random_score",
                ValueType::Double,
            );
        }

        for rel in &config.relationships {
            let rel_type = RelationshipType::of(rel.name.as_str());
            schema_builder.relationship_schema_mut().add_property(
                rel_type,
                direction,
                "weight",
                ValueType::Double,
            );
        }

        let schema = schema_builder.build();

        let mut capabilities = Capabilities::new();
        capabilities.add_feature("random");
        capabilities.add_feature("transient");

        let mut relationship_topologies: HashMap<RelationshipType, RelationshipTopology> =
            HashMap::new();

        for rel in &config.relationships {
            let rel_type = RelationshipType::of(rel.name.as_str());
            let topology = <RelationshipTopology as Randomizable<(
                &RandomGraphConfig,
                &RandomRelationshipConfig,
            )>>::random_with_rng(&(config, rel), rng)?;
            relationship_topologies.insert(rel_type, topology);
        }

        let graph_name = GraphName::new(&config.graph_name);
        let database_info = DatabaseInfo::new(
            DatabaseId::new(&config.database_name),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );

        let mut store = DefaultGraphStore::new(
            graph_name,
            database_info,
            schema,
            capabilities,
            id_map,
            relationship_topologies,
        );

        // Add a random floating-point score for each node.
        let node_property_values = Arc::new(<DefaultDoubleNodePropertyValues as Randomizable<
            RandomNodeDoublePropertyConfig,
        >>::random_with_rng(
            &RandomNodeDoublePropertyConfig {
                node_count: config.node_count,
                min: 0.0,
                max: 1.0,
            },
            rng,
        )?);
        let label_set: HashSet<NodeLabel> = node_labels.into_iter().collect();
        store.add_node_property(label_set, "random_score", node_property_values)?;

        // Graph-level edge density property.
        let total_relationships = store.relationship_count();
        let max_edges = if config.directed {
            config
                .node_count
                .saturating_mul(config.node_count.saturating_sub(1))
        } else {
            config
                .node_count
                .saturating_mul(config.node_count.saturating_sub(1))
                / 2
        };
        let density = if max_edges == 0 {
            0.0
        } else {
            total_relationships as f64 / max_edges as f64
        };
        let graph_property_values = Arc::new(DefaultDoubleGraphPropertyValues::singleton(density));
        store.add_graph_property("edge_density", graph_property_values)?;

        Ok(store)
    }
}

// ...existing code...

use crate::projection::{
    NodeLabel as ProjectionNodeLabel, RelationshipType as ProjectionRelationshipType,
};
use crate::types::graph::id_map::{IdMap, SimpleIdMap};
use crate::types::graph::topology::RelationshipTopology;
use crate::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore, GraphStoreError,
};
use crate::types::properties::graph::DefaultDoubleGraphPropertyValues;
use crate::types::properties::node::impls::default_node_property_values::DefaultDoubleNodePropertyValues;
use crate::types::schema::{
    Direction, MutableGraphSchema, NodeLabel as SchemaNodeLabel,
    RelationshipType as SchemaRelationshipType,
};
use crate::types::value_type::ValueType;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;

/// Trait implemented by types that can construct randomised instances from a configuration.
pub trait Randomizable<Config>: Sized {
    type Error;

    /// Build a randomised instance using the provided RNG.
    fn random_with_rng<R: Rng + ?Sized>(config: &Config, rng: &mut R) -> Result<Self, Self::Error>;
}

/// Configuration for generating a random relationship type.
#[derive(Debug, Clone)]
pub struct RandomRelationshipConfig {
    pub name: String,
    pub probability: f64,
}

impl RandomRelationshipConfig {
    pub fn new(name: impl Into<String>, probability: f64) -> Self {
        Self {
            name: name.into(),
            probability,
        }
    }
}

/// Configuration for generating a random in-memory graph store.
#[derive(Debug, Clone)]
pub struct RandomGraphConfig {
    pub graph_name: String,
    pub database_name: String,
    pub node_count: usize,
    pub node_labels: Vec<String>,
    pub relationships: Vec<RandomRelationshipConfig>,
    pub directed: bool,
    pub inverse_indexed: bool,
    pub seed: Option<u64>,
}

impl Default for RandomGraphConfig {
    fn default() -> Self {
        Self {
            graph_name: "random-graph".to_string(),
            database_name: "in-memory".to_string(),
            node_count: 16,
            node_labels: vec!["RandomNode".to_string()],
            relationships: vec![RandomRelationshipConfig::new("RELATES", 0.1)],
            directed: true,
            inverse_indexed: true,
            seed: None,
        }
    }
}

impl RandomGraphConfig {
    /// Override the random seed, enabling deterministic graph generation.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

/// Errors that can occur while generating random graph structures.
#[derive(Debug, Error)]
pub enum RandomGraphError {
    #[error("node count must be greater than zero")]
    EmptyGraph,
    #[error("at least one node label must be provided")]
    MissingNodeLabels,
    #[error("probability must be between 0.0 and 1.0 for relationship '{0}'")]
    InvalidProbability(String),
    #[error("invalid range: min={min}, max={max}")]
    InvalidRange { min: f64, max: f64 },
    #[error("graph store error: {0}")]
    GraphStore(#[from] GraphStoreError),
}

pub type RandomGraphResult<T> = Result<T, RandomGraphError>;

fn resolve_node_labels(config: &RandomGraphConfig) -> RandomGraphResult<Vec<SchemaNodeLabel>> {
    if config.node_labels.is_empty() {
        return Err(RandomGraphError::MissingNodeLabels);
    }

    Ok(config
        .node_labels
        .iter()
        .map(|label| SchemaNodeLabel::new(label.as_str()))
        .collect())
}

fn validate_relationship_probability(config: &RandomRelationshipConfig) -> RandomGraphResult<()> {
    if !(0.0..=1.0).contains(&config.probability) {
        return Err(RandomGraphError::InvalidProbability(config.name.clone()));
    }
    Ok(())
}

impl Randomizable<RandomGraphConfig> for SimpleIdMap {
    type Error = RandomGraphError;

    fn random_with_rng<R: Rng + ?Sized>(
        config: &RandomGraphConfig,
        rng: &mut R,
    ) -> RandomGraphResult<Self> {
        if config.node_count == 0 {
            return Err(RandomGraphError::EmptyGraph);
        }

        let labels = resolve_node_labels(config)?;
        let original_ids: Vec<i64> = (0..config.node_count).map(|id| id as i64).collect();
        let mut id_map = SimpleIdMap::from_original_ids(original_ids);

        for label in &labels {
            id_map.add_node_label(label.clone());
        }

        for mapped_id in 0..config.node_count as u64 {
            let label_index = rng.gen_range(0..labels.len());
            let label = labels[label_index].clone();
            id_map.add_node_id_to_label(mapped_id, label);
        }

        Ok(id_map)
    }
}

impl Randomizable<(&RandomGraphConfig, &RandomRelationshipConfig)> for RelationshipTopology {
    type Error = RandomGraphError;

    fn random_with_rng<R: Rng + ?Sized>(
        (graph_config, rel_config): &(&RandomGraphConfig, &RandomRelationshipConfig),
        rng: &mut R,
    ) -> RandomGraphResult<Self> {
        validate_relationship_probability(rel_config)?;

        let node_count = graph_config.node_count;
        if node_count == 0 {
            return Err(RandomGraphError::EmptyGraph);
        }

        let mut outgoing: Vec<Vec<u64>> = vec![Vec::new(); node_count];
        let mut incoming: Option<Vec<Vec<u64>>> = if graph_config.inverse_indexed {
            Some(vec![Vec::new(); node_count])
        } else {
            None
        };

        if graph_config.directed {
            for (source, neighbors) in outgoing.iter_mut().enumerate() {
                for target in 0..node_count {
                    if source == target {
                        continue;
                    }

                    if rng.gen_bool(rel_config.probability) {
                        neighbors.push(target as u64);
                        if let Some(incoming_lists) = incoming.as_mut() {
                            incoming_lists[target].push(source as u64);
                        }
                    }
                }
            }
        } else {
            for source in 0..node_count {
                let (left, right) = outgoing.split_at_mut(source + 1);
                let source_neighbors = &mut left[source];

                for (offset, target_neighbors) in right.iter_mut().enumerate() {
                    let target = source + 1 + offset;
                    if rng.gen_bool(rel_config.probability) {
                        source_neighbors.push(target as u64);
                        target_neighbors.push(source as u64);

                        if let Some(incoming_lists) = incoming.as_mut() {
                            incoming_lists[target].push(source as u64);
                            incoming_lists[source].push(target as u64);
                        }
                    }
                }
            }
        }

        Ok(RelationshipTopology::new(outgoing, incoming))
    }
}

/// Configuration for generating random double-valued node properties.
#[derive(Debug, Clone)]
pub struct RandomNodeDoublePropertyConfig {
    pub node_count: usize,
    pub min: f64,
    pub max: f64,
}

impl Default for RandomNodeDoublePropertyConfig {
    fn default() -> Self {
        Self {
            node_count: 0,
            min: 0.0,
            max: 1.0,
        }
    }
}

impl Randomizable<RandomNodeDoublePropertyConfig> for DefaultDoubleNodePropertyValues {
    type Error = RandomGraphError;

    fn random_with_rng<R: Rng + ?Sized>(
        config: &RandomNodeDoublePropertyConfig,
        rng: &mut R,
    ) -> RandomGraphResult<Self> {
        if config.node_count == 0 {
            return Err(RandomGraphError::EmptyGraph);
        }

        match config.min.partial_cmp(&config.max) {
            Some(Ordering::Less) => {}
            _ => {
                return Err(RandomGraphError::InvalidRange {
                    min: config.min,
                    max: config.max,
                });
            }
        }

        let values: Vec<f64> = (0..config.node_count)
            .map(|_| rng.gen_range(config.min..config.max))
            .collect();

        Ok(Self::new(values, config.node_count))
    }
}

impl Randomizable<RandomGraphConfig> for DefaultGraphStore {
    type Error = RandomGraphError;

    fn random_with_rng<R: Rng + ?Sized>(
        config: &RandomGraphConfig,
        rng: &mut R,
    ) -> RandomGraphResult<Self> {
        if config.node_count == 0 {
            return Err(RandomGraphError::EmptyGraph);
        }

        let schema_node_labels = resolve_node_labels(config)?;
        let projection_node_labels: Vec<ProjectionNodeLabel> = config
            .node_labels
            .iter()
            .map(|label| ProjectionNodeLabel::of(label.as_str()))
            .collect();

        let id_map =
            <SimpleIdMap as Randomizable<RandomGraphConfig>>::random_with_rng(config, rng)?;

        let direction = if config.directed {
            Direction::Directed
        } else {
            Direction::Undirected
        };

        let mut schema_builder = MutableGraphSchema::empty();
        for label in &schema_node_labels {
            schema_builder.node_schema_mut().add_label(label.clone());
            schema_builder.node_schema_mut().add_property(
                label.clone(),
                "random_score",
                ValueType::Double,
            );
        }

        for rel in &config.relationships {
            let rel_type = SchemaRelationshipType::new(rel.name.as_str());
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

        let mut relationship_topologies: HashMap<ProjectionRelationshipType, RelationshipTopology> =
            HashMap::new();
        let mut total_relationships = 0usize;

        for rel in &config.relationships {
            let rel_type = ProjectionRelationshipType::of(rel.name.as_str());
            let topology = <RelationshipTopology as Randomizable<(
                &RandomGraphConfig,
                &RandomRelationshipConfig,
            )>>::random_with_rng(&(config, rel), rng)?;
            total_relationships += topology.relationship_count();
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
        let label_set: HashSet<ProjectionNodeLabel> = projection_node_labels.into_iter().collect();
        store.add_node_property(label_set, "random_score", node_property_values)?;

        // Graph-level edge density property.
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

impl DefaultGraphStore {
    /// Generate a random [`DefaultGraphStore`] using the provided configuration.
    pub fn random(config: &RandomGraphConfig) -> RandomGraphResult<Self> {
        let mut rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        <DefaultGraphStore as Randomizable<RandomGraphConfig>>::random_with_rng(config, &mut rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::GraphStore;
    use crate::types::properties::node::NodePropertyValues;
    use rand::{Rng, SeedableRng};

    #[test]
    fn generates_random_graph_store() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 8,
            node_labels: vec!["Person".into(), "Movie".into()],
            relationships: vec![RandomRelationshipConfig::new("ACTED_IN", 0.25)],
            ..RandomGraphConfig::default()
        };

        let store = DefaultGraphStore::random(&config).expect("random graph generation");

        assert_eq!(store.node_count(), config.node_count);
        assert!(store.relationship_count() > 0);
        assert!(store.graph_property_keys().contains("edge_density"));
        assert!(store.node_property_keys().contains("random_score"));
    }

    #[test]
    fn random_node_double_property_values_are_deterministic() {
        let config = RandomNodeDoublePropertyConfig {
            node_count: 4,
            min: -1.0,
            max: 2.5,
        };

        let mut rng = StdRng::seed_from_u64(99);
        let values = DefaultDoubleNodePropertyValues::random_with_rng(&config, &mut rng)
            .expect("random node property values");

        let mut rng_control = StdRng::seed_from_u64(99);
        let expected: Vec<f64> = (0..config.node_count)
            .map(|_| rng_control.gen_range(config.min..config.max))
            .collect();

        for (idx, expected_value) in expected.iter().enumerate() {
            let actual = values.double_value(idx as u64).unwrap();
            assert!(
                (actual - expected_value).abs() < 1e-12,
                "value mismatch at index {idx}: {actual} vs {expected_value}"
            );
        }
    }
}

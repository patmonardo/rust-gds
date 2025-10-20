//! Graph construction and manipulation configuration types

use super::base_types::{AlgoBaseConfig, BuilderConfig, ConcurrencyConfig, Config};
use super::validation::{ConfigError, ConfigValidation};
use crate::core::Aggregation;
use crate::projection::{NodeLabel, Orientation, RelationshipType};
use crate::types::{DefaultValue, PropertyState};
use crate::define_config;

/// Property configuration for graph construction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyConfig {
    pub property_key: String,
    pub aggregation: Aggregation,
    pub default_value: DefaultValue,
    pub property_state: PropertyState,
}

impl PropertyConfig {
    pub fn new(property_key: String) -> Self {
        Self {
            property_key,
            aggregation: Aggregation::None,
            default_value: DefaultValue::double(f64::NAN),
            property_state: PropertyState::Transient,
        }
    }

    pub fn builder(property_key: String) -> PropertyConfigBuilder {
        PropertyConfigBuilder::new(property_key)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_property_key(&self.property_key)?;
        Ok(())
    }
}

impl Config for PropertyConfig {}

/// Builder for PropertyConfig
#[derive(Debug)]
pub struct PropertyConfigBuilder {
    property_key: String,
    aggregation: Option<Aggregation>,
    default_value: Option<DefaultValue>,
    property_state: Option<PropertyState>,
}

impl PropertyConfigBuilder {
    pub fn new(property_key: String) -> Self {
        Self {
            property_key,
            aggregation: None,
            default_value: None,
            property_state: None,
        }
    }

    pub fn aggregation(mut self, aggregation: Aggregation) -> Self {
        self.aggregation = Some(aggregation);
        self
    }

    pub fn default_value(mut self, default_value: DefaultValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn property_state(mut self, property_state: PropertyState) -> Self {
        self.property_state = Some(property_state);
        self
    }

    pub fn build(self) -> Result<PropertyConfig, ConfigError> {
        let defaults = PropertyConfig::new(String::new());

        let config = PropertyConfig {
            property_key: self.property_key,
            aggregation: self.aggregation.unwrap_or(defaults.aggregation),
            default_value: self.default_value.unwrap_or(defaults.default_value),
            property_state: self.property_state.unwrap_or(defaults.property_state),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Graph creation configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphCreateConfig {
    pub base: AlgoBaseConfig,
    pub graph_name: String,
    pub node_projection: Vec<String>,
    pub relationship_projection: Vec<String>,
    pub node_properties: Vec<String>,
    pub relationship_properties: Vec<String>,
    pub read_concurrency: usize,
}

impl Default for GraphCreateConfig {
    fn default() -> Self {
        Self {
            base: AlgoBaseConfig::default(),
            graph_name: String::from("graph"),
            node_projection: vec![String::from("*")],
            relationship_projection: vec![String::from("*")],
            node_properties: vec![],
            relationship_properties: vec![],
            read_concurrency: num_cpus::get(),
        }
    }
}

impl Config for GraphCreateConfig {}

impl ConcurrencyConfig for GraphCreateConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl GraphCreateConfig {
    pub fn builder(graph_name: String) -> GraphCreateConfigBuilder {
        GraphCreateConfigBuilder::new(graph_name)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_property_key(&self.graph_name)?;
        ConfigValidation::validate_positive(self.read_concurrency as f64, "readConcurrency")?;
        Ok(())
    }
}

/// Builder for GraphCreateConfig
#[derive(Debug)]
pub struct GraphCreateConfigBuilder {
    graph_name: String,
    concurrency: Option<usize>,
    node_labels: Option<Vec<NodeLabel>>,
    relationship_types: Option<Vec<RelationshipType>>,
    node_projection: Option<Vec<String>>,
    relationship_projection: Option<Vec<String>>,
    node_properties: Option<Vec<String>>,
    relationship_properties: Option<Vec<String>>,
    read_concurrency: Option<usize>,
}

impl GraphCreateConfigBuilder {
    pub fn new(graph_name: String) -> Self {
        Self {
            graph_name,
            concurrency: None,
            node_labels: None,
            relationship_types: None,
            node_projection: None,
            relationship_projection: None,
            node_properties: None,
            relationship_properties: None,
            read_concurrency: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn node_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn relationship_types(mut self, types: Vec<RelationshipType>) -> Self {
        self.relationship_types = Some(types);
        self
    }

    pub fn node_projection(mut self, projection: Vec<String>) -> Self {
        self.node_projection = Some(projection);
        self
    }

    pub fn relationship_projection(mut self, projection: Vec<String>) -> Self {
        self.relationship_projection = Some(projection);
        self
    }

    pub fn node_properties(mut self, properties: Vec<String>) -> Self {
        self.node_properties = Some(properties);
        self
    }

    pub fn relationship_properties(mut self, properties: Vec<String>) -> Self {
        self.relationship_properties = Some(properties);
        self
    }

    pub fn read_concurrency(mut self, concurrency: usize) -> Self {
        self.read_concurrency = Some(concurrency);
        self
    }

    pub fn build(self) -> Result<GraphCreateConfig, ConfigError> {
        let defaults = GraphCreateConfig::default();

        let config = GraphCreateConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: self.node_labels.unwrap_or(defaults.base.node_labels),
                relationship_types: self
                    .relationship_types
                    .unwrap_or(defaults.base.relationship_types),
            },
            graph_name: self.graph_name,
            node_projection: self.node_projection.unwrap_or(defaults.node_projection),
            relationship_projection: self
                .relationship_projection
                .unwrap_or(defaults.relationship_projection),
            node_properties: self.node_properties.unwrap_or(defaults.node_properties),
            relationship_properties: self
                .relationship_properties
                .unwrap_or(defaults.relationship_properties),
            read_concurrency: self.read_concurrency.unwrap_or(defaults.read_concurrency),
        };

        config.validate()?;
        Ok(config)
    }
}

define_config!(
    pub struct RandomGraphGeneratorConfig {
        validate = |cfg: &RandomGraphGeneratorConfig| {
            ConfigValidation::validate_positive(cfg.node_count as f64, "nodeCount")?;
            ConfigValidation::validate_positive(cfg.average_degree, "averageDegree")?;
            Ok(())
        },
        node_count: usize = 100,
        average_degree: f64 = 10.0,
        relationship_type: RelationshipType = RelationshipType::of("REL"),
        seed: Option<u64> = None,
        allow_self_loops: bool = false,
        force_dag: bool = false,
        inverse_index: bool = false,
    }
);

/// Relationships builder configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RelationshipsBuilderConfig {
    pub base: AlgoBaseConfig,
    pub builder: BuilderConfig,
    pub relationship_type: RelationshipType,
    pub orientation: Orientation,
    pub property_configs: Vec<PropertyConfig>,
    pub aggregation: Aggregation,
    pub skip_dangling_relationships: bool,
    pub index_inverse: bool,
}

impl Default for RelationshipsBuilderConfig {
    fn default() -> Self {
        Self {
            base: AlgoBaseConfig::default(),
            builder: BuilderConfig::default(),
            relationship_type: RelationshipType::of("REL"),
            orientation: Orientation::Natural,
            property_configs: vec![],
            aggregation: Aggregation::None,
            skip_dangling_relationships: false,
            index_inverse: false,
        }
    }
}

impl Config for RelationshipsBuilderConfig {}

impl ConcurrencyConfig for RelationshipsBuilderConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl RelationshipsBuilderConfig {
    pub fn builder(relationship_type: RelationshipType) -> RelationshipsBuilderConfigBuilder {
        RelationshipsBuilderConfigBuilder::new(relationship_type)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        for prop_config in &self.property_configs {
            prop_config.validate()?;
        }
        Ok(())
    }
}

/// Builder for RelationshipsBuilderConfig
#[derive(Debug)]
pub struct RelationshipsBuilderConfigBuilder {
    relationship_type: RelationshipType,
    concurrency: Option<usize>,
    node_labels: Option<Vec<NodeLabel>>,
    relationship_types: Option<Vec<RelationshipType>>,
    builder_config: Option<BuilderConfig>,
    orientation: Option<Orientation>,
    property_configs: Option<Vec<PropertyConfig>>,
    aggregation: Option<Aggregation>,
    skip_dangling_relationships: Option<bool>,
    index_inverse: Option<bool>,
}

impl RelationshipsBuilderConfigBuilder {
    pub fn new(relationship_type: RelationshipType) -> Self {
        Self {
            relationship_type,
            concurrency: None,
            node_labels: None,
            relationship_types: None,
            builder_config: None,
            orientation: None,
            property_configs: None,
            aggregation: None,
            skip_dangling_relationships: None,
            index_inverse: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn node_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn relationship_types(mut self, types: Vec<RelationshipType>) -> Self {
        self.relationship_types = Some(types);
        self
    }

    pub fn builder_config(mut self, config: BuilderConfig) -> Self {
        self.builder_config = Some(config);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn property_configs(mut self, configs: Vec<PropertyConfig>) -> Self {
        self.property_configs = Some(configs);
        self
    }

    pub fn aggregation(mut self, aggregation: Aggregation) -> Self {
        self.aggregation = Some(aggregation);
        self
    }

    pub fn skip_dangling_relationships(mut self, skip: bool) -> Self {
        self.skip_dangling_relationships = Some(skip);
        self
    }

    pub fn index_inverse(mut self, index: bool) -> Self {
        self.index_inverse = Some(index);
        self
    }

    pub fn build(self) -> Result<RelationshipsBuilderConfig, ConfigError> {
        let defaults = RelationshipsBuilderConfig::default();

        let config = RelationshipsBuilderConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: self.node_labels.unwrap_or(defaults.base.node_labels),
                relationship_types: self
                    .relationship_types
                    .unwrap_or(defaults.base.relationship_types),
            },
            builder: self.builder_config.unwrap_or(defaults.builder),
            relationship_type: self.relationship_type,
            orientation: self.orientation.unwrap_or(defaults.orientation),
            property_configs: self.property_configs.unwrap_or(defaults.property_configs),
            aggregation: self.aggregation.unwrap_or(defaults.aggregation),
            skip_dangling_relationships: self
                .skip_dangling_relationships
                .unwrap_or(defaults.skip_dangling_relationships),
            index_inverse: self.index_inverse.unwrap_or(defaults.index_inverse),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_config_default() {
        let config = PropertyConfig::new(String::from("test"));
        assert_eq!(config.property_key, "test");
        assert!(matches!(config.aggregation, Aggregation::None));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_property_config_builder() {
        let config = PropertyConfig::builder(String::from("test"))
            .aggregation(Aggregation::Sum)
            .property_state(PropertyState::Persistent)
            .build()
            .unwrap();

        assert_eq!(config.property_key, "test");
        assert!(matches!(config.aggregation, Aggregation::Sum));
        assert!(matches!(config.property_state, PropertyState::Persistent));
    }

    #[test]
    fn test_graph_create_config_default() {
        let config = GraphCreateConfig::default();
        assert_eq!(config.graph_name, "graph");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_random_graph_generator_config() {
        let config = RandomGraphGeneratorConfig::builder()
            .node_count(1000)
            .average_degree(5.0)
            .seed(Some(42))
            .build()
            .unwrap();

        assert_eq!(config.node_count, 1000);
        assert_eq!(config.average_degree, 5.0);
        assert_eq!(config.seed, Some(42));
    }

    #[test]
    fn test_relationships_builder_config() {
        let rel_type = RelationshipType::of("KNOWS");
        let config = RelationshipsBuilderConfig::builder(rel_type.clone())
            .orientation(Orientation::Undirected)
            .index_inverse(true)
            .build()
            .unwrap();

        assert_eq!(config.relationship_type, rel_type);
        assert!(matches!(config.orientation, Orientation::Undirected));
        assert!(config.index_inverse);
    }
}

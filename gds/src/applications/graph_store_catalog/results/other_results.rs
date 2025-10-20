// Other result types - direct translation from Java

use std::collections::HashMap;
use serde_json::Value;

/// Result for graph memory usage analysis.
/// Mirrors Java GraphMemoryUsage class.
#[derive(Clone, Debug)]
pub struct GraphMemoryUsage {
    pub graph_name: String,
    pub memory_usage: String,
    pub size_in_bytes: u64,
    pub detail_size_in_bytes: HashMap<String, Value>,
    pub node_count: u64,
    pub relationship_count: u64,
}

impl GraphMemoryUsage {
    pub fn new(
        graph_name: String,
        memory_usage: String,
        size_in_bytes: u64,
        detail_size_in_bytes: HashMap<String, Value>,
        node_count: u64,
        relationship_count: u64,
    ) -> Self {
        Self {
            graph_name,
            memory_usage,
            size_in_bytes,
            detail_size_in_bytes,
            node_count,
            relationship_count,
        }
    }
}

/// Result for mutating node labels.
/// Mirrors Java MutateLabelResult class.
#[derive(Clone, Debug)]
pub struct MutateLabelResult {
    pub mutate_millis: u64,
    pub graph_name: String,
    pub node_label: String,
    pub node_labels_written: u64,
    pub node_count: u64,
    pub configuration: HashMap<String, Value>,
}

impl MutateLabelResult {
    /// Private constructor - only accessible via Builder
    fn new(
        mutate_millis: u64,
        graph_name: String,
        node_label: String,
        node_labels_written: u64,
        node_count: u64,
        configuration: HashMap<String, Value>,
    ) -> Self {
        Self {
            mutate_millis,
            graph_name,
            node_label,
            node_labels_written,
            node_count,
            configuration,
        }
    }

    /// Creates a new Builder for MutateLabelResult
    /// Mirrors Java MutateLabelResult.builder(String graphName, String nodeLabel)
    pub fn builder(graph_name: String, node_label: String) -> MutateLabelResultBuilder {
        MutateLabelResultBuilder::new(graph_name, node_label)
    }
}

/// Builder for MutateLabelResult.
/// Mirrors Java MutateLabelResult.Builder class.
pub struct MutateLabelResultBuilder {
    graph_name: String,
    node_label: String,
    node_labels_written: u64,
    mutate_millis: u64,
    node_count: u64,
    configuration: HashMap<String, Value>,
}

impl MutateLabelResultBuilder {
    /// Creates a new Builder
    /// Mirrors Java Builder(String graphName, String nodeLabel)
    pub fn new(graph_name: String, node_label: String) -> Self {
        Self {
            graph_name,
            node_label,
            node_labels_written: 0,
            mutate_millis: 0,
            node_count: 0,
            configuration: HashMap::new(),
        }
    }

    /// Sets the number of node labels written
    /// Mirrors Java Builder.withNodeLabelsWritten(long propertiesWritten)
    pub fn with_node_labels_written(mut self, node_labels_written: u64) -> Self {
        self.node_labels_written = node_labels_written;
        self
    }

    /// Sets the configuration
    /// Mirrors Java Builder.withConfig(Map<String, Object> configuration)
    pub fn with_config(mut self, configuration: HashMap<String, Value>) -> Self {
        self.configuration = configuration;
        self
    }

    /// Builds the final MutateLabelResult
    /// Mirrors Java Builder.build()
    pub fn build(self) -> MutateLabelResult {
        MutateLabelResult::new(
            self.mutate_millis,
            self.graph_name,
            self.node_label,
            self.node_labels_written,
            self.node_count,
            self.configuration,
        )
    }
}

/// Result for topology operations.
/// Mirrors Java TopologyResult class.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopologyResult {
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub relationship_type: String,
}

impl TopologyResult {
    pub fn new(source_node_id: i64, target_node_id: i64, relationship_type: String) -> Self {
        Self {
            source_node_id,
            target_node_id,
            relationship_type,
        }
    }
}

impl std::fmt::Display for TopologyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TopologyResult({}, {}, type: {})", 
               self.source_node_id, self.target_node_id, self.relationship_type)
    }
}

/// Result for random walk sampling operations.
/// Mirrors Java RandomWalkSamplingResult class.
#[derive(Clone, Debug)]
pub struct RandomWalkSamplingResult {
    pub graph_name: String,
    pub from_graph_name: String,
    pub node_count: u64,
    pub relationship_count: u64,
    pub start_node_count: u64,
    pub project_millis: u64,
}

impl RandomWalkSamplingResult {
    pub fn new(
        graph_name: String,
        from_graph_name: String,
        node_count: u64,
        relationship_count: u64,
        start_node_count: u64,
        project_millis: u64,
    ) -> Self {
        Self {
            graph_name,
            from_graph_name,
            node_count,
            relationship_count,
            start_node_count,
            project_millis,
        }
    }
}

/// Statistics for graph generation operations.
/// Mirrors Java GraphGenerationStats class.
#[derive(Clone, Debug)]
pub struct GraphGenerationStats {
    pub name: String,
    pub nodes: u64,
    pub relationships: u64,
    pub generate_millis: u64,
    pub relationship_seed: Option<u64>,
    pub average_degree: f64,
    pub relationship_distribution: Value,
    pub relationship_property: Value,
}

impl GraphGenerationStats {
    pub fn new(
        graph_name: String,
        average_degree: f64,
        relationship_distribution: String,
        relationship_property: HashMap<String, Value>,
        relationship_seed: Option<u64>,
    ) -> Self {
        Self {
            name: graph_name,
            nodes: 0, // Will be set during generation
            relationships: 0, // Will be set during generation
            generate_millis: 0, // Will be set during generation
            relationship_seed,
            average_degree,
            relationship_distribution: Value::String(relationship_distribution),
            relationship_property: Value::Object(relationship_property.into_iter().collect()),
        }
    }
}
// Write result types - direct translation from Java

use std::collections::HashMap;
use serde_json::Value;

/// Result for writing node properties.
/// Mirrors Java NodePropertiesWriteResult class.
#[derive(Clone, Debug)]
pub struct NodePropertiesWriteResult {
    pub write_millis: u64,
    pub graph_name: String,
    pub node_properties: Vec<String>,
    pub properties_written: u64,
    pub configuration: HashMap<String, Value>,
}

impl NodePropertiesWriteResult {
    /// Private constructor - only accessible via Builder
    fn new(
        write_millis: u64,
        graph_name: String,
        node_properties: Vec<String>,
        properties_written: u64,
        configuration: HashMap<String, Value>,
    ) -> Self {
        Self {
            write_millis,
            graph_name,
            node_properties,
            properties_written,
            configuration,
        }
    }
}

/// Builder for NodePropertiesWriteResult.
/// Mirrors Java NodePropertiesWriteResult.Builder class.
pub struct NodePropertiesWriteResultBuilder {
    graph_name: String,
    node_properties: Vec<String>,
    properties_written: u64,
    write_millis: u64,
    configuration: HashMap<String, Value>,
}

impl NodePropertiesWriteResultBuilder {
    /// Creates a new Builder
    /// Mirrors Java Builder(String graphName, List<String> nodeProperties)
    pub fn new(graph_name: String, node_properties: Vec<String>) -> Self {
        Self {
            graph_name,
            node_properties,
            properties_written: 0,
            write_millis: 0,
            configuration: HashMap::new(),
        }
    }

    /// Sets the write time in milliseconds
    /// Mirrors Java Builder.withWriteMillis(long writeMillis)
    pub fn with_write_millis(mut self, write_millis: u64) -> Self {
        self.write_millis = write_millis;
        self
    }

    /// Sets the number of properties written
    /// Mirrors Java Builder.withPropertiesWritten(long propertiesWritten)
    pub fn with_properties_written(mut self, properties_written: u64) -> Self {
        self.properties_written = properties_written;
        self
    }

    /// Sets the configuration
    /// Mirrors Java Builder.withConfig(Map<String, Object> configuration)
    pub fn with_config(mut self, configuration: HashMap<String, Value>) -> Self {
        self.configuration = configuration;
        self
    }

    /// Builds the final NodePropertiesWriteResult
    /// Mirrors Java Builder.build()
    pub fn build(self) -> NodePropertiesWriteResult {
        NodePropertiesWriteResult::new(
            self.write_millis,
            self.graph_name,
            self.node_properties,
            self.properties_written,
            self.configuration,
        )
    }
}

/// Result for writing node labels.
/// Mirrors Java WriteLabelResult class.
#[derive(Clone, Debug)]
pub struct WriteLabelResult {
    pub write_millis: u64,
    pub graph_name: String,
    pub node_label: String,
    pub node_count: u64,
    pub node_labels_written: u64,
    pub configuration: HashMap<String, Value>,
}

impl WriteLabelResult {
    /// Private constructor - only accessible via Builder
    fn new(
        write_millis: u64,
        graph_name: String,
        node_label: String,
        node_labels_written: u64,
        node_count: u64,
        configuration: HashMap<String, Value>,
    ) -> Self {
        Self {
            write_millis,
            graph_name,
            node_label,
            node_count,
            node_labels_written,
            configuration,
        }
    }

    /// Creates a new Builder for WriteLabelResult
    /// Mirrors Java WriteLabelResult.builder(String graphName, String nodeLabel)
    pub fn builder(graph_name: String, node_label: String) -> WriteLabelResultBuilder {
        WriteLabelResultBuilder::new(graph_name, node_label)
    }
}

/// Builder for WriteLabelResult.
/// Mirrors Java WriteLabelResult.Builder class.
pub struct WriteLabelResultBuilder {
    graph_name: String,
    node_label: String,
    node_labels_written: u64,
    write_millis: u64,
    node_count: u64,
    configuration: HashMap<String, Value>,
}

impl WriteLabelResultBuilder {
    /// Creates a new Builder
    /// Mirrors Java Builder(String graphName, String nodeLabel)
    pub fn new(graph_name: String, node_label: String) -> Self {
        Self {
            graph_name,
            node_label,
            node_labels_written: 0,
            write_millis: 0,
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

    /// Builds the final WriteLabelResult
    /// Mirrors Java Builder.build()
    pub fn build(self) -> WriteLabelResult {
        WriteLabelResult::new(
            self.write_millis,
            self.graph_name,
            self.node_label,
            self.node_labels_written,
            self.node_count,
            self.configuration,
        )
    }
}

/// Result for writing relationship properties.
/// Mirrors Java WriteRelationshipPropertiesResult class.
#[derive(Clone, Debug)]
pub struct WriteRelationshipPropertiesResult {
    pub write_millis: u64,
    pub graph_name: String,
    pub relationship_type: String,
    pub relationship_properties: Vec<String>,
    pub relationships_written: u64,
    pub properties_written: u64,
    pub configuration: HashMap<String, Value>,
}

impl WriteRelationshipPropertiesResult {
    /// Private constructor - only accessible via Builder
    fn new(
        write_millis: u64,
        graph_name: String,
        relationship_type: String,
        relationship_properties: Vec<String>,
        relationships_written: u64,
        configuration: HashMap<String, Value>,
    ) -> Self {
        let properties_written = relationships_written * relationship_properties.len() as u64;
        Self {
            write_millis,
            graph_name,
            relationship_type,
            relationship_properties,
            relationships_written,
            properties_written,
            configuration,
        }
    }
}

/// Builder for WriteRelationshipPropertiesResult.
/// Mirrors Java WriteRelationshipPropertiesResult.Builder class.
pub struct WriteRelationshipPropertiesResultBuilder {
    graph_name: String,
    relationship_type: String,
    relationship_properties: Vec<String>,
    write_millis: u64,
    relationships_written: u64,
    configuration: HashMap<String, Value>,
}

impl WriteRelationshipPropertiesResultBuilder {
    /// Creates a new Builder
    /// Mirrors Java Builder(String graphName, String relationshipType, List<String> relationProperties)
    pub fn new(graph_name: String, relationship_type: String, relationship_properties: Vec<String>) -> Self {
        Self {
            graph_name,
            relationship_type,
            relationship_properties,
            write_millis: 0,
            relationships_written: 0,
            configuration: HashMap::new(),
        }
    }

    /// Sets the write time in milliseconds
    /// Mirrors Java Builder.withWriteMillis(long writeMillis)
    pub fn with_write_millis(mut self, write_millis: u64) -> Self {
        self.write_millis = write_millis;
        self
    }

    /// Sets the number of relationships written
    /// Mirrors Java Builder.withRelationshipsWritten(long relationshipsWritten)
    pub fn with_relationships_written(mut self, relationships_written: u64) -> Self {
        self.relationships_written = relationships_written;
        self
    }

    /// Sets the configuration
    /// Mirrors Java Builder.withConfiguration(Map<String, Object> configuration)
    pub fn with_configuration(mut self, configuration: HashMap<String, Value>) -> Self {
        self.configuration = configuration;
        self
    }

    /// Builds the final WriteRelationshipPropertiesResult
    /// Mirrors Java Builder.build()
    pub fn build(self) -> WriteRelationshipPropertiesResult {
        WriteRelationshipPropertiesResult::new(
            self.write_millis,
            self.graph_name,
            self.relationship_type,
            self.relationship_properties,
            self.relationships_written,
            self.configuration,
        )
    }
}

/// Result for writing relationships.
/// Mirrors Java WriteRelationshipResult class.
#[derive(Clone, Debug)]
pub struct WriteRelationshipResult {
    pub write_millis: u64,
    pub graph_name: String,
    pub relationship_type: String,
    pub relationship_property: Option<String>,
    pub relationships_written: u64,
    pub properties_written: u64,
    pub configuration: HashMap<String, Value>,
}

impl WriteRelationshipResult {
    /// Private constructor - only accessible via Builder
    fn new(
        write_millis: u64,
        graph_name: String,
        relationship_type: String,
        relationship_property: Option<String>,
        relationships_written: u64,
        configuration: HashMap<String, Value>,
    ) -> Self {
        let properties_written = if relationship_property.is_some() { relationships_written } else { 0 };
        Self {
            write_millis,
            graph_name,
            relationship_type,
            relationship_property,
            relationships_written,
            properties_written,
            configuration,
        }
    }
}

/// Builder for WriteRelationshipResult.
/// Mirrors Java WriteRelationshipResult.Builder class.
pub struct WriteRelationshipResultBuilder {
    graph_name: String,
    relationship_type: String,
    maybe_relationship_property: Option<String>,
    write_millis: u64,
    relationships_written: u64,
    configuration: HashMap<String, Value>,
}

impl WriteRelationshipResultBuilder {
    /// Creates a new Builder
    /// Mirrors Java Builder(String graphName, String relationshipType, Optional<String> maybeRelationshipProperty)
    pub fn new(graph_name: String, relationship_type: String, maybe_relationship_property: Option<String>) -> Self {
        Self {
            graph_name,
            relationship_type,
            maybe_relationship_property,
            write_millis: 0,
            relationships_written: 0,
            configuration: HashMap::new(),
        }
    }

    /// Sets the write time in milliseconds
    /// Mirrors Java Builder.withWriteMillis(long writeMillis)
    pub fn with_write_millis(mut self, write_millis: u64) -> Self {
        self.write_millis = write_millis;
        self
    }

    /// Sets the number of relationships written
    /// Mirrors Java Builder.withRelationshipsWritten(long relationshipsWritten)
    pub fn with_relationships_written(mut self, relationships_written: u64) -> Self {
        self.relationships_written = relationships_written;
        self
    }

    /// Sets the configuration
    /// Mirrors Java Builder.withConfiguration(Map<String, Object> configuration)
    pub fn with_configuration(mut self, configuration: HashMap<String, Value>) -> Self {
        self.configuration = configuration;
        self
    }

    /// Builds the final WriteRelationshipResult
    /// Mirrors Java Builder.build()
    pub fn build(self) -> WriteRelationshipResult {
        WriteRelationshipResult::new(
            self.write_millis,
            self.graph_name,
            self.relationship_type,
            self.maybe_relationship_property,
            self.relationships_written,
            self.configuration,
        )
    }
}
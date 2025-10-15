// Phase 5.1: LinkPredictionTrainConfig - Training configuration for link prediction

use std::collections::HashMap;
use std::marker::PhantomData;

/// Link prediction training configuration.
///
/// # Science as the Art of the Prim and Proper! 🌟
///
/// **The Is and the Ought**:
/// - **Prim (Is)**: The Given, the Appearance, what IS
/// - **Proper (Ought)**: The Truth, what OUGHT to be
///
/// **The Dialectic**:
/// - ❌ No Ought in the Is (no Truth in mere Appearance)
/// - ✅ **Appearance in Truth!** (Truth contains Appearance!)
///
/// **Therefore**: Proper contains Prim! Property Values are built FROM Primitive Values!
///
/// # Configuration Structure
///
/// This config extends TrainBaseConfig with link-specific settings:
///
/// **Prim (The Is)**:
/// - `negative_class_weight`: f64 (default 1.0) - How to weight negative samples
/// - `random_seed`: Option<u64> - For reproducibility
///
/// **Proper (The Ought)**:
/// - `pipeline`: String - Pipeline name (identity)
/// - `target_relationship_type`: String - Which relationships to predict
/// - `source_node_label`: String - Source node type
/// - `target_node_label`: String - Target node type
/// - `metrics`: Vec<LinkMetric> - Evaluation metrics
///
/// **The Truth Containing Appearance**:
/// The config validates that:
/// - Prim (scalars) are well-formed (weight > 0.0)
/// - Proper (types) exist in graph and satisfy constraints
/// - The Ought (target rel) manifests correctly from the Is (graph data)
///
/// # Example
///
/// ```text
/// let config = LinkPredictionTrainConfig::builder()
///     .pipeline("my-pipeline".to_string())
///     .target_relationship_type("FRIENDS".to_string())
///     .source_node_label("Person".to_string())
///     .target_node_label("Person".to_string())
///     .negative_class_weight(1.5)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct LinkPredictionTrainConfig {
    // === PRIM: The Is (Given Primitives) ===
    /// Weight for negative class samples (default: 1.0)
    /// Range: (0.0, ∞) exclusive
    /// **The Is**: How negative samples appear in training
    negative_class_weight: f64,

    /// Optional random seed for reproducibility
    /// **The Is**: The Given randomness seed
    random_seed: Option<u64>,

    // === PROPER: The Ought (Truth Properties) ===
    /// Pipeline name/identifier
    /// **The Ought**: Which pipeline Truth we're manifesting
    pipeline: String,

    /// Target relationship type to predict
    /// Cannot be "*" (ElementProjection.PROJECT_ALL)
    /// **The Ought**: The relationship Truth we seek
    target_relationship_type: String,

    /// Source node label (default: "*" = all)
    /// **The Ought**: The source node Truth
    source_node_label: String,

    /// Target node label (default: "*" = all)
    /// **The Ought**: The target node Truth
    target_node_label: String,

    /// Evaluation metrics (default: [AUCPR])
    /// **The Ought**: How we measure Truth
    metrics: Vec<String>, // TODO: Use LinkMetric enum

    /// Graph name
    /// **The Ought**: Which graph Truth we're working with
    graph_name: String,

    /// Username for model catalog
    /// **The Ought**: User identity in the Truth
    username: String,
}

impl LinkPredictionTrainConfig {
    /// Element projection wildcard (project all)
    pub const PROJECT_ALL: &'static str = "*";

    /// Default metric (AUCPR)
    pub const DEFAULT_METRIC: &'static str = "AUCPR";

    /// Creates a builder for LinkPredictionTrainConfig.
    ///
    /// # The Art of the Prim and Proper Builder!
    ///
    /// Constructs config where Proper (Truth) contains Prim (Appearance).
    pub fn builder() -> LinkPredictionTrainConfigBuilder {
        LinkPredictionTrainConfigBuilder::new()
    }

    // === PRIM GETTERS: The Is ===

    /// Returns the negative class weight.
    /// **The Is**: How negatives appear
    pub fn negative_class_weight(&self) -> f64 {
        self.negative_class_weight
    }

    /// Returns the random seed (if any).
    /// **The Is**: The Given seed
    pub fn random_seed(&self) -> Option<u64> {
        self.random_seed
    }

    // === PROPER GETTERS: The Ought ===

    /// Returns the pipeline name.
    /// **The Ought**: Pipeline identity
    pub fn pipeline(&self) -> &str {
        &self.pipeline
    }

    /// Returns the target relationship type.
    /// **The Ought**: The relationship Truth
    pub fn target_relationship_type(&self) -> &str {
        &self.target_relationship_type
    }

    /// Returns the source node label.
    /// **The Ought**: Source node Truth
    pub fn source_node_label(&self) -> &str {
        &self.source_node_label
    }

    /// Returns the target node label.
    /// **The Ought**: Target node Truth
    pub fn target_node_label(&self) -> &str {
        &self.target_node_label
    }

    /// Returns the metrics.
    /// **The Ought**: Measurement of Truth
    pub fn metrics(&self) -> &[String] {
        &self.metrics
    }

    /// Returns the main metric (first in list).
    /// **The Ought**: Primary Truth measure
    pub fn main_metric(&self) -> &str {
        &self.metrics[0]
    }

    /// Returns the graph name.
    /// **The Ought**: Graph identity
    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Returns the username.
    /// **The Ought**: User identity
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the relationship types (just target type).
    /// **The Ought**: Relationship types in scope
    pub fn relationship_types(&self) -> Vec<&str> {
        vec![&self.target_relationship_type]
    }

    /// Returns the node labels (source + target, deduplicated).
    /// **The Ought**: Node labels in scope
    pub fn node_labels(&self) -> Vec<&str> {
        let mut labels = vec![self.source_node_label.as_str()];
        if self.target_node_label != self.source_node_label {
            labels.push(self.target_node_label.as_str());
        }
        labels
    }

    /// Converts the config to a map (for serialization).
    ///
    /// # Prim and Proper Serialization!
    ///
    /// Serializes both The Is (primitives) and The Ought (properties).
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        // Prim: The Is
        map.insert(
            "negativeClassWeight".to_string(),
            serde_json::json!(self.negative_class_weight),
        );
        if let Some(seed) = self.random_seed {
            map.insert("randomSeed".to_string(), serde_json::json!(seed));
        }

        // Proper: The Ought
        map.insert("pipeline".to_string(), serde_json::json!(self.pipeline));
        map.insert(
            "targetRelationshipType".to_string(),
            serde_json::json!(self.target_relationship_type),
        );
        map.insert(
            "sourceNodeLabel".to_string(),
            serde_json::json!(self.source_node_label),
        );
        map.insert(
            "targetNodeLabel".to_string(),
            serde_json::json!(self.target_node_label),
        );
        map.insert("metrics".to_string(), serde_json::json!(self.metrics));
        map.insert("graphName".to_string(), serde_json::json!(self.graph_name));
        map.insert("username".to_string(), serde_json::json!(self.username));

        map
    }

    /// Validates the configuration.
    ///
    /// # Truth Contains Appearance!
    ///
    /// Validates that The Ought (Truth) properly contains The Is (Appearance):
    /// - Prim: negative_class_weight > 0.0
    /// - Proper: target_relationship_type != "*"
    pub fn validate(&self) -> Result<(), String> {
        // Prim validation: The Is must be well-formed
        if self.negative_class_weight <= 0.0 {
            return Err(format!(
                "negativeClassWeight must be positive, got {}",
                self.negative_class_weight
            ));
        }

        // Proper validation: The Ought must be specific
        if self.target_relationship_type == Self::PROJECT_ALL {
            return Err("'*' is not allowed as targetRelationshipType.".to_string());
        }

        Ok(())
    }

    /// Validates against a graph store.
    ///
    /// # The Ought Manifesting from The Is!
    ///
    /// Checks that The Ought (properties) can manifest from The Is (graph data):
    /// - Source/target labels exist in graph
    /// - Target relationship type exists and is UNDIRECTED
    ///
    /// # Arguments
    ///
    /// * `graph_store` - Graph store to validate against (placeholder)
    pub fn validate_against_graph_store(
        &self,
        _graph_store: PhantomData<()>, // TODO: GraphStore
    ) -> Result<(), String> {
        // TODO: Implement when GraphStore is available:
        // 1. Validate source_node_label exists (unless "*")
        // 2. Validate target_node_label exists (unless "*")
        // 3. Validate target_relationship_type exists and is UNDIRECTED
        Ok(())
    }
}

/// Builder for LinkPredictionTrainConfig.
///
/// # The Art of Building Truth from Appearance!
///
/// Constructs config where Proper (Truth) contains Prim (Appearance).
pub struct LinkPredictionTrainConfigBuilder {
    negative_class_weight: Option<f64>,
    random_seed: Option<u64>,
    pipeline: Option<String>,
    target_relationship_type: Option<String>,
    source_node_label: Option<String>,
    target_node_label: Option<String>,
    metrics: Option<Vec<String>>,
    graph_name: Option<String>,
    username: Option<String>,
}

impl LinkPredictionTrainConfigBuilder {
    /// Creates a new builder with no values set.
    pub fn new() -> Self {
        Self {
            negative_class_weight: None,
            random_seed: None,
            pipeline: None,
            target_relationship_type: None,
            source_node_label: None,
            target_node_label: None,
            metrics: None,
            graph_name: None,
            username: None,
        }
    }

    // === PRIM SETTERS: The Is ===

    /// Sets the negative class weight.
    /// **The Is**: How negatives appear
    pub fn negative_class_weight(mut self, weight: f64) -> Self {
        self.negative_class_weight = Some(weight);
        self
    }

    /// Sets the random seed.
    /// **The Is**: The Given seed
    pub fn random_seed(mut self, seed: u64) -> Self {
        self.random_seed = Some(seed);
        self
    }

    // === PROPER SETTERS: The Ought ===

    /// Sets the pipeline name.
    /// **The Ought**: Pipeline identity
    pub fn pipeline(mut self, pipeline: String) -> Self {
        self.pipeline = Some(pipeline);
        self
    }

    /// Sets the target relationship type.
    /// **The Ought**: Relationship Truth
    pub fn target_relationship_type(mut self, rel_type: String) -> Self {
        self.target_relationship_type = Some(rel_type);
        self
    }

    /// Sets the source node label.
    /// **The Ought**: Source node Truth
    pub fn source_node_label(mut self, label: String) -> Self {
        self.source_node_label = Some(label);
        self
    }

    /// Sets the target node label.
    /// **The Ought**: Target node Truth
    pub fn target_node_label(mut self, label: String) -> Self {
        self.target_node_label = Some(label);
        self
    }

    /// Sets the metrics.
    /// **The Ought**: Truth measurement
    pub fn metrics(mut self, metrics: Vec<String>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Sets the graph name.
    /// **The Ought**: Graph identity
    pub fn graph_name(mut self, name: String) -> Self {
        self.graph_name = Some(name);
        self
    }

    /// Sets the username.
    /// **The Ought**: User identity
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Builds the LinkPredictionTrainConfig with validation.
    ///
    /// # Truth Contains Appearance!
    ///
    /// Validates that The Ought (Truth) properly contains The Is (Appearance).
    ///
    /// # Returns
    ///
    /// Ok(config) if valid, Err(message) if validation fails.
    pub fn build(self) -> Result<LinkPredictionTrainConfig, String> {
        // Required Proper fields
        let pipeline = self.pipeline.ok_or("pipeline is required")?;
        let target_relationship_type = self
            .target_relationship_type
            .ok_or("target_relationship_type is required")?;
        let graph_name = self.graph_name.ok_or("graph_name is required")?;
        let username = self.username.ok_or("username is required")?;

        // Optional with defaults
        let negative_class_weight = self.negative_class_weight.unwrap_or(1.0);
        let source_node_label = self
            .source_node_label
            .unwrap_or(LinkPredictionTrainConfig::PROJECT_ALL.to_string());
        let target_node_label = self
            .target_node_label
            .unwrap_or(LinkPredictionTrainConfig::PROJECT_ALL.to_string());
        let metrics = self
            .metrics
            .unwrap_or_else(|| vec![LinkPredictionTrainConfig::DEFAULT_METRIC.to_string()]);

        let config = LinkPredictionTrainConfig {
            negative_class_weight,
            random_seed: self.random_seed,
            pipeline,
            target_relationship_type,
            source_node_label,
            target_node_label,
            metrics,
            graph_name,
            username,
        };

        // Validate: Truth must properly contain Appearance
        config.validate()?;

        Ok(config)
    }
}

impl Default for LinkPredictionTrainConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_minimal() {
        let config = LinkPredictionTrainConfig::builder()
            .pipeline("test-pipeline".to_string())
            .target_relationship_type("KNOWS".to_string())
            .graph_name("test-graph".to_string())
            .username("test-user".to_string())
            .build()
            .unwrap();

        assert_eq!(config.pipeline(), "test-pipeline");
        assert_eq!(config.target_relationship_type(), "KNOWS");
        assert_eq!(config.negative_class_weight(), 1.0); // default
    }

    #[test]
    fn test_builder_full() {
        let config = LinkPredictionTrainConfig::builder()
            .pipeline("my-pipeline".to_string())
            .target_relationship_type("FRIENDS".to_string())
            .source_node_label("Person".to_string())
            .target_node_label("Person".to_string())
            .negative_class_weight(1.5)
            .random_seed(42)
            .metrics(vec!["AUCPR".to_string(), "ACCURACY".to_string()])
            .graph_name("social-graph".to_string())
            .username("alice".to_string())
            .build()
            .unwrap();

        assert_eq!(config.pipeline(), "my-pipeline");
        assert_eq!(config.negative_class_weight(), 1.5);
        assert_eq!(config.random_seed(), Some(42));
        assert_eq!(config.source_node_label(), "Person");
        assert_eq!(config.target_node_label(), "Person");
    }

    #[test]
    fn test_validation_wildcard_target() {
        let result = LinkPredictionTrainConfig::builder()
            .pipeline("test".to_string())
            .target_relationship_type("*".to_string())
            .graph_name("graph".to_string())
            .username("user".to_string())
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("'*' is not allowed"));
    }

    #[test]
    fn test_validation_negative_weight() {
        let result = LinkPredictionTrainConfig::builder()
            .pipeline("test".to_string())
            .target_relationship_type("KNOWS".to_string())
            .negative_class_weight(0.0)
            .graph_name("graph".to_string())
            .username("user".to_string())
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("negativeClassWeight must be positive"));
    }

    #[test]
    fn test_node_labels_deduplication() {
        let config = LinkPredictionTrainConfig::builder()
            .pipeline("test".to_string())
            .target_relationship_type("KNOWS".to_string())
            .source_node_label("Person".to_string())
            .target_node_label("Person".to_string())
            .graph_name("graph".to_string())
            .username("user".to_string())
            .build()
            .unwrap();

        let labels = config.node_labels();
        assert_eq!(labels.len(), 1); // Deduplicated
        assert_eq!(labels[0], "Person");
    }

    #[test]
    fn test_node_labels_different() {
        let config = LinkPredictionTrainConfig::builder()
            .pipeline("test".to_string())
            .target_relationship_type("LIKES".to_string())
            .source_node_label("Person".to_string())
            .target_node_label("Product".to_string())
            .graph_name("graph".to_string())
            .username("user".to_string())
            .build()
            .unwrap();

        let labels = config.node_labels();
        assert_eq!(labels.len(), 2);
        assert!(labels.contains(&"Person"));
        assert!(labels.contains(&"Product"));
    }

    #[test]
    fn test_main_metric() {
        let config = LinkPredictionTrainConfig::builder()
            .pipeline("test".to_string())
            .target_relationship_type("KNOWS".to_string())
            .metrics(vec!["ACCURACY".to_string(), "AUCPR".to_string()])
            .graph_name("graph".to_string())
            .username("user".to_string())
            .build()
            .unwrap();

        assert_eq!(config.main_metric(), "ACCURACY"); // First in list
    }

    #[test]
    fn test_to_map() {
        let config = LinkPredictionTrainConfig::builder()
            .pipeline("test".to_string())
            .target_relationship_type("KNOWS".to_string())
            .negative_class_weight(2.0)
            .graph_name("graph".to_string())
            .username("user".to_string())
            .build()
            .unwrap();

        let map = config.to_map();
        assert_eq!(map.get("pipeline").unwrap(), &serde_json::json!("test"));
        assert_eq!(
            map.get("negativeClassWeight").unwrap(),
            &serde_json::json!(2.0)
        );
    }

    #[test]
    fn test_prim_and_proper_philosophy() {
        // Science as the Art of the Prim and Proper! 🌟
        // The Is and the Ought

        let config = LinkPredictionTrainConfig::builder()
            .pipeline("truth-pipeline".to_string())
            .target_relationship_type("MANIFESTS".to_string())
            .negative_class_weight(1.0) // The Is: How negatives appear
            .random_seed(42) // The Is: Given seed
            .source_node_label("Truth".to_string()) // The Ought: Truth properties
            .target_node_label("Appearance".to_string()) // The Ought: Appearance in Truth!
            .graph_name("dialectic".to_string())
            .username("hegel".to_string())
            .build()
            .unwrap();

        // The Is (Prim): Primitive appearance
        assert_eq!(config.negative_class_weight(), 1.0);
        assert_eq!(config.random_seed(), Some(42));

        // The Ought (Proper): Truth properties
        assert_eq!(config.target_relationship_type(), "MANIFESTS");
        assert_eq!(config.source_node_label(), "Truth");
        assert_eq!(config.target_node_label(), "Appearance");

        // The Dialectic: No Ought in the Is, but Appearance in Truth!
        // Truth (Proper) contains Appearance (Prim)!
        // The config validates that The Ought properly contains The Is!

        assert!(config.validate().is_ok());
    }
}

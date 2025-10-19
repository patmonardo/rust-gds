// Phase 1.1: LinkFeatureStep - Core trait for link feature extraction

use super::LinkFeatureAppender;
use crate::types::graph::Graph;
use std::collections::HashMap;

/// Core trait for link feature steps in Link Prediction pipelines.
/// Unlike NodeFeatureStep which operates on single nodes, LinkFeatureStep
/// operates on node pairs (source, target) to generate pair-based features.
///
/// Link features use mathematical operations on node property pairs:
/// - Hadamard: element-wise multiplication
/// - Cosine: angular similarity
/// - L2: Euclidean distance
/// - SameCategory: categorical equality
pub trait LinkFeatureStep {
    /// Creates a LinkFeatureAppender for this feature step on the given graph.
    /// The appender is responsible for extracting features for (source, target) pairs.
    fn link_feature_appender(&self, graph: &dyn Graph) -> Box<dyn LinkFeatureAppender>;

    /// Returns the name of this feature step (e.g., "HADAMARD", "COSINE").
    fn name(&self) -> &str;

    /// Returns the configuration of this feature step.
    fn configuration(&self) -> HashMap<String, serde_json::Value>;

    /// Returns the list of input node properties required by this feature step.
    fn input_node_properties(&self) -> Vec<String>;

    /// Converts this feature step to a map representation for serialization.
    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), serde_json::json!(self.name()));
        map.insert(
            "config".to_string(),
            serde_json::json!(self.configuration()),
        );
        map
    }

    /// Clones this feature step into a new boxed trait object.
    /// Required for copying pipelines (e.g., training → predict).
    fn clone_box(&self) -> Box<dyn LinkFeatureStep>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLinkFeatureStep {
        name: String,
        properties: Vec<String>,
    }

    struct TestLinkFeatureAppender;
    impl LinkFeatureAppender for TestLinkFeatureAppender {
        fn append_features(
            &self,
            _source: u64,
            _target: u64,
            _features: &mut [f64],
            _offset: usize,
        ) {
        }
        fn dimension(&self) -> usize {
            0
        }
    }

    impl LinkFeatureStep for TestLinkFeatureStep {
        fn link_feature_appender(&self, _graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
            Box::new(TestLinkFeatureAppender)
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn configuration(&self) -> HashMap<String, serde_json::Value> {
            let mut config = HashMap::new();
            config.insert(
                "nodeProperties".to_string(),
                serde_json::json!(self.properties),
            );
            config
        }

        fn input_node_properties(&self) -> Vec<String> {
            self.properties.clone()
        }

        fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
            Box::new(Self {
                name: self.name.clone(),
                properties: self.properties.clone(),
            })
        }
    }

    #[test]
    fn test_link_feature_step_basic() {
        let step = TestLinkFeatureStep {
            name: "TEST".to_string(),
            properties: vec!["prop1".to_string(), "prop2".to_string()],
        };

        assert_eq!(step.name(), "TEST");
        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_link_feature_step_to_map() {
        let step = TestLinkFeatureStep {
            name: "HADAMARD".to_string(),
            properties: vec!["embedding".to_string()],
        };

        let map = step.to_map();
        assert_eq!(map.get("name").unwrap(), "HADAMARD");
        assert!(map.contains_key("config"));
    }

    #[test]
    fn test_link_feature_step_appender_creation() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 10,
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).expect("random graph");
        let graph = store.graph();

        let step = TestLinkFeatureStep {
            name: "COSINE".to_string(),
            properties: vec!["features".to_string()],
        };

        let _appender = step.link_feature_appender(&graph);
        // Appender creation succeeds (placeholder for now)
    }
}

/*
 * Copyright (c) "Neo4j"
 * Neo4j Sweden AB [http://neo4j.com]
 *
 * This file is part of Neo4j.
 *
 * Neo4j is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::ml::core::subgraph::LocalIdMap;
use crate::projection::eval::ml::pipeline::node_pipeline::NodePropertyPipelineBaseTrainConfig;

// Placeholder types until ml-metrics and collections packages are translated
pub type ClassificationMetricSpecification = ();
pub type Metric = ();
pub type ClassificationMetric = ();
pub type LongMultiSet = std::collections::HashMap<i64, usize>;

/// Training configuration for node classification pipelines.
///
/// Extends the base training configuration with classification-specific metrics.
#[derive(Debug, Clone)]
pub struct NodeClassificationPipelineTrainConfig {
    pipeline_name: String,
    target_labels: Vec<String>,
    target_property: String,
    random_seed: Option<u64>,
    metrics: Vec<ClassificationMetricSpecification>,
}

impl NodeClassificationPipelineTrainConfig {
    pub fn new(
        pipeline_name: String,
        target_labels: Vec<String>,
        target_property: String,
        random_seed: Option<u64>,
        metrics: Vec<ClassificationMetricSpecification>,
    ) -> Self {
        Self {
            pipeline_name,
            target_labels,
            target_property,
            random_seed,
            metrics,
        }
    }

    pub fn metrics_specs(&self) -> &[ClassificationMetricSpecification] {
        &self.metrics
    }

    /// Create concrete metrics from specifications given class ID map and class counts.
    pub fn metrics(&self, _class_id_map: &LocalIdMap, _class_counts: &LongMultiSet) -> Vec<Metric> {
        // TODO: Implement when ClassificationMetricSpecification is translated
        // self.metrics.iter().flat_map(|spec| spec.create_metrics(class_id_map, class_counts)).collect()
        vec![]
    }

    /// Filter classification metrics (non-model-specific).
    pub fn classification_metrics(metrics: &[Metric]) -> Vec<ClassificationMetric> {
        // TODO: Implement when Metric trait is translated
        // metrics.iter().filter(|m| !m.is_model_specific()).map(|m| m as ClassificationMetric).collect()
        vec![]
    }
}

impl Default for NodeClassificationPipelineTrainConfig {
    fn default() -> Self {
        Self::new(
            "default_pipeline".to_string(),
            vec!["*".to_string()],
            "target".to_string(),
            Some(42),
            vec![],
        )
    }
}

impl NodePropertyPipelineBaseTrainConfig for NodeClassificationPipelineTrainConfig {
    fn pipeline(&self) -> &str {
        &self.pipeline_name
    }

    fn target_node_labels(&self) -> Vec<String> {
        self.target_labels.clone()
    }

    fn target_property(&self) -> &str {
        &self.target_property
    }

    fn random_seed(&self) -> Option<u64> {
        self.random_seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = NodeClassificationPipelineTrainConfig::new(
            "test_pipeline".to_string(),
            vec!["Label1".to_string()],
            "target_property".to_string(),
            Some(42),
            vec![],
        );
        assert_eq!(config.metrics_specs().len(), 0);
        assert_eq!(config.pipeline(), "test_pipeline");
        assert_eq!(config.target_property(), "target_property");
        assert_eq!(config.random_seed(), Some(42));
    }

    #[test]
    fn test_default_config() {
        let config = NodeClassificationPipelineTrainConfig::default();
        assert_eq!(config.metrics_specs().len(), 0);
        assert_eq!(config.pipeline(), "default_pipeline");
        assert_eq!(config.target_node_labels(), vec!["*"]);
        assert_eq!(config.target_property(), "target");
        assert_eq!(config.random_seed(), Some(42));
    }

    #[test]
    fn test_metrics_placeholder() {
        let config = NodeClassificationPipelineTrainConfig::default();
        let class_id_map = LocalIdMap::of(&[0, 1, 2]);
        let class_counts = LongMultiSet::new();

        let metrics = config.metrics(&class_id_map, &class_counts);
        // Until metrics are implemented, should return empty vec
        assert_eq!(metrics.len(), 0);
    }

    #[test]
    fn test_classification_metrics_filter() {
        let metrics: Vec<Metric> = vec![];
        let filtered = NodeClassificationPipelineTrainConfig::classification_metrics(&metrics);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_trait_implementation() {
        let config = NodeClassificationPipelineTrainConfig::new(
            "my_pipeline".to_string(),
            vec!["Person".to_string(), "Company".to_string()],
            "class_label".to_string(),
            Some(1337),
            vec![],
        );

        // Test NodePropertyPipelineBaseTrainConfig trait methods
        assert_eq!(config.pipeline(), "my_pipeline");
        assert_eq!(config.target_node_labels(), vec!["Person", "Company"]);
        assert_eq!(config.target_property(), "class_label");
        assert_eq!(config.random_seed(), Some(1337));
    }
}

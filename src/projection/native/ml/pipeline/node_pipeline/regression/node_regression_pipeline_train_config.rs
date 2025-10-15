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

use crate::projection::native::ml::pipeline::node_pipeline::NodePropertyPipelineBaseTrainConfig;

// Placeholder type until ml-metrics package is translated
pub type RegressionMetrics = ();

/// Training configuration for node regression pipelines.
///
/// Extends the base training configuration with regression-specific metrics.
/// Java source: `NodeRegressionPipelineTrainConfig.java`
#[derive(Debug, Clone)]
pub struct NodeRegressionPipelineTrainConfig {
    pipeline_name: String,
    target_labels: Vec<String>,
    target_property: String,
    random_seed: Option<u64>,
    metrics: Vec<RegressionMetrics>,
}

impl NodeRegressionPipelineTrainConfig {
    pub fn new(
        pipeline_name: String,
        target_labels: Vec<String>,
        target_property: String,
        random_seed: Option<u64>,
        metrics: Vec<RegressionMetrics>,
    ) -> Self {
        Self {
            pipeline_name,
            target_labels,
            target_property,
            random_seed,
            metrics,
        }
    }

    /// Returns the configured regression metrics.
    pub fn metrics(&self) -> &[RegressionMetrics] {
        &self.metrics
    }

    /// Validates that at least one metric is specified.
    ///
    /// Java source: `@Configuration.Check validateMetrics()`
    pub fn validate_metrics(&self) -> Result<(), String> {
        if self.metrics.is_empty() {
            Err(
                "Must specify at least one evaluation metric via the `metrics` parameter."
                    .to_string(),
            )
        } else {
            Ok(())
        }
    }
}

impl Default for NodeRegressionPipelineTrainConfig {
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

impl NodePropertyPipelineBaseTrainConfig for NodeRegressionPipelineTrainConfig {
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
        let config = NodeRegressionPipelineTrainConfig::new(
            "test_pipeline".to_string(),
            vec!["Label1".to_string()],
            "target_property".to_string(),
            Some(42),
            vec![()], // Placeholder metric
        );

        assert_eq!(config.pipeline(), "test_pipeline");
        assert_eq!(config.target_node_labels(), vec!["Label1".to_string()]);
        assert_eq!(config.target_property(), "target_property");
        assert_eq!(config.random_seed(), Some(42));
        assert_eq!(config.metrics().len(), 1);
    }

    #[test]
    fn test_default_config() {
        let config = NodeRegressionPipelineTrainConfig::default();

        assert_eq!(config.pipeline(), "default_pipeline");
        assert_eq!(config.target_node_labels(), vec!["*".to_string()]);
        assert_eq!(config.target_property(), "target");
        assert_eq!(config.random_seed(), Some(42));
        assert!(config.metrics().is_empty());
    }

    #[test]
    fn test_validate_metrics_empty_fails() {
        let config = NodeRegressionPipelineTrainConfig::default();

        let result = config.validate_metrics();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Must specify at least one evaluation metric via the `metrics` parameter."
        );
    }

    #[test]
    fn test_validate_metrics_non_empty_succeeds() {
        let config = NodeRegressionPipelineTrainConfig::new(
            "test_pipeline".to_string(),
            vec!["Label1".to_string()],
            "target_property".to_string(),
            Some(42),
            vec![()], // Placeholder metric
        );

        let result = config.validate_metrics();
        assert!(result.is_ok());
    }

    #[test]
    fn test_metrics_accessor() {
        let config = NodeRegressionPipelineTrainConfig::new(
            "test_pipeline".to_string(),
            vec!["Label1".to_string()],
            "target_property".to_string(),
            Some(42),
            vec![(), ()], // Two placeholder metrics
        );

        assert_eq!(config.metrics().len(), 2);
    }
}

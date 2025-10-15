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

//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineCompanion.java
//!
//! Companion utilities for ML pipeline configuration and validation.
//!
//! Provides helpers for:
//! - Preparing pipeline configurations with graph names
//! - Configuring auto-tuning parameters
//! - Validating metric compatibility with training methods

use std::collections::HashMap;

/// Anonymous graph name used when operating on in-memory graphs without a catalog entry.
///
/// # Java Source
/// ```java
/// public static final String ANONYMOUS_GRAPH = "__ANONYMOUS_GRAPH__";
/// ```
pub const ANONYMOUS_GRAPH: &str = "__ANONYMOUS_GRAPH__";

/// Name of the Out-of-Bag Error metric (specific to Random Forest).
///
/// This metric can only be used with Random Forest model candidates.
pub const OUT_OF_BAG_ERROR: &str = "OUT_OF_BAG_ERROR";

/// Prepare a pipeline configuration by setting the graph name.
///
/// If `graph_name_or_configuration` is a string, it's used as the graph name.
/// Otherwise, the anonymous graph name is used.
///
/// # Java Source
/// ```java
/// public static void preparePipelineConfig(
///     Object graphNameOrConfiguration,
///     Map<String, Object> algoConfiguration
/// ) {
///     if (graphNameOrConfiguration instanceof String) {
///         algoConfiguration.put("graphName", graphNameOrConfiguration);
///     } else {
///         algoConfiguration.put("graphName", ANONYMOUS_GRAPH);
///     }
/// }
/// ```
///
/// # Note
/// In Java GDS, this is used to handle the case where node property steps
/// modify the graph store. In the future, this might operate on a shallow
/// copy instead.
pub fn prepare_pipeline_config(
    graph_name: Option<&str>,
    algo_configuration: &mut HashMap<String, serde_json::Value>,
) {
    let graph_name_value = match graph_name {
        Some(name) => serde_json::Value::String(name.to_string()),
        None => serde_json::Value::String(ANONYMOUS_GRAPH.to_string()),
    };

    algo_configuration.insert("graphName".to_string(), graph_name_value);
}

/// Validate that the main metric is compatible with the pipeline's training methods.
///
/// If OUT_OF_BAG_ERROR is used as the main metric, only Random Forest model
/// candidates are allowed.
///
/// # Java Source
/// ```java
/// public static void validateMainMetric(TrainingPipeline<?> pipeline, String mainMetric) {
///     if (mainMetric.equals(OUT_OF_BAG_ERROR.name())) {
///         var nonRFMethods = pipeline.trainingParameterSpace().entrySet().stream()
///             .filter(entry -> entry.getKey() != TrainingMethod.RandomForestClassification && !entry.getValue().isEmpty() )
///             .map(Map.Entry::getKey)
///             .map(Enum::toString)
///             .collect(Collectors.toSet());
///         if (!nonRFMethods.isEmpty()) {
///             throw new IllegalArgumentException(formatWithLocale(
///                 "If %s is used as the main metric (the first one), then only RandomForest model candidates are allowed." +
///                 " Incompatible training methods used are: %s.",
///                 OUT_OF_BAG_ERROR.name(),
///                 StringJoining.join(nonRFMethods)
///             ));
///         }
///     }
/// }
/// ```
pub fn validate_main_metric(
    main_metric: &str,
    training_methods: &[String],
) -> Result<(), PipelineCompanionError> {
    if main_metric == OUT_OF_BAG_ERROR {
        let non_rf_methods: Vec<String> = training_methods
            .iter()
            .filter(|method| !method.contains("RandomForest"))
            .cloned()
            .collect();

        if !non_rf_methods.is_empty() {
            return Err(PipelineCompanionError::IncompatibleMetric {
                metric: OUT_OF_BAG_ERROR.to_string(),
                incompatible_methods: non_rf_methods,
            });
        }
    }

    Ok(())
}

/// Errors that can occur in pipeline companion operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineCompanionError {
    /// Main metric is incompatible with training methods
    IncompatibleMetric {
        /// The metric that caused the issue
        metric: String,
        /// Training methods that are incompatible
        incompatible_methods: Vec<String>,
    },
}

impl std::fmt::Display for PipelineCompanionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineCompanionError::IncompatibleMetric {
                metric,
                incompatible_methods,
            } => {
                write!(
                    f,
                    "If {} is used as the main metric (the first one), then only RandomForest model candidates are allowed. \
                    Incompatible training methods used are: {}.",
                    metric,
                    incompatible_methods.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for PipelineCompanionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_pipeline_config_with_name() {
        let mut config = HashMap::new();
        prepare_pipeline_config(Some("myGraph"), &mut config);

        assert_eq!(
            config.get("graphName").and_then(|v| v.as_str()),
            Some("myGraph")
        );
    }

    #[test]
    fn test_prepare_pipeline_config_anonymous() {
        let mut config = HashMap::new();
        prepare_pipeline_config(None, &mut config);

        assert_eq!(
            config.get("graphName").and_then(|v| v.as_str()),
            Some(ANONYMOUS_GRAPH)
        );
    }

    #[test]
    fn test_validate_main_metric_rf_only_with_rf() {
        let methods = vec!["RandomForestClassification".to_string()];
        let result = validate_main_metric(OUT_OF_BAG_ERROR, &methods);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_main_metric_rf_only_with_non_rf() {
        let methods = vec![
            "LogisticRegression".to_string(),
            "RandomForestClassification".to_string(),
        ];
        let result = validate_main_metric(OUT_OF_BAG_ERROR, &methods);
        assert!(result.is_err());

        if let Err(PipelineCompanionError::IncompatibleMetric {
            incompatible_methods,
            ..
        }) = result
        {
            assert_eq!(incompatible_methods, vec!["LogisticRegression"]);
        }
    }

    #[test]
    fn test_validate_main_metric_other_metric() {
        let methods = vec!["LogisticRegression".to_string()];
        let result = validate_main_metric("ACCURACY", &methods);
        assert!(result.is_ok());
    }
}

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

//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/NodePropertyStepFactory.java
//!
//! Factory for creating node property steps from configuration.
//!
//! Handles:
//! - Parsing context configuration (contextNodeLabels, contextRelationshipTypes)
//! - Validating reserved configuration keys
//! - Normalizing algorithm names (e.g., "pagerank" → "gds.pagerank.mutate")
//! - Creating NodePropertyStep instances
//!
//! **Rust Simplification**: This is a simplified direct-integration version without the Java
//! Stub factory infrastructure. Algorithm validation happens via basic checks rather than
//! the full Java GdsCallableFinder/AlgoConfigParser system.

use crate::projection::eval::ml::pipeline::{
    ExecutableNodePropertyStep, NodePropertyStep, NodePropertyStepContextConfig,
};
use std::collections::HashMap;

/// Reserved configuration keys that cannot be set in individual node property steps.
///
/// These are set by the pipeline executor based on the current execution context.
const RESERVED_CONFIG_KEYS: &[&str] = &["nodeLabels", "relationshipTypes"];

/// Create a node property step from a task name and configuration map.
///
/// This is the primary entry point for creating steps from user configuration.
///
/// # Java Source
/// ```java
/// public static ExecutableNodePropertyStep createNodePropertyStep(
///     String taskName,
///     Map<String, Object> configMap
/// ) {
///     var procConfigMap = new HashMap<>(configMap);
///     var contextNodeLabels = procConfigMap.remove(CONTEXT_NODE_LABELS);
///     var contextRelationshipTypes = procConfigMap.remove(CONTEXT_RELATIONSHIP_TYPES);
///     // ... creates context config and delegates
/// }
/// ```
pub fn create_node_property_step(
    task_name: &str,
    config_map: HashMap<String, serde_json::Value>,
) -> Result<Box<dyn ExecutableNodePropertyStep>, NodePropertyStepFactoryError> {
    let mut proc_config = config_map.clone();

    // Extract context configuration
    let context_node_labels = proc_config
        .remove(NodePropertyStepContextConfig::CONTEXT_NODE_LABELS)
        .and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
        })
        .unwrap_or_default();

    let context_relationship_types = proc_config
        .remove(NodePropertyStepContextConfig::CONTEXT_RELATIONSHIP_TYPES)
        .and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
        })
        .unwrap_or_default();

    create_node_property_step_with_context(
        task_name,
        proc_config,
        context_node_labels,
        context_relationship_types,
    )
}

/// Create a node property step with explicit context configuration.
///
/// # Java Source
/// ```java
/// public static ExecutableNodePropertyStep createNodePropertyStep(
///     String taskName,
///     Map<String, Object> procConfigMap,
///     List<String> contextNodeLabels,
///     List<String> contextRelationshipTypes
/// ) {
///     return createNodePropertyStep(procConfigMap, taskName, contextNodeLabels, contextRelationshipTypes);
/// }
/// ```
pub fn create_node_property_step_with_context(
    task_name: &str,
    proc_config_map: HashMap<String, serde_json::Value>,
    context_node_labels: Vec<String>,
    context_relationship_types: Vec<String>,
) -> Result<Box<dyn ExecutableNodePropertyStep>, NodePropertyStepFactoryError> {
    // Validate reserved keys
    validate_reserved_config_keys(&proc_config_map)?;

    // Normalize algorithm name
    let normalized_name = normalize_name(task_name);

    // TODO: Validate algorithm configuration
    // In Java, this uses AlgoConfigParser to validate the config against the algorithm spec.
    // In Rust direct integration, we can add validation when we have the algorithm registry.
    // For now, basic checks are sufficient.

    // Create the step
    let step = NodePropertyStep::with_context(
        normalized_name,
        proc_config_map,
        context_node_labels,
        context_relationship_types,
    );

    Ok(Box::new(step))
}

/// Validate that procedure configuration doesn't contain reserved keys.
///
/// # Java Source
/// ```java
/// private static void validateReservedConfigKeys(Map<String, Object> procedureConfig) {
///     if (RESERVED_CONFIG_KEYS.stream().anyMatch(procedureConfig::containsKey)) {
///         throw new IllegalArgumentException(formatWithLocale(
///             "Cannot configure %s for an individual node property step.",
///             StringJoining.join(RESERVED_CONFIG_KEYS)
///         ));
///     }
/// }
/// ```
fn validate_reserved_config_keys(
    procedure_config: &HashMap<String, serde_json::Value>,
) -> Result<(), NodePropertyStepFactoryError> {
    for key in RESERVED_CONFIG_KEYS {
        if procedure_config.contains_key(*key) {
            return Err(NodePropertyStepFactoryError::ReservedConfigKey {
                key: key.to_string(),
                reserved_keys: RESERVED_CONFIG_KEYS.iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    Ok(())
}

/// Normalize an algorithm name to the GDS canonical form.
///
/// Transformations:
/// - Convert to lowercase
/// - Ensure "gds." prefix
/// - Ensure ".mutate" suffix
///
/// Examples:
/// - "pagerank" → "gds.pagerank.mutate"
/// - "PageRank" → "gds.pagerank.mutate"
/// - "gds.fastRP" → "gds.fastrp.mutate"
/// - "gds.pagerank.mutate" → "gds.pagerank.mutate"
///
/// # Java Source
/// ```java
/// private static String normalizeName(String input) {
///     input = input.toLowerCase(Locale.ROOT);
///     input = !input.startsWith("gds.") ? formatWithLocale("gds.%s", input) : input;
///     input = !input.endsWith(".mutate") ? formatWithLocale("%s.mutate", input) : input;
///     return input;
/// }
/// ```
fn normalize_name(input: &str) -> String {
    let mut result = input.to_lowercase();

    if !result.starts_with("gds.") {
        result = format!("gds.{}", result);
    }

    if !result.ends_with(".mutate") {
        result = format!("{}.mutate", result);
    }

    result
}

/// Errors that can occur during node property step factory operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodePropertyStepFactoryError {
    /// Configuration contains a reserved key
    ReservedConfigKey {
        /// The reserved key that was found
        key: String,
        /// All reserved keys
        reserved_keys: Vec<String>,
    },

    /// Algorithm not found
    AlgorithmNotFound {
        /// The normalized algorithm name
        algorithm_name: String,
    },

    /// Algorithm does not support mutate mode
    NotMutateMode {
        /// The algorithm name
        algorithm_name: String,
    },

    /// Invalid configuration
    InvalidConfiguration {
        /// The algorithm name
        algorithm_name: String,
        /// Error message
        message: String,
    },
}

impl std::fmt::Display for NodePropertyStepFactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodePropertyStepFactoryError::ReservedConfigKey { key, reserved_keys } => {
                write!(
                    f,
                    "Cannot configure '{}' for an individual node property step. Reserved keys: [{}]",
                    key,
                    reserved_keys.join(", ")
                )
            }
            NodePropertyStepFactoryError::AlgorithmNotFound { algorithm_name } => {
                write!(f, "Could not find a procedure called {}", algorithm_name)
            }
            NodePropertyStepFactoryError::NotMutateMode { algorithm_name } => {
                write!(
                    f,
                    "The procedure {} does not mutate node properties and is thus not allowed as node property step",
                    algorithm_name
                )
            }
            NodePropertyStepFactoryError::InvalidConfiguration {
                algorithm_name,
                message,
            } => {
                write!(
                    f,
                    "Invalid configuration for algorithm '{}': {}",
                    algorithm_name, message
                )
            }
        }
    }
}

impl std::error::Error for NodePropertyStepFactoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_name_basic() {
        assert_eq!(normalize_name("pagerank"), "gds.pagerank.mutate");
        assert_eq!(normalize_name("PageRank"), "gds.pagerank.mutate");
        assert_eq!(normalize_name("fastRP"), "gds.fastrp.mutate");
    }

    #[test]
    fn test_normalize_name_with_prefix() {
        assert_eq!(normalize_name("gds.pagerank"), "gds.pagerank.mutate");
        assert_eq!(normalize_name("gds.fastRP"), "gds.fastrp.mutate");
    }

    #[test]
    fn test_normalize_name_with_suffix() {
        assert_eq!(normalize_name("pagerank.mutate"), "gds.pagerank.mutate");
        assert_eq!(normalize_name("fastRP.mutate"), "gds.fastrp.mutate");
    }

    #[test]
    fn test_normalize_name_full() {
        assert_eq!(normalize_name("gds.pagerank.mutate"), "gds.pagerank.mutate");
    }

    #[test]
    fn test_validate_reserved_keys_ok() {
        let mut config = HashMap::new();
        config.insert(
            "maxIterations".to_string(),
            serde_json::Value::Number(20i32.into()),
        );

        assert!(validate_reserved_config_keys(&config).is_ok());
    }

    #[test]
    fn test_validate_reserved_keys_error() {
        let mut config = HashMap::new();
        config.insert("nodeLabels".to_string(), serde_json::Value::Array(vec![]));

        let result = validate_reserved_config_keys(&config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            NodePropertyStepFactoryError::ReservedConfigKey { .. }
        ));
    }

    #[test]
    fn test_create_node_property_step() {
        let mut config = HashMap::new();
        config.insert(
            "mutateProperty".to_string(),
            serde_json::Value::String("pagerank".to_string()),
        );
        config.insert(
            "maxIterations".to_string(),
            serde_json::Value::Number(20i32.into()),
        );

        let result = create_node_property_step("pagerank", config);
        assert!(result.is_ok());

        let step = result.unwrap();
        assert_eq!(step.proc_name(), "gds.pagerank.mutate");
    }

    #[test]
    fn test_create_with_context() {
        let mut config = HashMap::new();
        config.insert(
            "mutateProperty".to_string(),
            serde_json::Value::String("embedding".to_string()),
        );
        config.insert(
            NodePropertyStepContextConfig::CONTEXT_NODE_LABELS.to_string(),
            serde_json::Value::Array(vec![serde_json::Value::String("Person".to_string())]),
        );

        let result = create_node_property_step("fastRP", config);
        assert!(result.is_ok());

        let step = result.unwrap();
        assert_eq!(step.context_node_labels(), &["Person"]);
    }

    #[test]
    fn test_reserved_key_error() {
        let mut config = HashMap::new();
        config.insert(
            "mutateProperty".to_string(),
            serde_json::Value::String("prop".to_string()),
        );
        config.insert(
            "relationshipTypes".to_string(),
            serde_json::Value::Array(vec![]),
        );

        let result = create_node_property_step("pagerank", config);
        assert!(result.is_err());
    }
}

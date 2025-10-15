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

//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/NodePropertyStepContextConfig.java
//!
//! Context configuration for node property steps in ML pipelines.
//!
//! Allows specification of which node labels and relationship types should be
//! considered when executing a node property computation step (e.g., running
//! an algorithm like PageRank as part of feature extraction).

use std::collections::HashMap;

/// Configuration for the execution context of a node property step.
///
/// Specifies which node labels and relationship types should be included
/// when running a node property computation algorithm (e.g., centrality measures,
/// community detection) as part of pipeline feature extraction.
///
/// # Java Source
/// ```java
/// @Configuration
/// public interface NodePropertyStepContextConfig {
///     String CONTEXT_NODE_LABELS = "contextNodeLabels";
///     String CONTEXT_RELATIONSHIP_TYPES = "contextRelationshipTypes";
///
///     default List<String> contextNodeLabels() { return List.of(); }
///     default List<String> contextRelationshipTypes() { return List.of(); }
///
///     static NodePropertyStepContextConfig of(Map<String, Object> contextConfigMap) {
///         var cypherMapWrapper = CypherMapWrapper.create(contextConfigMap);
///         return new NodePropertyStepContextConfigImpl(cypherMapWrapper);
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePropertyStepContextConfig {
    /// Node labels to include in the context (defaults to empty = all labels)
    context_node_labels: Vec<String>,

    /// Relationship types to include in the context (defaults to empty = all types)
    context_relationship_types: Vec<String>,
}

impl NodePropertyStepContextConfig {
    /// Configuration key for context node labels
    pub const CONTEXT_NODE_LABELS: &'static str = "contextNodeLabels";

    /// Configuration key for context relationship types
    pub const CONTEXT_RELATIONSHIP_TYPES: &'static str = "contextRelationshipTypes";

    /// Create a new context configuration.
    ///
    /// # Arguments
    /// * `context_node_labels` - Node labels to include (empty = all)
    /// * `context_relationship_types` - Relationship types to include (empty = all)
    pub fn new(context_node_labels: Vec<String>, context_relationship_types: Vec<String>) -> Self {
        Self {
            context_node_labels,
            context_relationship_types,
        }
    }

    /// Create a context configuration from a map.
    ///
    /// # Arguments
    /// * `config_map` - Map containing contextNodeLabels and/or contextRelationshipTypes
    pub fn from_map(config_map: &HashMap<String, serde_json::Value>) -> Self {
        let context_node_labels = config_map
            .get(Self::CONTEXT_NODE_LABELS)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let context_relationship_types = config_map
            .get(Self::CONTEXT_RELATIONSHIP_TYPES)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        Self {
            context_node_labels,
            context_relationship_types,
        }
    }

    /// Get the context node labels.
    pub fn context_node_labels(&self) -> &[String] {
        &self.context_node_labels
    }

    /// Get the context relationship types.
    pub fn context_relationship_types(&self) -> &[String] {
        &self.context_relationship_types
    }
}

impl Default for NodePropertyStepContextConfig {
    fn default() -> Self {
        Self {
            context_node_labels: Vec::new(),
            context_relationship_types: Vec::new(),
        }
    }
}

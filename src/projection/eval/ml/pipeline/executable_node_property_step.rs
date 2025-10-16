//! ExecutableNodePropertyStep trait
//!
//! Direct 1:1 translation of Java `org.neo4j.gds.ml.pipeline.ExecutableNodePropertyStep`.

use std::collections::HashMap;
use std::error::Error as StdError;

/// Executable node property step.
///
/// **Java Source**: `org.neo4j.gds.ml.pipeline.ExecutableNodePropertyStep`
///
/// **Purpose**: Execute a graph algorithm to compute and mutate node properties
/// that will be used as features for ML.
///
/// **Examples**:
/// - PageRank → `pagerank` property
/// - FastRP → `embedding` property
/// - Louvain → `community` property
///
/// **Note on Rust Design**: Uses concrete `DefaultGraphStore` for dyn-compatibility.
/// Java uses dependency injection via `Stub` interface; we use direct type reference.
///
/// ```java
/// public interface ExecutableNodePropertyStep extends ToMapConvertible {
///     void execute(ExecutionContext, String graphName, Collection<NodeLabel>,
///                  Collection<RelationshipType>, Concurrency, Stub);
///     Map<String, Object> config();
///     default List<String> contextNodeLabels() { return List.of(); }
///     default List<String> contextRelationshipTypes() { return List.of(); }
///     String procName();
///     default String rootTaskName() { return procName(); }
///     MemoryEstimation estimate(..., Stub);
///     String mutateNodeProperty();
/// }
/// ```
pub trait ExecutableNodePropertyStep {
    /// Execute the algorithm and mutate graph store with computed property.
    ///
    /// **Java**: `void execute(ExecutionContext, String graphName, ...)`
    ///
    /// **Rust Adaptation**: Uses `&mut DefaultGraphStore` directly instead of
    /// ExecutionContext + graphName + Stub pattern.
    fn execute(
        &self,
        graph_store: &mut crate::types::graph_store::DefaultGraphStore,
        node_labels: &[String],
        relationship_types: &[String],
        concurrency: usize,
    ) -> Result<(), Box<dyn StdError>>;

    /// Configuration for this step.
    ///
    /// **Java**: `Map<String, Object> config()`
    fn config(&self) -> &HashMap<String, serde_json::Value>;

    /// Context node labels (additional labels beyond train/test).
    ///
    /// **Java**: `default List<String> contextNodeLabels() { return List.of(); }`
    fn context_node_labels(&self) -> &[String] {
        &[]
    }

    /// Context relationship types (additional types beyond train/test).
    ///
    /// **Java**: `default List<String> contextRelationshipTypes() { return List.of(); }`
    fn context_relationship_types(&self) -> &[String] {
        &[]
    }

    /// Procedure name (e.g., "gds.pageRank.mutate").
    ///
    /// **Java**: `String procName()`
    fn proc_name(&self) -> &str;

    /// Root task name for progress tracking.
    ///
    /// **Java**: `default String rootTaskName() { return procName(); }`
    fn root_task_name(&self) -> &str {
        self.proc_name()
    }

    /// The property name that will be mutated.
    ///
    /// **Java**: `String mutateNodeProperty()`
    fn mutate_node_property(&self) -> &str;

    /// Convert to map for serialization (ToMapConvertible).
    ///
    /// **Java**: Inherited from `ToMapConvertible` interface
    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("procName".to_string(), serde_json::json!(self.proc_name()));
        map.insert(
            "mutateProperty".to_string(),
            serde_json::json!(self.mutate_node_property()),
        );
        map.insert("config".to_string(), serde_json::json!(self.config()));
        map
    }
}

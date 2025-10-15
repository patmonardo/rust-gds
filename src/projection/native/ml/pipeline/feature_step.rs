//! FeatureStep trait
//!
//! Direct 1:1 translation of Java `org.neo4j.gds.ml.pipeline.FeatureStep`.

use std::collections::HashMap;

/// Feature step abstraction.
///
/// **Java Source**: `org.neo4j.gds.ml.pipeline.FeatureStep`
///
/// ```java
/// public interface FeatureStep extends ToMapConvertible {
///     List<String> inputNodeProperties();
///     String name();
///     Map<String, Object> configuration();
/// }
/// ```
pub trait FeatureStep {
    /// Input node properties required for feature extraction.
    ///
    /// **Java**: `List<String> inputNodeProperties()`
    fn input_node_properties(&self) -> &[String];

    /// Name of this feature step.
    ///
    /// **Java**: `String name()`
    fn name(&self) -> &str;

    /// Configuration map for this step.
    ///
    /// **Java**: `Map<String, Object> configuration()`
    fn configuration(&self) -> &HashMap<String, serde_json::Value>;

    /// Convert to map for serialization (ToMapConvertible).
    ///
    /// **Java**: Inherited from `ToMapConvertible` interface
    fn to_map(&self) -> HashMap<String, serde_json::Value>;
}

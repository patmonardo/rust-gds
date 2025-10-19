//! Computation Result - Algorithm output with metadata
//!
//! Translated from: `org.neo4j.gds.executor.ComputationResult`
//! Source: ComputationResult.java (60 lines)
//!
//! Wraps algorithm results with timing information and metadata.

use serde_json::Value as JsonValue;
use std::time::Duration;

/// Computation Result - Algorithm output with metadata
///
/// Wraps the algorithm's raw result with:
/// - Timing information (preprocess + compute)
/// - Configuration used
/// - Graph empty flag
///
/// **Simplified from Java GDS**:
/// - No separate Algorithm type (algorithm logic is in implementations)
/// - No Graph/GraphStore reference (handled by executor)
/// - No ResultStore (we don't have catalog complexity yet)
/// - Simplified to essential timing + result data
pub struct ComputationResult<R> {
    /// The actual algorithm result
    result: R,

    /// Time taken for computation
    compute_time: Duration,

    /// Time taken for pre-processing (loading, projection)
    preprocess_time: Duration,

    /// Configuration used (JSON)
    config: JsonValue,

    /// Whether the graph was empty (no computation happened)
    is_graph_empty: bool,
}

impl<R> ComputationResult<R> {
    /// Create a new computation result with just the result and compute time
    pub fn new(result: R, compute_time: Duration) -> Self {
        Self {
            result,
            compute_time,
            preprocess_time: Duration::ZERO,
            config: JsonValue::Null,
            is_graph_empty: false,
        }
    }

    /// Get a reference to the result
    pub fn result(&self) -> &R {
        &self.result
    }

    /// Get a mutable reference to the result
    pub fn result_mut(&mut self) -> &mut R {
        &mut self.result
    }

    /// Consume this ComputationResult and return the inner result
    pub fn into_result(self) -> R {
        self.result
    }

    /// Get compute time in milliseconds
    pub fn compute_millis(&self) -> u64 {
        self.compute_time.as_millis() as u64
    }

    /// Get preprocess time in milliseconds
    pub fn preprocess_millis(&self) -> u64 {
        self.preprocess_time.as_millis() as u64
    }

    /// Get total time (preprocess + compute) in milliseconds
    pub fn total_millis(&self) -> u64 {
        self.preprocess_millis() + self.compute_millis()
    }

    /// Get the configuration used
    pub fn config(&self) -> &JsonValue {
        &self.config
    }

    /// Check if the graph was empty
    pub fn is_graph_empty(&self) -> bool {
        self.is_graph_empty
    }

    /// Builder: Set preprocess time
    pub fn with_preprocess_time(mut self, duration: Duration) -> Self {
        self.preprocess_time = duration;
        self
    }

    /// Builder: Set configuration
    pub fn with_config(mut self, config: JsonValue) -> Self {
        self.config = config;
        self
    }

    /// Builder: Mark graph as empty
    pub fn mark_graph_empty(mut self) -> Self {
        self.is_graph_empty = true;
        self
    }

    /// Map the result to a different type
    pub fn map<U, F>(self, f: F) -> ComputationResult<U>
    where
        F: FnOnce(R) -> U,
    {
        ComputationResult {
            result: f(self.result),
            compute_time: self.compute_time,
            preprocess_time: self.preprocess_time,
            config: self.config,
            is_graph_empty: self.is_graph_empty,
        }
    }
}

impl<R: Clone> Clone for ComputationResult<R> {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
            compute_time: self.compute_time,
            preprocess_time: self.preprocess_time,
            config: self.config.clone(),
            is_graph_empty: self.is_graph_empty,
        }
    }
}

impl<R: std::fmt::Debug> std::fmt::Debug for ComputationResult<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputationResult")
            .field("result", &self.result)
            .field("compute_millis", &self.compute_millis())
            .field("preprocess_millis", &self.preprocess_millis())
            .field("is_graph_empty", &self.is_graph_empty)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let result = ComputationResult::new(vec![1, 2, 3], Duration::from_secs(1));
        assert_eq!(result.result(), &vec![1, 2, 3]);
        assert_eq!(result.compute_millis(), 1000);
        assert_eq!(result.preprocess_millis(), 0);
        assert!(!result.is_graph_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let result = ComputationResult::new(42, Duration::from_secs(1))
            .with_preprocess_time(Duration::from_millis(500))
            .with_config(json!({"maxIterations": 20}))
            .mark_graph_empty();

        assert_eq!(result.compute_millis(), 1000);
        assert_eq!(result.preprocess_millis(), 500);
        assert_eq!(result.total_millis(), 1500);
        assert!(result.is_graph_empty());
        assert_eq!(result.config(), &json!({"maxIterations": 20}));
    }

    #[test]
    fn test_into_result() {
        let result = ComputationResult::new(vec![1, 2, 3], Duration::from_secs(1));
        let inner = result.into_result();
        assert_eq!(inner, vec![1, 2, 3]);
    }

    #[test]
    fn test_result_mut() {
        let mut result = ComputationResult::new(vec![1, 2, 3], Duration::from_secs(1));
        result.result_mut().push(4);
        assert_eq!(result.result(), &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_map() {
        let result = ComputationResult::new(5, Duration::from_secs(1))
            .with_preprocess_time(Duration::from_millis(500));

        let mapped = result.map(|x| x * 2);

        assert_eq!(mapped.result(), &10);
        assert_eq!(mapped.compute_millis(), 1000);
        assert_eq!(mapped.preprocess_millis(), 500);
    }

    #[test]
    fn test_clone() {
        let result = ComputationResult::new(vec![1, 2, 3], Duration::from_secs(1))
            .with_preprocess_time(Duration::from_millis(500));

        let cloned = result.clone();

        assert_eq!(cloned.result(), result.result());
        assert_eq!(cloned.compute_millis(), result.compute_millis());
        assert_eq!(cloned.preprocess_millis(), result.preprocess_millis());
    }

    #[test]
    fn test_total_millis() {
        let result = ComputationResult::new((), Duration::from_millis(1200))
            .with_preprocess_time(Duration::from_millis(800));

        assert_eq!(result.total_millis(), 2000);
    }

    #[test]
    fn test_debug_format() {
        let result = ComputationResult::new(42, Duration::from_secs(1));
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("ComputationResult"));
        assert!(debug_str.contains("result"));
        assert!(debug_str.contains("1000")); // compute_millis
    }
}

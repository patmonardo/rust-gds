//! Centrality Algorithm Result Types
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.centrality.CentralityAlgorithmResult`
//!
//! This module provides result types and traits for centrality algorithms.

use super::super::stubs::NodePropertyValues;

/// Result trait for centrality algorithms
///
/// Translation of: `org.neo4j.gds.algorithms.centrality.CentralityAlgorithmResult`
///
/// ## Java GDS Source
///
/// ```java
/// public interface CentralityAlgorithmResult {
///     NodePropertyValues nodePropertyValues();
///     LongToDoubleFunction centralityScoreProvider();
/// }
/// ```
///
/// ## Usage
///
/// ```rust,ignore
/// use gds::procedures::algorithms::centrality::CentralityAlgorithmResult;
///
/// struct PageRankResult {
///     scores: Vec<f64>,
/// }
///
/// impl CentralityAlgorithmResult for PageRankResult {
///     fn node_property_values(&self) -> &dyn NodePropertyValues {
///         // Return property values accessor
///     }
///     
///     fn centrality_score_provider(&self) -> Box<dyn Fn(usize) -> f64> {
///         let scores = self.scores.clone();
///         Box::new(move |node_id| scores[node_id])
///     }
/// }
/// ```
pub trait CentralityAlgorithmResult {
    /// Get node property values accessor
    ///
    /// Translation of: `NodePropertyValues nodePropertyValues()`
    fn node_property_values(&self) -> &dyn NodePropertyValues;
    
    /// Get centrality score provider function
    ///
    /// Translation of: `LongToDoubleFunction centralityScoreProvider()`
    fn centrality_score_provider(&self) -> Box<dyn Fn(usize) -> f64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::algorithms::stubs::{LongNodePropertyValues, ValueType};

    // Test implementation
    struct TestCentralityResult {
        scores: Vec<f64>,
        node_count: usize,
    }

    impl NodePropertyValues for TestCentralityResult {
        fn node_count(&self) -> usize {
            self.node_count
        }

        fn value_type(&self) -> ValueType {
            ValueType::Double
        }
    }

    impl CentralityAlgorithmResult for TestCentralityResult {
        fn node_property_values(&self) -> &dyn NodePropertyValues {
            self
        }
        
        fn centrality_score_provider(&self) -> Box<dyn Fn(usize) -> f64> {
            let scores = self.scores.clone();
            Box::new(move |node_id| scores[node_id])
        }
    }

    #[test]
    fn test_centrality_result_trait() {
        let result = TestCentralityResult {
            scores: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            node_count: 5,
        };

        assert_eq!(result.node_property_values().node_count(), 5);
        assert_eq!(result.node_property_values().value_type(), ValueType::Double);
        
        let score_fn = result.centrality_score_provider();
        assert_eq!(score_fn(0), 0.1);
        assert_eq!(score_fn(2), 0.3);
        assert_eq!(score_fn(4), 0.5);
    }
}

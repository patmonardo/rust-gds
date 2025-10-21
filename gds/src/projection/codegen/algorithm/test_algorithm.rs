//! Test PageRank using the new define_algorithm! macro

use crate::define_algorithm;
use crate::projection::eval::procedure::*;
use crate::types::prelude::GraphStore;
use std::time::Duration;

// Define PageRank using the new macro
define_algorithm! {
    name: "pagerank",
    category: Centrality,
    
    config: {
        damping_factor: f64 = 0.85,
        tolerance: f64 = 1e-6,
        max_iterations: usize = 100,
        source_nodes: Option<Vec<u64>> = None,
        weight_property: Option<String> = None,
    },
    
    result: PageRankResult {
        scores: Vec<f64>,
        iterations: usize,
        converged: bool,
        execution_time: Duration,
    },
    
    projection_hint: Dense,
    modes: [Stream, Stats],
    
    execute: {
        // Simple placeholder implementation
        Ok(PageRankResult {
            scores: vec![0.1, 0.2, 0.3],
            iterations: 10,
            converged: true,
            execution_time: Duration::from_millis(100),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_config() {
        let config = PAGERANKConfig::default();
        assert_eq!(config.damping_factor, 0.85);
        assert_eq!(config.tolerance, 1e-6);
        assert_eq!(config.max_iterations, 100);
        assert!(config.source_nodes.is_none());
        assert!(config.weight_property.is_none());
    }

    #[test]
    fn test_pagerank_config_validation() {
        let mut config = PAGERANKConfig::default();
        config.damping_factor = 1.5; // Invalid!
        
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_pagerank_algorithm_spec() {
        let spec = PAGERANKAlgorithmSpec::new("test_graph".to_string(), PAGERANKConfig::default());
        assert_eq!(spec.name(), "pagerank");
        assert_eq!(spec.graph_name(), "test_graph");
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_pagerank_json_parsing() {
        let json = serde_json::json!({
            "damping_factor": 0.9,
            "tolerance": 1e-5,
            "max_iterations": 50,
            "source_nodes": [1, 2, 3],
            "weight_property": "weight"
        });
        
        let config: PAGERANKConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.damping_factor, 0.9);
        assert_eq!(config.tolerance, 1e-5);
        assert_eq!(config.max_iterations, 50);
        assert!(config.source_nodes.is_some());
        assert!(config.weight_property.is_some());
    }

    #[test]
    fn test_pagerank_result_serialization() {
        let result = PageRankResult {
            scores: vec![0.1, 0.2, 0.3],
            iterations: 10,
            converged: true,
            execution_time: Duration::from_millis(50),
        };
        
        let json = serde_json::to_value(&result).unwrap();
        let deserialized: PageRankResult = serde_json::from_value(json).unwrap();
        
        assert_eq!(result.scores, deserialized.scores);
        assert_eq!(result.iterations, deserialized.iterations);
        assert_eq!(result.converged, deserialized.converged);
    }
}

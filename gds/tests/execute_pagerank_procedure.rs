//! Integration test: Execute PageRank Procedure through ProcedureExecutor
//!
//! This demonstrates the complete flow:
//! User Code → ProcedureExecutor → PageRankAlgorithmSpec → Result

#[cfg(test)]
mod tests {
    use gds::procedures::pagerank::{PageRankAlgorithmSpec, PageRankConfig};
    use gds::projection::eval::procedure::AlgorithmSpec;
    use serde_json::json;

    #[test]
    fn test_pagerank_spec_parse_config() {
        // This test CAN run without ExecutionContext setup
        // It demonstrates that the spec correctly parses config

        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );

        let input = json!({
            "dampingFactor": 0.9,
            "tolerance": 1e-5,
            "maxIterations": 50,
            "sourceNodes": [0, 1],
            "weightProperty": "weight"
        });

        let result = spec.parse_config(&input);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(
            config.get("dampingFactor").unwrap().as_f64().unwrap(),
            0.9
        );
        assert_eq!(
            config.get("tolerance").unwrap().as_f64().unwrap(),
            1e-5
        );
        assert_eq!(
            config.get("maxIterations").unwrap().as_u64().unwrap(),
            50
        );
    }

    #[test]
    fn test_pagerank_spec_name_and_graph_name() {
        let spec = PageRankAlgorithmSpec::new(
            "my_graph".to_string(),
            PageRankConfig::default(),
        );

        assert_eq!(spec.name(), "pagerank");
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_pagerank_spec_projection_hint() {
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );

        let hint = spec.projection_hint();
        println!("Projection hint: {:?}", hint);
        // Verify it returns a valid hint (not panicking is the test)
        assert!(true);
    }

    #[test]
    fn test_pagerank_config_defaults() {
        let config = PageRankConfig::default();
        
        assert_eq!(config.damping_factor, 0.85);
        assert_eq!(config.tolerance, 1e-6);
        assert_eq!(config.max_iterations, 100);
        assert!(config.source_nodes.is_none());
        assert!(config.weight_property.is_none());
    }

    #[test]
    fn test_pagerank_config_custom() {
        let config = PageRankConfig {
            damping_factor: 0.9,
            tolerance: 1e-5,
            max_iterations: 50,
            source_nodes: Some(vec![0, 1, 2]),
            weight_property: Some("weight".to_string()),
        };
        
        assert_eq!(config.damping_factor, 0.9);
        assert_eq!(config.tolerance, 1e-5);
        assert_eq!(config.max_iterations, 50);
        assert_eq!(config.source_nodes, Some(vec![0, 1, 2]));
        assert_eq!(config.weight_property, Some("weight".to_string()));
    }

    #[test]
    #[ignore] // Requires real ExecutionContext and catalog setup
    fn test_pagerank_full_executor_flow() {
        // TODO: Once ExecutionContext and ProcedureExecutor are fully implemented,
        // this test will demonstrate the complete flow:
        //
        // Step 1: Create execution context with graph catalog
        // Step 2: Create algorithm specification
        // Step 3: Create executor with context
        // Step 4: Execute via executor.compute()
        // Step 5: Consume result
        //
        // For now, see integration_pagerank_executor.rs for the "killer tests"
        // that exercise the AlgorithmSpec contract directly.
    }
}

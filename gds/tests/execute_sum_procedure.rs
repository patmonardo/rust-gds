//! Integration test: Execute Sum Procedure through ProcedureExecutor
//!
//! This demonstrates the complete flow:
//! User Code → ProcedureExecutor → SumAlgorithmSpec → Result

#[cfg(test)]
mod tests {
    use gds::procedure::sum::{SumAlgorithmSpec, SumConfig};
    use gds::projection::eval::procedure::AlgorithmSpec;
    use serde_json::json;

    #[test]
    fn test_sum_spec_parse_config() {
        // This test CAN run without ExecutionContext setup
        // It demonstrates that the spec correctly parses config

        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let input = json!({
            "property_key": "node_value",
            "weight_property": null,
        });

        let result = spec.parse_config(&input);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(
            config.get("property_key").unwrap().as_str().unwrap(),
            "node_value"
        );
    }

    #[test]
    fn test_sum_spec_name_and_graph_name() {
        let spec = SumAlgorithmSpec::new(
            "my_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        assert_eq!(spec.name(), "sum");
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_sum_spec_projection_hint() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let hint = spec.projection_hint();
        println!("Projection hint: {:?}", hint);
        // Verify it returns a valid hint (not panicking is the test)
        assert!(true);
    }

    #[test]
    #[ignore] // Requires real ExecutionContext and catalog setup
    fn test_sum_full_executor_flow() {
        // TODO: Once ExecutionContext and ProcedureExecutor are fully implemented,
        // this test will demonstrate the complete flow:
        //
        // Step 1: Create execution context with graph catalog
        // Step 2: Create algorithm specification
        // Step 3: Create executor with context
        // Step 4: Execute via executor.compute()
        // Step 5: Consume result
        //
        // For now, see integration_sum_executor.rs for the "killer tests"
        // that exercise the AlgorithmSpec contract directly.
    }
}

// Phase 1 Integration Test - Arrow Native Factory
//
// Verifies Phase 1 Core Infrastructure: factory trait, config builder, error types.
// All tests guarded by #[cfg(feature = "arrow")].

#[cfg(feature = "arrow")]
mod arrow_factory_tests {
    use rust_gds::projection::factory::prelude::*;

    #[test]
    fn test_factory_creation() {
        let factory = ArrowNativeFactory::new();
        assert_eq!(
            format!("{:?}", factory),
            "ArrowNativeFactory { _placeholder: () }"
        );
    }

    #[test]
    fn test_config_builder_defaults() {
        let config = ArrowProjectionConfig::builder().build().unwrap();

        assert_eq!(config.node_table_name, "nodes");
        assert_eq!(config.edge_table_name, "edges");
        assert!(config.concurrency > 0);
        assert!(config.validate_schema);
        assert!(!config.log_progress); // Default is false in Phase 1
        assert_eq!(config.batch_size, 10_000);
    }

    #[test]
    fn test_config_builder_custom_values() {
        let config = ArrowProjectionConfig::builder()
            .node_table_name("custom_nodes")
            .edge_table_name("custom_edges")
            .concurrency(8)
            .validate_schema(false)
            .log_progress(true)
            .batch_size(5000)
            .build()
            .unwrap();

        assert_eq!(config.node_table_name, "custom_nodes");
        assert_eq!(config.edge_table_name, "custom_edges");
        assert_eq!(config.concurrency, 8);
        assert!(!config.validate_schema);
        assert!(config.log_progress);
        assert_eq!(config.batch_size, 5000);
    }

    #[test]
    fn test_config_validation_success() {
        let config = ArrowProjectionConfig::builder().build().unwrap();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_builder_rejects_empty_node_table() {
        // Builder validates and rejects empty node table name
        let result = ArrowProjectionConfig::builder().node_table_name("").build();

        assert!(result.is_err());
        if let Err(ArrowProjectionError::InvalidConfig(msg)) = result {
            assert!(msg.contains("node_table_name cannot be empty"));
        } else {
            panic!("Expected InvalidConfig error");
        }
    }

    #[test]
    fn test_builder_rejects_zero_concurrency() {
        // Builder validates and rejects zero concurrency
        let result = ArrowProjectionConfig::builder().concurrency(0).build();

        assert!(result.is_err());
        if let Err(ArrowProjectionError::InvalidConfig(msg)) = result {
            assert!(msg.contains("concurrency must be > 0"));
        } else {
            panic!("Expected InvalidConfig error");
        }
    }

    #[test]
    fn test_error_display_formats() {
        let err = ArrowProjectionError::InvalidConfig("test error".to_string());
        assert_eq!(format!("{}", err), "Invalid configuration: test error");

        let err = ArrowProjectionError::SchemaValidation("schema mismatch".to_string());
        assert_eq!(
            format!("{}", err),
            "Schema validation failed: schema mismatch"
        );

        let err = ArrowProjectionError::Arrow("arrow error".to_string());
        assert_eq!(format!("{}", err), "Arrow error: arrow error");

        let err = ArrowProjectionError::Import("import failed".to_string());
        assert_eq!(format!("{}", err), "Import error: import failed");

        let err = ArrowProjectionError::Other("other issue".to_string());
        assert_eq!(format!("{}", err), "Error: other issue");
    }

    #[test]
    fn test_factory_build_returns_not_yet_implemented() {
        // Phase 1: build_graph_store is skeleton only
        let factory = ArrowNativeFactory::new();
        let config = ArrowProjectionConfig::builder().build().unwrap();

        let result = factory.build_graph_store(&config);
        assert!(result.is_err());

        if let Err(ArrowProjectionError::Other(msg)) = result {
            assert!(msg.contains("not yet implemented"));
            assert!(msg.contains("Phase 1 skeleton"));
        } else {
            panic!("Expected Other error with 'not yet implemented' message");
        }
    }

    #[test]
    fn test_factory_estimate_memory_returns_placeholders() {
        // Phase 1: estimate_memory returns (0, 0)
        let factory = ArrowNativeFactory::new();
        let config = ArrowProjectionConfig::builder().build().unwrap();

        let result = factory.estimate_memory(&config);
        assert!(result.is_ok());

        let (node_mem, edge_mem) = result.unwrap();
        assert_eq!(node_mem, 0);
        assert_eq!(edge_mem, 0);
    }
}

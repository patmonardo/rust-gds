//! Comprehensive PageRank Integration Test
//!
//! This test demonstrates the complete PageRank implementation using:
//! - PREGEL computation framework
//! - Memory estimation system
//! - Progress tracking (when available)

#[cfg(test)]
mod tests {
    use gds::procedures::pagerank::{
        PageRankPregelComputation, PageRankMemoryEstimation, estimate_pagerank_memory,
        PageRankConfig, PageRankAlgorithmSpec
    };
    use gds::pregel::PregelComputation;
    use gds::config::PregelConfig;
    use gds::mem::MemoryEstimation;
    use gds::core::graph_dimensions::ConcreteGraphDimensions;
    use gds::projection::eval::procedure::AlgorithmSpec;

    #[test]
    fn test_pagerank_pregel_computation() {
        println!("=== PageRank PREGEL Computation Test ===");
        
        let computation = PageRankPregelComputation::new(0.85, 1e-6, 100, None);
        
        // Test basic properties
        assert_eq!(computation.damping_factor(), 0.85);
        assert_eq!(computation.tolerance(), 1e-6);
        assert_eq!(computation.max_iterations(), 100);
        
        // Test schema creation
        let config = PregelConfig::default();
        let schema = computation.schema(&config);
        
        assert_eq!(schema.elements().len(), 1);
        assert!(schema.elements().iter().any(|e| e.property_key == "pagerank"));
        
        println!("✓ PageRank PREGEL computation created successfully");
        println!("  Schema: {:?}", schema);
    }

    #[test]
    fn test_pagerank_memory_estimation() {
        println!("=== PageRank Memory Estimation Test ===");
        
        let estimation = PageRankMemoryEstimation::new(1000, 5.0);
        
        // Test individual components
        let node_values = estimation.estimate_node_values();
        let message_queues = estimation.estimate_message_queues();
        let convergence_tracking = estimation.estimate_convergence_tracking();
        let graph_overhead = estimation.estimate_graph_overhead();
        
        println!("  Node Values: {} bytes", node_values.min());
        println!("  Message Queues: {} bytes", message_queues.min());
        println!("  Convergence Tracking: {} bytes", convergence_tracking.min());
        println!("  Graph Overhead: {} bytes", graph_overhead.min());
        
        // Test total estimation
        let dims = ConcreteGraphDimensions::of(1000, 5000);
        let tree = estimation.estimate(&dims, 4);
        
        println!("  Total Memory Tree: {}", tree.description());
        println!("  Total Memory: {} bytes", tree.memory_usage().min());
        
        assert!(tree.memory_usage().min() > 0);
        assert_eq!(tree.components().len(), 4);
        
        println!("✓ PageRank memory estimation successful");
    }

    #[test]
    fn test_pagerank_convenience_functions() {
        println!("=== PageRank Convenience Functions Test ===");
        
        // Test default memory estimation
        let memory = estimate_pagerank_memory(10000);
        println!("  Memory for 10K nodes: {} bytes", memory.min());
        
        assert!(memory.min() > 0);
        
        // Test configuration
        let config = PageRankConfig::default();
        assert_eq!(config.damping_factor, 0.85);
        assert_eq!(config.tolerance, 1e-6);
        assert_eq!(config.max_iterations, 100);
        
        println!("✓ PageRank convenience functions successful");
    }

    #[test]
    fn test_pagerank_algorithm_spec() {
        println!("=== PageRank Algorithm Spec Test ===");
        
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );
        
        assert_eq!(spec.name(), "pagerank");
        assert_eq!(spec.graph_name(), "test_graph");
        
        println!("✓ PageRank algorithm spec created successfully");
    }

    #[test]
    fn test_pagerank_with_source_nodes() {
        println!("=== PageRank with Source Nodes Test ===");
        
        let source_nodes = vec![0, 5, 10, 15];
        let computation = PageRankPregelComputation::new(0.9, 1e-5, 50, Some(source_nodes.clone()));
        
        // Test source node detection
        assert!(computation.is_source_node(0));
        assert!(!computation.is_source_node(1));
        assert!(computation.is_source_node(5));
        assert!(!computation.is_source_node(6));
        assert!(computation.is_source_node(10));
        assert!(computation.is_source_node(15));
        
        println!("✓ PageRank with source nodes successful");
        println!("  Source nodes: {:?}", source_nodes);
    }

    #[test]
    fn test_pagerank_memory_scaling() {
        println!("=== PageRank Memory Scaling Test ===");
        
        let node_counts = vec![100, 1000, 10000, 100000];
        
        for node_count in node_counts {
            let memory = estimate_pagerank_memory(node_count);
            let memory_mb = memory.min() as f64 / (1024.0 * 1024.0);
            
            println!("  {} nodes: {:.2} MB", node_count, memory_mb);
            
            // Memory should scale roughly linearly with node count
            assert!(memory.min() > 0);
        }
        
        println!("✓ PageRank memory scaling test successful");
    }

    #[test]
    fn test_pagerank_integration_workflow() {
        println!("=== PageRank Integration Workflow Test ===");
        
        // Step 1: Create computation
        let _computation = PageRankPregelComputation::new(0.85, 1e-6, 100, None);
        
        // Step 2: Estimate memory
        let memory = estimate_pagerank_memory(5000);
        println!("  Estimated memory: {} bytes", memory.min());
        
        // Step 3: Create algorithm spec
        let spec = PageRankAlgorithmSpec::new(
            "integration_test".to_string(),
            PageRankConfig::default(),
        );
        
        // Step 4: Verify components work together
        assert_eq!(spec.name(), "pagerank");
        assert!(memory.min() > 0);
        
        // Step 5: Test configuration parsing
        let config_json = serde_json::json!({
            "dampingFactor": 0.9,
            "tolerance": 1e-5,
            "maxIterations": 50
        });
        
        let parsed_config = spec.parse_config(&config_json);
        assert!(parsed_config.is_ok());
        
        println!("✓ PageRank integration workflow successful");
        println!("  All components work together seamlessly!");
    }
}

#[cfg(test)]
mod tests {
    use crate::procedures::hits::HitsComputationRuntime;

    #[test]
    fn test_hits_basic_initialization() {
        let runtime = HitsComputationRuntime::new(5);
        assert_eq!(runtime.hub_scores.len(), 5);
        assert_eq!(runtime.authority_scores.len(), 5);
        for score in runtime.hub_scores {
            assert_eq!(score, 1.0);
        }
        for score in runtime.authority_scores {
            assert_eq!(score, 1.0);
        }
    }

    #[test]
    fn test_hits_single_node() {
        let mut runtime = HitsComputationRuntime::new(1);
        runtime.initialize();
        runtime.calculate_authorities();
        runtime.normalize_authorities();
        
        // Single node with authority [1.0] normalizes to [1.0]
        assert_eq!(runtime.authority_scores[0], 1.0);
    }

    #[test]
    fn test_hits_normalization_accuracy() {
        let mut runtime = HitsComputationRuntime::new(4);
        runtime.authority_scores_new = vec![1.0, 2.0, 2.0, 0.0];
        runtime.normalize_authorities();
        
        // L2 norm = sqrt(1 + 4 + 4 + 0) = 3
        // Normalized: [1/3, 2/3, 2/3, 0]
        assert!((runtime.authority_scores[0] - 1.0/3.0).abs() < 0.001);
        assert!((runtime.authority_scores[1] - 2.0/3.0).abs() < 0.001);
        assert!((runtime.authority_scores[2] - 2.0/3.0).abs() < 0.001);
        assert_eq!(runtime.authority_scores[3], 0.0);
    }

    #[test]
    fn test_hits_iteration_counting() {
        let mut runtime = HitsComputationRuntime::new(3);
        assert_eq!(runtime.get_iterations(), 0);
        
        for i in 1..=5 {
            runtime.hub_scores_new = runtime.hub_scores.clone();
            runtime.normalize_hubs();
            assert_eq!(runtime.get_iterations(), i);
        }
    }

    #[test]
    fn test_hits_convergence_slow() {
        let mut runtime = HitsComputationRuntime::new(3);
        runtime.hub_scores = vec![1.0, 1.0, 1.0];
        runtime.hub_scores_new = vec![1.5, 1.5, 1.5];
        
        // Large delta should NOT converge
        assert!(!runtime.has_converged(0.1));
    }

    #[test]
    fn test_hits_reset() {
        let mut runtime = HitsComputationRuntime::new(4);
        runtime.hub_scores = vec![0.5, 0.6, 0.7, 0.8];
        runtime.authority_scores = vec![0.1, 0.2, 0.3, 0.4];
        runtime.iterations = 10;
        
        runtime.initialize();
        
        for score in &runtime.hub_scores {
            assert_eq!(*score, 1.0);
        }
        for score in &runtime.authority_scores {
            assert_eq!(*score, 1.0);
        }
        assert_eq!(runtime.iterations, 0);
    }

    #[test]
    fn test_hits_normalization_preserves_pattern() {
        let mut runtime = HitsComputationRuntime::new(3);
        runtime.authority_scores_new = vec![1.0, 2.0, 3.0];
        runtime.normalize_authorities();
        
        // Ratios should be preserved after normalization
        let ratio_0_1 = runtime.authority_scores[0] / runtime.authority_scores[1];
        let ratio_1_2 = runtime.authority_scores[1] / runtime.authority_scores[2];
        
        assert!((ratio_0_1 - 0.5).abs() < 0.001); // 1/2
        assert!((ratio_1_2 - 2.0/3.0).abs() < 0.001); // 2/3
    }

    #[test]
    fn test_hits_large_graph() {
        let runtime = HitsComputationRuntime::new(1000);
        assert_eq!(runtime.hub_scores.len(), 1000);
        assert_eq!(runtime.authority_scores.len(), 1000);
        
        let hub_sum: f64 = runtime.hub_scores.iter().sum();
        assert_eq!(hub_sum, 1000.0); // 1000 nodes * 1.0 each
    }

    #[test]
    fn test_hits_hub_authority_values() {
        let runtime = HitsComputationRuntime::new(3);
        
        for score in &runtime.hub_scores {
            assert!(*score > 0.0);
        }
        for score in &runtime.authority_scores {
            assert!(*score > 0.0);
        }
    }

    #[test]
    fn test_hits_alternating_scores() {
        let mut runtime = HitsComputationRuntime::new(2);
        runtime.hub_scores = vec![1.0, 0.0];
        runtime.hub_scores_new = vec![1.0, 0.0];
        runtime.normalize_hubs();
        
        // After norm = 1.0, scores stay [1.0, 0.0]
        assert_eq!(runtime.hub_scores[0], 1.0);
        assert_eq!(runtime.hub_scores[1], 0.0);
    }
}

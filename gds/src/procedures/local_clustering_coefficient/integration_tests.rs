#[cfg(test)]
mod tests {
    use crate::procedures::local_clustering_coefficient::LocalClusteringCoefficientComputationRuntime;

    #[test]
    fn test_single_triangle_clustering() {
        // Simple triangle: 0-1-2-0
        // All nodes have degree 2, 1 triangle each
        // C(v) = 2*1 / (2*1) = 1.0 for all
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(3);
        runtime.compute(&[1, 1, 1], &[2, 2, 2]);

        assert_eq!(runtime.local_clustering_coefficients[0], 1.0);
        assert_eq!(runtime.local_clustering_coefficients[1], 1.0);
        assert_eq!(runtime.local_clustering_coefficients[2], 1.0);
        assert_eq!(runtime.average_clustering_coefficient, 1.0);
    }

    #[test]
    fn test_path_graph_no_triangles() {
        // Path: 0-1-2-3
        // No triangles anywhere
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[0, 0, 0, 0], &[1, 2, 2, 1]);

        for i in 0..4 {
            assert_eq!(runtime.local_clustering_coefficients[i], 0.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 0.0);
    }

    #[test]
    fn test_complete_graph_k4() {
        // Complete graph K4
        // Each node: degree 3, in 3 triangles (C(3,2) = 3)
        // C(v) = 2*3 / (3*2) = 1.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[3, 3, 3, 3], &[3, 3, 3, 3]);

        for i in 0..4 {
            assert_eq!(runtime.local_clustering_coefficients[i], 1.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 1.0);
    }

    #[test]
    fn test_star_graph_no_clustering() {
        // Star: center 0 connected to 1, 2, 3
        // Center has degree 3, 0 triangles (no edges between leaves)
        // Leaves have degree 1, 0 triangles
        // All coefficients should be 0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[0, 0, 0, 0], &[3, 1, 1, 1]);

        assert_eq!(runtime.local_clustering_coefficients[0], 0.0);
        for i in 1..4 {
            assert_eq!(runtime.local_clustering_coefficients[i], 0.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 0.0);
    }

    #[test]
    fn test_isolated_nodes() {
        // Isolated nodes: no edges
        // All degrees 0, all coefficients 0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(3);
        runtime.compute(&[0, 0, 0], &[0, 0, 0]);

        for i in 0..3 {
            assert_eq!(runtime.local_clustering_coefficients[i], 0.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 0.0);
    }

    #[test]
    fn test_mixed_clustering() {
        // Graph: two triangles sharing an edge
        // Triangle 1: 0-1-2, Triangle 2: 2-3-4
        // Node 0: degree 2, 1 triangle → C = 2*1/(2*1) = 1.0
        // Node 1: degree 2, 1 triangle → C = 1.0
        // Node 2: degree 3, 2 triangles → C = 2*2/(3*2) = 0.667
        // Node 3: degree 2, 1 triangle → C = 1.0
        // Node 4: degree 2, 1 triangle → C = 1.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(5);
        runtime.compute(&[1, 1, 2, 1, 1], &[2, 2, 3, 2, 2]);

        assert_eq!(runtime.local_clustering_coefficients[0], 1.0);
        assert_eq!(runtime.local_clustering_coefficients[1], 1.0);
        assert!((runtime.local_clustering_coefficients[2] - 2.0/3.0).abs() < 0.001);
        assert_eq!(runtime.local_clustering_coefficients[3], 1.0);
        assert_eq!(runtime.local_clustering_coefficients[4], 1.0);

        let expected_avg = (1.0 + 1.0 + 2.0/3.0 + 1.0 + 1.0) / 5.0;
        assert!((runtime.average_clustering_coefficient - expected_avg).abs() < 0.001);
    }

    #[test]
    fn test_single_edge() {
        // Just one edge: 0-1
        // Both nodes: degree 1, 0 triangles → C = 0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(2);
        runtime.compute(&[0, 0], &[1, 1]);

        assert_eq!(runtime.local_clustering_coefficients[0], 0.0);
        assert_eq!(runtime.local_clustering_coefficients[1], 0.0);
        assert_eq!(runtime.average_clustering_coefficient, 0.0);
    }

    #[test]
    fn test_square_graph_with_diagonals() {
        // Square: 0-1-3-2-0, plus diagonal 0-3 and 1-2
        // Forms two triangles: 0-1-3 and 1-2-3
        // Actually wait, let's be precise:
        // Edges: 0-1, 1-3, 3-2, 2-0, 0-3, 1-2
        // Node 0: degree 3 (neighbors 1,2,3), triangles with 1-3, 2-3 = 2 triangles
        //         C = 2*2/(3*2) = 0.667
        // Node 1: degree 3 (neighbors 0,2,3), triangles with 0-3, 2-3 = 2 triangles
        //         C = 0.667
        // etc.
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[2, 2, 2, 2], &[3, 3, 3, 3]);

        let expected = 2.0 / 3.0;
        for i in 0..4 {
            assert!((runtime.local_clustering_coefficients[i] - expected).abs() < 0.001);
        }
        assert!((runtime.average_clustering_coefficient - expected).abs() < 0.001);
    }

    #[test]
    fn test_mixed_degrees_with_triangles() {
        // Diamond: 0 at top, 1 left, 2 right, 3 at bottom
        // Edges: 0-1, 0-2, 1-3, 2-3, 1-2 (extra edge making triangles)
        // Node 0: degree 2, triangles 1 (with 1-2) → C = 2*1/(2*1) = 1.0
        // Node 1: degree 3, triangles 2 (with 0-2, 3-2) → C = 2*2/(3*2) = 0.667
        // Node 2: degree 3, triangles 2 (with 0-1, 3-1) → C = 0.667
        // Node 3: degree 2, triangles 1 (with 1-2) → C = 1.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(4);
        runtime.compute(&[1, 2, 2, 1], &[2, 3, 3, 2]);

        assert_eq!(runtime.local_clustering_coefficients[0], 1.0);
        assert!((runtime.local_clustering_coefficients[1] - 2.0/3.0).abs() < 0.001);
        assert!((runtime.local_clustering_coefficients[2] - 2.0/3.0).abs() < 0.001);
        assert_eq!(runtime.local_clustering_coefficients[3], 1.0);

        let expected_avg = (1.0 + 2.0/3.0 + 2.0/3.0 + 1.0) / 4.0;
        assert!((runtime.average_clustering_coefficient - expected_avg).abs() < 0.001);
    }

    #[test]
    fn test_high_clustering_coefficient() {
        // K5 (complete graph on 5 nodes)
        // Each node: degree 4, triangles = C(4,2) = 6
        // C(v) = 2*6/(4*3) = 12/12 = 1.0
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(5);
        runtime.compute(&[6, 6, 6, 6, 6], &[4, 4, 4, 4, 4]);

        for i in 0..5 {
            assert_eq!(runtime.local_clustering_coefficients[i], 1.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 1.0);
    }

    #[test]
    fn test_low_clustering_coefficient() {
        // Wheel graph: center connected to all, rim nodes connected in a cycle
        // Center: degree 4, 0 triangles (rim not connected to each other directly)
        //         C = 0.0
        // Rim nodes: degree 3 (2 rim neighbors + center), 1 triangle each
        //         C = 2*1/(3*2) = 0.333
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(5);
        // Center node 0, rim nodes 1,2,3,4
        runtime.compute(&[0, 1, 1, 1, 1], &[4, 3, 3, 3, 3]);

        assert_eq!(runtime.local_clustering_coefficients[0], 0.0); // center
        for i in 1..5 {
            assert!((runtime.local_clustering_coefficients[i] - 1.0/3.0).abs() < 0.001);
        }

        let expected_avg = (0.0 + 4.0 * 1.0/3.0) / 5.0;
        assert!((runtime.average_clustering_coefficient - expected_avg).abs() < 0.001);
    }

    #[test]
    fn test_degree_zero_nodes() {
        // Some isolated nodes
        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(5);
        runtime.compute(&[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0]);

        for i in 0..5 {
            assert_eq!(runtime.local_clustering_coefficients[i], 0.0);
        }
        assert_eq!(runtime.average_clustering_coefficient, 0.0);
    }
}

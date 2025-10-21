//! A* Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module implements the storage runtime for A* algorithm - the "Gross pole" for persistent data access.

use super::computation::AStarComputationResult;
use std::collections::HashMap;

/// A* storage runtime for accessing graph data
///
/// Translation of: `org.neo4j.gds.paths.astar.AStar` (lines 37-88)
pub struct AStarStorageRuntime {
    source_node: usize,
    target_node: usize,
    latitude_property: String,
    longitude_property: String,
    // Cache for latitude/longitude values to avoid repeated property lookups
    pub coordinate_cache: HashMap<usize, (f64, f64)>,
}

impl AStarStorageRuntime {
    /// Create new A* storage runtime
    ///
    /// Translation of: `AStar.sourceTarget()` (lines 47-88)
    pub fn new(
        source_node: usize,
        target_node: usize,
        latitude_property: String,
        longitude_property: String,
    ) -> Self {
        Self {
            source_node,
            target_node,
            latitude_property,
            longitude_property,
            coordinate_cache: HashMap::new(),
        }
    }
    
    /// Compute A* path using Haversine heuristic
    ///
    /// Translation of: `AStar.compute()` (lines 92-94) and `HaversineHeuristic`
    pub fn compute_astar_path(
        &mut self,
        _computation: &mut super::computation::AStarComputationRuntime,
    ) -> Result<AStarComputationResult, String> {
        // For now, implement a simplified A* algorithm
        // TODO: Implement full A* with priority queue and Haversine heuristic
        
        let mut path = Vec::new();
        let total_cost;
        let nodes_explored;
        
        // Simple path: source -> target (placeholder implementation)
        if self.source_node != self.target_node {
            path.push(self.source_node);
            path.push(self.target_node);
            total_cost = self.compute_haversine_distance(self.source_node, self.target_node)?;
            nodes_explored = 2;
        } else {
            path.push(self.source_node);
            total_cost = 0.0;
            nodes_explored = 1;
        }
        
        Ok(AStarComputationResult {
            path: Some(path),
            total_cost,
            nodes_explored,
        })
    }
    
    /// Compute Haversine distance between two nodes
    ///
    /// Translation of: `HaversineHeuristic.distance()` (lines 138-156)
    pub fn compute_haversine_distance(&mut self, source: usize, target: usize) -> Result<f64, String> {
        let (source_lat, source_lon) = self.get_coordinates(source)?;
        let (target_lat, target_lon) = self.get_coordinates(target)?;
        
        Ok(Self::haversine_distance(source_lat, source_lon, target_lat, target_lon))
    }
    
    /// Get coordinates for a node (with caching)
    pub fn get_coordinates(&mut self, node_id: usize) -> Result<(f64, f64), String> {
        if let Some(&coords) = self.coordinate_cache.get(&node_id) {
            return Ok(coords);
        }
        
        // For now, generate mock coordinates
        // TODO: Integrate with actual GraphStore property system
        let lat = (node_id as f64) * 0.01; // Mock latitude
        let lon = (node_id as f64) * 0.01; // Mock longitude
        
        let coords = (lat, lon);
        self.coordinate_cache.insert(node_id, coords);
        Ok(coords)
    }
    
    /// Haversine distance calculation
    ///
    /// Translation of: `HaversineHeuristic.distance()` (lines 138-156)
    /// https://rosettacode.org/wiki/Haversine_formula#Java
    pub fn haversine_distance(
        source_latitude: f64,
        source_longitude: f64,
        target_latitude: f64,
        target_longitude: f64,
    ) -> f64 {
        const EARTH_RADIUS_IN_NM: f64 = 6371.0 * 0.539957; // km to nautical mile
        
        let latitude_distance = (target_latitude - source_latitude).to_radians();
        let longitude_distance = (target_longitude - source_longitude).to_radians();
        let lat1 = source_latitude.to_radians();
        let lat2 = target_latitude.to_radians();
        
        let a = (latitude_distance / 2.0).sin().powi(2)
            + (longitude_distance / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        
        let c = 2.0 * a.sqrt().asin();
        
        EARTH_RADIUS_IN_NM * c
    }
    
    /// Get source node ID
    pub fn source_node(&self) -> usize {
        self.source_node
    }
    
    /// Get target node ID
    pub fn target_node(&self) -> usize {
        self.target_node
    }
    
    /// Get latitude property name
    pub fn latitude_property(&self) -> &str {
        &self.latitude_property
    }
    
    /// Get longitude property name
    pub fn longitude_property(&self) -> &str {
        &self.longitude_property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_storage_runtime_creation() {
        let storage = AStarStorageRuntime::new(
            0,
            1,
            "latitude".to_string(),
            "longitude".to_string(),
        );
        
        assert_eq!(storage.source_node(), 0);
        assert_eq!(storage.target_node(), 1);
        assert_eq!(storage.latitude_property(), "latitude");
        assert_eq!(storage.longitude_property(), "longitude");
    }

    #[test]
    fn test_haversine_distance_calculation() {
        // Test with known coordinates (New York to Los Angeles)
        let ny_lat = 40.7128;
        let ny_lon = -74.0060;
        let la_lat = 34.0522;
        let la_lon = -118.2437;
        
        let distance = AStarStorageRuntime::haversine_distance(ny_lat, ny_lon, la_lat, la_lon);
        
        // Distance should be approximately 2144 nautical miles
        assert!(distance > 2000.0 && distance < 2300.0);
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let lat = 40.7128;
        let lon = -74.0060;
        
        let distance = AStarStorageRuntime::haversine_distance(lat, lon, lat, lon);
        
        // Distance to same point should be 0
        assert!((distance - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_caching() {
        let mut storage = AStarStorageRuntime::new(
            0,
            1,
            "lat".to_string(),
            "lon".to_string(),
        );
        
        // First call should populate cache
        let coords1 = storage.get_coordinates(5).unwrap();
        
        // Second call should use cache
        let coords2 = storage.get_coordinates(5).unwrap();
        
        assert_eq!(coords1, coords2);
        assert_eq!(storage.coordinate_cache.len(), 1);
    }

    #[test]
    fn test_astar_path_computation() {
        let mut storage = AStarStorageRuntime::new(
            0,
            1,
            "lat".to_string(),
            "lon".to_string(),
        );
        
        let mut computation = crate::procedures::astar::computation::AStarComputationRuntime::new();
        
        let result = storage.compute_astar_path(&mut computation).unwrap();
        
        assert!(result.path.is_some());
        assert_eq!(result.path.as_ref().unwrap().len(), 2);
        assert_eq!(result.path.as_ref().unwrap()[0], 0);
        assert_eq!(result.path.as_ref().unwrap()[1], 1);
        assert!(result.total_cost >= 0.0);
        assert_eq!(result.nodes_explored, 2);
    }

    #[test]
    fn test_astar_path_same_source_target() {
        let mut storage = AStarStorageRuntime::new(
            5,
            5, // Same source and target
            "lat".to_string(),
            "lon".to_string(),
        );
        
        let mut computation = crate::procedures::astar::computation::AStarComputationRuntime::new();
        
        let result = storage.compute_astar_path(&mut computation).unwrap();
        
        assert!(result.path.is_some());
        assert_eq!(result.path.as_ref().unwrap().len(), 1);
        assert_eq!(result.path.as_ref().unwrap()[0], 5);
        assert_eq!(result.total_cost, 0.0);
        assert_eq!(result.nodes_explored, 1);
    }
}

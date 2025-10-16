//! Mock property values for testing ML pipelines.
//!
//! Provides simple implementations that generate test data without
//! requiring full graph integration. Used in Phase 2.2 to test
//! pipeline execution without implementing actual algorithms.

use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;

/// Mock double array property values for embeddings.
///
/// Generates random embeddings for testing feature extraction steps.
/// Phase 2.2: Simple random values
/// Phase 2.3+: Could be replaced with actual FastRP/Node2Vec implementations
#[derive(Debug, Clone)]
pub struct MockEmbeddingPropertyValues {
    node_count: usize,
    dimension: usize,
    seed: u64,
}

impl MockEmbeddingPropertyValues {
    pub fn new(node_count: usize, dimension: usize) -> Self {
        Self {
            node_count,
            dimension,
            seed: 42,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Generate deterministic "random" embedding for a node
    fn generate_embedding(&self, node_id: u64) -> Vec<f64> {
        // Simple deterministic pseudo-random generation
        // Uses node_id and seed to generate reproducible values
        let mut values = Vec::with_capacity(self.dimension);
        for i in 0..self.dimension {
            // Simple hash-based generation (not cryptographic, just deterministic)
            // Multiply dimension index and node_id by different large primes to ensure variation
            let hash = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(node_id.wrapping_mul(1099511628211))
                .wrapping_add((i as u64).wrapping_mul(16777619));
            let normalized = (hash as f64) / (u64::MAX as f64);
            // Map to [-1.0, 1.0] range (common for embeddings)
            values.push(normalized * 2.0 - 1.0);
        }
        values
    }
}

impl PropertyValues for MockEmbeddingPropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for MockEmbeddingPropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Double,
        ))
    }

    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Long,
        ))
    }

    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        if node_id as usize >= self.node_count {
            return Err(PropertyValuesError::InvalidNodeId(node_id));
        }
        Ok(self.generate_embedding(node_id))
    }

    fn float_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Ok(self
            .double_array_value(node_id)?
            .iter()
            .map(|&v| v as f32)
            .collect())
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::LongArray,
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.double_array_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(self.dimension)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        Some(1.0) // Embeddings are in [-1, 1]
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.node_count
    }
}

/// Mock double property values for scalar floating-point properties.
///
/// Generates deterministic values for testing feature transformations.
#[derive(Debug, Clone)]
pub struct MockDoublePropertyValues {
    values: Vec<f64>,
}

impl MockDoublePropertyValues {
    pub fn new(node_count: usize) -> Self {
        // Generate deterministic values
        let values = (0..node_count)
            .map(|i| (i as f64) / (node_count as f64))
            .collect();
        Self { values }
    }

    pub fn from_vec(values: Vec<f64>) -> Self {
        Self { values }
    }
}

impl PropertyValues for MockDoublePropertyValues {
    fn element_count(&self) -> usize {
        self.values.len()
    }

    fn value_type(&self) -> ValueType {
        ValueType::Double
    }
}

impl NodePropertyValues for MockDoublePropertyValues {
    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::UnsupportedType {
            expected: ValueType::Long,
            actual: ValueType::Double,
        })
    }

    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        self.values
            .get(node_id as usize)
            .copied()
            .ok_or(PropertyValuesError::InvalidNodeId(node_id))
    }

    fn float_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::UnsupportedType {
            expected: ValueType::FloatArray,
            actual: ValueType::Double,
        })
    }

    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::UnsupportedType {
            expected: ValueType::DoubleArray,
            actual: ValueType::Double,
        })
    }

    fn long_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::UnsupportedType {
            expected: ValueType::LongArray,
            actual: ValueType::Double,
        })
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.double_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        self.values
            .iter()
            .copied()
            .fold(None, |max, val| Some(max.map_or(val, |m: f64| m.max(val))))
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.values.len()
    }
}

/// Mock long property values for simple scalar properties.
///
/// Generates deterministic values for testing node property steps.
#[derive(Debug, Clone)]
pub struct MockLongPropertyValues {
    node_count: usize,
    seed: u64,
}

impl MockLongPropertyValues {
    pub fn new(node_count: usize) -> Self {
        Self {
            node_count,
            seed: 42,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Generate deterministic value for a node
    fn generate_value(&self, node_id: u64) -> i64 {
        // Simple hash-based generation
        let hash = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(node_id);
        (hash % 100) as i64 // Values in [0, 100)
    }
}

impl PropertyValues for MockLongPropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for MockLongPropertyValues {
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        Ok(self.long_value(node_id)? as f64)
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        if node_id as usize >= self.node_count {
            return Err(PropertyValuesError::InvalidNodeId(node_id));
        }
        Ok(self.generate_value(node_id))
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::DoubleArray,
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::FloatArray,
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::LongArray,
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.long_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        Some(99)
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        Some(99.0)
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.node_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_embedding_values() {
        let values = MockEmbeddingPropertyValues::new(10, 128);

        // Check dimension
        assert_eq!(values.dimension(), Some(128));
        assert_eq!(values.element_count(), 10);

        // Get embedding for node 0
        let embedding = values.double_array_value(0).unwrap();
        assert_eq!(embedding.len(), 128);

        // Values should be in [-1, 1] range
        for &val in &embedding {
            assert!(val >= -1.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_mock_embedding_deterministic() {
        let values1 = MockEmbeddingPropertyValues::new(10, 64);
        let values2 = MockEmbeddingPropertyValues::new(10, 64);

        // Same seed should produce same values
        let emb1 = values1.double_array_value(5).unwrap();
        let emb2 = values2.double_array_value(5).unwrap();

        assert_eq!(emb1, emb2);
    }

    #[test]
    fn test_mock_embedding_different_nodes() {
        let values = MockEmbeddingPropertyValues::new(10, 64);

        let emb0 = values.double_array_value(0).unwrap();
        let emb1 = values.double_array_value(1).unwrap();

        // Different nodes should have different embeddings
        assert_ne!(emb0, emb1);
    }

    #[test]
    fn test_mock_long_values() {
        let values = MockLongPropertyValues::new(100);

        // Check basic properties
        assert_eq!(values.element_count(), 100);
        assert_eq!(values.dimension(), Some(1));

        // Get value for node 0
        let val = values.long_value(0).unwrap();
        assert!(val >= 0 && val < 100);
    }

    #[test]
    fn test_mock_long_deterministic() {
        let values1 = MockLongPropertyValues::new(100);
        let values2 = MockLongPropertyValues::new(100);

        // Same seed should produce same values
        for i in 0..10 {
            assert_eq!(
                values1.long_value(i).unwrap(),
                values2.long_value(i).unwrap()
            );
        }
    }

    #[test]
    fn test_mock_long_out_of_bounds() {
        let values = MockLongPropertyValues::new(10);

        let result = values.long_value(100);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(PropertyValuesError::InvalidNodeId(100))
        ));
    }
}

/// Feature assembler for orchestrating feature transformations.
use crate::projection::codegen::ml::step_descriptor::FeatureStepDescriptor;
use crate::projection::codegen::ComputeError;
use crate::types::properties::PropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

use super::transformation::{IdentityTransformation, Transformation};
use super::validation::{property_dimension, validate_features};

/// Assemble features from properties.
///
/// This trait orchestrates the feature assembly process:
/// 1. Lookup source properties
/// 2. Apply transformation
/// 3. Validate result
/// 4. Return feature values
///
/// Mirrors Java GDS feature assembly logic.
pub trait FeatureAssembler: Send + Sync {
    /// Assemble feature from source properties.
    ///
    /// # Arguments
    ///
    /// * `properties` - Available properties (from node property steps)
    /// * `step` - Feature step descriptor (defines transformation)
    ///
    /// # Returns
    ///
    /// Transformed and validated feature values
    ///
    /// # Errors
    ///
    /// - `ComputeError::InitFailed` if source property not found
    /// - `ComputeError::ExecutionFailed` if transformation fails
    /// - `ComputeError::InvalidFeature` if validation fails
    fn assemble(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
        step: &FeatureStepDescriptor,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError>;

    /// Get expected feature dimension for a step.
    ///
    /// Used for pre-allocation and validation.
    fn feature_dimension(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
        step: &FeatureStepDescriptor,
    ) -> Result<usize, ComputeError>;
}

/// Default feature assembler implementation.
///
/// Phase 2.3: Uses identity transformation (copy property → feature).
/// Phase 2.5: Will support normalize, combine, etc.
///
/// # Examples
///
/// ```ignore
/// use rust_gds::projection::native::form::features::DefaultFeatureAssembler;
///
/// let assembler = DefaultFeatureAssembler::new();
/// let feature = assembler.assemble(&properties, &step_descriptor)?;
/// ```
pub struct DefaultFeatureAssembler {
    identity: IdentityTransformation,
}

impl DefaultFeatureAssembler {
    /// Create a new default feature assembler.
    pub fn new() -> Self {
        Self {
            identity: IdentityTransformation,
        }
    }

    /// Select transformation based on step descriptor.
    ///
    /// Phase 2.3: Always returns identity transformation.
    /// Phase 2.5: Will parse step config and return appropriate transformation.
    fn select_transformation(&self, _step: &FeatureStepDescriptor) -> &dyn Transformation {
        // Phase 2.3: Always identity
        &self.identity as &dyn Transformation

        // Phase 2.5: Parse step config
        // match step.transformation_type {
        //     "identity" => &self.identity,
        //     "normalize" => &self.normalize,
        //     "combine" => &self.combine,
        //     _ => &self.identity,
        // }
    }
}

impl Default for DefaultFeatureAssembler {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureAssembler for DefaultFeatureAssembler {
    fn assemble(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
        step: &FeatureStepDescriptor,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Get first source property (Phase 2.3: single property only)
        let source_property = step.source_properties.first().ok_or_else(|| {
            ComputeError::InitFailed(format!("feature '{}' has no source properties", step.name))
        })?;

        // Lookup property in state
        let property_values = properties.get(source_property).ok_or_else(|| {
            ComputeError::InitFailed(format!(
                "source property '{}' not found for feature '{}'",
                source_property, step.name
            ))
        })?;

        // Select and apply transformation
        let transformation = self.select_transformation(step);
        let feature_values = transformation.transform(property_values)?;

        // Validate features (check for NaN, Inf)
        validate_features(&feature_values)?;

        Ok(feature_values)
    }

    fn feature_dimension(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
        step: &FeatureStepDescriptor,
    ) -> Result<usize, ComputeError> {
        // Get first source property
        let source_property = step.source_properties.first().ok_or_else(|| {
            ComputeError::InitFailed(format!("feature '{}' has no source properties", step.name))
        })?;

        // Lookup property
        let property_values = properties.get(source_property).ok_or_else(|| {
            ComputeError::InitFailed(format!("source property '{}' not found", source_property))
        })?;

        // Get dimension from property
        Ok(property_dimension(property_values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::codegen::ml::step_descriptor::FeatureType;
    use crate::projection::native::form::mock_property_values::{
        MockDoublePropertyValues, MockEmbeddingPropertyValues, MockLongPropertyValues,
    };

    #[test]
    fn test_assembler_creation() {
        let assembler = DefaultFeatureAssembler::new();
        assert_eq!(assembler.identity.name(), "identity");
    }

    #[test]
    fn test_assemble_scalar_property() {
        let assembler = DefaultFeatureAssembler::new();

        // Create properties map
        let mut properties: HashMap<String, Arc<dyn PropertyValues>> = HashMap::new();
        properties.insert(
            "pagerank".to_string(),
            Arc::new(MockDoublePropertyValues::new(10)),
        );

        // Create feature step
        let step = FeatureStepDescriptor {
            name: "pr_feature".to_string(),
            feature_type: FeatureType::Scalar,
            source_properties: vec!["pagerank".to_string()],
            target_dimension: None,
        };

        // Assemble feature
        let result = assembler.assemble(&properties, &step);
        assert!(result.is_ok());

        let feature = result.unwrap();
        assert_eq!(feature.element_count(), 10);
    }

    #[test]
    fn test_assemble_embedding_property() {
        let assembler = DefaultFeatureAssembler::new();

        // Create properties map
        let mut properties: HashMap<String, Arc<dyn PropertyValues>> = HashMap::new();
        properties.insert(
            "embedding".to_string(),
            Arc::new(MockEmbeddingPropertyValues::new(10, 128)),
        );

        // Create feature step
        let step = FeatureStepDescriptor {
            name: "emb_feature".to_string(),
            feature_type: FeatureType::Embedding,
            source_properties: vec!["embedding".to_string()],
            target_dimension: None,
        };

        // Assemble feature
        let result = assembler.assemble(&properties, &step);
        assert!(result.is_ok());

        let feature = result.unwrap();
        assert_eq!(feature.element_count(), 10);
    }

    #[test]
    fn test_assemble_missing_property() {
        let assembler = DefaultFeatureAssembler::new();

        // Empty properties map
        let properties: HashMap<String, Arc<dyn PropertyValues>> = HashMap::new();

        // Create feature step
        let step = FeatureStepDescriptor {
            name: "pr_feature".to_string(),
            feature_type: FeatureType::Scalar,
            source_properties: vec!["pagerank".to_string()],
            target_dimension: None,
        };

        // Assemble feature - should fail
        let result = assembler.assemble(&properties, &step);
        assert!(result.is_err());
        assert!(matches!(result, Err(ComputeError::InitFailed(_))));
    }

    #[test]
    fn test_assemble_no_source_properties() {
        let assembler = DefaultFeatureAssembler::new();

        let properties: HashMap<String, Arc<dyn PropertyValues>> = HashMap::new();

        // Create feature step with no source properties
        let step = FeatureStepDescriptor {
            name: "empty_feature".to_string(),
            feature_type: FeatureType::Scalar,
            source_properties: vec![],
            target_dimension: None,
        };

        // Assemble feature - should fail
        let result = assembler.assemble(&properties, &step);
        assert!(result.is_err());
    }

    #[test]
    fn test_feature_dimension_scalar() {
        let assembler = DefaultFeatureAssembler::new();

        let mut properties: HashMap<String, Arc<dyn PropertyValues>> = HashMap::new();
        properties.insert(
            "degree".to_string(),
            Arc::new(MockLongPropertyValues::new(10)),
        );

        let step = FeatureStepDescriptor {
            name: "deg_feature".to_string(),
            feature_type: FeatureType::Scalar,
            source_properties: vec!["degree".to_string()],
            target_dimension: None,
        };

        let dim = assembler.feature_dimension(&properties, &step);
        assert!(dim.is_ok());
        assert_eq!(dim.unwrap(), 1);
    }

    #[test]
    fn test_feature_dimension_embedding() {
        let assembler = DefaultFeatureAssembler::new();

        let mut properties: HashMap<String, Arc<dyn PropertyValues>> = HashMap::new();
        properties.insert(
            "embedding".to_string(),
            Arc::new(MockEmbeddingPropertyValues::new(10, 64)),
        );

        let step = FeatureStepDescriptor {
            name: "emb_feature".to_string(),
            feature_type: FeatureType::Embedding,
            source_properties: vec!["embedding".to_string()],
            target_dimension: None,
        };

        let dim = assembler.feature_dimension(&properties, &step);
        assert!(dim.is_ok());
        // TODO Phase 2.5: Should return 64 (embedding dimension)
        // For Phase 2.3: Returns 1 (simplified)
        assert_eq!(dim.unwrap(), 1);
    }
}

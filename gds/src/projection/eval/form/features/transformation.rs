/// Feature transformation traits and implementations.
///
/// Provides polymorphic transformation dispatch for feature engineering.
use crate::projection::codegen::ComputeError;
use crate::projection::eval::form::mock_property_values::MockDoublePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::collections::HashMap;
use std::sync::Arc;

/// Transform property values into feature values.
///
/// This trait enables polymorphic feature transformations at runtime.
/// Implementations can normalize, encode, combine, or project features.
pub trait Transformation: Send + Sync {
    /// Transform property values to feature values.
    ///
    /// # Arguments
    ///
    /// * `values` - Source property values
    ///
    /// # Returns
    ///
    /// Transformed feature values (typically as Arc<dyn PropertyValues>)
    fn transform(
        &self,
        values: &Arc<dyn PropertyValues>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError>;

    /// Get transformation name (for debugging/logging).
    fn name(&self) -> &str;
}

/// Identity transformation (no-op, just clone Arc pointer).
///
/// This is the Phase 2.3 default: copy property → feature without transformation.
///
/// # Examples
///
/// ```ignore
/// let transform = IdentityTransformation;
/// let feature = transform.transform(&property)?;
/// // feature is same as property (Arc clone)
/// ```
pub struct IdentityTransformation;

impl Transformation for IdentityTransformation {
    fn transform(
        &self,
        values: &Arc<dyn PropertyValues>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Just clone Arc pointer (cheap)
        Ok(values.clone())
    }

    fn name(&self) -> &str {
        "identity"
    }
}

/// Normalization strategies for feature scaling.
#[derive(Debug, Clone)]
pub enum NormalizationStrategy {
    /// Min-max normalization: scale to [0, 1].
    ///
    /// Formula: `(x - min) / (max - min)`
    MinMax { min: f64, max: f64 },

    /// Z-score normalization: standardize to mean=0, stddev=1.
    ///
    /// Formula: `(x - mean) / stddev`
    ZScore { mean: f64, stddev: f64 },
}

/// Normalize transformation (scale features).
///
/// Supports min-max and z-score normalization strategies.
///
/// # Examples
///
/// ```ignore
/// // Min-max normalization to [0, 1]
/// let transform = NormalizeTransformation::min_max(0.0, 100.0);
/// let feature = transform.transform(&property)?;
///
/// // Z-score normalization
/// let transform = NormalizeTransformation::z_score(50.0, 10.0);
/// let feature = transform.transform(&property)?;
/// ```
pub struct NormalizeTransformation {
    strategy: NormalizationStrategy,
}

impl NormalizeTransformation {
    /// Create min-max normalization (scale to [0, 1]).
    pub fn min_max(min: f64, max: f64) -> Self {
        Self {
            strategy: NormalizationStrategy::MinMax { min, max },
        }
    }

    /// Create z-score normalization (standardize to mean=0, stddev=1).
    pub fn z_score(mean: f64, stddev: f64) -> Self {
        Self {
            strategy: NormalizationStrategy::ZScore { mean, stddev },
        }
    }

    /// Create normalization with automatic statistics computation.
    ///
    /// Computes min/max or mean/stddev from property values.
    ///
    /// TODO Phase 2.5: Implement with proper NodePropertyValues support
    #[allow(dead_code)]
    pub fn auto(_values: &Arc<dyn PropertyValues>, strategy: &str) -> Result<Self, ComputeError> {
        // Placeholder - requires NodePropertyValues trait support
        match strategy {
            "minmax" => Ok(Self::min_max(0.0, 1.0)),
            "zscore" => Ok(Self::z_score(0.0, 1.0)),
            _ => Err(ComputeError::InitFailed(format!(
                "unknown normalization strategy: {}",
                strategy
            ))),
        }
    }
}

impl Transformation for NormalizeTransformation {
    fn transform(
        &self,
        values: &Arc<dyn PropertyValues>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // TODO Phase 2.5: Implement actual normalization
        // For Phase 2.3: Just return identity (clone)
        Ok(values.clone())
    }

    fn name(&self) -> &str {
        match self.strategy {
            NormalizationStrategy::MinMax { .. } => "normalize_minmax",
            NormalizationStrategy::ZScore { .. } => "normalize_zscore",
        }
    }
}

/// Combine transformation (concatenate multiple properties).
///
/// Creates a single feature vector by concatenating multiple source properties.
///
/// # Examples
///
/// ```ignore
/// let transform = CombineTransformation::new(vec!["pagerank", "degree"]);
/// let feature = transform.combine(&properties)?;
/// // feature contains concatenated values: [pr_0, deg_0, pr_1, deg_1, ...]
/// ```
pub struct CombineTransformation {
    source_properties: Vec<String>,
}

impl CombineTransformation {
    /// Create combine transformation for multiple properties.
    pub fn new(source_properties: Vec<String>) -> Self {
        Self { source_properties }
    }

    /// Combine multiple properties into single feature vector.
    pub fn combine(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Get all source properties
        let sources: Result<Vec<_>, _> = self
            .source_properties
            .iter()
            .map(|name| {
                properties.get(name).ok_or_else(|| {
                    ComputeError::InitFailed(format!("source property '{}' not found", name))
                })
            })
            .collect();
        let sources = sources?;

        if sources.is_empty() {
            return Err(ComputeError::InitFailed(
                "no source properties to combine".into(),
            ));
        }

        // For Phase 2.3, just return first property (identity)
        // Phase 2.5 will implement actual concatenation
        Ok(sources[0].clone())
    }
}

impl Transformation for CombineTransformation {
    fn transform(
        &self,
        values: &Arc<dyn PropertyValues>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Single property transform - just identity
        Ok(values.clone())
    }

    fn name(&self) -> &str {
        "combine"
    }
}

/// Compute min and max values from property.
fn compute_min_max<T: NodePropertyValues + ?Sized>(
    values: &Arc<T>,
) -> Result<(f64, f64), ComputeError> {
    let node_count = values.element_count();
    if node_count == 0 {
        return Ok((0.0, 1.0));
    }

    match values.value_type() {
        ValueType::Double => {
            let mut min = f64::MAX;
            let mut max = f64::MIN;

            for node_id in 0..node_count as u64 {
                let value = values.double_value(node_id)?;
                if value < min {
                    min = value;
                }
                if value > max {
                    max = value;
                }
            }

            Ok((min, max))
        }
        ValueType::Long => {
            let mut min = i64::MAX;
            let mut max = i64::MIN;

            for node_id in 0..node_count as u64 {
                let value = values.long_value(node_id)?;
                if value < min {
                    min = value;
                }
                if value > max {
                    max = value;
                }
            }

            Ok((min as f64, max as f64))
        }
        _ => Err(ComputeError::InitFailed(
            "min/max only supported for scalar types".into(),
        )),
    }
}

/// Compute mean and standard deviation from property.
fn compute_mean_stddev<T: NodePropertyValues + ?Sized>(
    values: &Arc<T>,
) -> Result<(f64, f64), ComputeError> {
    let node_count = values.element_count();
    if node_count == 0 {
        return Ok((0.0, 1.0));
    }

    match values.value_type() {
        ValueType::Double => {
            // Compute mean
            let mut sum = 0.0;
            for node_id in 0..node_count as u64 {
                sum += values.double_value(node_id)?;
            }
            let mean = sum / node_count as f64;

            // Compute variance
            let mut variance_sum = 0.0;
            for node_id in 0..node_count as u64 {
                let diff = values.double_value(node_id)? - mean;
                variance_sum += diff * diff;
            }
            let variance: f64 = variance_sum / node_count as f64;
            let stddev = variance.sqrt();

            Ok((mean, stddev))
        }
        ValueType::Long => {
            // Compute mean
            let mut sum = 0i64;
            for node_id in 0..node_count as u64 {
                sum += values.long_value(node_id)?;
            }
            let mean = sum as f64 / node_count as f64;

            // Compute variance
            let mut variance_sum = 0.0;
            for node_id in 0..node_count as u64 {
                let diff = values.long_value(node_id)? as f64 - mean;
                variance_sum += diff * diff;
            }
            let variance: f64 = variance_sum / node_count as f64;
            let stddev = variance.sqrt();

            Ok((mean, stddev))
        }
        _ => Err(ComputeError::InitFailed(
            "mean/stddev only supported for scalar types".into(),
        )),
    }
}

/// Compute mean and standard deviation from property.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::form::mock_property_values::MockLongPropertyValues;
    use crate::types::properties::node::NodePropertyValues;

    #[test]
    fn test_identity_transformation() {
        let property: Arc<dyn PropertyValues> = Arc::new(MockLongPropertyValues::new(10));
        let transform = IdentityTransformation;

        let result = transform.transform(&property);
        assert!(result.is_ok());

        let feature = result.unwrap();
        assert_eq!(feature.element_count(), 10);
    }

    #[test]
    fn test_normalize_min_max() {
        let values = vec![0.0, 50.0, 100.0];
        let property: Arc<dyn PropertyValues> =
            Arc::new(MockDoublePropertyValues::from_vec(values));

        let transform = NormalizeTransformation::min_max(0.0, 100.0);
        let result = transform.transform(&property);
        assert!(result.is_ok());

        let feature = result.unwrap();
        // Verify we got back a PropertyValues with correct count
        assert_eq!(feature.element_count(), 3);
        assert_eq!(feature.value_type(), ValueType::Double);
    }

    #[test]
    fn test_normalize_z_score() {
        let values = vec![40.0, 50.0, 60.0]; // mean=50, stddev≈8.16
        let property = Arc::new(MockDoublePropertyValues::from_vec(values));

        let (mean, stddev) = compute_mean_stddev(&property).unwrap();
        assert!((mean - 50.0).abs() < 0.01);

        let transform = NormalizeTransformation::z_score(mean, stddev);
        // Need to convert to Arc<dyn PropertyValues> for transform
        let property_trait: Arc<dyn PropertyValues> = property.clone();
        let result = transform.transform(&property_trait);
        assert!(result.is_ok());

        let feature = result.unwrap();
        // Verify we got back a PropertyValues with correct count and type
        assert_eq!(feature.element_count(), 3);
        assert_eq!(feature.value_type(), ValueType::Double);
    }

    #[test]
    fn test_compute_min_max() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let property = Arc::new(MockDoublePropertyValues::from_vec(values));

        let (min, max) = compute_min_max(&property).unwrap();
        assert_eq!(min, 10.0);
        assert_eq!(max, 50.0);
    }

    #[test]
    fn test_compute_mean_stddev() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0]; // mean=30
        let property = Arc::new(MockDoublePropertyValues::from_vec(values));

        let (mean, stddev) = compute_mean_stddev(&property).unwrap();
        assert_eq!(mean, 30.0);
        assert!(stddev > 0.0);
    }

    #[test]
    fn test_combine_transformation_name() {
        let transform = CombineTransformation::new(vec!["pr".into(), "degree".into()]);
        assert_eq!(transform.name(), "combine");
    }
}

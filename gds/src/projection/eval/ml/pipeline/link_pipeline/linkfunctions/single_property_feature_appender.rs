// Phase 2.7: SinglePropertyFeatureAppender - Base class for single-property appenders

use super::super::LinkFeatureAppender;
use std::marker::PhantomData;

/// Base implementation for link feature appenders that operate on a single property.
///
/// # The Atomic Appender
///
/// This is the **simplest unit** - an appender that works with one property.
/// It holds:
/// - The property values (NodePropertyValues)
/// - The dimension (how many features it produces)
///
/// Subclasses override `append_features()` to implement type-specific logic
/// (e.g., L2DoubleArrayFeatureAppender, HadamardLongFeatureAppender, etc.).
///
/// # The Four-Fold (Atomic Level)
///
/// - **Pure Container**: SinglePropertyFeatureAppender struct
/// - **Given Container**: Type-specific subclasses (L2DoubleArray, HadamardLong, etc.)
/// - **Given Contained**: Property extraction logic (empirical)
/// - **Pure Contained**: Computation algorithm (deferred to subclass)
///
/// # Genetic Unity (Atom)
///
/// This is the **singular predicate** - one property, one computation.
/// "Socrates is Musical" - single attribute, single appender.
///
/// The **Union** comes later (UnionLinkFeatureAppender) where we synthesize
/// multiple SinglePropertyFeatureAppenders: "Socrates is Musical, Sitting".
///
/// # Usage
///
/// Created by AbstractLinkFeatureAppenderFactory type dispatch:
/// ```text
/// Factory sees DoubleArray property → creates L2DoubleArrayFeatureAppender
///   → which extends SinglePropertyFeatureAppender
///   → which holds NodePropertyValues + dimension
/// ```
#[derive(Debug)]
pub struct SinglePropertyFeatureAppender {
    /// The node property values this appender operates on.
    /// TODO: Replace PhantomData with actual NodePropertyValues type
    props: PhantomData<()>,

    /// The feature dimension this appender produces.
    /// - Scalar properties: dimension = 1
    /// - Array properties: dimension = array length
    dimension: usize,
}

impl SinglePropertyFeatureAppender {
    /// Creates a new SinglePropertyFeatureAppender.
    ///
    /// # Arguments
    ///
    /// * `props` - Node property values
    /// * `dimension` - Feature dimension
    pub fn new(props: PhantomData<()>, dimension: usize) -> Self {
        Self { props, dimension }
    }

    /// Returns the property values.
    pub fn props(&self) -> &PhantomData<()> {
        &self.props
    }
}

impl LinkFeatureAppender for SinglePropertyFeatureAppender {
    fn append_features(&self, _source: u64, _target: u64, _features: &mut [f64], _offset: usize) {
        // Abstract - subclasses must override
        // Type-specific implementations:
        // - L2DoubleArray: features[offset..] = (source[i] - target[i])²
        // - HadamardDouble: features[offset] = source * target
        // - CosineFloatArray: features[offset] = dot(src,tgt) / (||src|| * ||tgt||)
        panic!("SinglePropertyFeatureAppender::append_features must be overridden by subclass")
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_property_creation() {
        let appender = SinglePropertyFeatureAppender::new(PhantomData, 10);
        assert_eq!(appender.dimension(), 10);
    }

    #[test]
    fn test_single_property_dimension() {
        let scalar_appender = SinglePropertyFeatureAppender::new(PhantomData, 1);
        let array_appender = SinglePropertyFeatureAppender::new(PhantomData, 128);

        assert_eq!(scalar_appender.dimension(), 1);
        assert_eq!(array_appender.dimension(), 128);
    }

    #[test]
    fn test_props_access() {
        let appender = SinglePropertyFeatureAppender::new(PhantomData, 5);
        let _props = appender.props();
        // Verify we can access props
    }

    #[test]
    fn test_atomic_predicate() {
        // The atomic unit: one property, one appender
        // "Socrates is Musical" - single predicate

        let musical_appender = SinglePropertyFeatureAppender::new(PhantomData, 1);
        assert_eq!(musical_appender.dimension(), 1);

        // This is the ATOM that will be synthesized in UnionLinkFeatureAppender!
    }

    #[test]
    fn test_genetic_unity_atom() {
        // The simplest Genetic Unity: singular property computation
        // Pure Container: SinglePropertyFeatureAppender
        // Given Container: Concrete type-specific subclass
        // Given Contained: Property extraction
        // Pure Contained: Computation algorithm

        let appender = SinglePropertyFeatureAppender::new(PhantomData, 8);
        assert_eq!(appender.dimension(), 8);

        // This atom will jump in the garden (UnionLink)! 🫘
    }
}

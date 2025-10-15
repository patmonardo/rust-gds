// Phase 2.6: AbstractLinkFeatureAppenderFactory - Abstract factory for type-specific appenders

use super::super::LinkFeatureAppender;
use crate::types::graph::Graph;
use std::marker::PhantomData;

/// Abstract factory for creating type-specific LinkFeatureAppenders.
///
/// # The Mexican Jumping Bean Container! 🫘
///
/// This is the **Scientific Syllogism** - the Container that holds
/// jumping implementations (type-specific appenders) that dispatch
/// based on property value types.
///
/// # Pattern: Abstract Factory with Type Dispatch
///
/// Subclasses implement methods for each supported type:
/// - `double_array_appender` - for f64[] properties
/// - `float_array_appender` - for f32[] properties  
/// - `long_array_appender` - for i64[] properties
/// - `long_appender` - for i64 scalar properties
/// - `double_appender` - for f64 scalar properties
///
/// The factory inspects Graph property types and dispatches to
/// the appropriate constructor.
///
/// # Used By
///
/// - L2FeatureStep (L2LinkFeatureAppenderFactory)
/// - HadamardFeatureStep (HadamardLinkFeatureAppenderFactory)
/// - CosineFeatureStep (CosineLinkFeatureAppenderFactory)
///
/// # The Four-Fold
///
/// - **Pure Container**: AbstractLinkFeatureAppenderFactory trait
/// - **Given Container**: Concrete factories (L2Factory, HadamardFactory, etc.)
/// - **Given Contained**: createAppender() type dispatch logic (empirical)
/// - **Pure Contained**: Type-specific appender construction (deferred)
///
/// This is the **Container of Jumping Beans** - each bean (appender)
/// knows how to jump (compute) for its specific type!
pub trait AbstractLinkFeatureAppenderFactory {
    /// Creates a DoubleArray-typed appender.
    fn double_array_appender(
        &self,
        props: PhantomData<()>, // TODO: NodePropertyValues
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a FloatArray-typed appender.
    fn float_array_appender(
        &self,
        props: PhantomData<()>, // TODO: NodePropertyValues
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a LongArray-typed appender.
    fn long_array_appender(
        &self,
        props: PhantomData<()>, // TODO: NodePropertyValues
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a Long scalar-typed appender.
    fn long_appender(
        &self,
        props: PhantomData<()>, // TODO: NodePropertyValues
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a Double scalar-typed appender.
    fn double_appender(
        &self,
        props: PhantomData<()>, // TODO: NodePropertyValues
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender>;

    /// Creates a single appender for the given property name.
    ///
    /// Inspects the property's ValueType in the graph and dispatches
    /// to the appropriate type-specific constructor method.
    ///
    /// # Type Dispatch (Given Contained)
    ///
    /// ```text
    /// match propertyType:
    ///   DoubleArray -> double_array_appender()
    ///   FloatArray  -> float_array_appender()
    ///   LongArray   -> long_array_appender()
    ///   Long        -> long_appender()
    ///   Double      -> double_appender()
    ///   Other       -> Error: Unsupported ValueType
    /// ```
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph containing the property
    /// * `property_name` - Name of the property to create appender for
    ///
    /// # Returns
    ///
    /// Type-specific LinkFeatureAppender implementation.
    fn create_appender(
        &self,
        _graph: &dyn Graph,
        _property_name: &str,
    ) -> Box<dyn LinkFeatureAppender> {
        // TODO: Implement type dispatch:
        // 1. Get props = graph.nodeProperties(property_name)
        // 2. Get propertyType = props.valueType()
        // 3. Get dimension = FeatureStepUtil::propertyDimension(graph, property_name)
        // 4. Match on propertyType:
        //    - DoubleArray -> self.double_array_appender(props, dimension)
        //    - FloatArray  -> self.float_array_appender(props, dimension)
        //    - LongArray   -> self.long_array_appender(props, dimension)
        //    - Long        -> self.long_appender(props, dimension)
        //    - Double      -> self.double_appender(props, dimension)
        //    - Other       -> Error: "Unsupported ValueType {}"

        // Placeholder for Gamma quality
        panic!("AbstractLinkFeatureAppenderFactory::create_appender not yet implemented");
    }

    /// Creates appenders for multiple properties.
    ///
    /// Returns array of appenders, one per property name.
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph containing the properties
    /// * `property_names` - List of property names
    ///
    /// # Returns
    ///
    /// Vector of LinkFeatureAppenders, one per property.
    fn create_appenders(
        &self,
        graph: &dyn Graph,
        property_names: &[String],
    ) -> Vec<Box<dyn LinkFeatureAppender>> {
        property_names
            .iter()
            .map(|name| self.create_appender(graph, name))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test factory implementation
    struct TestFactory;

    impl AbstractLinkFeatureAppenderFactory for TestFactory {
        fn double_array_appender(
            &self,
            _props: PhantomData<()>,
            dimension: usize,
        ) -> Box<dyn LinkFeatureAppender> {
            Box::new(TestAppender { dimension })
        }

        fn float_array_appender(
            &self,
            _props: PhantomData<()>,
            dimension: usize,
        ) -> Box<dyn LinkFeatureAppender> {
            Box::new(TestAppender { dimension })
        }

        fn long_array_appender(
            &self,
            _props: PhantomData<()>,
            dimension: usize,
        ) -> Box<dyn LinkFeatureAppender> {
            Box::new(TestAppender { dimension })
        }

        fn long_appender(
            &self,
            _props: PhantomData<()>,
            dimension: usize,
        ) -> Box<dyn LinkFeatureAppender> {
            Box::new(TestAppender { dimension })
        }

        fn double_appender(
            &self,
            _props: PhantomData<()>,
            dimension: usize,
        ) -> Box<dyn LinkFeatureAppender> {
            Box::new(TestAppender { dimension })
        }
    }

    struct TestAppender {
        dimension: usize,
    }

    impl LinkFeatureAppender for TestAppender {
        fn append_features(
            &self,
            _source: u64,
            _target: u64,
            _features: &mut [f64],
            _offset: usize,
        ) {
            // Test implementation
        }

        fn dimension(&self) -> usize {
            self.dimension
        }
    }

    #[test]
    fn test_factory_creates_double_array_appender() {
        let factory = TestFactory;
        let appender = factory.double_array_appender(PhantomData, 10);
        assert_eq!(appender.dimension(), 10);
    }

    #[test]
    fn test_factory_creates_float_array_appender() {
        let factory = TestFactory;
        let appender = factory.float_array_appender(PhantomData, 5);
        assert_eq!(appender.dimension(), 5);
    }

    #[test]
    fn test_factory_creates_long_array_appender() {
        let factory = TestFactory;
        let appender = factory.long_array_appender(PhantomData, 8);
        assert_eq!(appender.dimension(), 8);
    }

    #[test]
    fn test_factory_creates_long_appender() {
        let factory = TestFactory;
        let appender = factory.long_appender(PhantomData, 1);
        assert_eq!(appender.dimension(), 1);
    }

    #[test]
    fn test_factory_creates_double_appender() {
        let factory = TestFactory;
        let appender = factory.double_appender(PhantomData, 1);
        assert_eq!(appender.dimension(), 1);
    }

    #[test]
    fn test_mexican_jumping_bean_container() {
        // The Scientific Syllogism - Container of Jumping Beans!
        // Each bean (appender) knows how to jump (compute) for its type

        let factory = TestFactory;

        // The beans jump differently based on type!
        let double_bean = factory.double_appender(PhantomData, 1);
        let array_bean = factory.double_array_appender(PhantomData, 10);

        assert_eq!(double_bean.dimension(), 1); // Scalar bean
        assert_eq!(array_bean.dimension(), 10); // Array bean

        // Same factory, different jumps! 🫘
        // Pure Container holds Given Contained type dispatch
        // Contained is WITHIN Container - Being qua Being!
    }
}

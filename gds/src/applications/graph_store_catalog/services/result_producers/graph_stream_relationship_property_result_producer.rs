/// Producer for streaming relationship property results.
/// 
/// Mirrors Java GraphStreamRelationshipPropertyOrPropertiesResultProducer interface.
/// Single method trait for producing results from relationship property streaming operations.
pub trait GraphStreamRelationshipPropertyResultProducer<R> {
    /// Produces a result from relationship property data.
    /// 
    /// # Arguments
    /// * `source_id` - The ID of the source node
    /// * `target_id` - The ID of the target node
    /// * `relationship_type` - The type of the relationship
    /// * `property_name` - The optional name of the property (None if no property)
    /// * `property_value` - The value of the property
    fn produce(&self, source_id: u64, target_id: u64, relationship_type: &str, property_name: Option<&str>, property_value: &dyn std::any::Any) -> R;
}

/// Default implementation for producing relationship property results as tuples.
/// 
/// This is a simple implementation that produces (source_id, target_id, relationship_type, property_name, property_value) tuples.
#[derive(Clone, Debug)]
pub struct DefaultRelationshipPropertyResultProducer;

impl DefaultRelationshipPropertyResultProducer {
    /// Creates a new DefaultRelationshipPropertyResultProducer.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultRelationshipPropertyResultProducer {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> GraphStreamRelationshipPropertyResultProducer<R> for DefaultRelationshipPropertyResultProducer
where
    R: From<(u64, u64, String, Option<String>, String)>,
{
    fn produce(&self, source_id: u64, target_id: u64, relationship_type: &str, property_name: Option<&str>, property_value: &dyn std::any::Any) -> R {
        // Convert property value to string for simplicity
        let value_str = format!("{:?}", property_value);
        let result = (
            source_id,
            target_id,
            relationship_type.to_string(),
            property_name.map(|s| s.to_string()),
            value_str,
        );
        R::from(result)
    }
}

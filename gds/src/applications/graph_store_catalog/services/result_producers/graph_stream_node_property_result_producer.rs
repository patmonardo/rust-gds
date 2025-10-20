/// Producer for streaming node property results.
/// 
/// Mirrors Java GraphStreamNodePropertyOrPropertiesResultProducer interface.
/// Single method trait for producing results from node property streaming operations.
pub trait GraphStreamNodePropertyResultProducer<R> {
    /// Produces a result from node property data.
    /// 
    /// # Arguments
    /// * `node_id` - The ID of the node
    /// * `property_name` - The name of the property
    /// * `property_value` - The value of the property
    /// * `node_labels` - The labels associated with the node
    fn produce(&self, node_id: u64, property_name: &str, property_value: &dyn std::any::Any, node_labels: &[String]) -> R;
}

/// Default implementation for producing node property results as tuples.
/// 
/// This is a simple implementation that produces (node_id, property_name, property_value, node_labels) tuples.
#[derive(Clone, Debug)]
pub struct DefaultNodePropertyResultProducer;

impl DefaultNodePropertyResultProducer {
    /// Creates a new DefaultNodePropertyResultProducer.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultNodePropertyResultProducer {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> GraphStreamNodePropertyResultProducer<R> for DefaultNodePropertyResultProducer
where
    R: From<(u64, String, String, Vec<String>)>,
{
    fn produce(&self, node_id: u64, property_name: &str, property_value: &dyn std::any::Any, node_labels: &[String]) -> R {
        // Convert property value to string for simplicity
        let value_str = format!("{:?}", property_value);
        let result = (
            node_id,
            property_name.to_string(),
            value_str,
            node_labels.to_vec(),
        );
        R::from(result)
    }
}

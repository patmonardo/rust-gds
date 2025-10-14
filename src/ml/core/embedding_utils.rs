//! Embedding utilities for ML in GDS.
//!
//! Translated from Java GDS ml-core EmbeddingUtils.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Retrieve a checked double array node property.
pub fn get_checked_double_array_node_property(
    graph: &(),
    property_key: &str,
    node_id: u64,
) -> Vec<f64> {
    let _ = (graph, property_key, node_id);
    todo!("Call Graph::node_properties(property_key).double_array_value(node_id) once available.");
}

/// Retrieve a checked long array node property.
pub fn get_checked_long_array_node_property(
    graph: &(),
    property_key: &str,
    node_id: u64,
) -> Vec<i64> {
    let _ = (graph, property_key, node_id);
    todo!("Call Graph::node_properties(property_key).long_array_value(node_id) once available.");
}

/// Retrieve a checked long array node property with expected length validation.
pub fn get_checked_long_array_node_property_with_length(
    graph: &(),
    property_key: &str,
    node_id: u64,
    expected_length: usize,
) -> Vec<i64> {
    let property_value = get_checked_long_array_node_property(graph, property_key, node_id);
    if property_value.len() != expected_length {
        panic!(
            "The property `{}` contains arrays of differing lengths `{}` and `{}`.",
            property_key,
            property_value.len(),
            expected_length
        );
    }
    property_value
}

/// Validate relationship weight property values using the default validator.
pub fn validate_relationship_weight_property_value(
    graph: &(),
    concurrency: (),
    executor_service: (),
) {
    validate_relationship_weight_property_value_with_validator(
        graph,
        concurrency,
        |weight| !weight.is_nan(),
        "Consider using `defaultValue` when loading the graph.",
        executor_service,
    );
}

/// Validate relationship weight property values with a custom validator.
pub fn validate_relationship_weight_property_value_with_validator<F>(
    graph: &(),
    concurrency: (),
    validator: F,
    error_details: &str,
    executor_service: (),
) where
    F: Fn(f64) -> bool,
{
    let _ = (graph, concurrency, executor_service, error_details);
    let _ = validator;
    todo!(
        "Translate RunWithConcurrency and PartitionUtils once the concurrency utilities are available."
    );
}

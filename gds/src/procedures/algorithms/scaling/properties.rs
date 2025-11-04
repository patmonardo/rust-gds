use crate::pregel::NodeValue;

/// Minimal property access abstraction for scalers
pub trait ScalarPropertyValues {
    fn node_count(&self) -> usize;
    fn value(&self, node_id: usize) -> f64;
}

/// Adapter over a slice of f64 values
pub struct VecPropertyValues<'a> {
    data: &'a [f64],
}

impl<'a> VecPropertyValues<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        Self { data }
    }
}

impl<'a> ScalarPropertyValues for VecPropertyValues<'a> {
    fn node_count(&self) -> usize {
        self.data.len()
    }

    fn value(&self, node_id: usize) -> f64 {
        self.data[node_id]
    }
}

/// Adapter over Pregel `NodeValue` for a specific key
pub struct NodeValuePropertyValues<'a> {
    node_values: &'a NodeValue,
    key: &'a str,
}

impl<'a> NodeValuePropertyValues<'a> {
    pub fn new(node_values: &'a NodeValue, key: &'a str) -> Self {
        Self { node_values, key }
    }
}

impl<'a> ScalarPropertyValues for NodeValuePropertyValues<'a> {
    fn node_count(&self) -> usize {
        // NodeValue holds arrays sized to node_count; get length via a known property
        // As NodeValue does not expose count directly, derive from any property's length by probing index 0 until fail is not ideal.
        // We require callers to know node_count; in absence, fall back to 0.
        // For now, return 0 if the key is missing; otherwise binary search last index isn't available.
        // Consumers using this adapter should iterate with an external node_count.
        // Provide best-effort: not available -> 0.
        // NOTE: Prefer providing node_count externally where needed.
        // Since NodeValue lacks a direct node_count, return 0 here.
        0
    }

    fn value(&self, node_id: usize) -> f64 {
        self.node_values.double_value(self.key, node_id)
    }
}



use crate::core::GraphDimensions;

/// Dimension Transformer - transforms graph dimensions intelligently for better estimates
/// For some algorithms we can transform dimensions intelligently,
/// to give better estimates
pub trait DimensionTransformer {
    fn transform(&self, graph_dimensions: Box<dyn GraphDimensions>) -> Box<dyn GraphDimensions>;
}

/// Disabled Dimension Transformer - no transformation
pub struct DisabledDimensionTransformer;

impl DimensionTransformer for DisabledDimensionTransformer {
    fn transform(&self, graph_dimensions: Box<dyn GraphDimensions>) -> Box<dyn GraphDimensions> {
        graph_dimensions
    }
}

impl DisabledDimensionTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DisabledDimensionTransformer {
    fn default() -> Self {
        Self::new()
    }
}

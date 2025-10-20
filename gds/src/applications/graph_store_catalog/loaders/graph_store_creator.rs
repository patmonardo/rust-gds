use super::GraphStoreLoader;
use crate::core::GraphDimensions;

/// Extended trait for creating graph stores with memory estimation.
/// 
/// Mirrors Java GraphStoreCreator interface.
/// Extends GraphStoreLoader with 2 estimation methods for memory usage.
pub trait GraphStoreCreator: GraphStoreLoader {
    /// Estimates memory usage during the loading process.
    /// In Java, this returns MemoryEstimation.
    fn estimate_memory_usage_during_loading(&self) -> Box<dyn MemoryEstimation>;
    
    /// Estimates memory usage after loading is complete.
    /// In Java, this returns MemoryEstimation.
    fn estimate_memory_usage_after_loading(&self) -> Box<dyn MemoryEstimation>;
}

/// Placeholder for MemoryEstimation trait.
/// In real implementation, this would be the actual MemoryEstimation type.
pub trait MemoryEstimation {
    fn estimate(&self, dimensions: &dyn GraphDimensions) -> u64;
}

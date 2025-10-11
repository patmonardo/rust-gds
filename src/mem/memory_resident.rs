//! Memory resident calculation interface
//!
//! Provides a calculation interface for objects that have resources residing in memory.

use super::memory_range::MemoryRange;
use crate::core::graph_dimensions::GraphDimensions;

/// A calculation of an object that has resources residing in memory
///
/// This trait represents a function that estimates memory usage based on
/// graph dimensions and concurrency settings.
pub trait MemoryResident {
    /// Estimates the number of bytes that this object occupies in memory
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The dimensions of the graph
    /// * `concurrency` - The concurrency level (number of threads)
    ///
    /// # Returns
    ///
    /// The memory range representing minimum and maximum memory usage
    fn estimate_memory_usage(
        &self,
        dimensions: &dyn GraphDimensions,
        concurrency: usize,
    ) -> MemoryRange;
}

/// Function-based memory resident implementation
pub struct FunctionMemoryResident<F>
where
    F: Fn(&dyn GraphDimensions, usize) -> MemoryRange,
{
    func: F,
}

impl<F> FunctionMemoryResident<F>
where
    F: Fn(&dyn GraphDimensions, usize) -> MemoryRange,
{
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> MemoryResident for FunctionMemoryResident<F>
where
    F: Fn(&dyn GraphDimensions, usize) -> MemoryRange,
{
    fn estimate_memory_usage(
        &self,
        dimensions: &dyn GraphDimensions,
        concurrency: usize,
    ) -> MemoryRange {
        (self.func)(dimensions, concurrency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::ConcreteGraphDimensions;

    #[test]
    fn test_function_memory_resident() {
        let resident =
            FunctionMemoryResident::new(|dims, _conc| MemoryRange::of(dims.node_count() * 8));

        let dims = ConcreteGraphDimensions::of(1000, 5000);
        let range = resident.estimate_memory_usage(&dims, 4);

        assert_eq!(range.min(), 8000);
        assert_eq!(range.max(), 8000);
    }
}

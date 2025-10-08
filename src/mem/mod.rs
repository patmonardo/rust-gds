//! Memory estimation and tracking system
//!
//! This module provides sophisticated memory estimation, tracking, and management
//! capabilities for graph data structures and algorithms.
//!
//! ## Core Components
//!
//! - **BitUtil** - Bit manipulation utilities (power of two, alignment, leading zeros)
//! - **Estimate** - Memory size calculations for data structures
//! - **MemoryRange** - Min/max byte ranges with arithmetic operations
//! - **MemoryEstimation** - Trait for components that can estimate memory usage
//! - **MemoryTree** - Tree-shaped memory descriptions for hierarchical estimation
//! - **HugeArrays** - Page-based huge array management
//! - **Containers** - Track memory usage per user for graphs and tasks
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use rust_gds::mem::*;
//!
//! // Calculate memory for an array
//! let array_size = Estimate::size_of_long_array(1_000_000);
//! println!("Array needs: {}", Estimate::human_readable(array_size));
//!
//! // Create a memory range
//! let range = MemoryRange::of(1024, 2048);
//! let doubled = range.times(2);
//!
//! // Track graph memory per user
//! let mut container = GraphStoreMemoryContainer::new();
//! container.add_graph("alice", "my-graph", 1024 * 1024 * 100);
//! ```

pub mod bit_util;
pub mod estimate;
pub mod graph_store_memory_container;
pub mod huge_arrays;
pub mod memory_estimation;
pub mod memory_range;
pub mod memory_reservation_exception;
pub mod memory_resident;
pub mod memory_tree;
pub mod task_memory_container;
pub mod user_entity_memory;
pub mod user_memory_summary;

// Re-export public API
pub use bit_util::BitUtil;
pub use estimate::Estimate;
pub use graph_store_memory_container::{
    GraphStoreAddedEvent, GraphStoreMemoryContainer, GraphStoreRemovedEvent,
};
pub use huge_arrays::HugeArrays;
pub use memory_estimation::{MemoryEstimation, MemoryEstimationWithDimensions};
pub use memory_range::MemoryRange;
pub use memory_reservation_exception::MemoryReservationExceededException;
pub use memory_resident::MemoryResident;
pub use memory_tree::{MemoryTree, MemoryTreeWithDimensions};
pub use task_memory_container::TaskMemoryContainer;
pub use user_entity_memory::UserEntityMemory;
pub use user_memory_summary::UserMemorySummary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_system_integration() {
        // Test basic memory range operations
        let range1 = MemoryRange::of(1000);
        let range2 = MemoryRange::of(2000);
        let combined = range1.add(&range2);
        assert_eq!(combined.min(), 3000);
        assert_eq!(combined.max(), 3000);
    }

    #[test]
    fn test_bit_util_basics() {
        assert!(BitUtil::is_power_of_two(16));
        assert!(!BitUtil::is_power_of_two(15));
        assert_eq!(BitUtil::next_highest_power_of_two(15), 16);
        assert_eq!(BitUtil::previous_power_of_two(17), 16);
    }

    #[test]
    fn test_estimate_sizes() {
        let int_array = Estimate::size_of_int_array(1000);
        assert!(int_array > 0);

        let long_array = Estimate::size_of_long_array(1000);
        assert!(long_array > int_array); // longs are bigger than ints
    }

    #[test]
    fn test_huge_arrays_paging() {
        let index = 100_000;
        let page = HugeArrays::page_index(index);
        let in_page = HugeArrays::index_in_page(index);

        let reconstructed = HugeArrays::index_from_page_index_and_index_in_page(page, in_page);
        assert_eq!(reconstructed, index);
    }

    #[test]
    fn test_graph_store_container() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        assert_eq!(container.graph_store_reserved_memory(), 1000);

        container.add_graph("alice", "graph2", 2000);
        assert_eq!(container.graph_store_reserved_memory(), 3000);

        container.remove_graph("alice", "graph1");
        assert_eq!(container.graph_store_reserved_memory(), 2000);
    }

    #[test]
    fn test_user_entity_memory() {
        let graph_mem = UserEntityMemory::create_graph("bob", "my-graph", 5000);
        assert_eq!(graph_mem.user(), "bob");
        assert_eq!(graph_mem.name(), "my-graph");
        assert_eq!(graph_mem.memory_in_bytes(), 5000);
    }
}

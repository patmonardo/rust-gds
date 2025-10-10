//! HugeIntArray - i32 array supporting billions of elements
//!
//! Generated using the huge_primitive_array! macro.
//! See huge_array_macro.rs for the macro implementation.

// Import the macro
use crate::huge_primitive_array;

// Generate the complete HugeIntArray implementation
huge_primitive_array! {
    HugeIntArray,              // Main enum name
    SingleHugeIntArray,        // Single-page struct name
    PagedHugeIntArray,         // Paged struct name
    i32,                       // Element type
    "Int",                     // Type display name
    "A long-indexable i32 array that can contain more than 2 billion elements."  // Doc description
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let array = HugeIntArray::new(100);
        assert_eq!(array.size(), 100);
    }

    #[test]
    fn test_get_set() {
        let mut array = HugeIntArray::new(10);
        array.set(5, 42);
        assert_eq!(array.get(5), 42);
        assert_eq!(array.get(0), 0); // Default value
    }

    #[test]
    fn test_fill() {
        let mut array = HugeIntArray::new(100);
        array.fill(99);
        assert_eq!(array.get(0), 99);
        assert_eq!(array.get(50), 99);
        assert_eq!(array.get(99), 99);
    }

    #[test]
    fn test_set_all() {
        let mut array = HugeIntArray::new(10);
        array.set_all(|i| (i * 2) as i32);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(1), 2);
        assert_eq!(array.get(5), 10);
    }

    #[test]
    fn test_from_vec() {
        let array = HugeIntArray::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(array.size(), 5);
        assert_eq!(array.get(0), 1);
        assert_eq!(array.get(4), 5);
    }

    #[test]
    fn test_copy_of() {
        let mut array = HugeIntArray::new(5);
        array.set_all(|i| i as i32);

        let copy = array.copy_of(10);
        assert_eq!(copy.size(), 10);
        assert_eq!(copy.get(0), 0);
        assert_eq!(copy.get(4), 4);
        assert_eq!(copy.get(9), 0); // Extended with default
    }

    #[test]
    fn test_binary_search() {
        let mut array = HugeIntArray::new(10);
        array.set_all(|i| (i * 10) as i32);

        assert_eq!(array.binary_search(30), 3);
        assert!(array.binary_search(35) < 0); // Not found
    }

    #[test]
    fn test_large_array() {
        // Test that paged implementation kicks in for large arrays
        let large_size = (1 << 28) + 1000; // Just over 268M
        let mut array = HugeIntArray::new(large_size);

        array.set(0, 100);
        array.set(large_size - 1, 200);

        assert_eq!(array.get(0), 100);
        assert_eq!(array.get(large_size - 1), 200);
    }
}

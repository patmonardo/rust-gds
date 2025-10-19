//! Memory size estimation utilities
//!
//! Provides utilities for estimating memory usage of various data structures
//! based on JVM memory patterns, adapted for Rust.

/// Memory estimation constants and utilities
///
/// Note: These estimates are based on JVM memory patterns and provide
/// approximations for Rust data structures. Actual memory usage may vary.
pub struct Estimate;

impl Estimate {
    /// Size of an object reference (pointer)
    pub const BYTES_OBJECT_REF: usize = 8;

    /// Maximum array size (safe limit)
    pub const MAX_ARRAY_SIZE: usize = usize::MAX - 8;

    /// Array header overhead
    pub const BYTES_ARRAY_HEADER: usize = 24;

    /// Object header overhead
    pub const BYTES_OBJECT_HEADER: usize = 16;

    /// Size of a byte array
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::Estimate;
    ///
    /// let size = Estimate::size_of_byte_array(1000);
    /// assert!(size > 1000); // Includes header overhead
    /// ```
    pub fn size_of_byte_array(length: usize) -> usize {
        Self::size_of_array(length, 1)
    }

    /// Size of a char array (2 bytes per char)
    pub fn size_of_char_array(length: usize) -> usize {
        Self::size_of_array(length, 2)
    }

    /// Size of a short array (2 bytes per short)
    pub fn size_of_short_array(length: usize) -> usize {
        Self::size_of_array(length, 2)
    }

    /// Size of an int array (4 bytes per int)
    pub fn size_of_int_array(length: usize) -> usize {
        Self::size_of_array(length, 4)
    }

    /// Size of a float array (4 bytes per float)
    pub fn size_of_float_array(length: usize) -> usize {
        Self::size_of_array(length, 4)
    }

    /// Size of a long array (8 bytes per long)
    pub fn size_of_long_array(length: usize) -> usize {
        Self::size_of_array(length, 8)
    }

    /// Size of a double array (8 bytes per double)
    pub fn size_of_double_array(length: usize) -> usize {
        Self::size_of_array(length, 8)
    }

    /// Size of an object array (references only, not the objects themselves)
    pub fn size_of_object_array(length: usize) -> usize {
        Self::size_of_array(length, Self::BYTES_OBJECT_REF)
    }

    /// Size of object array elements (just the references)
    pub fn size_of_object_array_elements(length: usize) -> usize {
        length * Self::BYTES_OBJECT_REF
    }

    /// Generic array size calculation
    ///
    /// Includes array header and aligns to 8-byte boundary
    pub fn size_of_array(length: usize, bytes_per_element: usize) -> usize {
        let size = Self::BYTES_ARRAY_HEADER + (length * bytes_per_element);
        Self::align_to_8_bytes(size)
    }

    /// Size of a bitset (bit array)
    pub fn size_of_bitset(length: usize) -> usize {
        let num_longs = length.div_ceil(64); // Ceiling division
        Self::BYTES_OBJECT_HEADER + Self::size_of_long_array(num_longs)
    }

    /// Size of a long-double hash map
    pub fn size_of_long_double_hash_map(length: usize) -> usize {
        Self::BYTES_OBJECT_HEADER
            + Self::size_of_open_hash_container(length)
            + Self::size_of_long_array(length)
            + Self::size_of_double_array(length)
    }

    /// Size of a long-double scatter map
    pub fn size_of_long_double_scatter_map(length: usize) -> usize {
        Self::size_of_long_double_hash_map(length)
    }

    /// Size of a long hash set
    pub fn size_of_long_hash_set(length: usize) -> usize {
        Self::BYTES_OBJECT_HEADER
            + Self::size_of_open_hash_container(length)
            + Self::size_of_long_array(length)
    }

    /// Size of empty open hash container overhead
    pub fn size_of_empty_open_hash_container() -> usize {
        72
    }

    /// Size of a long array list
    pub fn size_of_long_array_list(length: usize) -> usize {
        Self::BYTES_OBJECT_HEADER + Self::size_of_long_array(length)
    }

    /// Size of an int array list
    pub fn size_of_int_array_list(length: usize) -> usize {
        Self::BYTES_OBJECT_HEADER + Self::size_of_int_array(length)
    }

    /// Size of a double array list
    pub fn size_of_double_array_list(length: usize) -> usize {
        Self::BYTES_OBJECT_HEADER + Self::size_of_double_array(length)
    }

    /// Size of open hash container with elements
    pub fn size_of_open_hash_container(elements: usize) -> usize {
        if elements == 0 {
            return Self::size_of_empty_open_hash_container();
        }

        // Hash containers typically use load factor of 0.75
        let capacity = (elements as f64 / 0.75).ceil() as usize;
        let capacity = capacity.next_power_of_two();

        Self::size_of_empty_open_hash_container() + Self::size_of_byte_array(capacity)
    }

    /// Estimate instance size by name
    ///
    /// Returns a rough estimate for common data structure types
    pub fn size_of_instance(class_name: &str) -> usize {
        match class_name {
            "LongArrayList" => Self::size_of_long_array_list(0),
            "IntArrayList" => Self::size_of_int_array_list(0),
            "DoubleArrayList" => Self::size_of_double_array_list(0),
            "LongHashSet" => Self::size_of_long_hash_set(0),
            "BitSet" => Self::size_of_bitset(0),
            _ => Self::BYTES_OBJECT_HEADER,
        }
    }

    /// Convert bytes to human-readable format
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::Estimate;
    ///
    /// assert_eq!(Estimate::human_readable(512), "512 Bytes");
    /// assert_eq!(Estimate::human_readable(1024), "1.00 KiB");
    /// assert_eq!(Estimate::human_readable(1024 * 1024), "1.00 MiB");
    /// ```
    pub fn human_readable(bytes: usize) -> String {
        const UNITS: &[&str] = &["Bytes", "KiB", "MiB", "GiB", "TiB", "PiB"];

        if bytes == 0 {
            return "0 Bytes".to_string();
        }

        let bytes_f = bytes as f64;
        let exp = (bytes_f.log2() / 10.0).floor() as usize;
        let exp = exp.min(UNITS.len() - 1);

        let size = bytes_f / (1024_f64.powi(exp as i32));

        if exp == 0 {
            format!("{} {}", bytes, UNITS[exp])
        } else {
            format!("{:.2} {}", size, UNITS[exp])
        }
    }

    /// Align size to 8-byte boundary
    fn align_to_8_bytes(size: usize) -> usize {
        (size + 7) & !7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_array_sizes() {
        assert_eq!(Estimate::size_of_byte_array(0), 24);
        assert_eq!(Estimate::size_of_int_array(100), 424); // 24 + 400
        assert_eq!(Estimate::size_of_long_array(100), 824); // 24 + 800
    }

    #[test]
    fn test_alignment() {
        // All sizes should be 8-byte aligned
        assert_eq!(Estimate::size_of_byte_array(1) % 8, 0);
        assert_eq!(Estimate::size_of_int_array(1) % 8, 0);
        assert_eq!(Estimate::size_of_long_array(1) % 8, 0);
    }

    #[test]
    fn test_human_readable() {
        assert_eq!(Estimate::human_readable(0), "0 Bytes");
        assert_eq!(Estimate::human_readable(512), "512 Bytes");
        assert_eq!(Estimate::human_readable(1024), "1.00 KiB");
        assert_eq!(Estimate::human_readable(1024 * 1024), "1.00 MiB");
        assert_eq!(Estimate::human_readable(1024 * 1024 * 1024), "1.00 GiB");
        assert_eq!(Estimate::human_readable(1536 * 1024), "1.50 MiB");
    }

    #[test]
    fn test_container_sizes() {
        let hash_set = Estimate::size_of_long_hash_set(100);
        assert!(hash_set > 0);

        let array_list = Estimate::size_of_long_array_list(100);
        assert!(array_list > 0);

        let bitset = Estimate::size_of_bitset(1000);
        assert!(bitset > 0);
    }

    #[test]
    fn test_size_comparison() {
        let small = Estimate::size_of_int_array(100);
        let large = Estimate::size_of_long_array(100);
        assert!(large > small); // longs are bigger than ints
    }
}

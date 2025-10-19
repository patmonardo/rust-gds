//! Bit manipulation utilities
//!
//! Provides efficient bit-level operations for power-of-two calculations,
//! alignment, and bit counting operations.

/// Utility for bit manipulation operations
///
/// All operations work with `usize` values and are optimized for performance.
pub struct BitUtil;

impl BitUtil {
    /// Checks if a number is a power of two
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert!(BitUtil::is_power_of_two(16));
    /// assert!(!BitUtil::is_power_of_two(15));
    /// assert!(!BitUtil::is_power_of_two(0));
    /// ```
    pub fn is_power_of_two(value: usize) -> bool {
        value != 0 && (value & (value - 1)) == 0
    }

    /// Returns the previous highest power of two for a number
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::previous_power_of_two(17), 16);
    /// assert_eq!(BitUtil::previous_power_of_two(16), 16);
    /// assert_eq!(BitUtil::previous_power_of_two(15), 8);
    /// ```
    pub fn previous_power_of_two(mut v: usize) -> usize {
        if v == 0 {
            return 0;
        }

        v |= v >> 1;
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;
        #[cfg(target_pointer_width = "64")]
        {
            v |= v >> 32;
        }

        v - (v >> 1)
    }

    /// Returns the next highest power of two for a number
    ///
    /// Returns 0 if the input is 0 or negative (when cast from signed).
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::next_highest_power_of_two(15), 16);
    /// assert_eq!(BitUtil::next_highest_power_of_two(16), 16);
    /// assert_eq!(BitUtil::next_highest_power_of_two(17), 32);
    /// ```
    pub fn next_highest_power_of_two(mut v: usize) -> usize {
        if v == 0 {
            return 0;
        }

        v -= 1;
        v |= v >> 1;
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;
        #[cfg(target_pointer_width = "64")]
        {
            v |= v >> 32;
        }

        v + 1
    }

    /// Returns the nearest power of two to the given number
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::nearby_power_of_two(10), 8);
    /// assert_eq!(BitUtil::nearby_power_of_two(12), 8);  // Equidistant, prefers lower
    /// assert_eq!(BitUtil::nearby_power_of_two(13), 16);
    /// assert_eq!(BitUtil::nearby_power_of_two(16), 16);
    /// ```
    pub fn nearby_power_of_two(x: usize) -> usize {
        if x == 0 {
            return 0;
        }

        let next_power = Self::next_highest_power_of_two(x);
        let prev_power = next_power >> 1;

        // Return whichever is closer
        if (next_power - x) < (x - prev_power) {
            next_power
        } else {
            prev_power
        }
    }

    /// Aligns a value to the specified power-of-two alignment
    ///
    /// # Panics
    ///
    /// Panics if alignment is not a positive power of two.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::align(10, 8), 16);
    /// assert_eq!(BitUtil::align(16, 8), 16);
    /// assert_eq!(BitUtil::align(17, 8), 24);
    /// ```
    pub fn align(value: usize, alignment: usize) -> usize {
        assert!(alignment > 0, "Alignment must be positive");
        assert!(
            Self::is_power_of_two(alignment),
            "Alignment must be a power of two"
        );

        let mask = alignment - 1;
        (value + mask) & !mask
    }

    /// Returns the number of leading zero bits (32-bit)
    ///
    /// Essential for determining the minimum number of bits needed to represent a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::number_of_leading_zeros_32(1), 31);
    /// assert_eq!(BitUtil::number_of_leading_zeros_32(0xFF), 24);
    /// ```
    pub fn number_of_leading_zeros_32(value: u32) -> u32 {
        value.leading_zeros()
    }

    /// Returns the number of leading zero bits (native size)
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::number_of_leading_zeros(1), usize::BITS - 1);
    /// ```
    pub fn number_of_leading_zeros(value: usize) -> u32 {
        value.leading_zeros()
    }

    /// Returns the number of trailing zero bits
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::number_of_trailing_zeros(8), 3);
    /// assert_eq!(BitUtil::number_of_trailing_zeros(12), 2);
    /// ```
    pub fn number_of_trailing_zeros(value: usize) -> u32 {
        value.trailing_zeros()
    }

    /// Ceiling division: computes ⌈dividend / divisor⌉
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::BitUtil;
    ///
    /// assert_eq!(BitUtil::ceil_div(10, 3), 4);
    /// assert_eq!(BitUtil::ceil_div(9, 3), 3);
    /// assert_eq!(BitUtil::ceil_div(1, 3), 1);
    /// ```
    pub fn ceil_div(dividend: usize, divisor: usize) -> usize {
        assert!(divisor > 0, "Divisor must be positive");
        dividend.div_ceil(divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(!BitUtil::is_power_of_two(0));
        assert!(BitUtil::is_power_of_two(1));
        assert!(BitUtil::is_power_of_two(2));
        assert!(!BitUtil::is_power_of_two(3));
        assert!(BitUtil::is_power_of_two(4));
        assert!(BitUtil::is_power_of_two(1024));
        assert!(!BitUtil::is_power_of_two(1023));
    }

    #[test]
    fn test_previous_power_of_two() {
        assert_eq!(BitUtil::previous_power_of_two(0), 0);
        assert_eq!(BitUtil::previous_power_of_two(1), 1);
        assert_eq!(BitUtil::previous_power_of_two(2), 2);
        assert_eq!(BitUtil::previous_power_of_two(3), 2);
        assert_eq!(BitUtil::previous_power_of_two(17), 16);
        assert_eq!(BitUtil::previous_power_of_two(1000), 512);
    }

    #[test]
    fn test_next_highest_power_of_two() {
        assert_eq!(BitUtil::next_highest_power_of_two(0), 0);
        assert_eq!(BitUtil::next_highest_power_of_two(1), 1);
        assert_eq!(BitUtil::next_highest_power_of_two(2), 2);
        assert_eq!(BitUtil::next_highest_power_of_two(3), 4);
        assert_eq!(BitUtil::next_highest_power_of_two(15), 16);
        assert_eq!(BitUtil::next_highest_power_of_two(16), 16);
        assert_eq!(BitUtil::next_highest_power_of_two(17), 32);
    }

    #[test]
    fn test_nearby_power_of_two() {
        assert_eq!(BitUtil::nearby_power_of_two(0), 0);
        assert_eq!(BitUtil::nearby_power_of_two(10), 8); // 10 is closer to 8 than 16
        assert_eq!(BitUtil::nearby_power_of_two(12), 8); // 12 - 8 = 4, 16 - 12 = 4, but we prefer lower
        assert_eq!(BitUtil::nearby_power_of_two(13), 16); // 16 - 13 = 3 < 13 - 8 = 5
        assert_eq!(BitUtil::nearby_power_of_two(16), 16);
    }

    #[test]
    fn test_align() {
        assert_eq!(BitUtil::align(0, 8), 0);
        assert_eq!(BitUtil::align(1, 8), 8);
        assert_eq!(BitUtil::align(8, 8), 8);
        assert_eq!(BitUtil::align(9, 8), 16);
        assert_eq!(BitUtil::align(10, 8), 16);
        assert_eq!(BitUtil::align(100, 64), 128);
    }

    #[test]
    #[should_panic(expected = "power of two")]
    fn test_align_invalid() {
        BitUtil::align(10, 7);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(BitUtil::number_of_leading_zeros_32(0), 32);
        assert_eq!(BitUtil::number_of_leading_zeros_32(1), 31);
        assert_eq!(BitUtil::number_of_leading_zeros_32(0xFF), 24);
        assert_eq!(BitUtil::number_of_leading_zeros_32(0x8000_0000), 0);
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(BitUtil::number_of_trailing_zeros(0), usize::BITS);
        assert_eq!(BitUtil::number_of_trailing_zeros(1), 0);
        assert_eq!(BitUtil::number_of_trailing_zeros(8), 3);
        assert_eq!(BitUtil::number_of_trailing_zeros(12), 2);
    }

    #[test]
    fn test_ceil_div() {
        assert_eq!(BitUtil::ceil_div(0, 3), 0);
        assert_eq!(BitUtil::ceil_div(1, 3), 1);
        assert_eq!(BitUtil::ceil_div(2, 3), 1);
        assert_eq!(BitUtil::ceil_div(3, 3), 1);
        assert_eq!(BitUtil::ceil_div(4, 3), 2);
        assert_eq!(BitUtil::ceil_div(9, 3), 3);
        assert_eq!(BitUtil::ceil_div(10, 3), 4);
    }
}

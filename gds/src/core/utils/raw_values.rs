// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Bit manipulation utilities for combining and extracting integer pairs.
//!
//! This module provides efficient methods for packing two 32-bit integers into
//! a single 64-bit value and extracting them back. This is commonly used for:
//! - Storing node pairs in graph edges
//! - Encoding directional relationships
//! - Memory-efficient storage of integer tuples

/// Bit manipulation utilities for raw value encoding.
pub struct RawValues;

impl RawValues {
    /// Shifts head into the most significant 4 bytes of the long
    /// and places the tail in the least significant bytes.
    ///
    /// # Arguments
    /// * `head` - An arbitrary i32 value
    /// * `tail` - An arbitrary i32 value
    ///
    /// # Returns
    /// Combination of head and tail as i64
    ///
    /// # Example
    /// ```
    /// use gds::core::utils::RawValues;
    ///
    /// let combined = RawValues::combine_int_int(42, 100);
    /// assert_eq!(RawValues::get_head(combined), 42);
    /// assert_eq!(RawValues::get_tail(combined), 100);
    /// ```
    #[inline]
    pub fn combine_int_int(head: i32, tail: i32) -> i64 {
        ((head as i64) << 32) | ((tail as i64) & 0xFFFF_FFFF)
    }

    /// Combines two integers based on whether to reverse them.
    /// When reversed is true, it swaps head and tail.
    ///
    /// # Arguments
    /// * `head` - First int value
    /// * `tail` - Second int value
    /// * `reverse` - Whether to swap head and tail
    ///
    /// # Returns
    /// Combination of head and tail, possibly reversed
    #[inline]
    pub fn combine_int_int_reversed(head: i32, tail: i32, reverse: bool) -> i64 {
        if reverse {
            Self::combine_int_int(tail, head)
        } else {
            Self::combine_int_int(head, tail)
        }
    }

    /// Gets the head value from a combined value.
    ///
    /// # Arguments
    /// * `combined_value` - A value built of 2 ints
    ///
    /// # Returns
    /// The most significant 4 bytes as i32
    #[inline]
    pub fn get_head(combined_value: i64) -> i32 {
        (combined_value >> 32) as i32
    }

    /// Gets the tail value from a combined value.
    ///
    /// # Arguments
    /// * `combined_value` - A value built of 2 ints
    ///
    /// # Returns
    /// The least significant 4 bytes as i32
    #[inline]
    pub fn get_tail(combined_value: i64) -> i32 {
        combined_value as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_and_extract() {
        let head = 12345;
        let tail = 67890;

        let combined = RawValues::combine_int_int(head, tail);

        assert_eq!(RawValues::get_head(combined), head);
        assert_eq!(RawValues::get_tail(combined), tail);
    }

    #[test]
    fn test_combine_with_negative_values() {
        let head = -12345;
        let tail = -67890;

        let combined = RawValues::combine_int_int(head, tail);

        assert_eq!(RawValues::get_head(combined), head);
        assert_eq!(RawValues::get_tail(combined), tail);
    }

    #[test]
    fn test_reverse_swap() {
        let head = 100;
        let tail = 200;

        let normal = RawValues::combine_int_int_reversed(head, tail, false);
        let reversed = RawValues::combine_int_int_reversed(head, tail, true);

        assert_eq!(RawValues::get_head(normal), head);
        assert_eq!(RawValues::get_tail(normal), tail);

        assert_eq!(RawValues::get_head(reversed), tail);
        assert_eq!(RawValues::get_tail(reversed), head);
    }
}

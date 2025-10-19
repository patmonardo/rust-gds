//! Iterable over set bits in a BitSet.
//!
//! Provides efficient iteration over the indices of set (1) bits in a BitSet,
//! without materializing all indices into a collection.

use crate::collections::bit_set::BitSet;

/// An iterable over the set bits in a BitSet.
///
/// This provides a memory-efficient way to iterate over only the set bits
/// in a BitSet, rather than iterating over all possible indices.
///
/// # Examples
///
/// ```text
/// use gds::core::utils::SetBitsIterable;
/// use gds::collections::BitSet;
///
/// let mut bitset = BitSet::new(100);
/// bitset.set(5);
/// bitset.set(10);
/// bitset.set(50);
///
/// let iterable = SetBitsIterable::new(bitset, 0);
/// let bits: Vec<usize> = iterable.collect();
/// assert_eq!(bits, vec![5, 10, 50]);
/// ```
pub struct SetBitsIterable {
    set: BitSet,
    offset: usize,
}

impl SetBitsIterable {
    /// Creates a new iterable over the set bits in a BitSet.
    ///
    /// # Arguments
    ///
    /// * `set` - The BitSet to iterate over
    /// * `offset` - Starting offset for iteration (default: 0)
    pub fn new(set: BitSet, offset: usize) -> Self {
        Self { set, offset }
    }

    /// Creates a new iterable starting from index 0.
    pub fn from_bitset(set: BitSet) -> Self {
        Self::new(set, 0)
    }

    /// Returns the number of set bits.
    pub fn size(&self) -> usize {
        self.set.cardinality()
    }

    /// Creates a Vec containing all the set bits.
    pub fn to_vec(&self) -> Vec<usize> {
        self.into_iter().collect()
    }

    /// Creates an iterator over the set bits.
    pub fn iter(&self) -> SetBitsIterator<'_> {
        SetBitsIterator::new(&self.set, self.offset)
    }
}

impl IntoIterator for SetBitsIterable {
    type Item = usize;
    type IntoIter = SetBitsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SetBitsIntoIterator {
            value: self.set.next_set_bit(self.offset),
            set: self.set,
        }
    }
}

impl<'a> IntoIterator for &'a SetBitsIterable {
    type Item = usize;
    type IntoIter = SetBitsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over set bits (borrowed).
pub struct SetBitsIterator<'a> {
    set: &'a BitSet,
    value: Option<usize>,
}

impl<'a> SetBitsIterator<'a> {
    fn new(set: &'a BitSet, index: usize) -> Self {
        let value = set.next_set_bit(index);
        Self { set, value }
    }
}

impl<'a> Iterator for SetBitsIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.value {
            let return_value = current;
            self.value = self.set.next_set_bit(current + 1);
            Some(return_value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.set.cardinality();
        (0, Some(size))
    }
}

/// Iterator over set bits (owned).
pub struct SetBitsIntoIterator {
    set: BitSet,
    value: Option<usize>,
}

impl Iterator for SetBitsIntoIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.value {
            let return_value = current;
            self.value = self.set.next_set_bit(current + 1);
            Some(return_value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.set.cardinality();
        (0, Some(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bitset() {
        let bitset = BitSet::new(100);
        let iterable = SetBitsIterable::from_bitset(bitset);

        let bits = iterable.to_vec();
        assert!(bits.is_empty());
    }

    #[test]
    fn test_single_bit() {
        let mut bitset = BitSet::new(100);
        bitset.set(42);

        let iterable = SetBitsIterable::from_bitset(bitset);
        let bits = iterable.to_vec();

        assert_eq!(bits, vec![42]);
    }

    #[test]
    fn test_multiple_bits() {
        let mut bitset = BitSet::new(100);
        bitset.set(5);
        bitset.set(10);
        bitset.set(50);
        bitset.set(99);

        let iterable = SetBitsIterable::from_bitset(bitset);
        let bits = iterable.to_vec();

        assert_eq!(bits, vec![5, 10, 50, 99]);
    }

    #[test]
    fn test_with_offset() {
        let mut bitset = BitSet::new(100);
        bitset.set(5);
        bitset.set(10);
        bitset.set(50);
        bitset.set(99);

        let iterable = SetBitsIterable::new(bitset, 20);
        let bits = iterable.to_vec();

        // Should only return bits >= 20
        assert_eq!(bits, vec![50, 99]);
    }

    #[test]
    fn test_size() {
        let mut bitset = BitSet::new(100);
        bitset.set(5);
        bitset.set(10);
        bitset.set(50);

        let iterable = SetBitsIterable::from_bitset(bitset);
        assert_eq!(iterable.size(), 3);
    }

    #[test]
    fn test_to_vec() {
        let mut bitset = BitSet::new(100);
        bitset.set(1);
        bitset.set(2);
        bitset.set(3);

        let iterable = SetBitsIterable::from_bitset(bitset);
        let vec = iterable.to_vec();

        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_borrowed_iterator() {
        let mut bitset = BitSet::new(100);
        bitset.set(10);
        bitset.set(20);
        bitset.set(30);

        let iterable = SetBitsIterable::from_bitset(bitset);

        // Use borrowed iterator multiple times
        let bits1: Vec<usize> = iterable.iter().collect();
        let bits2: Vec<usize> = iterable.iter().collect();

        assert_eq!(bits1, vec![10, 20, 30]);
        assert_eq!(bits2, vec![10, 20, 30]);
    }

    #[test]
    fn test_for_loop() {
        let mut bitset = BitSet::new(100);
        bitset.set(1);
        bitset.set(2);
        bitset.set(3);

        let iterable = SetBitsIterable::from_bitset(bitset);

        let mut collected = Vec::new();
        for bit in &iterable {
            collected.push(bit);
        }

        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_iterator_size_hint() {
        let mut bitset = BitSet::new(100);
        bitset.set(5);
        bitset.set(10);
        bitset.set(50);

        let iterable = SetBitsIterable::from_bitset(bitset);
        let mut iter = iterable.iter();

        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(3));

        // Advance iterator
        iter.next();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(3)); // Hint doesn't change (conservative)
    }

    #[test]
    fn test_consecutive_bits() {
        let mut bitset = BitSet::new(100);
        for i in 10..20 {
            bitset.set(i);
        }

        let iterable = SetBitsIterable::from_bitset(bitset);
        let bits = iterable.to_vec();

        assert_eq!(bits, (10..20).collect::<Vec<usize>>());
    }

    #[test]
    fn test_sparse_bits() {
        let mut bitset = BitSet::new(10000);
        bitset.set(0);
        bitset.set(1000);
        bitset.set(5000);
        bitset.set(9999);

        let iterable = SetBitsIterable::from_bitset(bitset);
        let bits = iterable.to_vec();

        assert_eq!(bits, vec![0, 1000, 5000, 9999]);
    }
}

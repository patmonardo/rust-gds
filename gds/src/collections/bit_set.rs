//! BitSet implementation for fast and memory-efficient set of bits
//!
//! Similar to carrotsearch.hppc.BitSet, this provides efficient bit manipulation
//! operations for graph algorithms that need to track boolean flags for large
//! numbers of elements (nodes, edges, etc.).

/// Fast and memory-efficient set of bits
///
/// Provides efficient operations for tracking boolean flags across large index ranges.
/// Uses 32-bit words internally for optimal performance on most platforms.
pub struct BitSet {
    /// The bits in this set (32-bit words)
    bits: Vec<u32>,

    /// The current word count (number of words actually used)
    word_count: usize,
}

impl BitSet {
    /// Number of bits per word
    const BITS_PER_WORD: usize = 32;

    /// Word mask for bit operations
    const WORD_MASK: u32 = 0xffff_ffff;

    /// Creates a new BitSet with the given initial capacity.
    ///
    /// # Arguments
    ///
    /// * `initial_capacity` - Initial capacity in bits (default: 16)
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(128);
    /// bitset.set(42);
    /// assert!(bitset.get(42));
    /// ```
    pub fn new(initial_capacity: usize) -> Self {
        let initial_word_count = usize::max(1, initial_capacity.div_ceil(Self::BITS_PER_WORD));
        Self {
            bits: vec![0; initial_word_count],
            word_count: 0,
        }
    }

    /// Sets the bit at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to set
    ///
    /// # Returns
    ///
    /// Returns `&mut self` for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(64);
    /// bitset.set(10).set(20).set(30);
    /// assert!(bitset.get(10) && bitset.get(20) && bitset.get(30));
    /// ```
    pub fn set(&mut self, index: usize) -> &mut Self {
        let word_index = index >> 5; // Divide by 32
        self.ensure_capacity(word_index);

        self.bits[word_index] |= 1 << (index & 31); // index % 32

        if word_index >= self.word_count {
            self.word_count = word_index + 1;
        }

        self
    }

    /// Clears the bit at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to clear
    ///
    /// # Returns
    ///
    /// Returns `&mut self` for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(64);
    /// bitset.set(42);
    /// assert!(bitset.get(42));
    /// bitset.clear(42);
    /// assert!(!bitset.get(42));
    /// ```
    pub fn clear(&mut self, index: usize) -> &mut Self {
        let word_index = index >> 5;
        if word_index < self.word_count {
            self.bits[word_index] &= !(1 << (index & 31));
        }
        self
    }

    /// Clears all bits in this set.
    ///
    /// # Returns
    ///
    /// Returns `&mut self` for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(64);
    /// bitset.set(10).set(20).set(30);
    /// assert_eq!(bitset.cardinality(), 3);
    /// bitset.clear_all();
    /// assert_eq!(bitset.cardinality(), 0);
    /// ```
    pub fn clear_all(&mut self) -> &mut Self {
        self.bits.fill(0);
        self.word_count = 0;
        self
    }

    /// Returns the value of the bit at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit
    ///
    /// # Returns
    ///
    /// `true` if the bit is set, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(64);
    /// bitset.set(42);
    /// assert!(bitset.get(42));
    /// assert!(!bitset.get(43));
    /// ```
    pub fn get(&self, index: usize) -> bool {
        let word_index = index >> 5;
        word_index < self.word_count && (self.bits[word_index] & (1 << (index & 31))) != 0
    }

    /// Returns the number of bits set to true.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(64);
    /// bitset.set(1).set(5).set(10);
    /// assert_eq!(bitset.cardinality(), 3);
    /// ```
    pub fn cardinality(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.word_count {
            sum += self.bits[i].count_ones() as usize;
        }
        sum
    }

    /// Returns the index of the first bit that is set to true
    /// that occurs on or after the specified index.
    ///
    /// # Arguments
    ///
    /// * `from_index` - The index to start checking from
    ///
    /// # Returns
    ///
    /// The index of the next set bit, or `None` if no such bit exists
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset = BitSet::new(64);
    /// bitset.set(10).set(20).set(30);
    ///
    /// assert_eq!(bitset.next_set_bit(0), Some(10));
    /// assert_eq!(bitset.next_set_bit(11), Some(20));
    /// assert_eq!(bitset.next_set_bit(21), Some(30));
    /// assert_eq!(bitset.next_set_bit(31), None);
    /// ```
    pub fn next_set_bit(&self, from_index: usize) -> Option<usize> {
        let mut word_index = from_index >> 5;
        if word_index >= self.word_count {
            return None;
        }

        // Check if there are any bits set in the current word at or after from_index
        let mut word = self.bits[word_index] & (Self::WORD_MASK << (from_index & 31));

        loop {
            if word != 0 {
                return Some((word_index * Self::BITS_PER_WORD) + word.trailing_zeros() as usize);
            }
            word_index += 1;
            if word_index >= self.word_count {
                return None;
            }
            word = self.bits[word_index];
        }
    }

    /// Performs an OR operation with the specified BitSet.
    ///
    /// # Arguments
    ///
    /// * `other` - The other BitSet
    ///
    /// # Returns
    ///
    /// Returns `&mut self` for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::BitSet;
    ///
    /// let mut bitset1 = BitSet::new(64);
    /// bitset1.set(10).set(20);
    ///
    /// let mut bitset2 = BitSet::new(64);
    /// bitset2.set(20).set(30);
    ///
    /// bitset1.or(&bitset2);
    /// assert!(bitset1.get(10) && bitset1.get(20) && bitset1.get(30));
    /// ```
    pub fn or(&mut self, other: &BitSet) -> &mut Self {
        let min_words = usize::min(self.word_count, other.word_count);
        for i in 0..min_words {
            self.bits[i] |= other.bits[i];
        }

        if other.word_count > self.word_count {
            self.ensure_capacity(other.word_count - 1);
            for i in min_words..other.word_count {
                self.bits[i] = other.bits[i];
            }
            self.word_count = other.word_count;
        }

        self
    }

    /// Ensures capacity for the given word index.
    fn ensure_capacity(&mut self, word_index: usize) {
        if word_index >= self.bits.len() {
            let new_capacity = usize::max(self.bits.len() * 2, word_index + 1);
            self.bits.resize(new_capacity, 0);
        }
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new(16)
    }
}

impl Clone for BitSet {
    fn clone(&self) -> Self {
        BitSet {
            bits: self.bits.clone(),
            word_count: self.word_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut bitset = BitSet::new(64);
        bitset.set(42);
        assert!(bitset.get(42));
        assert!(!bitset.get(43));
    }

    #[test]
    fn test_clear() {
        let mut bitset = BitSet::new(64);
        bitset.set(42);
        assert!(bitset.get(42));
        bitset.clear(42);
        assert!(!bitset.get(42));
    }

    #[test]
    fn test_cardinality() {
        let mut bitset = BitSet::new(64);
        bitset.set(1).set(5).set(10);
        assert_eq!(bitset.cardinality(), 3);
    }

    #[test]
    fn test_next_set_bit() {
        let mut bitset = BitSet::new(64);
        bitset.set(10).set(20).set(30);

        assert_eq!(bitset.next_set_bit(0), Some(10));
        assert_eq!(bitset.next_set_bit(11), Some(20));
        assert_eq!(bitset.next_set_bit(21), Some(30));
        assert_eq!(bitset.next_set_bit(31), None);
    }

    #[test]
    fn test_or_operation() {
        let mut bitset1 = BitSet::new(64);
        bitset1.set(10).set(20);

        let mut bitset2 = BitSet::new(64);
        bitset2.set(20).set(30);

        bitset1.or(&bitset2);
        assert!(bitset1.get(10) && bitset1.get(20) && bitset1.get(30));
    }

    #[test]
    fn test_clear_all() {
        let mut bitset = BitSet::new(64);
        bitset.set(10).set(20).set(30);
        assert_eq!(bitset.cardinality(), 3);
        bitset.clear_all();
        assert_eq!(bitset.cardinality(), 0);
    }
}

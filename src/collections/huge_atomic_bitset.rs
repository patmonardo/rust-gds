//! HugeAtomicBitSet - Thread-safe atomic bitset for billion-scale concurrent processing
//!
//! Essential for parallel graph algorithms requiring shared state:
//! - Concurrent visited node tracking in parallel BFS/DFS
//! - Thread-safe membership testing in distributed algorithms
//! - Atomic flag setting in parallel graph construction
//! - Lock-free synchronization in multi-threaded processing
//! - Parallel community detection with shared membership
//! - Concurrent duplicate detection in graph streaming
//!
//! # Performance Characteristics
//!
//! - Atomic operations using compare-and-swap (CAS)
//! - Lock-free bit manipulation for high concurrency
//! - Word-level operations for bulk bit setting
//! - Cache-friendly i64 word alignment
//! - Billion-bit capacity with paged backing storage
//!
//! # Concurrency Features
//!
//! - Thread-safe set/get/flip operations
//! - Atomic get_and_set for synchronization primitives
//! - Range setting with consistent intermediate states
//! - Safe iteration while other threads modify
//! - Compare-and-exchange for conflict resolution
//!
//! # Memory Efficiency
//!
//! - Packed bit storage (64 bits per i64 word)
//! - Paged memory layout for huge capacity
//! - Efficient word-level bulk operations
//! - Minimal atomic operation overhead

use crate::collections::HugeAtomicLongArray;
use crate::mem::BitUtil;

const NUM_BITS: usize = 64; // 64 bits per i64 word

/// Thread-safe atomic bitset supporting billions of bits.
///
/// Uses `HugeAtomicLongArray` for storage, packing 64 bits per word.
/// All operations are lock-free and thread-safe.
///
/// # Example
///
/// ```ignore
/// use rust_gds::collections::HugeAtomicBitSet;
///
/// // Bitset for tracking visited nodes in parallel BFS
/// let node_count = 1_000_000_000;
/// let visited = HugeAtomicBitSet::new(node_count);
///
/// // Thread-safe concurrent access
/// std::thread::scope(|s| {
///     for worker_id in 0..8 {
///         s.spawn(|| {
///             for node_id in (worker_id..node_count).step_by(8) {
///                 if !visited.get_and_set(node_id) {
///                     // First thread to visit this node
///                     process_node(node_id);
///                 }
///             }
///         });
///     }
/// });
/// ```
pub struct HugeAtomicBitSet {
    bits: HugeAtomicLongArray,
    num_bits: usize,
    remainder: usize,
}

impl HugeAtomicBitSet {
    /// Estimates memory usage for capacity planning.
    ///
    /// Essential for resource allocation in large-scale processing.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of bits in the bitset
    ///
    /// # Returns
    ///
    /// Estimated memory usage in bytes
    pub fn memory_estimation(size: usize) -> usize {
        let words_size = BitUtil::ceil_div(size, NUM_BITS);
        // HugeAtomicLongArray size + instance overhead
        words_size * std::mem::size_of::<i64>() + std::mem::size_of::<Self>()
    }

    /// Creates a new atomic bitset with specified size.
    ///
    /// All bits are initialized to 0 (unset).
    ///
    /// # Arguments
    ///
    /// * `size` - Number of bits in the bitset
    ///
    /// # Returns
    ///
    /// New atomic bitset instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let visited = HugeAtomicBitSet::new(1_000_000);
    /// ```
    pub fn new(size: usize) -> Self {
        let words_size = BitUtil::ceil_div(size, NUM_BITS);
        let remainder = size % NUM_BITS;
        let bits = HugeAtomicLongArray::new(words_size);

        Self {
            bits,
            num_bits: size,
            remainder,
        }
    }

    /// Returns the state of the bit at the given index.
    ///
    /// Thread-safe read operation.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index (0-based)
    ///
    /// # Returns
    ///
    /// `true` if bit is set, `false` otherwise
    ///
    /// # Performance
    ///
    /// O(1) atomic read
    ///
    /// # Concurrency
    ///
    /// Safe to call while other threads modify the bitset
    ///
    /// # Panics
    ///
    /// Panics in debug mode if index >= size
    pub fn get(&self, index: usize) -> bool {
        debug_assert!(
            index < self.num_bits,
            "Index {} out of bounds (size: {})",
            index,
            self.num_bits
        );

        let word_index = index / NUM_BITS;
        let bit_index = index % NUM_BITS;
        let bitmask = 1i64 << bit_index;

        (self.bits.get(word_index) & bitmask) != 0
    }

    /// Sets the bit at the given index to true.
    ///
    /// Thread-safe atomic operation using compare-and-swap.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to set
    ///
    /// # Performance
    ///
    /// O(1) with possible CAS retry loops under contention
    ///
    /// # Concurrency
    ///
    /// Multiple threads can safely set different or same bits
    ///
    /// # Graph Algorithm Use Cases
    ///
    /// - Mark node as visited in concurrent traversal
    /// - Set membership flags in parallel clustering
    /// - Atomic state updates in distributed algorithms
    ///
    /// # Panics
    ///
    /// Panics in debug mode if index >= size
    pub fn set(&self, index: usize) {
        debug_assert!(
            index < self.num_bits,
            "Index {} out of bounds (size: {})",
            index,
            self.num_bits
        );

        let word_index = index / NUM_BITS;
        let bit_index = index % NUM_BITS;
        let bitmask = 1i64 << bit_index;

        let mut old_word = self.bits.get(word_index);
        loop {
            let new_word = old_word | bitmask;
            if new_word == old_word {
                // Bit already set - nothing to do
                return;
            }

            let current_word = self
                .bits
                .compare_and_exchange(word_index, old_word, new_word);
            if current_word == old_word {
                // CAS successful - bit set atomically
                return;
            }

            // CAS failed - retry with current value
            old_word = current_word;
        }
    }

    /// Sets the bits from start_index (inclusive) to end_index (exclusive).
    ///
    /// Efficient bulk operation for range setting.
    ///
    /// # Arguments
    ///
    /// * `start_index` - First bit index to set (inclusive)
    /// * `end_index` - Last bit index to set (exclusive)
    ///
    /// # Optimizations
    ///
    /// - Word-aligned operations for interior words
    /// - Bit masking for partial words at boundaries
    /// - Atomic operations maintain consistency
    ///
    /// # Panics
    ///
    /// Panics in debug mode if:
    /// - start_index > end_index
    /// - end_index > size
    pub fn set_range(&self, start_index: usize, end_index: usize) {
        debug_assert!(
            start_index <= end_index,
            "Invalid range: [{}, {})",
            start_index,
            end_index
        );
        debug_assert!(
            end_index <= self.num_bits,
            "End index {} out of bounds (size: {})",
            end_index,
            self.num_bits
        );

        if start_index == end_index {
            return;
        }

        let start_word_index = start_index / NUM_BITS;
        // Since end_index is exclusive, we need the word before that index
        let end_word_index = (end_index - 1) / NUM_BITS;

        let start_bit_offset = start_index % NUM_BITS;
        let end_bit_offset = end_index % NUM_BITS;

        let start_bit_mask = (-1i64) << start_bit_offset;
        let end_bit_mask = if end_bit_offset == 0 {
            -1i64 // All bits if end aligns with word boundary
        } else {
            (1i64 << end_bit_offset) - 1 // Bits 0 to end_bit_offset-1
        };
        if start_word_index == end_word_index {
            // Set within single word
            self.set_word(start_word_index, start_bit_mask & end_bit_mask);
        } else {
            // Set within range - start word, full interior words, end word
            self.set_word(start_word_index, start_bit_mask);

            // Set all bits in interior words
            for word_index in (start_word_index + 1)..end_word_index {
                self.bits.set(word_index, -1i64); // All bits set
            }

            self.set_word(end_word_index, end_bit_mask);
        }
    }

    /// Sets a bit and returns the previous value.
    ///
    /// Atomic test-and-set operation essential for synchronization.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to set
    ///
    /// # Returns
    ///
    /// Previous value of the bit (`true` if was set, `false` if was unset)
    ///
    /// # Synchronization Use Cases
    ///
    /// - Claim exclusive access to graph nodes
    /// - Implement atomic locks on graph regions
    /// - Coordinate parallel algorithm phases
    /// - Detect first-time visitation in concurrent traversal
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Claim exclusive processing rights to a node
    /// let was_already_claimed = bitset.get_and_set(node_id);
    /// if !was_already_claimed {
    ///     // First thread to claim this node - process it
    ///     process_node_exclusively(node_id);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics in debug mode if index >= size
    pub fn get_and_set(&self, index: usize) -> bool {
        debug_assert!(
            index < self.num_bits,
            "Index {} out of bounds (size: {})",
            index,
            self.num_bits
        );

        let word_index = index / NUM_BITS;
        let bit_index = index % NUM_BITS;
        let bitmask = 1i64 << bit_index;

        let mut old_word = self.bits.get(word_index);
        loop {
            let new_word = old_word | bitmask;
            if new_word == old_word {
                // Bit was already set
                return true;
            }

            let current_word = self
                .bits
                .compare_and_exchange(word_index, old_word, new_word);
            if current_word == old_word {
                // CAS successful - we set the bit
                return false;
            }

            // CAS failed - retry
            old_word = current_word;
        }
    }

    /// Toggles the bit at the given index.
    ///
    /// Atomic flip operation using XOR.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to flip
    ///
    /// # Panics
    ///
    /// Panics in debug mode if index >= size
    pub fn flip(&self, index: usize) {
        debug_assert!(
            index < self.num_bits,
            "Index {} out of bounds (size: {})",
            index,
            self.num_bits
        );

        let word_index = index / NUM_BITS;
        let bit_index = index % NUM_BITS;
        let bitmask = 1i64 << bit_index;

        let mut old_word = self.bits.get(word_index);
        loop {
            let new_word = old_word ^ bitmask;
            let current_word = self
                .bits
                .compare_and_exchange(word_index, old_word, new_word);
            if current_word == old_word {
                // CAS successful
                return;
            }

            // CAS failed - retry
            old_word = current_word;
        }
    }

    /// Iterates the bitset in increasing order and calls the consumer for each set bit.
    ///
    /// # Warning
    ///
    /// This method is NOT thread-safe. Use only when no concurrent modifications occur.
    ///
    /// # Arguments
    ///
    /// * `consumer` - Function to call for each set bit index
    pub fn for_each_set_bit<F>(&self, mut consumer: F)
    where
        F: FnMut(usize),
    {
        let size = self.bits.size();

        for word_index in 0..size {
            let mut word = self.bits.get(word_index);
            while word != 0 {
                let next = word.trailing_zeros() as usize;
                let bit_index = NUM_BITS * word_index + next;
                consumer(bit_index);
                word ^= word & word.wrapping_neg(); // Clear lowest set bit
            }
        }
    }

    /// Returns the number of set bits in the bitset.
    ///
    /// # Warning
    ///
    /// This method is NOT thread-safe.
    ///
    /// # Returns
    ///
    /// Count of set bits
    pub fn cardinality(&self) -> usize {
        let size = self.bits.size();
        let mut set_bit_count = 0;

        for word_index in 0..size {
            set_bit_count += self.bits.get(word_index).count_ones() as usize;
        }

        set_bit_count
    }

    /// Returns `true` if no bit is set.
    ///
    /// # Warning
    ///
    /// This method is NOT thread-safe.
    pub fn is_empty(&self) -> bool {
        let size = self.bits.size();

        for word_index in 0..size {
            if self.bits.get(word_index).count_ones() > 0 {
                return false;
            }
        }

        true
    }

    /// Returns `true` if all bits are set.
    ///
    /// # Warning
    ///
    /// This method is NOT thread-safe.
    pub fn all_set(&self) -> bool {
        let size = self.bits.size();

        // Check all complete words
        for word_index in 0..(size - 1) {
            if self.bits.get(word_index).count_ones() < NUM_BITS as u32 {
                return false;
            }
        }

        // Check last (potentially partial) word
        let last_word_bit_count = self.bits.get(size - 1).count_ones() as usize;
        last_word_bit_count >= self.remainder
    }

    /// Returns the number of bits in the bitset.
    pub fn size(&self) -> usize {
        self.num_bits
    }

    /// Resets all bits in the bitset to 0.
    ///
    /// # Warning
    ///
    /// This method is NOT thread-safe.
    pub fn clear(&self) {
        self.bits.set_all(0);
    }

    /// Resets the bit at the given index to 0.
    ///
    /// Thread-safe atomic operation.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to clear
    ///
    /// # Panics
    ///
    /// Panics in debug mode if index >= size
    pub fn clear_bit(&self, index: usize) {
        debug_assert!(
            index < self.num_bits,
            "Index {} out of bounds (size: {})",
            index,
            self.num_bits
        );

        let word_index = index / NUM_BITS;
        let bit_index = index % NUM_BITS;
        let bitmask = !(1i64 << bit_index);

        let mut old_word = self.bits.get(word_index);
        loop {
            let new_word = old_word & bitmask;
            if new_word == old_word {
                // Bit already cleared
                return;
            }

            let current_word = self
                .bits
                .compare_and_exchange(word_index, old_word, new_word);
            if current_word == old_word {
                // CAS successful
                return;
            }

            // CAS failed - retry
            old_word = current_word;
        }
    }

    /// Atomic word-level bit setting with OR operation.
    fn set_word(&self, word_index: usize, bit_mask: i64) {
        let mut old_word = self.bits.get(word_index);
        loop {
            let new_word = old_word | bit_mask;
            if new_word == old_word {
                // Already set
                return;
            }

            let current_word = self
                .bits
                .compare_and_exchange(word_index, old_word, new_word);
            if current_word == old_word {
                // CAS successful
                return;
            }

            old_word = current_word;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bitset() {
        let bitset = HugeAtomicBitSet::new(100);
        assert_eq!(bitset.size(), 100);
        assert!(bitset.is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let bitset = HugeAtomicBitSet::new(100);

        assert!(!bitset.get(0));
        bitset.set(0);
        assert!(bitset.get(0));

        assert!(!bitset.get(63));
        bitset.set(63);
        assert!(bitset.get(63));

        assert!(!bitset.get(64)); // Next word
        bitset.set(64);
        assert!(bitset.get(64));
    }

    #[test]
    fn test_get_and_set() {
        let bitset = HugeAtomicBitSet::new(100);

        assert!(!bitset.get_and_set(5));
        assert!(bitset.get_and_set(5));
        assert!(bitset.get(5));
    }

    #[test]
    fn test_clear_bit() {
        let bitset = HugeAtomicBitSet::new(100);

        bitset.set(10);
        assert!(bitset.get(10));

        bitset.clear_bit(10);
        assert!(!bitset.get(10));
    }

    #[test]
    fn test_flip() {
        let bitset = HugeAtomicBitSet::new(100);

        assert!(!bitset.get(5));
        bitset.flip(5);
        assert!(bitset.get(5));
        bitset.flip(5);
        assert!(!bitset.get(5));
    }

    #[test]
    fn test_set_range_single_word() {
        let bitset = HugeAtomicBitSet::new(100);

        bitset.set_range(10, 20);

        for i in 0..10 {
            assert!(!bitset.get(i));
        }
        for i in 10..20 {
            assert!(bitset.get(i));
        }
        for i in 20..100 {
            assert!(!bitset.get(i));
        }
    }

    #[test]
    fn test_set_range_multiple_words() {
        let bitset = HugeAtomicBitSet::new(200);

        bitset.set_range(50, 150);

        for i in 0..50 {
            assert!(!bitset.get(i));
        }
        for i in 50..150 {
            assert!(bitset.get(i));
        }
        for i in 150..200 {
            assert!(!bitset.get(i));
        }
    }

    #[test]
    fn test_cardinality() {
        let bitset = HugeAtomicBitSet::new(100);

        assert_eq!(bitset.cardinality(), 0);

        bitset.set(0);
        bitset.set(1);
        bitset.set(50);

        assert_eq!(bitset.cardinality(), 3);
    }

    #[test]
    fn test_is_empty() {
        let bitset = HugeAtomicBitSet::new(100);

        assert!(bitset.is_empty());

        bitset.set(5);
        assert!(!bitset.is_empty());

        bitset.clear_bit(5);
        assert!(bitset.is_empty());
    }

    #[test]
    fn test_all_set() {
        let bitset = HugeAtomicBitSet::new(10);

        assert!(!bitset.all_set());

        bitset.set_range(0, 10);
        assert!(bitset.all_set());
    }

    #[test]
    fn test_clear() {
        let bitset = HugeAtomicBitSet::new(100);

        bitset.set(10);
        bitset.set(20);
        bitset.set(30);

        assert_eq!(bitset.cardinality(), 3);

        bitset.clear();

        assert_eq!(bitset.cardinality(), 0);
        assert!(bitset.is_empty());
    }

    #[test]
    fn test_for_each_set_bit() {
        let bitset = HugeAtomicBitSet::new(100);

        bitset.set(5);
        bitset.set(15);
        bitset.set(25);
        bitset.set(70);

        let mut collected = Vec::new();
        bitset.for_each_set_bit(|index| {
            collected.push(index);
        });

        assert_eq!(collected, vec![5, 15, 25, 70]);
    }

    #[test]
    fn test_large_bitset() {
        let bitset = HugeAtomicBitSet::new(1_000_000);

        bitset.set(0);
        bitset.set(999_999);
        bitset.set(500_000);

        assert!(bitset.get(0));
        assert!(bitset.get(999_999));
        assert!(bitset.get(500_000));
        assert!(!bitset.get(1));

        assert_eq!(bitset.cardinality(), 3);
    }

    #[test]
    fn test_memory_estimation() {
        let size = 1_000_000;
        let estimated = HugeAtomicBitSet::memory_estimation(size);

        // Should be approximately size/64 * 8 bytes (for i64 words) + overhead
        let words = BitUtil::ceil_div(size, 64);
        let min_expected = words * 8;

        assert!(estimated >= min_expected);
    }

    #[test]
    fn test_concurrent_set() {
        use std::sync::Arc;

        let bitset = Arc::new(HugeAtomicBitSet::new(1000));

        std::thread::scope(|s| {
            for thread_id in 0..4 {
                let bitset = Arc::clone(&bitset);
                s.spawn(move || {
                    for i in (thread_id..1000).step_by(4) {
                        bitset.set(i);
                    }
                });
            }
        });

        // All bits should be set
        assert_eq!(bitset.cardinality(), 1000);
    }

    #[test]
    fn test_concurrent_get_and_set() {
        use std::sync::Arc;

        let bitset = Arc::new(HugeAtomicBitSet::new(100));
        let counters = std::sync::Arc::new(std::sync::Mutex::new(vec![0; 100]));

        std::thread::scope(|s| {
            for _ in 0..4 {
                let bitset = Arc::clone(&bitset);
                let counters_clone = counters.clone();
                s.spawn(move || {
                    for i in 0..100 {
                        if !bitset.get_and_set(i) {
                            // First thread to set this bit
                            let mut locked = counters_clone.lock().unwrap();
                            locked[i] += 1;
                        }
                    }
                });
            }
        });

        // Each bit should have been claimed by exactly one thread
        let locked = counters.lock().unwrap();
        for count in locked.iter() {
            assert_eq!(*count, 1);
        }
    }
}

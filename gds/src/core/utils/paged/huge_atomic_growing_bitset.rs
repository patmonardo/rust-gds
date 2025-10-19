//! Self-growing thread-safe atomic bitset for dynamic billion-scale processing.
//!
//! Essential for algorithms with unknown or dynamic memory requirements:
//! - Streaming graph processing with unknown node counts
//! - Dynamic visited tracking that grows with exploration
//! - Online algorithms that discover new entities
//! - Concurrent data structure building with unknown final size
//! - Real-time graph analysis with expanding datasets
//!
//! # Performance Characteristics
//!
//! - Atomic operations using compare-and-swap (CAS)
//! - Lock-free growth through atomic page array updates
//! - Efficient bit operations with 64-bit word alignment
//! - Page-based allocation reduces memory waste
//!
//! # Dynamic Growth Features
//!
//! - Thread-safe capacity expansion during runtime
//! - Atomic page allocation race resolution
//! - Zero-copy page transfer during growth
//! - Minimal synchronization overhead
//! - Predictable memory allocation patterns
//!
//! # Example
//!
//! ```rust
//! use gds::core::utils::paged::HugeAtomicGrowingBitSet;
//! use std::sync::Arc;
//!
//! // Start small - will grow as needed
//! let bitset = Arc::new(HugeAtomicGrowingBitSet::create(1000));
//!
//! // Set bits way beyond initial capacity - automatic growth!
//! std::thread::scope(|s| {
//!     for thread_id in 0..4 {
//!         let bitset = Arc::clone(&bitset);
//!         s.spawn(move || {
//!             let start = thread_id * 500_000;
//!             let end = start + 500_000;
//!             for i in start..end {
//!                 if !bitset.get_and_set(i) {
//!                     // First visit - process this node
//!                 }
//!             }
//!         });
//!     }
//! });
//!
//! println!("Final capacity: {} bits", bitset.capacity());
//! println!("Set bits: {}", bitset.cardinality());
//! ```

use crate::collections::page_util::PageUtil;
use crate::mem::bit_util::BitUtil;
use std::sync::atomic::{AtomicI64, AtomicPtr, Ordering};
use std::sync::Arc;

/// Self-growing thread-safe atomic bitset.
///
/// Automatically expands capacity when bits are set beyond current size.
/// All operations are thread-safe and lock-free.
pub struct HugeAtomicGrowingBitSet {
    /// Each page stores 2^PAGE_SHIFT_BITS entries (bits).
    /// Word-size is 64 bit, so we store 2^(PAGE_SHIFT_BITS - 6) words per page.
    page_size: usize,
    page_shift: u32,
    page_mask: usize,
    /// Atomic reference to pages - enables thread-safe growth
    pages: AtomicPtr<Pages>,
}

/// Number of bits per word (i64).
const NUM_BITS: usize = 64;
const BIT_MASK: usize = NUM_BITS - 1;

/// Page shift for bit addressing (2^16 bits per page = 64K bits = 1K i64 words).
const PAGE_SHIFT_BITS: usize = 16;

impl HugeAtomicGrowingBitSet {
    /// Creates a growing atomic bitset with initial capacity.
    ///
    /// # Arguments
    ///
    /// * `bit_size` - Initial number of bits to support
    ///
    /// # Returns
    ///
    /// New growing atomic bitset instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use gds::core::utils::paged::HugeAtomicGrowingBitSet;
    ///
    /// let bitset = HugeAtomicGrowingBitSet::create(1_000_000);
    /// bitset.set(999_999);
    /// bitset.set(5_000_000); // Auto-grows!
    /// assert!(bitset.get(5_000_000));
    /// ```
    pub fn create(bit_size: usize) -> Self {
        // Number of words (i64) required to represent the bit size
        let word_size = BitUtil::ceil_div(bit_size, NUM_BITS);

        // Parameters for pages of i64 words representing the bits
        let page_shift = (PAGE_SHIFT_BITS - 6) as u32; // 2^6 == 64 bits per i64
        let page_size = 1 << page_shift;
        let page_mask = page_size - 1;

        // Calculate initial page count
        let page_count = PageUtil::num_pages_for_shift(word_size, page_shift, page_mask);

        let pages = Box::into_raw(Box::new(Pages::new(page_count, page_size)));

        Self {
            page_size,
            page_shift,
            page_mask,
            pages: AtomicPtr::new(pages),
        }
    }

    /// Sets the bit at the given index to true.
    /// Automatically grows capacity if index exceeds current size.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to set
    ///
    /// # Thread Safety
    ///
    /// Multiple threads can safely set different or same bits.
    /// Growth is atomic and race-free.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gds::core::utils::paged::HugeAtomicGrowingBitSet;
    /// use std::sync::Arc;
    ///
    /// let bitset = Arc::new(HugeAtomicGrowingBitSet::create(100));
    ///
    /// std::thread::scope(|s| {
    ///     for i in 0..4 {
    ///         let bitset = Arc::clone(&bitset);
    ///         s.spawn(move || {
    ///             bitset.set(i * 1000); // Each thread sets different bit
    ///         });
    ///     }
    /// });
    /// ```
    pub fn set(&self, index: usize) {
        let long_index = index >> 6; // Divide by 64
        let page_index = PageUtil::page_index(long_index, self.page_shift);
        let word_index = PageUtil::index_in_page(long_index, self.page_mask);
        let bit_index = index & BIT_MASK;

        let page = self.get_page(page_index);
        let bitmask = 1i64 << bit_index;

        let mut old_word = page.get(word_index);
        loop {
            let new_word = old_word | bitmask;
            if new_word == old_word {
                // Bit already set
                return;
            }

            match page.compare_exchange(word_index, old_word, new_word) {
                Ok(_) => return,                    // CAS successful
                Err(current) => old_word = current, // CAS failed, retry
            }
        }
    }

    /// Returns the state of the bit at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to check
    ///
    /// # Returns
    ///
    /// `true` if bit is set, `false` otherwise
    pub fn get(&self, index: usize) -> bool {
        let long_index = index >> 6;
        let page_index = PageUtil::page_index(long_index, self.page_shift);
        let word_index = PageUtil::index_in_page(long_index, self.page_mask);
        let bit_index = index & BIT_MASK;

        let page = self.get_page(page_index);
        let bitmask = 1i64 << bit_index;

        (page.get(word_index) & bitmask) != 0
    }

    /// Sets a bit and returns the previous value.
    /// Essential for atomic test-and-set synchronization patterns.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to set
    ///
    /// # Returns
    ///
    /// Previous value of the bit (`true` if was set, `false` if was clear)
    ///
    /// # Synchronization Use Cases
    ///
    /// - Claim exclusive access to dynamic resources
    /// - First-time discovery in streaming algorithms
    /// - Atomic reservation in growing data structures
    ///
    /// # Example
    ///
    /// ```rust
    /// use gds::core::utils::paged::HugeAtomicGrowingBitSet;
    /// use std::sync::Arc;
    ///
    /// let bitset = Arc::new(HugeAtomicGrowingBitSet::create(1000));
    ///
    /// std::thread::scope(|s| {
    ///     for _ in 0..4 {
    ///         let bitset = Arc::clone(&bitset);
    ///         s.spawn(move || {
    ///             for i in 0..100 {
    ///                 if !bitset.get_and_set(i) {
    ///                     // First thread to set this bit - process it
    ///                 }
    ///             }
    ///         });
    ///     }
    /// });
    /// ```
    pub fn get_and_set(&self, index: usize) -> bool {
        let long_index = index >> 6;
        let page_index = PageUtil::page_index(long_index, self.page_shift);
        let word_index = PageUtil::index_in_page(long_index, self.page_mask);
        let bit_index = index & BIT_MASK;

        let page = self.get_page(page_index);
        let bitmask = 1i64 << bit_index;

        let mut old_word = page.get(word_index);
        loop {
            let new_word = old_word | bitmask;
            if new_word == old_word {
                // Bit was already set
                return true;
            }

            match page.compare_exchange(word_index, old_word, new_word) {
                Ok(_) => return false,              // CAS successful - we set the bit
                Err(current) => old_word = current, // CAS failed, retry
            }
        }
    }

    /// Returns the number of set bits in the bitset.
    ///
    /// # Warning
    ///
    /// Result may not include effects of concurrent writes.
    ///
    /// # Returns
    ///
    /// Count of set bits
    pub fn cardinality(&self) -> usize {
        let pages = unsafe { &*self.pages.load(Ordering::Acquire) };
        let page_count = pages.length();
        let page_size = self.page_size;

        let mut set_bit_count = 0;

        for page_index in 0..page_count {
            let page = pages.get_page(page_index);
            for word_index in 0..page_size {
                set_bit_count += page.get(word_index).count_ones() as usize;
            }
        }

        set_bit_count
    }

    /// Iterates the bitset and calls consumer for each set bit.
    ///
    /// # Warning
    ///
    /// May not include effects of concurrent writes during iteration.
    ///
    /// # Arguments
    ///
    /// * `consumer` - Function to call for each set bit index
    pub fn for_each_set_bit<F>(&self, mut consumer: F)
    where
        F: FnMut(usize),
    {
        let pages = unsafe { &*self.pages.load(Ordering::Acquire) };
        let page_count = pages.length();
        let page_size = self.page_size;

        let mut base = 0;

        for page_index in 0..page_count {
            let page = pages.get_page(page_index);
            for word_index in 0..page_size {
                let mut word = page.get(word_index);

                while word != 0 {
                    let next = word.trailing_zeros() as usize;
                    let bit_index = NUM_BITS * (base + word_index) + next;
                    consumer(bit_index);
                    word ^= word & word.wrapping_neg(); // Clear lowest set bit
                }
            }
            base += page_size;
        }
    }

    /// Resets the bit at the given index.
    /// Thread-safe atomic clear operation.
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index to clear
    pub fn clear(&self, index: usize) {
        let long_index = index >> 6;
        let page_index = PageUtil::page_index(long_index, self.page_shift);
        let word_index = PageUtil::index_in_page(long_index, self.page_mask);
        let bit_index = index & BIT_MASK;

        let page = self.get_page(page_index);
        let bitmask = !(1i64 << bit_index);

        let mut old_word = page.get(word_index);
        loop {
            let new_word = old_word & bitmask;
            if new_word == old_word {
                // Bit already cleared
                return;
            }

            match page.compare_exchange(word_index, old_word, new_word) {
                Ok(_) => return,                    // CAS successful
                Err(current) => old_word = current, // CAS failed, retry
            }
        }
    }

    /// Returns the current capacity of the bitset.
    /// Setting a bit beyond this capacity triggers automatic growth.
    ///
    /// # Returns
    ///
    /// Current bit capacity
    pub fn capacity(&self) -> usize {
        let pages = unsafe { &*self.pages.load(Ordering::Acquire) };
        pages.length() * (1 << self.page_shift) * NUM_BITS
    }

    /// Returns the page at the given index, growing the structure if necessary.
    /// Thread-safe growth through atomic page array updates.
    ///
    /// # Arguments
    ///
    /// * `page_index` - Page index to retrieve
    ///
    /// # Returns
    ///
    /// Reference to the atomic page at the specified index
    fn get_page(&self, page_index: usize) -> &AtomicPage {
        let mut pages_ptr = self.pages.load(Ordering::Acquire);
        let mut pages = unsafe { &*pages_ptr };

        while pages.length() <= page_index {
            // Need to grow the number of pages to fit the requested index
            // Loop handles race conditions where multiple threads try to grow
            let new_pages = Box::into_raw(Box::new(Pages::from_existing(
                pages,
                page_index + 1,
                self.page_size,
            )));

            // Atomic update - only one thread succeeds
            match self.pages.compare_exchange(
                pages_ptr,
                new_pages,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    // Success - we updated the pages reference
                    // The old pages pointer will be leaked intentionally
                    // (we can't safely drop it while other threads may still be using it)
                    pages_ptr = new_pages;
                    pages = unsafe { &*pages_ptr };
                }
                Err(current) => {
                    // Another thread won the race - use their pages
                    // Drop our speculative allocation
                    unsafe {
                        let _ = Box::from_raw(new_pages);
                    }
                    pages_ptr = current;
                    pages = unsafe { &*pages_ptr };
                }
            }
        }

        pages.get_page(page_index)
    }
}

impl Drop for HugeAtomicGrowingBitSet {
    fn drop(&mut self) {
        let pages_ptr = self.pages.load(Ordering::Acquire);
        unsafe {
            let _ = Box::from_raw(pages_ptr);
        }
    }
}

// Safety: HugeAtomicGrowingBitSet uses atomic operations for all shared state
unsafe impl Send for HugeAtomicGrowingBitSet {}
unsafe impl Sync for HugeAtomicGrowingBitSet {}

/// Atomic page - a fixed-size array of atomic i64 words.
struct AtomicPage {
    words: Vec<AtomicI64>,
}

impl AtomicPage {
    fn new(size: usize) -> Self {
        let mut words = Vec::with_capacity(size);
        for _ in 0..size {
            words.push(AtomicI64::new(0));
        }
        Self { words }
    }

    fn get(&self, index: usize) -> i64 {
        self.words[index].load(Ordering::Acquire)
    }

    fn compare_exchange(&self, index: usize, expected: i64, update: i64) -> Result<i64, i64> {
        self.words[index].compare_exchange(expected, update, Ordering::AcqRel, Ordering::Acquire)
    }
}

/// Container for atomic page arrays with thread-safe growth.
struct Pages {
    pages: Vec<Arc<AtomicPage>>,
}

impl Pages {
    fn new(page_count: usize, page_size: usize) -> Self {
        let mut pages = Vec::with_capacity(page_count);
        for _ in 0..page_count {
            pages.push(Arc::new(AtomicPage::new(page_size)));
        }
        Self { pages }
    }

    /// Creates a new Pages by copying existing pages and adding new ones.
    fn from_existing(old_pages: &Pages, new_page_count: usize, page_size: usize) -> Self {
        let mut pages = Vec::with_capacity(new_page_count);

        // Transfer existing pages (Arc clone is cheap)
        for page in &old_pages.pages {
            pages.push(Arc::clone(page));
        }

        // Create new pages for the remaining slots
        for _ in old_pages.pages.len()..new_page_count {
            pages.push(Arc::new(AtomicPage::new(page_size)));
        }

        Self { pages }
    }

    fn get_page(&self, page_index: usize) -> &AtomicPage {
        &self.pages[page_index]
    }

    fn length(&self) -> usize {
        self.pages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_create() {
        let bitset = HugeAtomicGrowingBitSet::create(1000);
        assert!(bitset.capacity() >= 1000);
    }

    #[test]
    fn test_set_and_get() {
        let bitset = HugeAtomicGrowingBitSet::create(100);

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
        let bitset = HugeAtomicGrowingBitSet::create(100);

        assert!(!bitset.get_and_set(5));
        assert!(bitset.get_and_set(5));
        assert!(bitset.get(5));
    }

    #[test]
    fn test_clear() {
        let bitset = HugeAtomicGrowingBitSet::create(100);

        bitset.set(10);
        assert!(bitset.get(10));

        bitset.clear(10);
        assert!(!bitset.get(10));
    }

    #[test]
    fn test_cardinality() {
        let bitset = HugeAtomicGrowingBitSet::create(100);

        assert_eq!(bitset.cardinality(), 0);

        bitset.set(0);
        bitset.set(1);
        bitset.set(50);

        assert_eq!(bitset.cardinality(), 3);
    }

    #[test]
    fn test_for_each_set_bit() {
        let bitset = HugeAtomicGrowingBitSet::create(100);

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
    fn test_automatic_growth() {
        let bitset = HugeAtomicGrowingBitSet::create(100);
        let initial_capacity = bitset.capacity();

        // Set bit way beyond initial capacity
        let far_index = initial_capacity + 10_000;
        bitset.set(far_index);

        assert!(bitset.get(far_index));
        assert!(bitset.capacity() > initial_capacity);
    }

    #[test]
    fn test_concurrent_set() {
        let bitset = Arc::new(HugeAtomicGrowingBitSet::create(1000));

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
        let bitset = Arc::new(HugeAtomicGrowingBitSet::create(100));
        let counters = Arc::new(std::sync::Mutex::new(vec![0; 100]));

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

    #[test]
    fn test_concurrent_growth() {
        let bitset = Arc::new(HugeAtomicGrowingBitSet::create(100));

        std::thread::scope(|s| {
            for thread_id in 0..4 {
                let bitset = Arc::clone(&bitset);
                s.spawn(move || {
                    // Each thread sets bits in different ranges, causing growth
                    let start = thread_id * 100_000;
                    for i in start..(start + 100) {
                        bitset.set(i);
                    }
                });
            }
        });

        // Verify all bits are set
        for thread_id in 0..4 {
            let start = thread_id * 100_000;
            for i in start..(start + 100) {
                assert!(bitset.get(i), "Bit {} should be set", i);
            }
        }
    }

    #[test]
    fn test_large_growing_bitset() {
        let bitset = HugeAtomicGrowingBitSet::create(1000);

        bitset.set(0);
        bitset.set(1_000_000); // Triggers growth
        bitset.set(500_000);

        assert!(bitset.get(0));
        assert!(bitset.get(1_000_000));
        assert!(bitset.get(500_000));
        assert!(!bitset.get(1));

        assert_eq!(bitset.cardinality(), 3);
    }
}

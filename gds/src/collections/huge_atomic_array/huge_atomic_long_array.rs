/// A huge atomic array of i64 integers supporting lock-free concurrent operations.
///
/// This array provides thread-safe atomic operations on massive datasets that can
/// exceed standard array size limits. Built on Rust's `AtomicI64`, it offers true
/// lock-free operations with strong memory ordering guarantees.
///
/// # Thread Safety
///
/// All operations are atomic and thread-safe. Multiple threads can safely read,
/// write, and perform read-modify-write operations concurrently.
///
/// # Design
///
/// Uses the same paged architecture as `HugeLongArray` but with atomic storage:
/// - Single array for ≤268M elements using `Vec<AtomicI64>`
/// - Paged architecture for larger arrays
/// - Automatic selection based on `MAX_ARRAY_LENGTH`
///
/// # Performance
///
/// - Atomic operations: O(1) with minimal contention overhead
/// - Memory ordering: Uses `Ordering::SeqCst` for strongest guarantees
/// - Cache-friendly: Paged layout optimized for sequential access
///
/// # Examples
///
/// ```
/// use gds::collections::HugeAtomicLongArray;
///
/// // Create atomic array
/// let array = HugeAtomicLongArray::new(1000);
///
/// // Atomic operations
/// array.set(0, 42);
/// let old_value = array.get_and_add(0, 10); // old_value = 42, now 52
/// let success = array.compare_and_set(0, 52, 100); // success = true
///
/// // Atomic update with function
/// array.update(0, |current| current * 2); // now 200
/// ```
use crate::collections::PageUtil;
use std::sync::atomic::{AtomicI64, Ordering};

const MAX_ARRAY_LENGTH: usize = 1 << 28; // 268,435,456 elements
const PAGE_SIZE_IN_BYTES: usize = 4096; // 4KB pages

/// Huge atomic array supporting >2 billion i64 elements with lock-free operations.
pub enum HugeAtomicLongArray {
    Single(SingleHugeAtomicLongArray),
    Paged(PagedHugeAtomicLongArray),
}

impl HugeAtomicLongArray {
    /// Creates a new huge atomic long array of the specified size.
    ///
    /// All elements are initialized to 0.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of elements in the array
    ///
    /// # Returns
    ///
    /// A new `HugeAtomicLongArray` optimized for the given size
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            HugeAtomicLongArray::Single(SingleHugeAtomicLongArray::new(size))
        } else {
            HugeAtomicLongArray::Paged(PagedHugeAtomicLongArray::new(size))
        }
    }

    /// Atomically gets the value at the specified index.
    ///
    /// Uses `Ordering::SeqCst` for strongest consistency guarantees.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to read from
    ///
    /// # Returns
    ///
    /// The current value at the index
    pub fn get(&self, index: usize) -> i64 {
        match self {
            HugeAtomicLongArray::Single(s) => s.get(index),
            HugeAtomicLongArray::Paged(p) => p.get(index),
        }
    }

    /// Atomically sets the value at the specified index.
    ///
    /// Uses `Ordering::SeqCst` for strongest consistency guarantees.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to write to
    /// * `value` - Value to store
    pub fn set(&self, index: usize, value: i64) {
        match self {
            HugeAtomicLongArray::Single(s) => s.set(index, value),
            HugeAtomicLongArray::Paged(p) => p.set(index, value),
        }
    }

    /// Atomically adds delta to the value at index and returns the previous value.
    ///
    /// This is the fundamental atomic accumulation operation for concurrent algorithms.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to modify
    /// * `delta` - Value to add (can be negative)
    ///
    /// # Returns
    ///
    /// The value before the addition
    pub fn get_and_add(&self, index: usize, delta: i64) -> i64 {
        match self {
            HugeAtomicLongArray::Single(s) => s.get_and_add(index, delta),
            HugeAtomicLongArray::Paged(p) => p.get_and_add(index, delta),
        }
    }

    /// Atomically replaces the value at index and returns the previous value.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to modify
    /// * `value` - New value to store
    ///
    /// # Returns
    ///
    /// The value before replacement
    pub fn get_and_replace(&self, index: usize, value: i64) -> i64 {
        match self {
            HugeAtomicLongArray::Single(s) => s.get_and_replace(index, value),
            HugeAtomicLongArray::Paged(p) => p.get_and_replace(index, value),
        }
    }

    /// Atomically updates the value if it equals the expected value.
    ///
    /// This is the fundamental compare-and-swap operation for lock-free algorithms.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to update
    /// * `expect` - Expected current value
    /// * `update` - New value to set if expectation is met
    ///
    /// # Returns
    ///
    /// `true` if successful, `false` if actual value didn't match expected
    pub fn compare_and_set(&self, index: usize, expect: i64, update: i64) -> bool {
        match self {
            HugeAtomicLongArray::Single(s) => s.compare_and_set(index, expect, update),
            HugeAtomicLongArray::Paged(p) => p.compare_and_set(index, expect, update),
        }
    }

    /// Atomically updates the value if it equals expected, returning the witness value.
    ///
    /// This is optimized for CAS loops as it returns the actual current value on failure,
    /// eliminating the need for an additional read operation.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to update
    /// * `expect` - Expected current value
    /// * `update` - New value to set if expectation is met
    ///
    /// # Returns
    ///
    /// The witness value (equals `expect` if successful, or current value if not)
    pub fn compare_and_exchange(&self, index: usize, expect: i64, update: i64) -> i64 {
        match self {
            HugeAtomicLongArray::Single(s) => s.compare_and_exchange(index, expect, update),
            HugeAtomicLongArray::Paged(p) => p.compare_and_exchange(index, expect, update),
        }
    }

    /// Atomically updates the value using a transformation function.
    ///
    /// The function is applied in a CAS retry loop until the update succeeds.
    /// The function should be pure (side-effect-free) as it may be called multiple times.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to update
    /// * `f` - Pure function to transform the current value
    pub fn update<F>(&self, index: usize, f: F)
    where
        F: Fn(i64) -> i64,
    {
        match self {
            HugeAtomicLongArray::Single(s) => s.update(index, f),
            HugeAtomicLongArray::Paged(p) => p.update(index, f),
        }
    }

    /// Returns the total number of elements in the array.
    pub fn size(&self) -> usize {
        match self {
            HugeAtomicLongArray::Single(s) => s.size(),
            HugeAtomicLongArray::Paged(p) => p.size(),
        }
    }

    /// Returns the memory used by this array in bytes.
    pub fn size_of(&self) -> usize {
        match self {
            HugeAtomicLongArray::Single(s) => s.size_of(),
            HugeAtomicLongArray::Paged(p) => p.size_of(),
        }
    }

    /// Sets all elements to the specified value (not atomic - for initialization only).
    ///
    /// # Warning
    ///
    /// This operation is NOT thread-safe and should only be called when no other
    /// threads are accessing the array.
    pub fn set_all(&self, value: i64) {
        match self {
            HugeAtomicLongArray::Single(s) => s.set_all(value),
            HugeAtomicLongArray::Paged(p) => p.set_all(value),
        }
    }
}

/// Single-page implementation for atomic long arrays ≤268M elements.
pub struct SingleHugeAtomicLongArray {
    size: usize,
    storage: Vec<AtomicI64>,
}

impl SingleHugeAtomicLongArray {
    fn new(size: usize) -> Self {
        let mut storage = Vec::with_capacity(size);
        for _ in 0..size {
            storage.push(AtomicI64::new(0));
        }
        Self { size, storage }
    }

    fn get(&self, index: usize) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index].load(Ordering::SeqCst)
    }

    fn set(&self, index: usize, value: i64) {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index].store(value, Ordering::SeqCst);
    }

    fn get_and_add(&self, index: usize, delta: i64) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index].fetch_add(delta, Ordering::SeqCst)
    }

    fn get_and_replace(&self, index: usize, value: i64) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index].swap(value, Ordering::SeqCst)
    }

    fn compare_and_set(&self, index: usize, expect: i64, update: i64) -> bool {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index]
            .compare_exchange(expect, update, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    fn compare_and_exchange(&self, index: usize, expect: i64, update: i64) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        match self.storage[index].compare_exchange(
            expect,
            update,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(val) => val,
            Err(val) => val,
        }
    }

    fn update<F>(&self, index: usize, f: F)
    where
        F: Fn(i64) -> i64,
    {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let mut old_value = self.storage[index].load(Ordering::SeqCst);
        loop {
            let new_value = f(old_value);
            match self.storage[index].compare_exchange(
                old_value,
                new_value,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(witness) => old_value = witness,
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_of(&self) -> usize {
        std::mem::size_of::<Self>() + self.size * std::mem::size_of::<AtomicI64>()
    }

    fn set_all(&self, value: i64) {
        for atom in &self.storage {
            atom.store(value, Ordering::SeqCst);
        }
    }
}

/// Paged implementation for atomic long arrays >268M elements.
pub struct PagedHugeAtomicLongArray {
    size: usize,
    pages: Vec<Vec<AtomicI64>>,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeAtomicLongArray {
    fn new(size: usize) -> Self {
        let element_size = std::mem::size_of::<AtomicI64>();
        let page_size = PageUtil::page_size_for(PAGE_SIZE_IN_BYTES, element_size);
        let page_shift = page_size.trailing_zeros();
        let page_mask = page_size - 1;
        let num_pages = PageUtil::num_pages_for(size, page_size);

        let mut pages = Vec::with_capacity(num_pages);

        // Create full pages
        for _ in 0..(num_pages - 1) {
            let mut page = Vec::with_capacity(page_size);
            for _ in 0..page_size {
                page.push(AtomicI64::new(0));
            }
            pages.push(page);
        }

        // Create last (potentially partial) page
        let last_page_size = PageUtil::exclusive_index_of_page(size, page_mask);
        let mut last_page = Vec::with_capacity(last_page_size);
        for _ in 0..last_page_size {
            last_page.push(AtomicI64::new(0));
        }
        pages.push(last_page);

        Self {
            size,
            pages,
            page_shift,
            page_mask,
        }
    }

    fn get(&self, index: usize) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page].load(Ordering::SeqCst)
    }

    fn set(&self, index: usize, value: i64) {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page].store(value, Ordering::SeqCst);
    }

    fn get_and_add(&self, index: usize, delta: i64) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page].fetch_add(delta, Ordering::SeqCst)
    }

    fn get_and_replace(&self, index: usize, value: i64) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page].swap(value, Ordering::SeqCst)
    }

    fn compare_and_set(&self, index: usize, expect: i64, update: i64) -> bool {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page]
            .compare_exchange(expect, update, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    fn compare_and_exchange(&self, index: usize, expect: i64, update: i64) -> i64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        match self.pages[page_index][index_in_page].compare_exchange(
            expect,
            update,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(val) => val,
            Err(val) => val,
        }
    }

    fn update<F>(&self, index: usize, f: F)
    where
        F: Fn(i64) -> i64,
    {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        let atom = &self.pages[page_index][index_in_page];

        let mut old_value = atom.load(Ordering::SeqCst);
        loop {
            let new_value = f(old_value);
            match atom.compare_exchange(old_value, new_value, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(witness) => old_value = witness,
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_of(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();
        for page in &self.pages {
            total += page.len() * std::mem::size_of::<AtomicI64>();
        }
        total
    }

    fn set_all(&self, value: i64) {
        for page in &self.pages {
            for atom in page {
                atom.store(value, Ordering::SeqCst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_small() {
        let array = HugeAtomicLongArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(99), 0);
    }

    #[test]
    fn test_get_set() {
        let array = HugeAtomicLongArray::new(100);
        array.set(0, 42);
        array.set(50, -100);
        array.set(99, 999);

        assert_eq!(array.get(0), 42);
        assert_eq!(array.get(50), -100);
        assert_eq!(array.get(99), 999);
    }

    #[test]
    fn test_get_and_add() {
        let array = HugeAtomicLongArray::new(100);
        array.set(0, 10);

        let old_val = array.get_and_add(0, 5);
        assert_eq!(old_val, 10);
        assert_eq!(array.get(0), 15);

        let old_val2 = array.get_and_add(0, -3);
        assert_eq!(old_val2, 15);
        assert_eq!(array.get(0), 12);
    }

    #[test]
    fn test_get_and_replace() {
        let array = HugeAtomicLongArray::new(100);
        array.set(0, 42);

        let old_val = array.get_and_replace(0, 100);
        assert_eq!(old_val, 42);
        assert_eq!(array.get(0), 100);
    }

    #[test]
    fn test_compare_and_set() {
        let array = HugeAtomicLongArray::new(100);
        array.set(0, 42);

        // Successful CAS
        let success = array.compare_and_set(0, 42, 100);
        assert!(success);
        assert_eq!(array.get(0), 100);

        // Failed CAS
        let failure = array.compare_and_set(0, 42, 200);
        assert!(!failure);
        assert_eq!(array.get(0), 100);
    }

    #[test]
    fn test_compare_and_exchange() {
        let array = HugeAtomicLongArray::new(100);
        array.set(0, 42);

        // Successful exchange
        let witness = array.compare_and_exchange(0, 42, 100);
        assert_eq!(witness, 42);
        assert_eq!(array.get(0), 100);

        // Failed exchange - returns current value
        let witness2 = array.compare_and_exchange(0, 42, 200);
        assert_eq!(witness2, 100); // Returns actual value, not expected
        assert_eq!(array.get(0), 100); // Value unchanged
    }

    #[test]
    fn test_update() {
        let array = HugeAtomicLongArray::new(100);
        array.set(0, 10);

        // Atomic multiply by 2
        array.update(0, |x| x * 2);
        assert_eq!(array.get(0), 20);

        // Atomic max update
        array.update(0, |x| x.max(100));
        assert_eq!(array.get(0), 100);

        array.update(0, |x| x.max(50));
        assert_eq!(array.get(0), 100); // Unchanged
    }

    #[test]
    fn test_set_all() {
        let array = HugeAtomicLongArray::new(100);
        array.set_all(42);

        for i in 0..100 {
            assert_eq!(array.get(i), 42);
        }
    }

    #[test]
    fn test_paged_array() {
        let size = MAX_ARRAY_LENGTH + 1000;
        let array = HugeAtomicLongArray::new(size);

        assert_eq!(array.size(), size);

        // Test across page boundaries
        array.set(MAX_ARRAY_LENGTH - 1, 111);
        array.set(MAX_ARRAY_LENGTH, 222);
        array.set(MAX_ARRAY_LENGTH + 1, 333);

        assert_eq!(array.get(MAX_ARRAY_LENGTH - 1), 111);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 222);
        assert_eq!(array.get(MAX_ARRAY_LENGTH + 1), 333);

        // Test atomic operations on paged array
        let old = array.get_and_add(MAX_ARRAY_LENGTH, 100);
        assert_eq!(old, 222);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 322);
    }

    #[test]
    fn test_concurrent_increments() {
        use std::sync::Arc;
        use std::thread;

        let array = Arc::new(HugeAtomicLongArray::new(10));
        let num_threads = 4;
        let increments_per_thread = 1000;

        let mut handles = vec![];

        for _ in 0..num_threads {
            let array_clone = Arc::clone(&array);
            let handle = thread::spawn(move || {
                for _ in 0..increments_per_thread {
                    array_clone.get_and_add(0, 1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let expected = (num_threads * increments_per_thread) as i64;
        assert_eq!(array.get(0), expected);
    }
}

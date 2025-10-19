/// A huge atomic array of f64 floating-point values supporting lock-free concurrent operations.
///
/// This array provides thread-safe atomic operations on massive floating-point datasets.
/// Since Rust doesn't have `AtomicF64`, we use `AtomicU64` with bitwise transmutation
/// to achieve atomic operations on f64 values.
///
/// # Thread Safety
///
/// All operations are atomic and thread-safe. Multiple threads can safely read,
/// write, and perform read-modify-write operations concurrently.
///
/// # Implementation Note
///
/// Atomic f64 operations are implemented using `AtomicU64` with `f64::to_bits()` and
/// `f64::from_bits()`. This provides the same memory guarantees as native atomic types
/// while supporting floating-point values.
///
/// # Design
///
/// Uses the same paged architecture as `HugeDoubleArray` but with atomic storage:
/// - Single array for ≤268M elements using `Vec<AtomicU64>`
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
/// use rust_gds::collections::HugeAtomicDoubleArray;
///
/// // Create atomic array
/// let array = HugeAtomicDoubleArray::new(1000);
///
/// // Atomic operations
/// array.set(0, 3.14);
/// let old_value = array.get_and_add(0, 1.0); // old_value = 3.14, now 4.14
/// let success = array.compare_and_set(0, 4.14, 10.0); // success = true
///
/// // Atomic update with function
/// array.update(0, |current| current * 2.0); // now 20.0
/// ```
use crate::collections::PageUtil;
use std::sync::atomic::{AtomicU64, Ordering};

const MAX_ARRAY_LENGTH: usize = 1 << 28; // 268,435,456 elements
const PAGE_SIZE_IN_BYTES: usize = 4096; // 4KB pages

/// Huge atomic array supporting >2 billion f64 elements with lock-free operations.
pub enum HugeAtomicDoubleArray {
    Single(SingleHugeAtomicDoubleArray),
    Paged(PagedHugeAtomicDoubleArray),
}

impl HugeAtomicDoubleArray {
    /// Creates a new huge atomic double array of the specified size.
    ///
    /// All elements are initialized to 0.0.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of elements in the array
    ///
    /// # Returns
    ///
    /// A new `HugeAtomicDoubleArray` optimized for the given size
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            HugeAtomicDoubleArray::Single(SingleHugeAtomicDoubleArray::new(size))
        } else {
            HugeAtomicDoubleArray::Paged(PagedHugeAtomicDoubleArray::new(size))
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
    pub fn get(&self, index: usize) -> f64 {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.get(index),
            HugeAtomicDoubleArray::Paged(p) => p.get(index),
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
    pub fn set(&self, index: usize, value: f64) {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.set(index, value),
            HugeAtomicDoubleArray::Paged(p) => p.set(index, value),
        }
    }

    /// Atomically adds delta to the value at index and returns the previous value.
    ///
    /// This is the fundamental atomic accumulation operation for concurrent numerical algorithms.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to modify
    /// * `delta` - Value to add (can be negative)
    ///
    /// # Returns
    ///
    /// The value before the addition
    pub fn get_and_add(&self, index: usize, delta: f64) -> f64 {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.get_and_add(index, delta),
            HugeAtomicDoubleArray::Paged(p) => p.get_and_add(index, delta),
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
    pub fn get_and_replace(&self, index: usize, value: f64) -> f64 {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.get_and_replace(index, value),
            HugeAtomicDoubleArray::Paged(p) => p.get_and_replace(index, value),
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
    pub fn compare_and_set(&self, index: usize, expect: f64, update: f64) -> bool {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.compare_and_set(index, expect, update),
            HugeAtomicDoubleArray::Paged(p) => p.compare_and_set(index, expect, update),
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
    pub fn compare_and_exchange(&self, index: usize, expect: f64, update: f64) -> f64 {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.compare_and_exchange(index, expect, update),
            HugeAtomicDoubleArray::Paged(p) => p.compare_and_exchange(index, expect, update),
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
        F: Fn(f64) -> f64,
    {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.update(index, f),
            HugeAtomicDoubleArray::Paged(p) => p.update(index, f),
        }
    }

    /// Returns the total number of elements in the array.
    pub fn size(&self) -> usize {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.size(),
            HugeAtomicDoubleArray::Paged(p) => p.size(),
        }
    }

    /// Returns the memory used by this array in bytes.
    pub fn size_of(&self) -> usize {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.size_of(),
            HugeAtomicDoubleArray::Paged(p) => p.size_of(),
        }
    }

    /// Sets all elements to the specified value (not atomic - for initialization only).
    ///
    /// # Warning
    ///
    /// This operation is NOT thread-safe and should only be called when no other
    /// threads are accessing the array.
    pub fn set_all(&self, value: f64) {
        match self {
            HugeAtomicDoubleArray::Single(s) => s.set_all(value),
            HugeAtomicDoubleArray::Paged(p) => p.set_all(value),
        }
    }
}

/// Single-page implementation for atomic double arrays ≤268M elements.
pub struct SingleHugeAtomicDoubleArray {
    size: usize,
    storage: Vec<AtomicU64>,
}

impl SingleHugeAtomicDoubleArray {
    fn new(size: usize) -> Self {
        let mut storage = Vec::with_capacity(size);
        for _ in 0..size {
            storage.push(AtomicU64::new(0.0_f64.to_bits()));
        }
        Self { size, storage }
    }

    fn get(&self, index: usize) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let bits = self.storage[index].load(Ordering::SeqCst);
        f64::from_bits(bits)
    }

    fn set(&self, index: usize, value: f64) {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index].store(value.to_bits(), Ordering::SeqCst);
    }

    fn get_and_add(&self, index: usize, delta: f64) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let atom = &self.storage[index];
        let mut old_bits = atom.load(Ordering::SeqCst);
        loop {
            let old_value = f64::from_bits(old_bits);
            let new_value = old_value + delta;
            let new_bits = new_value.to_bits();

            match atom.compare_exchange(old_bits, new_bits, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return old_value,
                Err(witness) => old_bits = witness,
            }
        }
    }

    fn get_and_replace(&self, index: usize, value: f64) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let old_bits = self.storage[index].swap(value.to_bits(), Ordering::SeqCst);
        f64::from_bits(old_bits)
    }

    fn compare_and_set(&self, index: usize, expect: f64, update: f64) -> bool {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        self.storage[index]
            .compare_exchange(
                expect.to_bits(),
                update.to_bits(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
    }

    fn compare_and_exchange(&self, index: usize, expect: f64, update: f64) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        match self.storage[index].compare_exchange(
            expect.to_bits(),
            update.to_bits(),
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(bits) => f64::from_bits(bits),
            Err(bits) => f64::from_bits(bits),
        }
    }

    fn update<F>(&self, index: usize, f: F)
    where
        F: Fn(f64) -> f64,
    {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let atom = &self.storage[index];
        let mut old_bits = atom.load(Ordering::SeqCst);
        loop {
            let old_value = f64::from_bits(old_bits);
            let new_value = f(old_value);
            let new_bits = new_value.to_bits();

            match atom.compare_exchange(old_bits, new_bits, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(witness) => old_bits = witness,
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_of(&self) -> usize {
        std::mem::size_of::<Self>() + self.size * std::mem::size_of::<AtomicU64>()
    }

    fn set_all(&self, value: f64) {
        let bits = value.to_bits();
        for atom in &self.storage {
            atom.store(bits, Ordering::SeqCst);
        }
    }
}

/// Paged implementation for atomic double arrays >268M elements.
pub struct PagedHugeAtomicDoubleArray {
    size: usize,
    pages: Vec<Vec<AtomicU64>>,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeAtomicDoubleArray {
    fn new(size: usize) -> Self {
        let element_size = std::mem::size_of::<AtomicU64>();
        let page_size = PageUtil::page_size_for(PAGE_SIZE_IN_BYTES, element_size);
        let page_shift = page_size.trailing_zeros();
        let page_mask = page_size - 1;
        let num_pages = PageUtil::num_pages_for(size, page_size);

        let mut pages = Vec::with_capacity(num_pages);

        // Create full pages
        for _ in 0..(num_pages - 1) {
            let mut page = Vec::with_capacity(page_size);
            for _ in 0..page_size {
                page.push(AtomicU64::new(0.0_f64.to_bits()));
            }
            pages.push(page);
        }

        // Create last (potentially partial) page
        let last_page_size = PageUtil::exclusive_index_of_page(size, page_mask);
        let mut last_page = Vec::with_capacity(last_page_size);
        for _ in 0..last_page_size {
            last_page.push(AtomicU64::new(0.0_f64.to_bits()));
        }
        pages.push(last_page);

        Self {
            size,
            pages,
            page_shift,
            page_mask,
        }
    }

    fn get(&self, index: usize) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        let bits = self.pages[page_index][index_in_page].load(Ordering::SeqCst);
        f64::from_bits(bits)
    }

    fn set(&self, index: usize, value: f64) {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page].store(value.to_bits(), Ordering::SeqCst);
    }

    fn get_and_add(&self, index: usize, delta: f64) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        let atom = &self.pages[page_index][index_in_page];

        let mut old_bits = atom.load(Ordering::SeqCst);
        loop {
            let old_value = f64::from_bits(old_bits);
            let new_value = old_value + delta;
            let new_bits = new_value.to_bits();

            match atom.compare_exchange(old_bits, new_bits, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return old_value,
                Err(witness) => old_bits = witness,
            }
        }
    }

    fn get_and_replace(&self, index: usize, value: f64) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        let old_bits =
            self.pages[page_index][index_in_page].swap(value.to_bits(), Ordering::SeqCst);
        f64::from_bits(old_bits)
    }

    fn compare_and_set(&self, index: usize, expect: f64, update: f64) -> bool {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page]
            .compare_exchange(
                expect.to_bits(),
                update.to_bits(),
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
    }

    fn compare_and_exchange(&self, index: usize, expect: f64, update: f64) -> f64 {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        match self.pages[page_index][index_in_page].compare_exchange(
            expect.to_bits(),
            update.to_bits(),
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(bits) => f64::from_bits(bits),
            Err(bits) => f64::from_bits(bits),
        }
    }

    fn update<F>(&self, index: usize, f: F)
    where
        F: Fn(f64) -> f64,
    {
        debug_assert!(index < self.size, "index {} >= size {}", index, self.size);
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        let atom = &self.pages[page_index][index_in_page];

        let mut old_bits = atom.load(Ordering::SeqCst);
        loop {
            let old_value = f64::from_bits(old_bits);
            let new_value = f(old_value);
            let new_bits = new_value.to_bits();

            match atom.compare_exchange(old_bits, new_bits, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(witness) => old_bits = witness,
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_of(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();
        for page in &self.pages {
            total += page.len() * std::mem::size_of::<AtomicU64>();
        }
        total
    }

    fn set_all(&self, value: f64) {
        let bits = value.to_bits();
        for page in &self.pages {
            for atom in page {
                atom.store(bits, Ordering::SeqCst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_small() {
        let array = HugeAtomicDoubleArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(99), 0.0);
    }

    #[test]
    fn test_get_set() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set(0, 3.14);
        array.set(50, -100.5);
        array.set(99, 999.999);

        assert_eq!(array.get(0), 3.14);
        assert_eq!(array.get(50), -100.5);
        assert_eq!(array.get(99), 999.999);
    }

    #[test]
    fn test_get_and_add() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set(0, 10.5);

        let old_val = array.get_and_add(0, 5.25);
        assert_eq!(old_val, 10.5);
        assert_eq!(array.get(0), 15.75);

        let old_val2 = array.get_and_add(0, -3.75);
        assert_eq!(old_val2, 15.75);
        assert_eq!(array.get(0), 12.0);
    }

    #[test]
    fn test_get_and_replace() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set(0, 42.0);

        let old_val = array.get_and_replace(0, 100.5);
        assert_eq!(old_val, 42.0);
        assert_eq!(array.get(0), 100.5);
    }

    #[test]
    fn test_compare_and_set() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set(0, 42.0);

        // Successful CAS
        let success = array.compare_and_set(0, 42.0, 100.0);
        assert!(success);
        assert_eq!(array.get(0), 100.0);

        // Failed CAS
        let failure = array.compare_and_set(0, 42.0, 200.0);
        assert!(!failure);
        assert_eq!(array.get(0), 100.0);
    }

    #[test]
    fn test_compare_and_exchange() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set(0, 42.0);

        // Successful exchange
        let witness = array.compare_and_exchange(0, 42.0, 100.0);
        assert_eq!(witness, 42.0);
        assert_eq!(array.get(0), 100.0);

        // Failed exchange - returns current value
        let witness2 = array.compare_and_exchange(0, 42.0, 200.0);
        assert_eq!(witness2, 100.0); // Returns actual value, not expected
        assert_eq!(array.get(0), 100.0); // Value unchanged
    }

    #[test]
    fn test_update() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set(0, 10.0);

        // Atomic multiply by 2
        array.update(0, |x| x * 2.0);
        assert_eq!(array.get(0), 20.0);

        // Atomic max update
        array.update(0, |x| x.max(100.0));
        assert_eq!(array.get(0), 100.0);

        array.update(0, |x| x.max(50.0));
        assert_eq!(array.get(0), 100.0); // Unchanged
    }

    #[test]
    fn test_set_all() {
        let array = HugeAtomicDoubleArray::new(100);
        array.set_all(3.14);

        for i in 0..100 {
            assert_eq!(array.get(i), 3.14);
        }
    }

    #[test]
    fn test_paged_array() {
        let size = MAX_ARRAY_LENGTH + 1000;
        let array = HugeAtomicDoubleArray::new(size);

        assert_eq!(array.size(), size);

        // Test across page boundaries
        array.set(MAX_ARRAY_LENGTH - 1, 11.1);
        array.set(MAX_ARRAY_LENGTH, 22.2);
        array.set(MAX_ARRAY_LENGTH + 1, 33.3);

        assert_eq!(array.get(MAX_ARRAY_LENGTH - 1), 11.1);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 22.2);
        assert_eq!(array.get(MAX_ARRAY_LENGTH + 1), 33.3);

        // Test atomic operations on paged array
        let old = array.get_and_add(MAX_ARRAY_LENGTH, 10.0);
        assert_eq!(old, 22.2);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 32.2);
    }

    #[test]
    fn test_concurrent_additions() {
        use std::sync::Arc;
        use std::thread;

        let array = Arc::new(HugeAtomicDoubleArray::new(10));
        let num_threads = 4;
        let additions_per_thread = 1000;

        let mut handles = vec![];

        for _ in 0..num_threads {
            let array_clone = Arc::clone(&array);
            let handle = thread::spawn(move || {
                for _ in 0..additions_per_thread {
                    array_clone.get_and_add(0, 0.1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let expected = (num_threads * additions_per_thread) as f64 * 0.1;
        let result = array.get(0);
        // Allow for floating-point rounding errors
        assert!((result - expected).abs() < 0.001);
    }
}

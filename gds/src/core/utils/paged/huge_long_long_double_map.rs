use crate::collections::{HugeDoubleArray, HugeLongArray};

/// Hash map: `(long, long) → double` with composite key hashing.
///
/// Essential for graph algorithms requiring pair-based mappings:
/// - Edge weights: (sourceNode, targetNode) → weight
/// - Graph embeddings: (node, dimension) → feature value
/// - Temporal graphs: (node, timestamp) → temporal score
/// - Community pairs: (community1, community2) → similarity
/// - Collaborative filtering: (user, item) → rating
///
/// Uses +1 key shifting internally to distinguish empty slots (0) from key 0.
pub struct HugeLongLongDoubleMap {
    keys1: HugeLongArray,
    keys2: HugeLongArray,
    values: HugeDoubleArray,
    key_mixer: i32,
    assigned: usize,
    mask: usize,
    resize_at: usize,
}

const DEFAULT_EXPECTED_ELEMENTS: usize = 4;
const LOAD_FACTOR: f64 = 0.75;
const MIN_HASH_ARRAY_LENGTH: usize = 4;

impl HugeLongLongDoubleMap {
    /// Creates a new map with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_EXPECTED_ELEMENTS)
    }

    /// Creates a new map with specified expected element count.
    pub fn with_capacity(expected_elements: usize) -> Self {
        let array_size = Self::min_buffer_size(expected_elements);
        let mut map = Self {
            keys1: HugeLongArray::new(array_size),
            keys2: HugeLongArray::new(array_size),
            values: HugeDoubleArray::new(array_size),
            key_mixer: Self::random_seed(),
            assigned: 0,
            mask: array_size - 1,
            resize_at: Self::expand_at_count(array_size),
        };
        map.keys1.fill(0);
        map.keys2.fill(0);
        map.values.fill(0.0);
        map
    }

    /// Sets a value for the given key pair.
    pub fn set(&mut self, key1: i64, key2: i64, value: f64) {
        self.set0(key1 + 1, key2 + 1, value);
    }

    /// Adds a value to the existing value for the given key pair.
    pub fn add_to(&mut self, key1: i64, key2: i64, value: f64) {
        self.add_to0(key1 + 1, key2 + 1, value);
    }

    /// Gets the value for a key pair, or returns default if not found.
    pub fn get_or_default(&self, key1: i64, key2: i64, default_value: f64) -> f64 {
        self.get_or_default0(key1 + 1, key2 + 1, default_value)
    }

    /// Returns the number of entries in the map.
    pub fn size(&self) -> usize {
        self.assigned
    }

    /// Checks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.assigned == 0
    }

    /// Clears all entries from the map.
    pub fn clear(&mut self) {
        self.assigned = 0;
        self.keys1.fill(0);
        self.keys2.fill(0);
        self.values.fill(0.0);
    }

    fn set0(&mut self, key1: i64, key2: i64, value: f64) {
        assert!(self.assigned < self.mask + 1);
        let key = self.hash_key(key1, key2);
        let mut slot = self.find_slot(key1, key2, (key as usize) & self.mask);
        assert!(slot != -1);

        if slot >= 0 {
            self.values.set(slot as usize, value);
            return;
        }

        slot = !(slot + 1);
        if self.assigned == self.resize_at {
            self.allocate_then_insert_then_rehash(slot as usize, key1, key2, value);
        } else {
            self.values.set(slot as usize, value);
            self.keys1.set(slot as usize, key1);
            self.keys2.set(slot as usize, key2);
        }

        self.assigned += 1;
    }

    fn add_to0(&mut self, key1: i64, key2: i64, value: f64) {
        assert!(self.assigned < self.mask + 1);
        let key = self.hash_key(key1, key2);
        let mut slot = self.find_slot(key1, key2, (key as usize) & self.mask);
        assert!(slot != -1);

        if slot >= 0 {
            self.values.add_to(slot as usize, value);
            return;
        }

        slot = !(slot + 1);
        if self.assigned == self.resize_at {
            self.allocate_then_insert_then_rehash(slot as usize, key1, key2, value);
        } else {
            self.values.set(slot as usize, value);
            self.keys1.set(slot as usize, key1);
            self.keys2.set(slot as usize, key2);
        }

        self.assigned += 1;
    }

    fn get_or_default0(&self, key1: i64, key2: i64, default_value: f64) -> f64 {
        let key = self.hash_key(key1, key2);
        let slot = self.find_slot(key1, key2, (key as usize) & self.mask);
        if slot >= 0 {
            return self.values.get(slot as usize);
        }
        default_value
    }

    fn hash_key(&self, key1: i64, key2: i64) -> u64 {
        let combined = key1 ^ key2 ^ (self.key_mixer as i64);
        Self::mix_hash(combined)
    }

    fn find_slot(&self, key1: i64, key2: i64, start: usize) -> isize {
        let size = self.keys1.size();
        let mut slot = self.find_slot_in_range(key1, key2, start, size);
        if slot == -1 {
            slot = self.find_slot_in_range(key1, key2, 0, start);
        }
        slot
    }

    fn find_slot_in_range(&self, key1: i64, key2: i64, start: usize, end: usize) -> isize {
        let mut slot = start;
        while slot < end {
            let existing1 = self.keys1.get(slot);
            if existing1 == 0 {
                return !((slot + 1) as isize);
            }
            if existing1 == key1 && self.keys2.get(slot) == key2 {
                return slot as isize;
            }
            slot += 1;
        }
        -1
    }

    fn allocate_then_insert_then_rehash(
        &mut self,
        slot: usize,
        pending_key1: i64,
        pending_key2: i64,
        pending_value: f64,
    ) {
        let old_keys1 = std::mem::replace(&mut self.keys1, HugeLongArray::new(1));
        let old_keys2 = std::mem::replace(&mut self.keys2, HugeLongArray::new(1));
        let old_values = std::mem::replace(&mut self.values, HugeDoubleArray::new(1));

        let new_size = Self::next_buffer_size(self.mask + 1);
        self.keys1 = HugeLongArray::new(new_size);
        self.keys2 = HugeLongArray::new(new_size);
        self.values = HugeDoubleArray::new(new_size);
        self.keys1.fill(0);
        self.keys2.fill(0);
        self.values.fill(0.0);
        self.key_mixer = Self::random_seed();
        self.resize_at = Self::expand_at_count(new_size);
        self.mask = new_size - 1;

        // Insert pending entry into old arrays
        let mut temp_keys1 = old_keys1;
        let mut temp_keys2 = old_keys2;
        let mut temp_values = old_values;
        temp_keys1.set(slot, pending_key1);
        temp_keys2.set(slot, pending_key2);
        temp_values.set(slot, pending_value);

        // Rehash
        self.rehash(&temp_keys1, &temp_keys2, &temp_values);
    }

    fn rehash(
        &mut self,
        from_keys1: &HugeLongArray,
        from_keys2: &HugeLongArray,
        from_values: &HugeDoubleArray,
    ) {
        let old_size = from_keys1.size();
        for i in 0..old_size {
            let key1 = from_keys1.get(i);
            if key1 == 0 {
                continue;
            }

            let key2 = from_keys2.get(i);
            let value = from_values.get(i);
            let key = self.hash_key(key1, key2);
            let start = (key as usize) & self.mask;

            let mut slot = start;
            loop {
                if self.keys1.get(slot) == 0 {
                    self.keys1.set(slot, key1);
                    self.keys2.set(slot, key2);
                    self.values.set(slot, value);
                    break;
                }
                slot = (slot + 1) & self.mask;
            }
        }
    }

    fn mix_hash(key: i64) -> u64 {
        let mut k = key as u64;
        k = k.wrapping_mul(0x9e3779b97f4a7c15u64);
        k = k ^ (k >> 32);
        k = k.wrapping_mul(0x9e3779b97f4a7c15u64);
        k = k ^ (k >> 32);
        k
    }

    fn random_seed() -> i32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        (nanos as i32).wrapping_mul(-1640531527i32)
    }

    fn min_buffer_size(elements: usize) -> usize {
        let mut length = ((elements as f64) / LOAD_FACTOR).ceil() as usize;
        if length == elements {
            length += 1;
        }
        length = length.max(MIN_HASH_ARRAY_LENGTH);
        length.next_power_of_two()
    }

    fn next_buffer_size(array_size: usize) -> usize {
        assert!(array_size.is_power_of_two());
        array_size << 1
    }

    fn expand_at_count(array_size: usize) -> usize {
        assert!(array_size.is_power_of_two());
        array_size.min(((array_size as f64) * LOAD_FACTOR).ceil() as usize)
    }
}

impl Default for HugeLongLongDoubleMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut map = HugeLongLongDoubleMap::new();
        map.set(1, 10, 100.5);
        map.set(2, 20, 200.5);
        map.set(3, 30, 300.5);

        assert!((map.get_or_default(1, 10, 0.0) - 100.5).abs() < 1e-10);
        assert!((map.get_or_default(2, 20, 0.0) - 200.5).abs() < 1e-10);
        assert!((map.get_or_default(3, 30, 0.0) - 300.5).abs() < 1e-10);
        assert!((map.get_or_default(999, 999, -1.0) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn add_to_accumulates() {
        let mut map = HugeLongLongDoubleMap::new();
        map.add_to(1, 2, 0.1);
        map.add_to(1, 2, 0.2);
        map.add_to(1, 2, 0.3);

        assert!((map.get_or_default(1, 2, 0.0) - 0.6).abs() < 1e-10);
    }

    #[test]
    fn different_key_pairs_are_distinct() {
        let mut map = HugeLongLongDoubleMap::new();
        map.set(1, 2, 12.0);
        map.set(2, 1, 21.0);

        assert!((map.get_or_default(1, 2, 0.0) - 12.0).abs() < 1e-10);
        assert!((map.get_or_default(2, 1, 0.0) - 21.0).abs() < 1e-10);
    }

    #[test]
    fn handles_key_zero() {
        let mut map = HugeLongLongDoubleMap::new();
        map.set(0, 0, 3.14);
        map.set(0, 1, 2.71);
        map.set(1, 0, 1.41);

        assert!((map.get_or_default(0, 0, 0.0) - 3.14).abs() < 1e-10);
        assert!((map.get_or_default(0, 1, 0.0) - 2.71).abs() < 1e-10);
        assert!((map.get_or_default(1, 0, 0.0) - 1.41).abs() < 1e-10);
    }

    #[test]
    fn resize_on_growth() {
        let mut map = HugeLongLongDoubleMap::with_capacity(2);
        for i in 0..50 {
            for j in 0..2 {
                map.set(i, j, (i * 10 + j) as f64);
            }
        }

        for i in 0..50 {
            for j in 0..2 {
                let expected = (i * 10 + j) as f64;
                assert!((map.get_or_default(i, j, -1.0) - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn clear_works() {
        let mut map = HugeLongLongDoubleMap::new();
        map.set(1, 2, 1.2);
        map.set(3, 4, 3.4);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
        assert!((map.get_or_default(1, 2, -1.0) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn size_tracking() {
        let mut map = HugeLongLongDoubleMap::new();
        assert_eq!(map.size(), 0);

        map.set(1, 2, 1.0);
        assert_eq!(map.size(), 1);

        map.set(3, 4, 2.0);
        assert_eq!(map.size(), 2);

        map.set(1, 2, 1.5); // Overwrite
        assert_eq!(map.size(), 2);
    }
}

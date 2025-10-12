use crate::collections::HugeLongArray;

/// Hash map: `long → long` with open addressing and linear probing.
///
/// Essential for graph algorithms requiring node-to-value mappings:
/// - Node ID → Community ID (community detection)
/// - Node ID → Parent node (Union-Find, spanning trees)
/// - Node ID → Distance/rank (shortest paths)
/// - Original ID → Internal ID (ID mapping/compression)
///
/// Uses +1 key shifting internally to distinguish empty slots (0) from key 0.
pub struct HugeLongLongMap {
    keys: HugeLongArray,
    values: HugeLongArray,
    assigned: usize,
    mask: usize,
    resize_at: usize,
}

const DEFAULT_EXPECTED_ELEMENTS: usize = 4;
const LOAD_FACTOR: f64 = 0.75;
const MIN_HASH_ARRAY_LENGTH: usize = 4;

impl HugeLongLongMap {
    /// Creates a new map with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_EXPECTED_ELEMENTS)
    }

    /// Creates a new map with specified expected element count.
    pub fn with_capacity(expected_elements: usize) -> Self {
        let array_size = Self::min_buffer_size(expected_elements);
        let mut map = Self {
            keys: HugeLongArray::new(array_size),
            values: HugeLongArray::new(array_size),
            assigned: 0,
            mask: array_size - 1,
            resize_at: Self::expand_at_count(array_size),
        };
        map.keys.fill(0);
        map.values.fill(0);
        map
    }

    /// Sets a value for the given key.
    pub fn put(&mut self, key: i64, value: i64) {
        self.put0(key + 1, value);
    }

    /// Adds a value to the existing value for the given key.
    pub fn add_to(&mut self, key: i64, value: i64) {
        self.add_to0(key + 1, value);
    }

    /// Gets the value for a key, or returns default if not found.
    pub fn get_or_default(&self, key: i64, default_value: i64) -> i64 {
        self.get_or_default0(key + 1, default_value)
    }

    /// Checks if the map contains the given key.
    pub fn contains_key(&self, key: i64) -> bool {
        self.contains_key0(key + 1)
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
        self.keys.fill(0);
        self.values.fill(0);
    }

    fn contains_key0(&self, key: i64) -> bool {
        let hash = Self::mix_hash(key);
        self.find_slot(key, (hash as usize) & self.mask) >= 0
    }

    fn put0(&mut self, key: i64, value: i64) {
        assert!(self.assigned < self.mask + 1);
        let hash = Self::mix_hash(key);
        let mut slot = self.find_slot(key, (hash as usize) & self.mask);
        assert!(slot != -1);

        if slot >= 0 {
            self.values.set(slot as usize, value);
            return;
        }

        slot = !(slot + 1);
        if self.assigned == self.resize_at {
            self.allocate_then_insert_then_rehash(slot as usize, key, value);
        } else {
            self.values.set(slot as usize, value);
            self.keys.set(slot as usize, key);
        }

        self.assigned += 1;
    }

    fn add_to0(&mut self, key: i64, value: i64) {
        assert!(self.assigned < self.mask + 1);
        let hash = Self::mix_hash(key);
        let mut slot = self.find_slot(key, (hash as usize) & self.mask);
        assert!(slot != -1);

        if slot >= 0 {
            self.values.add_to(slot as usize, value);
            return;
        }

        slot = !(slot + 1);
        if self.assigned == self.resize_at {
            self.allocate_then_insert_then_rehash(slot as usize, key, value);
        } else {
            self.values.set(slot as usize, value);
            self.keys.set(slot as usize, key);
        }

        self.assigned += 1;
    }

    fn get_or_default0(&self, key: i64, default_value: i64) -> i64 {
        let hash = Self::mix_hash(key);
        let slot = self.find_slot(key, (hash as usize) & self.mask);
        if slot >= 0 {
            return self.values.get(slot as usize);
        }
        default_value
    }

    fn find_slot(&self, key: i64, start: usize) -> isize {
        let size = self.keys.size();
        let mut slot = self.find_slot_in_range(key, start, size);
        if slot == -1 {
            slot = self.find_slot_in_range(key, 0, start);
        }
        slot
    }

    fn find_slot_in_range(&self, key: i64, start: usize, end: usize) -> isize {
        let mut slot = start;
        while slot < end {
            let existing = self.keys.get(slot);
            if existing == 0 {
                return !((slot + 1) as isize);
            }
            if existing == key {
                return slot as isize;
            }
            slot += 1;
        }
        -1
    }

    fn allocate_then_insert_then_rehash(
        &mut self,
        slot: usize,
        pending_key: i64,
        pending_value: i64,
    ) {
        let old_keys = std::mem::replace(&mut self.keys, HugeLongArray::new(1));
        let old_values = std::mem::replace(&mut self.values, HugeLongArray::new(1));

        let new_size = Self::next_buffer_size(self.mask + 1);
        self.keys = HugeLongArray::new(new_size);
        self.values = HugeLongArray::new(new_size);
        self.keys.fill(0);
        self.values.fill(0);
        self.resize_at = Self::expand_at_count(new_size);
        self.mask = new_size - 1;

        // Insert pending entry into old arrays
        let mut temp_keys = old_keys;
        let mut temp_values = old_values;
        temp_keys.set(slot, pending_key);
        temp_values.set(slot, pending_value);

        // Rehash
        self.rehash(&temp_keys, &temp_values);
    }

    fn rehash(&mut self, from_keys: &HugeLongArray, from_values: &HugeLongArray) {
        let old_size = from_keys.size();
        for i in 0..old_size {
            let key = from_keys.get(i);
            if key == 0 {
                continue;
            }

            let value = from_values.get(i);
            let hash = Self::mix_hash(key);
            let start = (hash as usize) & self.mask;

            let mut slot = start;
            loop {
                if self.keys.get(slot) == 0 {
                    self.keys.set(slot, key);
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

impl Default for HugeLongLongMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_and_get() {
        let mut map = HugeLongLongMap::new();
        map.put(1, 100);
        map.put(2, 200);
        map.put(3, 300);

        assert_eq!(map.get_or_default(1, -1), 100);
        assert_eq!(map.get_or_default(2, -1), 200);
        assert_eq!(map.get_or_default(3, -1), 300);
        assert_eq!(map.get_or_default(999, -1), -1);
    }

    #[test]
    fn add_to_accumulates() {
        let mut map = HugeLongLongMap::new();
        map.add_to(1, 10);
        map.add_to(1, 20);
        map.add_to(1, 30);

        assert_eq!(map.get_or_default(1, 0), 60);
    }

    #[test]
    fn contains_key_works() {
        let mut map = HugeLongLongMap::new();
        map.put(42, 100);

        assert!(map.contains_key(42));
        assert!(!map.contains_key(43));
    }

    #[test]
    fn handles_key_zero() {
        let mut map = HugeLongLongMap::new();
        map.put(0, 999);
        assert_eq!(map.get_or_default(0, -1), 999);
        assert!(map.contains_key(0));
    }

    #[test]
    fn resize_on_growth() {
        let mut map = HugeLongLongMap::with_capacity(2);
        for i in 0..100 {
            map.put(i, i * 10);
        }

        for i in 0..100 {
            assert_eq!(map.get_or_default(i, -1), i * 10);
        }
    }

    #[test]
    fn clear_works() {
        let mut map = HugeLongLongMap::new();
        map.put(1, 100);
        map.put(2, 200);
        assert_eq!(map.size(), 2);

        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
        assert_eq!(map.get_or_default(1, -1), -1);
    }

    #[test]
    fn size_tracking() {
        let mut map = HugeLongLongMap::new();
        assert_eq!(map.size(), 0);

        map.put(1, 10);
        assert_eq!(map.size(), 1);

        map.put(2, 20);
        assert_eq!(map.size(), 2);

        map.put(1, 15); // Overwrite
        assert_eq!(map.size(), 2);
    }
}

//! A multiset (bag) data structure for efficiently counting occurrences of long integers.
//!
//! This class provides **high-performance counting operations** for large integer values
//! commonly found in graph analytics, such as node IDs, edge counts, degree distributions,
//! and frequency analysis.

use std::collections::HashMap;

/// A multiset (bag) data structure for efficiently counting occurrences of integers.
///
/// Unlike a regular set that stores unique elements, a multiset allows duplicate elements
/// and maintains a count of how many times each element has been added. Essential for
/// statistical analysis and frequency counting in graph algorithms.
///
/// ## Performance Profile
///
/// - **Add operations**: O(1) average case, O(n) worst case
/// - **Count queries**: O(1) average case, O(n) worst case
/// - **Key enumeration**: O(k) where k is number of unique keys
/// - **Sum calculation**: O(k) where k is number of unique keys
/// - **Memory usage**: O(k) where k is number of unique keys
///
/// ## Common Use Cases
///
/// - **Degree distribution analysis**: Count node degrees in a graph
/// - **Community size counting**: Track sizes of detected communities
/// - **Edge weight frequency**: Analyze distribution of edge weights
/// - **Algorithm convergence tracking**: Track PageRank score convergence
///
/// # Examples
///
/// ```
/// use rust_gds::collections::LongMultiSet;
///
/// let mut multiset = LongMultiSet::new();
/// multiset.add(42);
/// multiset.add(42);
/// multiset.add(17);
///
/// assert_eq!(multiset.count(42), 2);
/// assert_eq!(multiset.count(17), 1);
/// assert_eq!(multiset.size(), 2); // 2 unique values
/// assert_eq!(multiset.sum(), 3);  // 3 total occurrences
/// ```
#[derive(Clone, Debug)]
pub struct LongMultiSet {
    map: HashMap<i64, i64>,
}

impl LongMultiSet {
    /// Creates a new empty multiset.
    ///
    /// The multiset is initialized with an empty hash map optimized for integer keys.
    /// The underlying HashMap will automatically resize as elements are added.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let multiset = LongMultiSet::new();
    /// assert_eq!(multiset.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Adds one occurrence of the specified value to the multiset.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to add to the multiset
    ///
    /// # Returns
    ///
    /// The new total count of the value after addition
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// assert_eq!(multiset.add(42), 1);
    /// assert_eq!(multiset.add(42), 2);
    /// ```
    pub fn add(&mut self, value: i64) -> i64 {
        self.add_count(value, 1)
    }

    /// Adds multiple occurrences of the specified value to the multiset.
    ///
    /// # Arguments
    ///
    /// * `key` - The integer value to add to the multiset
    /// * `count` - The number of occurrences to add (can be negative to remove)
    ///
    /// # Returns
    ///
    /// The new total count of the key after addition
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(42, 5);
    /// assert_eq!(multiset.count(42), 5);
    ///
    /// multiset.add_count(42, -2);
    /// assert_eq!(multiset.count(42), 3);
    /// ```
    pub fn add_count(&mut self, key: i64, count: i64) -> i64 {
        let entry = self.map.entry(key).or_insert(0);
        *entry += count;

        // Clean up zero or negative counts
        if *entry <= 0 {
            self.map.remove(&key);
            0
        } else {
            *entry
        }
    }

    /// Returns the number of occurrences of the specified value in the multiset.
    ///
    /// This method provides **O(1) lookup** for the count of any value in the multiset.
    /// If the value has never been added to the multiset, it returns 0.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to query
    ///
    /// # Returns
    ///
    /// The number of times the value has been added to the multiset
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(42, 5);
    ///
    /// assert_eq!(multiset.count(42), 5);
    /// assert_eq!(multiset.count(100), 0); // never added
    /// ```
    pub fn count(&self, value: i64) -> i64 {
        *self.map.get(&value).unwrap_or(&0)
    }

    /// Returns a vector of all unique values (keys) in the multiset.
    ///
    /// This method provides **access to the distinct elements** that have been added
    /// to the multiset, regardless of their frequency.
    ///
    /// # Returns
    ///
    /// Vector containing all unique values in the multiset
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(10, 3);
    /// multiset.add_count(20, 1);
    /// multiset.add(10); // 10 now has count 4
    ///
    /// let mut keys = multiset.keys();
    /// keys.sort();
    /// assert_eq!(keys, vec![10, 20]);
    /// ```
    pub fn keys(&self) -> Vec<i64> {
        self.map.keys().copied().collect()
    }

    /// Returns the number of unique values in the multiset.
    ///
    /// This method returns the **cardinality of the underlying set** - the number
    /// of distinct values that have been added, regardless of their frequency.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(10, 3);
    /// multiset.add_count(20, 1);
    /// multiset.add_count(30, 2);
    ///
    /// assert_eq!(multiset.size(), 3); // 3 unique values: 10, 20, 30
    /// assert_eq!(multiset.sum(), 6);  // 6 total occurrences: 3+1+2
    /// ```
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Returns the sum of all counts in the multiset.
    ///
    /// This method calculates the **total number of occurrences** across all values
    /// in the multiset. It's equivalent to the sum of all individual counts.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(1, 10);
    /// multiset.add_count(2, 20);
    /// multiset.add_count(3, 30);
    ///
    /// assert_eq!(multiset.sum(), 60); // 10+20+30
    /// ```
    pub fn sum(&self) -> i64 {
        self.map.values().sum()
    }

    /// Removes a value from the multiset entirely.
    ///
    /// This is a utility method for completely removing a value and its count
    /// from the multiset.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove completely
    ///
    /// # Returns
    ///
    /// `true` if the value was present and removed, `false` if it wasn't present
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(42, 5);
    ///
    /// assert!(multiset.remove(42));
    /// assert_eq!(multiset.count(42), 0);
    /// assert!(!multiset.remove(42)); // already removed
    /// ```
    pub fn remove(&mut self, value: i64) -> bool {
        self.map.remove(&value).is_some()
    }

    /// Returns an iterator over all [value, count] pairs in the multiset.
    ///
    /// This method provides **efficient iteration** over the multiset contents
    /// without requiring separate calls to `keys()` and `count()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(10, 3);
    /// multiset.add_count(20, 2);
    ///
    /// for (value, count) in multiset.entries() {
    ///     println!("Value {} appears {} times", value, count);
    /// }
    /// ```
    pub fn entries(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.map.iter().map(|(&k, &v)| (k, v))
    }

    /// Clears all values from the multiset.
    ///
    /// Removes all values and their counts, returning the multiset to an empty state.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::LongMultiSet;
    ///
    /// let mut multiset = LongMultiSet::new();
    /// multiset.add_count(42, 5);
    /// assert_eq!(multiset.size(), 1);
    ///
    /// multiset.clear();
    /// assert_eq!(multiset.size(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for LongMultiSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_count() {
        let mut multiset = LongMultiSet::new();
        assert_eq!(multiset.add(42), 1);
        assert_eq!(multiset.add(42), 2);
        assert_eq!(multiset.count(42), 2);
        assert_eq!(multiset.count(17), 0);
    }

    #[test]
    fn test_add_count() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(42, 5);
        assert_eq!(multiset.count(42), 5);

        multiset.add_count(42, -2);
        assert_eq!(multiset.count(42), 3);
    }

    #[test]
    fn test_keys() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(10, 3);
        multiset.add_count(20, 1);
        multiset.add(10);

        let mut keys = multiset.keys();
        keys.sort();
        assert_eq!(keys, vec![10, 20]);
    }

    #[test]
    fn test_size_and_sum() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(10, 3);
        multiset.add_count(20, 1);
        multiset.add_count(30, 2);

        assert_eq!(multiset.size(), 3);
        assert_eq!(multiset.sum(), 6);
    }

    #[test]
    fn test_remove() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(42, 5);

        assert!(multiset.remove(42));
        assert_eq!(multiset.count(42), 0);
        assert!(!multiset.remove(42));
    }

    #[test]
    fn test_entries() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(10, 3);
        multiset.add_count(20, 2);

        let entries: Vec<_> = multiset.entries().collect();
        assert_eq!(entries.len(), 2);

        // Check that both entries exist
        assert!(entries.contains(&(10, 3)));
        assert!(entries.contains(&(20, 2)));
    }

    #[test]
    fn test_clear() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(42, 5);
        assert_eq!(multiset.size(), 1);

        multiset.clear();
        assert_eq!(multiset.size(), 0);
        assert_eq!(multiset.sum(), 0);
    }

    #[test]
    fn test_negative_count_removes_entry() {
        let mut multiset = LongMultiSet::new();
        multiset.add_count(42, 5);
        multiset.add_count(42, -10); // Should remove entry

        assert_eq!(multiset.count(42), 0);
        assert_eq!(multiset.size(), 0);
    }
}

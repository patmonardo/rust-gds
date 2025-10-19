/*!
Atomic Disjoint Set Structure with wait-free parallel algorithms.

Adaptation of the C++ implementation [1] for the
"Wait-free Parallel Algorithms for the Union-Find Problem" [2]
with some input from an atomic DSS implementation in Rust [3].

This implementation uses Compare-And-Swap operations to atomically update
the disjoint sets, allowing for concurrent modifications without locking.
It implements path halving for efficient find operations and uses
a Union-by-Min strategy.

# Key Differences from Java Implementation

The major difference for our DSS is that we don't support the
Union-by-Rank strategy [4], for technical and performance reasons.

The reference implementation in C++ uses 32bit unsigned integers for
both the id values and the rank values. Those two values have to be
updated atomically, which [1] does by merging them into a single
64bit unsigned integer and doing atomic/cas operations on that value.

We need 64bits for the id value alone and since there is no u128 data type
in Java, the only way to update those values would be to use a class for
the combination of id+rank and update the references to that atomically.

We drop the by-Rank functionality and just support Union-by-Min for this DSS.

The main difference in implementation compared to the regular DSS is that we
use CAS operations to atomically set a set id for some value.
We will retry union operations until a thread succeeds in changing the set id
for a node. Other threads that might have wanted to write a different value
will fail the CAS operation and redo their union step. This allows for concurrent
writes into a single DSS and does not require an additional merge step.

# References

- [1]: <https://github.com/wjakob/dset/blob/7967ef0e6041cd9d73b9c7f614ab8ae92e9e587a/dset.h>
- [2]: <http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.56.8354&rep=rep1&type=pdf>
- [3]: <https://github.com/tov/disjoint-sets-rs/blob/88ab08df21f04fcf7c157b6e042efd561ee873ba/src/concurrent.rs>
- [4]: <https://en.wikipedia.org/wiki/Disjoint-set_data_structure#by_rank>
*/

use super::DisjointSetStruct;
use crate::collections::HugeAtomicLongArray;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Atomic disjoint set structure supporting wait-free parallel union-find operations.
///
/// This implementation allows multiple threads to perform union and find operations
/// concurrently without locks, using atomic compare-and-swap operations.
///
/// # Thread Safety
///
/// All operations are thread-safe and wait-free. Multiple threads can:
/// - Perform union operations concurrently
/// - Find set representatives concurrently
/// - Mix union and find operations freely
///
/// # Algorithm Details
///
/// ## Path Halving (not full path compression)
///
/// During find operations, we use path halving instead of full path compression:
/// - Each node is made to point to its grandparent
/// - Reduces tree height over time
/// - Wait-free (no retry loops)
/// - Good enough for practical performance
///
/// ## Union-by-Min Strategy
///
/// When merging sets, the set with the smaller ID wins:
/// - Ensures deterministic community IDs
/// - Important for seeded community detection
/// - Simpler than union-by-rank (no need for atomic rank updates)
///
/// # Examples
///
/// ```rust
/// use rust_gds::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
/// use std::sync::Arc;
/// use std::thread;
///
/// // Create DSS for 1 million nodes
/// let dss = Arc::new(HugeAtomicDisjointSetStruct::new(1_000_000));
///
/// // Spawn threads to process edges concurrently
/// let mut handles = vec![];
/// for thread_id in 0..4 {
///     let dss_clone = Arc::clone(&dss);
///     let handle = thread::spawn(move || {
///         let start = thread_id * 250_000;
///         let end = start + 250_000;
///         for i in start..end-1 {
///             dss_clone.union(i, i + 1); // Connect consecutive nodes
///         }
///     });
///     handles.push(handle);
/// }
///
/// // Wait for all threads
/// for handle in handles {
///     handle.join().unwrap();
/// }
///
/// // Verify connectivity within each partition
/// for thread_id in 0..4 {
///     let start = thread_id * 250_000;
///     let end = start + 250_000;
///     let set_id = dss.set_id_of(start);
///     for i in start+1..end {
///         assert_eq!(dss.set_id_of(i), set_id);
///     }
/// }
/// ```
pub struct HugeAtomicDisjointSetStruct {
    /// Parent pointers for the union-find tree
    parent: HugeAtomicLongArray,
    /// Optional community IDs for incremental seeding
    communities: Option<HugeAtomicLongArray>,
    /// Maximum community ID assigned (for generating new IDs)
    max_community_id: Option<AtomicUsize>,
}

const NO_SUCH_SEED_VALUE: i64 = -1;

impl HugeAtomicDisjointSetStruct {
    /// Creates a new disjoint set structure with the specified capacity.
    ///
    /// Each element is initially in its own set (parent points to itself).
    ///
    /// # Arguments
    ///
    /// * `capacity` - Number of elements to support
    /// * `concurrency` - Number of threads for parallel initialization
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::dss::HugeAtomicDisjointSetStruct;
    ///
    /// // Create DSS for 1 million nodes
    /// let dss = HugeAtomicDisjointSetStruct::new(1_000_000);
    ///
    /// // Each element starts in its own set
    /// assert_eq!(dss.set_id_of(0), 0);
    /// assert_eq!(dss.set_id_of(999_999), 999_999);
    /// ```
    pub fn new(capacity: usize) -> Self {
        // Create array and initialize with identity (each node is its own parent)
        let parent = HugeAtomicLongArray::new(capacity);
        for i in 0..capacity {
            parent.set(i, i as i64);
        }

        Self {
            parent,
            communities: None,
            max_community_id: None,
        }
    }

    /// Creates a new disjoint set structure with initial community seeding.
    ///
    /// This supports incremental community detection where some nodes
    /// already have assigned community IDs.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Number of elements to support
    /// * `community_mapping` - Function providing initial community ID for each node (returns -1 for unseeded)
    /// * `concurrency` - Number of threads for parallel initialization
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::dss::HugeAtomicDisjointSetStruct;
    ///
    /// // Seed first 100 nodes with community IDs, rest unseeded
    /// let dss = HugeAtomicDisjointSetStruct::with_communities(
    ///     1000,
    ///     |node_id| if node_id < 100 { node_id as i64 / 10 } else { -1 }
    /// );
    ///
    /// // Seeded nodes have their assigned community IDs
    /// assert_eq!(dss.set_id_of(0), 0);
    /// assert_eq!(dss.set_id_of(15), 1);
    /// ```
    pub fn with_communities<F>(capacity: usize, community_mapping: F) -> Self
    where
        F: Fn(usize) -> i64,
    {
        // Initialize parent array with identity
        let parent = HugeAtomicLongArray::new(capacity);
        for i in 0..capacity {
            parent.set(i, i as i64);
        }

        // Initialize communities array with mapping function
        let communities = HugeAtomicLongArray::new(capacity);
        let mut max_value = 0i64;
        for i in 0..capacity {
            let seed_community = community_mapping(i);
            let value = if seed_community < 0 {
                NO_SUCH_SEED_VALUE
            } else {
                seed_community
            };
            communities.set(i, value);
            if value > max_value {
                max_value = value;
            }
        }

        Self {
            parent,
            communities: Some(communities),
            max_community_id: Some(AtomicUsize::new(max_value as usize)),
        }
    }

    /// Gets the parent of a node (internal helper).
    #[inline]
    fn parent(&self, id: usize) -> usize {
        self.parent.get(id) as usize
    }

    /// Finds the representative (root) of the set containing the specified node.
    ///
    /// Uses path halving for efficiency: each node is made to point to its grandparent.
    ///
    /// # Arguments
    ///
    /// * `id` - Node ID to find the representative for
    ///
    /// # Returns
    ///
    /// The representative (root) node ID
    ///
    /// # Algorithm
    ///
    /// Path halving works as follows:
    /// 1. Follow parent pointers toward the root
    /// 2. For each node, try to make it point to its grandparent (via CAS)
    /// 3. Even if CAS fails, continue toward the root
    /// 4. Eventually reach the root node (where parent == self)
    ///
    /// This provides most of the benefits of full path compression
    /// while being wait-free (no retry loops).
    fn find(&self, mut id: usize) -> usize {
        loop {
            let parent = self.parent(id);
            if id == parent {
                return id; // Reached root
            }

            let grandparent = self.parent(parent);
            if parent != grandparent {
                // Try to apply path-halving by setting the value
                // for some id to its grand parent. This might fail
                // if another thread is also changing the same value
                // but that's ok. The CAS operation guarantees
                // that at least one of the contending threads will
                // succeed. That's enough for the path-halving to work
                // and there is no need to retry in case of a CAS failure.
                let _ = self
                    .parent
                    .compare_and_set(id, parent as i64, grandparent as i64);
            }

            id = grandparent;
        }
    }
}

impl DisjointSetStruct for HugeAtomicDisjointSetStruct {
    fn union(&self, mut id1: usize, mut id2: usize) {
        loop {
            id1 = self.find(id1);
            id2 = self.find(id2);

            if id1 == id2 {
                return; // Already in same set
            }

            // Union-by-Min: smaller community ID wins
            // We also only update the entry for id1 and if that
            // is the smaller value, we need to swap ids so we update
            // only the value for id2, not id1.
            if self.set_id_of(id1) < self.set_id_of(id2) {
                std::mem::swap(&mut id1, &mut id2);
            }

            let old_entry = id1 as i64;
            let new_entry = id2 as i64;

            if self.parent.compare_and_set(id1, old_entry, new_entry) {
                return; // Success
            }

            // CAS failed, another thread modified the parent
            // Retry the union operation
        }
    }

    fn set_id_of(&self, node_id: usize) -> usize {
        let set_id = self.find(node_id);

        // If no communities are defined, return the representative directly
        let communities = match &self.communities {
            None => return set_id,
            Some(c) => c,
        };

        // Otherwise, get or generate a community ID
        loop {
            let provided_set_id = communities.get(set_id);

            if provided_set_id >= 0 {
                return provided_set_id as usize;
            }

            // Need to assign a new community ID
            let new_set_id = self
                .max_community_id
                .as_ref()
                .unwrap()
                .fetch_add(1, Ordering::SeqCst)
                + 1;

            if communities.compare_and_set(set_id, provided_set_id, new_set_id as i64) {
                return new_set_id;
            }

            // CAS failed, another thread assigned an ID
            // Retry to read the assigned value
        }
    }

    fn same_set(&self, mut id1: usize, mut id2: usize) -> bool {
        loop {
            id1 = self.find(id1);
            id2 = self.find(id2);

            if id1 == id2 {
                return true;
            }

            if self.parent(id1) == id1 {
                return false;
            }

            // id1 might have been updated by another thread
            // Retry to ensure we have the current root
        }
    }

    fn size(&self) -> usize {
        self.parent.size()
    }
}

// Thread safety: All atomic operations are thread-safe
unsafe impl Send for HugeAtomicDisjointSetStruct {}
unsafe impl Sync for HugeAtomicDisjointSetStruct {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_create() {
        let dss = HugeAtomicDisjointSetStruct::new(100);
        assert_eq!(dss.size(), 100);

        // Each element is initially its own set
        for i in 0..100 {
            assert_eq!(dss.set_id_of(i), i);
        }
    }

    #[test]
    fn test_union_basic() {
        let dss = HugeAtomicDisjointSetStruct::new(10);

        dss.union(1, 2);
        assert!(dss.same_set(1, 2));
        assert_eq!(dss.set_id_of(1), dss.set_id_of(2));
    }

    #[test]
    fn test_union_transitive() {
        let dss = HugeAtomicDisjointSetStruct::new(10);

        dss.union(1, 2);
        dss.union(2, 3);
        dss.union(3, 4);

        // All should be in same set
        assert!(dss.same_set(1, 2));
        assert!(dss.same_set(2, 3));
        assert!(dss.same_set(3, 4));
        assert!(dss.same_set(1, 4));

        let set_id = dss.set_id_of(1);
        assert_eq!(dss.set_id_of(2), set_id);
        assert_eq!(dss.set_id_of(3), set_id);
        assert_eq!(dss.set_id_of(4), set_id);
    }

    #[test]
    fn test_multiple_components() {
        let dss = HugeAtomicDisjointSetStruct::new(20);

        // Component 1: 0-4
        dss.union(0, 1);
        dss.union(1, 2);
        dss.union(2, 3);
        dss.union(3, 4);

        // Component 2: 10-14
        dss.union(10, 11);
        dss.union(11, 12);
        dss.union(12, 13);
        dss.union(13, 14);

        // Verify within-component connectivity
        assert!(dss.same_set(0, 4));
        assert!(dss.same_set(10, 14));

        // Verify between-component separation
        assert!(!dss.same_set(0, 10));
        assert!(!dss.same_set(4, 14));
    }

    #[test]
    fn test_concurrent_unions() {
        let dss = Arc::new(HugeAtomicDisjointSetStruct::new(1000));
        let mut handles = vec![];

        // Each thread connects consecutive nodes in its range
        for thread_id in 0..4 {
            let dss_clone = Arc::clone(&dss);
            let handle = thread::spawn(move || {
                let start = thread_id * 250;
                let end = start + 250;
                for i in start..end - 1 {
                    dss_clone.union(i, i + 1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify each partition is connected
        for thread_id in 0..4 {
            let start = thread_id * 250;
            let end = start + 250;
            let set_id = dss.set_id_of(start);
            for i in start + 1..end {
                assert_eq!(
                    dss.set_id_of(i),
                    set_id,
                    "Thread {} range broken",
                    thread_id
                );
            }
        }
    }

    #[test]
    fn test_with_communities() {
        let dss = HugeAtomicDisjointSetStruct::with_communities(100, |node_id| {
            if node_id < 50 {
                node_id as i64 / 10
            } else {
                -1
            }
        });

        // Nodes 0-9 should have community 0
        let set_id_0 = dss.set_id_of(5);
        assert_eq!(set_id_0, 0);

        // Nodes 10-19 should have community 1
        let set_id_1 = dss.set_id_of(15);
        assert_eq!(set_id_1, 1);

        // Nodes 50+ are unseeded and should get newly assigned community IDs (starting from max_value+1 = 5)
        let set_id_50 = dss.set_id_of(50);
        assert!(set_id_50 >= 5); // Should be a new community ID >= 5 (max seeded value was 4)

        // Each unseeded node should get its own community initially
        let set_id_51 = dss.set_id_of(51);
        assert!(set_id_51 >= 5);
        assert_ne!(set_id_50, set_id_51); // Different unseeded nodes have different communities initially
    }

    #[test]
    fn test_large_scale() {
        let size = 100_000;
        let dss = HugeAtomicDisjointSetStruct::new(size);

        // Create a long chain
        for i in 0..size - 1 {
            dss.union(i, i + 1);
        }

        // All should be in same set
        let set_id = dss.set_id_of(0);
        assert_eq!(dss.set_id_of(size / 2), set_id);
        assert_eq!(dss.set_id_of(size - 1), set_id);
    }

    #[test]
    fn test_idempotent_union() {
        let dss = HugeAtomicDisjointSetStruct::new(10);

        dss.union(1, 2);
        let set_id_before = dss.set_id_of(1);

        // Union again - should be no-op
        dss.union(1, 2);
        let set_id_after = dss.set_id_of(1);

        assert_eq!(set_id_before, set_id_after);
    }
}

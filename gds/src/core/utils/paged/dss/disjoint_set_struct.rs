//! Disjoint Set Structure (Union-Find) interface for tracking partitioned elements.
//!
//! Essential for graph algorithms requiring connected component analysis:
//! - Union-Find operations for efficient set merging
//! - Path compression and union by rank optimizations
//! - Connected component detection in large graphs
//! - Clustering and community detection algorithms
//! - Cycle detection in undirected graphs
//!
//! # Performance Characteristics
//!
//! - Union operation: Nearly O(1) amortized with optimizations
//! - Find operation: Nearly O(1) amortized with path compression
//! - Space complexity: O(n) for n elements
//! - Excellent cache locality with proper implementation
//! - Supports billions of elements efficiently
//!
//! # Algorithm Optimizations
//!
//! - Path compression: flattens tree structure during find
//! - Union by rank/size: keeps trees balanced
//! - Inverse Ackermann function complexity: α(n) ≈ constant
//! - Memory-efficient representation with huge arrays
//! - Batch processing for high-throughput scenarios
//!
//! # Use Cases
//!
//! - Connected components in undirected graphs
//! - Community detection and clustering
//! - Minimum spanning tree algorithms (Kruskal's)
//! - Image segmentation and blob detection
//! - Network connectivity analysis
//! - Social network cluster analysis
//!
//! # Examples
//!
//! ```rust
//! use gds::core::utils::paged::dss::DisjointSetStruct;
//! use gds::core::utils::paged::dss::HugeAtomicDisjointSetStruct;
//!
//! // Create union-find structure for 1000 nodes
//! let dss = HugeAtomicDisjointSetStruct::new(1000, 4);
//!
//! // Connect nodes in a graph component
//! dss.union(1, 2);  // Connect nodes 1 and 2
//! dss.union(2, 3);  // Connect node 3 to the component
//! dss.union(5, 6);  // Create separate component
//!
//! assert!(dss.same_set(1, 3));  // true - connected through node 2
//! assert!(!dss.same_set(1, 5)); // false - different components
//! ```

/// Disjoint-set-struct is a data structure that keeps track of a set
/// of elements partitioned into a number of disjoint (non-overlapping) subsets.
///
/// # Algorithm Background
///
/// The disjoint-set data structure provides near-constant-time operations
/// to add new sets, merge existing sets, and determine whether elements
/// are in the same set. This is achieved through two key optimizations:
///
/// 1. **Path Compression**: During find operations, flatten the tree by making
///    nodes point directly to the root
/// 2. **Union by Rank/Size**: When merging sets, attach the smaller tree under
///    the root of the larger tree
///
/// These optimizations give an amortized time complexity of α(n) per operation,
/// where α is the inverse Ackermann function, which grows extremely slowly
/// (effectively constant for all practical values of n).
///
/// # References
///
/// - [Wikipedia: Disjoint-set data structure](https://en.wikipedia.org/wiki/Disjoint-set_data_structure)
/// - Tarjan, R. E., van Leeuwen, J. (1984). "Worst-case analysis of set union algorithms"
pub trait DisjointSetStruct {
    /// Joins the set of p (Sp) with set of q (Sq).
    ///
    /// After this operation, `set_id_of(p)` will equal `set_id_of(q)`.
    ///
    /// # Arguments
    ///
    /// * `p` - An element from the first set
    /// * `q` - An element from the second set
    ///
    /// # Performance
    ///
    /// Nearly O(1) amortized with union by rank/size optimization
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
    ///
    /// let dss = HugeAtomicDisjointSetStruct::new(1000);
    ///
    /// // Connect nodes in a graph component
    /// dss.union(1, 2);  // Connect nodes 1 and 2
    /// dss.union(2, 3);  // Connect node 3 to the component
    /// dss.union(5, 6);  // Create separate component
    ///
    /// assert!(dss.same_set(1, 3)); // true - connected through node 2
    /// assert!(!dss.same_set(1, 5)); // false - different components
    /// ```
    fn union(&self, p: usize, q: usize);

    /// Find set Id of element p.
    ///
    /// Elements in the same set will return the same set ID.
    /// The set ID is typically the root element of the set's tree.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The element in the set we are looking for
    ///
    /// # Returns
    ///
    /// An id of the set it belongs to (the root of the tree)
    ///
    /// # Performance
    ///
    /// Nearly O(1) amortized with path compression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
    ///
    /// let dss = HugeAtomicDisjointSetStruct::new(1000);
    ///
    /// // Initially, each element is its own set
    /// assert_eq!(dss.set_id_of(100), 100);
    /// assert_eq!(dss.set_id_of(200), 200);
    ///
    /// // After union, both elements have the same set ID
    /// dss.union(100, 200);
    /// let set_id = dss.set_id_of(100);
    /// assert_eq!(dss.set_id_of(200), set_id);
    /// ```
    fn set_id_of(&self, node_id: usize) -> usize;

    /// Check if p and q belong to the same set.
    ///
    /// This is equivalent to `set_id_of(p) == set_id_of(q)` but may be
    /// optimized to avoid redundant path compression operations.
    ///
    /// # Arguments
    ///
    /// * `p` - A set element
    /// * `q` - Another set element
    ///
    /// # Returns
    ///
    /// `true` if both elements belong to the same set, `false` otherwise
    ///
    /// # Note
    ///
    /// Primarily intended for testing and validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
    ///
    /// let dss = HugeAtomicDisjointSetStruct::new(1000);
    ///
    /// // Test connectivity in graph processing
    /// fn process_edge(dss: &impl DisjointSetStruct, source: usize, target: usize) {
    ///     if !dss.same_set(source, target) {
    ///         dss.union(source, target);
    ///         println!("Connected components {} and {}", source, target);
    ///     }
    /// }
    ///
    /// process_edge(&dss, 1, 2);
    /// assert!(dss.same_set(1, 2));
    /// ```
    fn same_set(&self, p: usize, q: usize) -> bool;

    /// Number of elements stored in the data structure.
    ///
    /// # Returns
    ///
    /// Element count
    ///
    /// # Note
    ///
    /// This returns the total capacity, not the number of disjoint sets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
    ///
    /// let dss = HugeAtomicDisjointSetStruct::new(1000);
    /// assert_eq!(dss.size(), 1000);
    ///
    /// // Size doesn't change with union operations
    /// dss.union(1, 2);
    /// assert_eq!(dss.size(), 1000);
    /// ```
    fn size(&self) -> usize;
}

//! Disjoint Set Structure (Union-Find) implementations.
//!
//! Core data structure for efficiently tracking partitioned elements
//! and performing union-find operations on large graphs.

mod disjoint_set_struct;
mod huge_atomic_disjoint_set_struct;

pub use disjoint_set_struct::DisjointSetStruct;
pub use huge_atomic_disjoint_set_struct::HugeAtomicDisjointSetStruct;

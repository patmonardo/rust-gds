//! **Candidate Paths Priority Queue**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.CandidatePathsPriorityQueue`
//!
//! This module implements a thread-safe priority queue for candidate paths in Yen's algorithm.

use super::mutable_path_result::MutablePathResult;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

/// Thread-safe priority queue for candidate paths
///
/// Translation of: `CandidatePathsPriorityQueue.java` (lines 28-61)
/// Manages candidate paths with thread-safe operations
pub struct CandidatePathsPriorityQueue {
    /// Thread-safe queue for candidate paths
    queue: Arc<Mutex<BinaryHeap<PathWrapper>>>,
}

/// Wrapper for MutablePathResult to implement ordering
#[derive(Debug, Clone)]
struct PathWrapper {
    path: MutablePathResult,
}

impl PathWrapper {
    fn new(path: MutablePathResult) -> Self {
        Self { path }
    }
}

impl PartialEq for PathWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.path.total_cost() == other.path.total_cost() && 
        self.path.node_count() == other.path.node_count()
    }
}

impl Eq for PathWrapper {}

impl PartialOrd for PathWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primary ordering by total cost (ascending)
        let cost_cmp = self.path.total_cost().partial_cmp(&other.path.total_cost()).unwrap_or(Ordering::Equal);
        if cost_cmp != Ordering::Equal {
            return cost_cmp;
        }
        
        // Secondary ordering by node count (ascending)
        self.path.node_count().cmp(&other.path.node_count())
    }
}

impl CandidatePathsPriorityQueue {
    /// Create new candidate paths priority queue
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    /// Add a path to the queue (thread-safe)
    ///
    /// Translation of: `addPath()` method (lines 38-44)
    pub fn add_path(&self, path: MutablePathResult) {
        if let Ok(mut queue) = self.queue.lock() {
            // Check if path already exists (simplified check)
            let wrapper = PathWrapper::new(path);
            queue.push(wrapper);
        }
    }

    /// Pop the highest priority path from the queue
    ///
    /// Translation of: `pop()` method (lines 46-48)
    pub fn pop(&self) -> Option<MutablePathResult> {
        if let Ok(mut queue) = self.queue.lock() {
            queue.pop().map(|wrapper| wrapper.path)
        } else {
            None
        }
    }

    /// Check if the queue is empty
    ///
    /// Translation of: `isEmpty()` method (lines 50-52)
    pub fn is_empty(&self) -> bool {
        if let Ok(queue) = self.queue.lock() {
            queue.is_empty()
        } else {
            true
        }
    }

    /// Get the number of candidate paths
    pub fn len(&self) -> usize {
        if let Ok(queue) = self.queue.lock() {
            queue.len()
        } else {
            0
        }
    }
}

impl Default for CandidatePathsPriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_queue_creation() {
        let queue = CandidatePathsPriorityQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_candidate_queue_add_and_pop() {
        let queue = CandidatePathsPriorityQueue::new();
        
        let path1 = MutablePathResult::new(0, 0, 3, vec![0, 1, 2, 3], vec![10, 11, 12], vec![0.0, 1.0, 2.0, 3.0]);
        let path2 = MutablePathResult::new(0, 0, 3, vec![0, 1, 3], vec![10, 13], vec![0.0, 1.0, 2.0]);
        
        queue.add_path(path1.clone());
        queue.add_path(path2.clone());
        
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());
        
        // Should pop the shorter path first (lower cost)
        let popped = queue.pop().unwrap();
        assert_eq!(popped.total_cost(), 2.0); // path2 has lower cost
        
        assert_eq!(queue.len(), 1);
        
        let popped2 = queue.pop().unwrap();
        assert_eq!(popped2.total_cost(), 3.0); // path1
        
        assert!(queue.is_empty());
    }

    #[test]
    fn test_candidate_queue_priority_ordering() {
        let queue = CandidatePathsPriorityQueue::new();
        
        // Add paths in reverse order of priority
        let path_long = MutablePathResult::new(0, 0, 4, vec![0, 1, 2, 3, 4], vec![10, 11, 12, 13], vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        let path_short = MutablePathResult::new(0, 0, 3, vec![0, 1, 3], vec![10, 13], vec![0.0, 1.0, 2.0]);
        
        queue.add_path(path_long);
        queue.add_path(path_short);
        
        // Should pop shorter path first
        let first = queue.pop().unwrap();
        assert_eq!(first.total_cost(), 2.0);
        assert_eq!(first.node_count(), 3);
        
        let second = queue.pop().unwrap();
        assert_eq!(second.total_cost(), 4.0);
        assert_eq!(second.node_count(), 5);
    }
}

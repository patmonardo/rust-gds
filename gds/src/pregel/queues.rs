//! Pregel Message Queues - Double-buffered message storage for BSP computation
//!
//! Provides efficient message queue implementations for Pregel's Bulk Synchronous Parallel
//! computation model. Messages are stored in queues per node and swapped between iterations.

use crate::collections::HugeObjectArray;
use crate::pregel::MessageIterator;

/// Synchronous double-buffered message queues for BSP Pregel computation.
///
/// Maintains two sets of queues that are swapped between supersteps:
/// - **Read queues**: Messages from the previous iteration (read-only during compute)
/// - **Write queues**: Messages being sent in the current iteration
///
/// This ensures proper BSP semantics where messages sent in iteration N are only
/// visible in iteration N+1.
///
/// # Memory Layout
///
/// ```text
/// Iteration 0:  Read: []          Write: [msg1, msg2, ...]
///              ↓ swap
/// Iteration 1:  Read: [msg1, ...]  Write: [msg3, msg4, ...]
///              ↓ swap
/// Iteration 2:  Read: [msg3, ...]  Write: [msg5, msg6, ...]
/// ```
///
/// # Example
///
/// ```ignore
/// use rust_gds::pregel::SyncDoubleQueues;
///
/// let mut queues = SyncDoubleQueues::new(node_count);
///
/// // Iteration 0: send messages
/// queues.push(target_node, 42.0);
///
/// // Swap queues before next iteration
/// queues.swap();
///
/// // Iteration 1: read messages from previous iteration
/// let messages = queues.messages(node_id);
/// ```
pub struct SyncDoubleQueues {
    /// Current queues for writing messages
    write_queues: HugeObjectArray<Vec<f64>>,
    /// Previous iteration's queues for reading messages
    read_queues: HugeObjectArray<Vec<f64>>,
}

impl SyncDoubleQueues {
    /// Create new double-buffered queues for the given number of nodes.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let queues = SyncDoubleQueues::new(1_000_000);
    /// ```
    pub fn new(node_count: usize) -> Self {
        Self {
            write_queues: HugeObjectArray::new(node_count),
            read_queues: HugeObjectArray::new(node_count),
        }
    }

    /// Push a message to a node's write queue.
    ///
    /// Messages pushed during iteration N will be available to read in iteration N+1
    /// after calling `swap()`.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Target node to receive the message
    /// * `message` - Message value to send
    ///
    /// # Examples
    ///
    /// ```ignore
    /// queues.push(target_node, 42.0);
    /// ```
    pub fn push(&mut self, node_id: usize, message: f64) {
        self.write_queues.get_mut(node_id).push(message);
    }

    /// Swap the read and write queues.
    ///
    /// This should be called at the end of each superstep to make the messages
    /// sent in the current iteration available for reading in the next iteration.
    ///
    /// After swapping:
    /// - The write queues become the new read queues
    /// - The old read queues are cleared and become the new write queues
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // End of iteration
    /// queues.swap();
    /// // Start of next iteration - can now read messages
    /// ```
    pub fn swap(&mut self) {
        // Swap the queues
        std::mem::swap(&mut self.read_queues, &mut self.write_queues);

        // Clear the new write queues (which were the old read queues)
        for i in 0..self.write_queues.size() {
            self.write_queues.get_mut(i).clear();
        }
    }

    /// Get a reference to the messages for a specific node from the previous iteration.
    ///
    /// Returns messages that were sent to this node in the previous superstep.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node to get messages for
    ///
    /// # Returns
    ///
    /// A slice containing all messages sent to this node in the previous iteration
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let messages = queues.messages(node_id);
    /// for &msg in messages {
    ///     println!("Received: {}", msg);
    /// }
    /// ```
    pub fn messages(&self, node_id: usize) -> &[f64] {
        self.read_queues.get(node_id)
    }

    /// Create a message iterator for a specific node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node to create iterator for
    ///
    /// # Returns
    ///
    /// An iterator over the messages for this node
    pub fn iter(&self, node_id: usize) -> SyncQueueIterator<'_> {
        SyncQueueIterator {
            messages: self.read_queues.get(node_id),
            index: 0,
        }
    }
}

/// Iterator over messages in a synchronous queue.
///
/// Provides iteration over f64 message values for a single node.
pub struct SyncQueueIterator<'a> {
    messages: &'a [f64],
    index: usize,
}

impl<'a> Iterator for SyncQueueIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.messages.len() {
            let msg = self.messages[self.index];
            self.index += 1;
            Some(msg)
        } else {
            None
        }
    }
}

impl<'a> MessageIterator for SyncQueueIterator<'a> {
    fn reset(&mut self) {
        self.index = 0;
    }

    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_queues_basic() {
        let mut queues = SyncDoubleQueues::new(10);

        // Send messages in iteration 0
        queues.push(0, 1.0);
        queues.push(0, 2.0);
        queues.push(5, 3.0);

        // Messages not visible yet
        assert_eq!(queues.messages(0).len(), 0);
        assert_eq!(queues.messages(5).len(), 0);

        // Swap queues
        queues.swap();

        // Now messages are visible
        assert_eq!(queues.messages(0), &[1.0, 2.0]);
        assert_eq!(queues.messages(5), &[3.0]);
    }

    #[test]
    fn test_sync_queues_multiple_iterations() {
        let mut queues = SyncDoubleQueues::new(5);

        // Iteration 0
        queues.push(0, 1.0);
        queues.swap();
        assert_eq!(queues.messages(0), &[1.0]);

        // Iteration 1
        queues.push(0, 2.0);
        queues.push(0, 3.0);
        queues.swap();
        assert_eq!(queues.messages(0), &[2.0, 3.0]);

        // Iteration 2
        queues.push(0, 4.0);
        queues.swap();
        assert_eq!(queues.messages(0), &[4.0]);
    }

    #[test]
    fn test_sync_queue_iterator() {
        let mut queues = SyncDoubleQueues::new(5);

        queues.push(0, 1.0);
        queues.push(0, 2.0);
        queues.push(0, 3.0);
        queues.swap();

        let mut iter = queues.iter(0);
        assert!(!iter.is_empty());
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next(), Some(3.0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_sync_queue_iterator_reset() {
        let mut queues = SyncDoubleQueues::new(5);

        queues.push(0, 1.0);
        queues.push(0, 2.0);
        queues.swap();

        let mut iter = queues.iter(0);
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));

        iter.reset();
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));
    }

    #[test]
    fn test_sync_queue_empty() {
        let queues = SyncDoubleQueues::new(5);
        let mut iter = queues.iter(0);
        assert!(iter.is_empty());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_sync_queues_clear_on_swap() {
        let mut queues = SyncDoubleQueues::new(3);

        // Iteration 0
        queues.push(0, 1.0);
        queues.swap();

        // Iteration 1 - send new messages
        queues.push(0, 2.0);

        // Old messages still readable
        assert_eq!(queues.messages(0), &[1.0]);

        // Swap again
        queues.swap();

        // Now only new messages visible
        assert_eq!(queues.messages(0), &[2.0]);
    }

    #[test]
    fn test_iterator_beyond_end() {
        let mut queues = SyncDoubleQueues::new(5);
        queues.push(0, 1.0);
        queues.swap();

        let mut iter = queues.iter(0);
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), None); // Returns None, doesn't panic
        assert_eq!(iter.next(), None); // Still None
    }
}

// ============================================================================
// ASYNCHRONOUS QUEUES
// ============================================================================

/// Asynchronous single-buffered message queues for async Pregel computation.
///
/// Unlike synchronous queues, async queues use a single set of queues with head/tail
/// pointers. Messages can be consumed while new messages are being sent, making them
/// suitable for asynchronous computation models.
///
/// # Memory Layout
///
/// ```text
/// Queue: [msg1, msg2, msg3, msg4, ...]
///         ↑                    ↑
///        head                 tail
///         (read)              (write)
/// ```
///
/// # Compaction
///
/// When a queue's head pointer moves past 25% of its capacity, the queue is
/// compacted to reclaim space and prevent unbounded growth.
///
/// # Example
///
/// ```ignore
/// use rust_gds::pregel::AsyncDoubleQueues;
///
/// let mut queues = AsyncDoubleQueues::new(node_count);
///
/// // Send and read messages in the same iteration
/// queues.push(target_node, 42.0);
///
/// while !queues.is_empty(source_node) {
///     let msg = queues.pop(source_node);
///     // Process message...
/// }
///
/// // Compact periodically to reclaim space
/// queues.compact();
/// ```
pub struct AsyncDoubleQueues {
    /// Message queues with head/tail tracking
    queues: HugeObjectArray<AsyncQueue>,
}

/// A single async queue with head/tail pointers for efficient FIFO operations.
#[derive(Default, Clone)]
struct AsyncQueue {
    messages: Vec<f64>,
    head: usize,
}

impl AsyncDoubleQueues {
    /// Threshold for compaction (when head > 25% of capacity)
    pub const COMPACT_THRESHOLD: f64 = 0.25;

    /// Create new async queues for the given number of nodes.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    pub fn new(node_count: usize) -> Self {
        Self {
            queues: HugeObjectArray::new(node_count),
        }
    }

    /// Push a message to a node's queue.
    ///
    /// Messages are immediately available for reading via `pop()`.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Target node to receive the message
    /// * `message` - Message value to send
    pub fn push(&mut self, node_id: usize, message: f64) {
        let queue = self.queues.get_mut(node_id);
        queue.messages.push(message);
    }

    /// Pop a message from a node's queue.
    ///
    /// Returns the next message in FIFO order.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node to pop message from
    ///
    /// # Returns
    ///
    /// The next message value
    ///
    /// # Panics
    ///
    /// Panics if the queue is empty
    pub fn pop(&mut self, node_id: usize) -> f64 {
        let queue = self.queues.get_mut(node_id);
        if queue.head >= queue.messages.len() {
            panic!("Queue is empty");
        }
        let msg = queue.messages[queue.head];
        queue.head += 1;
        msg
    }

    /// Check if a node's queue is empty.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node to check
    ///
    /// # Returns
    ///
    /// `true` if the queue has no messages available, `false` otherwise
    pub fn is_empty(&self, node_id: usize) -> bool {
        let queue = self.queues.get(node_id);
        queue.head >= queue.messages.len()
    }

    /// Compact all queues to reclaim space.
    ///
    /// Removes consumed messages (before head pointer) from queues where the
    /// head has advanced past the compaction threshold.
    ///
    /// This should be called periodically (e.g., once per iteration) to prevent
    /// memory growth from consumed messages.
    pub fn compact(&mut self) {
        for i in 0..self.queues.size() {
            let queue = self.queues.get_mut(i);
            if queue.head > 0 {
                let capacity = queue.messages.capacity();
                // Compact if head has moved past threshold
                if capacity > 0 && (queue.head as f64 / capacity as f64) > Self::COMPACT_THRESHOLD {
                    // Remove consumed messages
                    queue.messages.drain(0..queue.head);
                    queue.head = 0;
                }
            }
        }
    }

    /// Get all remaining messages for a node.
    ///
    /// Returns a slice of all unread messages in FIFO order.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node to get messages for
    ///
    /// # Returns
    ///
    /// A slice containing all unread messages
    pub fn messages(&self, node_id: usize) -> &[f64] {
        let queue = self.queues.get(node_id);
        &queue.messages[queue.head..]
    }

    /// Create a message iterator for a specific node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node to create iterator for
    ///
    /// # Returns
    ///
    /// An iterator over the unread messages for this node
    pub fn iter(&self, node_id: usize) -> AsyncQueueIterator<'_> {
        AsyncQueueIterator {
            messages: self.messages(node_id),
            index: 0,
        }
    }
}

/// Iterator over messages in an asynchronous queue.
pub struct AsyncQueueIterator<'a> {
    messages: &'a [f64],
    index: usize,
}

impl<'a> Iterator for AsyncQueueIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.messages.len() {
            let msg = self.messages[self.index];
            self.index += 1;
            Some(msg)
        } else {
            None
        }
    }
}

impl<'a> MessageIterator for AsyncQueueIterator<'a> {
    fn reset(&mut self) {
        self.index = 0;
    }

    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

#[cfg(test)]
mod async_tests {
    use super::*;

    #[test]
    fn test_async_queues_basic() {
        let mut queues = AsyncDoubleQueues::new(10);

        queues.push(0, 1.0);
        queues.push(0, 2.0);
        queues.push(5, 3.0);

        // Messages immediately available
        assert!(!queues.is_empty(0));
        assert!(!queues.is_empty(5));
        assert!(queues.is_empty(1));

        assert_eq!(queues.pop(0), 1.0);
        assert_eq!(queues.pop(0), 2.0);
        assert!(queues.is_empty(0));

        assert_eq!(queues.pop(5), 3.0);
        assert!(queues.is_empty(5));
    }

    #[test]
    fn test_async_queues_fifo_order() {
        let mut queues = AsyncDoubleQueues::new(5);

        for i in 0..10 {
            queues.push(0, i as f64);
        }

        for i in 0..10 {
            assert_eq!(queues.pop(0), i as f64);
        }
    }

    #[test]
    fn test_async_queue_iterator() {
        let mut queues = AsyncDoubleQueues::new(5);

        queues.push(0, 1.0);
        queues.push(0, 2.0);
        queues.push(0, 3.0);

        let mut iter = queues.iter(0);
        assert!(!iter.is_empty());
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next(), Some(3.0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_async_queue_compact() {
        let mut queues = AsyncDoubleQueues::new(5);

        // Fill queue
        for i in 0..100 {
            queues.push(0, i as f64);
        }

        // Consume most messages
        for _ in 0..90 {
            queues.pop(0);
        }

        // Before compact, head is at 90
        assert_eq!(queues.messages(0).len(), 10);

        // Compact should reclaim space
        queues.compact();

        // After compact, remaining messages still accessible
        assert_eq!(queues.messages(0).len(), 10);
        assert_eq!(queues.pop(0), 90.0);
    }

    #[test]
    fn test_async_queue_messages() {
        let mut queues = AsyncDoubleQueues::new(5);

        queues.push(0, 1.0);
        queues.push(0, 2.0);
        queues.push(0, 3.0);

        assert_eq!(queues.messages(0), &[1.0, 2.0, 3.0]);

        queues.pop(0);
        assert_eq!(queues.messages(0), &[2.0, 3.0]);

        queues.pop(0);
        assert_eq!(queues.messages(0), &[3.0]);

        queues.pop(0);
        let empty: &[f64] = &[];
        assert_eq!(queues.messages(0), empty);
    }

    #[test]
    #[should_panic(expected = "Queue is empty")]
    fn test_async_queue_pop_empty() {
        let mut queues = AsyncDoubleQueues::new(5);
        queues.pop(0); // Should panic
    }

    #[test]
    fn test_async_queue_interleaved_push_pop() {
        let mut queues = AsyncDoubleQueues::new(5);

        queues.push(0, 1.0);
        queues.push(0, 2.0);
        assert_eq!(queues.pop(0), 1.0);

        queues.push(0, 3.0);
        assert_eq!(queues.pop(0), 2.0);
        assert_eq!(queues.pop(0), 3.0);
        assert!(queues.is_empty(0));
    }
}

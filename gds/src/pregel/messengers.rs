//! Messenger Implementations - Concrete message passing strategies for Pregel
//!
//! Provides three core messenger implementations for different Pregel message passing models:
//!
//! 1. **SyncQueueMessenger**: Double-buffered queues for synchronous BSP message passing
//! 2. **AsyncQueueMessenger**: Single-buffered queues for asynchronous message passing
//! 3. **ReducingMessenger**: Atomic double arrays with reducers for aggregated message passing

use crate::collections::{HugeAtomicDoubleArray, HugeAtomicLongArray};
use crate::pregel::{AsyncDoubleQueues, SyncDoubleQueues};
use crate::pregel::{MessageIterator, MessageReducer, Messenger};

// ================================================================================================
// SyncQueueMessenger - Synchronous BSP message passing
// ================================================================================================

/// Message iterator for synchronous queue-based message passing.
///
/// Stores a copy of messages and provides iteration over them.
/// This iterator is reused across multiple nodes for efficiency.
pub struct SyncQueueMessageIterator {
    messages: Vec<f64>,
    index: usize,
}

impl SyncQueueMessageIterator {
    /// Create a new iterator (typically called by messageIterator())
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            index: 0,
        }
    }

    /// Initialize the iterator for a specific node's messages
    fn init(&mut self, messages: &[f64]) {
        self.messages.clear();
        self.messages.extend_from_slice(messages);
        self.index = 0;
    }
}

impl Iterator for SyncQueueMessageIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.messages.len() {
            let value = self.messages[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl MessageIterator for SyncQueueMessageIterator {
    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}

/// Synchronous queue-based messenger for BSP Pregel computation.
///
/// Uses double-buffered queues that are swapped between supersteps to ensure
/// proper BSP semantics: messages sent in iteration N are only visible in iteration N+1.
///
/// # Example
///
/// ```ignore
/// let mut messenger = SyncQueueMessenger::new(node_count);
///
/// // Iteration 0: send messages
/// messenger.send_to(source, target, 42.0);
///
/// // Swap buffers for next iteration
/// messenger.init_iteration(1);
///
/// // Iteration 1: receive messages
/// let mut iter = messenger.message_iterator();
/// messenger.init_message_iterator(&mut iter, target, false);
/// for msg in iter {
///     // Process messages sent in iteration 0
/// }
/// ```
pub struct SyncQueueMessenger {
    queues: parking_lot::RwLock<SyncDoubleQueues>,
}

impl SyncQueueMessenger {
    /// Create a new synchronous messenger for the given number of nodes.
    pub fn new(node_count: usize) -> Self {
        Self {
            queues: parking_lot::RwLock::new(SyncDoubleQueues::new(node_count)),
        }
    }
}

impl Messenger<SyncQueueMessageIterator> for SyncQueueMessenger {
    fn init_iteration(&self, _iteration: usize) {
        self.queues.write().swap();
    }

    fn send_to(&self, _source_node_id: u64, target_node_id: u64, message: f64) {
        self.queues.write().push(target_node_id as usize, message);
    }

    fn message_iterator(&self) -> SyncQueueMessageIterator {
        SyncQueueMessageIterator::new()
    }

    fn init_message_iterator(
        &self,
        message_iterator: &mut SyncQueueMessageIterator,
        node_id: u64,
        is_first_iteration: bool,
    ) {
        if is_first_iteration {
            // No messages in the first iteration
            message_iterator.init(&[]);
        } else {
            let queues = self.queues.read();
            let messages = queues.messages(node_id as usize);
            message_iterator.init(messages);
        }
    }

    fn release(&self) {
        // HugeObjectArray doesn't require explicit release in Rust
        // Memory will be freed when dropped
    }
}

// ================================================================================================
// AsyncQueueMessenger - Asynchronous message passing
// ================================================================================================

/// Message iterator for asynchronous queue-based message passing.
///
/// Iterates over messages that may have been sent in the current iteration.
/// Unlike the sync version, this supports immediate message visibility.
pub struct AsyncQueueMessageIterator {
    messages: Vec<f64>,
    index: usize,
}

impl AsyncQueueMessageIterator {
    /// Create a new iterator
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            index: 0,
        }
    }

    /// Initialize the iterator for a specific node's messages
    fn init(&mut self, messages: &[f64]) {
        self.messages.clear();
        self.messages.extend_from_slice(messages);
        self.index = 0;
    }
}

impl Iterator for AsyncQueueMessageIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.messages.len() {
            let value = self.messages[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl MessageIterator for AsyncQueueMessageIterator {
    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}

/// Asynchronous queue-based messenger for Pregel computation.
///
/// Uses single-buffered queues where messages are immediately visible.
/// This supports asynchronous computation models where nodes can read
/// messages sent in the same iteration.
///
/// # Example
///
/// ```ignore
/// let mut messenger = AsyncQueueMessenger::new(node_count);
///
/// // Send a message
/// messenger.send_to(source, target, 42.0);
///
/// // Immediately read it (async model)
/// let mut iter = messenger.message_iterator();
/// messenger.init_message_iterator(&mut iter, target, false);
/// for msg in iter {
///     // Process messages
/// }
/// ```
pub struct AsyncQueueMessenger {
    queues: parking_lot::RwLock<AsyncDoubleQueues>,
}

impl AsyncQueueMessenger {
    /// Create a new asynchronous messenger for the given number of nodes.
    pub fn new(node_count: usize) -> Self {
        Self {
            queues: parking_lot::RwLock::new(AsyncDoubleQueues::new(node_count)),
        }
    }
}

impl Messenger<AsyncQueueMessageIterator> for AsyncQueueMessenger {
    fn init_iteration(&self, iteration: usize) {
        if iteration > 0 {
            self.queues.write().compact();
        }
    }

    fn send_to(&self, _source_node_id: u64, target_node_id: u64, message: f64) {
        assert!(!message.is_nan(), "Cannot send NaN as a message");
        self.queues.write().push(target_node_id as usize, message);
    }

    fn message_iterator(&self) -> AsyncQueueMessageIterator {
        AsyncQueueMessageIterator::new()
    }

    fn init_message_iterator(
        &self,
        message_iterator: &mut AsyncQueueMessageIterator,
        node_id: u64,
        _is_first_iteration: bool,
    ) {
        // In async mode, messages are always available
        let queues = self.queues.read();
        let messages = queues.messages(node_id as usize);
        message_iterator.init(messages);
    }

    fn release(&self) {
        // Memory will be freed when dropped
    }
}

// ================================================================================================
// ReducingMessenger - Atomic message reduction
// ================================================================================================

/// Message iterator for reduced messages.
///
/// Since messages are reduced to a single value per node, this iterator
/// yields at most one message.
pub struct ReducingMessageIterator {
    has_message: bool,
    message: f64,
    sender: Option<u64>,
    consumed: bool,
}

impl ReducingMessageIterator {
    /// Create a new reducing message iterator
    fn new() -> Self {
        Self {
            has_message: false,
            message: 0.0,
            sender: None,
            consumed: false,
        }
    }

    /// Initialize the iterator with a message value
    fn init(&mut self, message: f64, has_message: bool, sender: Option<u64>) {
        self.message = message;
        self.has_message = has_message;
        self.sender = sender;
        self.consumed = false;
    }
}

impl Iterator for ReducingMessageIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_message && !self.consumed {
            self.consumed = true;
            Some(self.message)
        } else {
            None
        }
    }
}

impl MessageIterator for ReducingMessageIterator {
    fn is_empty(&self) -> bool {
        !self.has_message
    }

    fn reset(&mut self) {
        self.consumed = false;
    }

    fn sender(&self) -> Option<u64> {
        self.sender
    }
}

/// Reducing messenger that uses atomic arrays to aggregate messages.
///
/// Multiple messages sent to the same node are combined using a `MessageReducer`,
/// reducing memory consumption and improving performance. This is ideal for
/// algorithms like PageRank where messages are summed.
///
/// Uses double-buffered atomic arrays that are swapped between iterations.
///
/// # Example
///
/// ```ignore
/// use rust_gds::pregel::{ReducingMessenger, SumReducer};
///
/// let reducer = Box::new(SumReducer);
/// let mut messenger = ReducingMessenger::new(node_count, reducer, false);
///
/// // Multiple messages to the same node are summed
/// messenger.send_to(src1, target, 5.0);
/// messenger.send_to(src2, target, 3.0);
///
/// messenger.init_iteration(1);
///
/// // Receive single reduced message (8.0)
/// let mut iter = messenger.message_iterator();
/// messenger.init_message_iterator(&mut iter, target, false);
/// assert_eq!(iter.next(), Some(8.0));
/// ```
pub struct ReducingMessenger {
    send_array: parking_lot::RwLock<HugeAtomicDoubleArray>,
    receive_array: parking_lot::RwLock<HugeAtomicDoubleArray>,
    reducer: Box<dyn MessageReducer<f64>>,
    track_sender: bool,
    send_sender_array: Option<parking_lot::RwLock<HugeAtomicLongArray>>,
    receive_sender_array: Option<parking_lot::RwLock<HugeAtomicLongArray>>,
}

impl ReducingMessenger {
    /// Create a new reducing messenger.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Number of nodes in the graph
    /// * `reducer` - Message reducer for combining messages
    /// * `track_sender` - Whether to track message senders
    pub fn new(
        node_count: usize,
        reducer: Box<dyn MessageReducer<f64>>,
        track_sender: bool,
    ) -> Self {
        assert!(
            !reducer.identity().is_nan(),
            "Reducer identity element must not be NaN"
        );

        let send_array = HugeAtomicDoubleArray::new(node_count);
        let receive_array = HugeAtomicDoubleArray::new(node_count);

        // Initialize both arrays with identity value
        let identity = reducer.identity();
        for i in 0..node_count {
            send_array.set(i, identity);
            receive_array.set(i, identity);
        }

        let (send_sender_array, receive_sender_array) = if track_sender {
            (
                Some(parking_lot::RwLock::new(HugeAtomicLongArray::new(
                    node_count,
                ))),
                Some(parking_lot::RwLock::new(HugeAtomicLongArray::new(
                    node_count,
                ))),
            )
        } else {
            (None, None)
        };

        Self {
            send_array: parking_lot::RwLock::new(send_array),
            receive_array: parking_lot::RwLock::new(receive_array),
            reducer,
            track_sender,
            send_sender_array,
            receive_sender_array,
        }
    }
}

impl Messenger<ReducingMessageIterator> for ReducingMessenger {
    fn init_iteration(&self, _iteration: usize) {
        // Swap the arrays - need write locks
        let mut send = self.send_array.write();
        let mut receive = self.receive_array.write();
        std::mem::swap(&mut *send, &mut *receive);

        // Swap sender arrays if tracking
        if self.track_sender {
            let mut send_sender = self.send_sender_array.as_ref().unwrap().write();
            let mut receive_sender = self.receive_sender_array.as_ref().unwrap().write();
            std::mem::swap(&mut *send_sender, &mut *receive_sender);
        }

        // Reset send array to identity values
        let identity = self.reducer.identity();
        let size = send.size();
        for i in 0..size {
            send.set(i, identity);
        }
    }

    fn send_to(&self, source_node_id: u64, target_node_id: u64, message: f64) {
        let target = target_node_id as usize;

        if self.track_sender {
            // Atomic update with sender tracking
            let reducer = &self.reducer;
            let send_array = self.send_array.read();
            let send_sender_array = self.send_sender_array.as_ref().unwrap();

            loop {
                let current = send_array.get(target);
                let reduced = reducer.reduce(current, message);

                // compare_and_exchange returns witness value (equals current if successful)
                let witness = send_array.compare_and_exchange(target, current, reduced);

                if (witness - current).abs() < f64::EPSILON {
                    // Success - update sender if the reduced value changed
                    if (reduced - current).abs() > f64::EPSILON {
                        send_sender_array.write().set(target, source_node_id as i64);
                    }
                    break;
                } else {
                    // Another thread updated the value, retry with witness value
                    continue;
                }
            }
        } else {
            // Simple atomic update without sender tracking
            let reducer = &self.reducer;
            let send_array = self.send_array.read();
            loop {
                let current = send_array.get(target);
                let reduced = reducer.reduce(current, message);

                let witness = send_array.compare_and_exchange(target, current, reduced);

                if (witness - current).abs() < f64::EPSILON {
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    fn message_iterator(&self) -> ReducingMessageIterator {
        ReducingMessageIterator::new()
    }

    fn init_message_iterator(
        &self,
        message_iterator: &mut ReducingMessageIterator,
        node_id: u64,
        _is_first_iteration: bool,
    ) {
        let node = node_id as usize;
        let identity = self.reducer.identity();

        // Get and replace with identity (consume the message)
        let receive_array = self.receive_array.read();
        let message = receive_array.get_and_replace(node, identity);
        let has_message = (message - identity).abs() > f64::EPSILON;

        let sender = if self.track_sender && has_message {
            let sender_array = self.receive_sender_array.as_ref().unwrap();
            Some(sender_array.read().get(node) as u64)
        } else {
            None
        };

        message_iterator.init(message, has_message, sender);
    }

    fn sender(&self, node_id: u64) -> Option<u64> {
        if self.track_sender {
            let sender_array = self.receive_sender_array.as_ref().unwrap();
            Some(sender_array.read().get(node_id as usize) as u64)
        } else {
            None
        }
    }

    fn release(&self) {
        // Memory will be freed when dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregel::{MaxReducer, MinReducer, SumReducer};

    #[test]
    fn test_sync_queue_messenger_basic() {
        let messenger = SyncQueueMessenger::new(3);

        // Send messages in iteration 0
        messenger.send_to(0, 1, 1.0);
        messenger.send_to(0, 1, 2.0);
        messenger.send_to(1, 2, 3.0);

        // Init iteration 1 (swap buffers)
        messenger.init_iteration(1);

        // Read messages from iteration 0
        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        let messages: Vec<f64> = iter.collect();
        assert_eq!(messages, vec![1.0, 2.0]);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 2, false);
        assert_eq!(iter.next(), Some(3.0));
    }

    #[test]
    fn test_sync_queue_messenger_first_iteration() {
        let messenger = SyncQueueMessenger::new(3);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 0, true);

        assert!(iter.is_empty());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_async_queue_messenger_basic() {
        let messenger = AsyncQueueMessenger::new(3);

        // Send messages
        messenger.send_to(0, 1, 1.0);
        messenger.send_to(0, 1, 2.0);

        // Messages are immediately available (async model)
        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        let messages: Vec<f64> = iter.collect();
        assert_eq!(messages, vec![1.0, 2.0]);
    }

    #[test]
    fn test_async_queue_messenger_compact() {
        let messenger = AsyncQueueMessenger::new(3);

        messenger.send_to(0, 1, 1.0);
        messenger.init_iteration(0); // Should not compact on first iteration

        messenger.send_to(0, 1, 2.0);
        messenger.init_iteration(1); // Should compact
    }

    #[test]
    #[should_panic(expected = "Cannot send NaN as a message")]
    fn test_async_queue_messenger_rejects_nan() {
        let messenger = AsyncQueueMessenger::new(3);
        messenger.send_to(0, 1, f64::NAN);
    }

    #[test]
    fn test_reducing_messenger_sum() {
        let reducer = Box::new(SumReducer);
        let messenger = ReducingMessenger::new(3, reducer, false);

        // Send multiple messages to same node
        messenger.send_to(0, 1, 5.0);
        messenger.send_to(2, 1, 3.0);

        // Init iteration (swap buffers)
        messenger.init_iteration(1);

        // Should get reduced (summed) message
        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        assert!(!iter.is_empty());
        assert_eq!(iter.next(), Some(8.0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_reducing_messenger_min() {
        let reducer = Box::new(MinReducer);
        let messenger = ReducingMessenger::new(3, reducer, false);

        messenger.send_to(0, 1, 5.0);
        messenger.send_to(2, 1, 3.0);
        messenger.send_to(0, 1, 7.0);

        messenger.init_iteration(1);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        assert_eq!(iter.next(), Some(3.0));
    }

    #[test]
    fn test_reducing_messenger_max() {
        let reducer = Box::new(MaxReducer);
        let messenger = ReducingMessenger::new(3, reducer, false);

        messenger.send_to(0, 1, 5.0);
        messenger.send_to(2, 1, 3.0);
        messenger.send_to(0, 1, 7.0);

        messenger.init_iteration(1);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        assert_eq!(iter.next(), Some(7.0));
    }

    #[test]
    fn test_reducing_messenger_no_messages() {
        let reducer = Box::new(SumReducer);
        let messenger = ReducingMessenger::new(3, reducer, false);

        messenger.init_iteration(1);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        assert!(iter.is_empty());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_reducing_messenger_iterator_reset() {
        let reducer = Box::new(SumReducer);
        let messenger = ReducingMessenger::new(3, reducer, false);

        messenger.send_to(0, 1, 5.0);
        messenger.init_iteration(1);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        assert_eq!(iter.next(), Some(5.0));
        assert_eq!(iter.next(), None);

        iter.reset();
        assert_eq!(iter.next(), Some(5.0));
    }

    #[test]
    fn test_reducing_messenger_with_sender_tracking() {
        let reducer = Box::new(MaxReducer);
        let messenger = ReducingMessenger::new(3, reducer, true);

        messenger.send_to(10, 1, 5.0);
        messenger.send_to(20, 1, 8.0); // This should be tracked as sender
        messenger.send_to(30, 1, 3.0);

        messenger.init_iteration(1);

        let mut iter = messenger.message_iterator();
        messenger.init_message_iterator(&mut iter, 1, false);

        assert_eq!(iter.next(), Some(8.0));
        assert_eq!(iter.sender(), Some(20));
    }

    #[test]
    fn test_message_iterator_traits() {
        // Test that iterators implement MessageIterator properly
        let mut sync_iter = SyncQueueMessageIterator::new();
        sync_iter.init(&[1.0, 2.0, 3.0]);

        assert!(!sync_iter.is_empty());
        assert_eq!(sync_iter.next(), Some(1.0));
        sync_iter.reset();
        assert_eq!(sync_iter.next(), Some(1.0));

        let mut async_iter = AsyncQueueMessageIterator::new();
        async_iter.init(&[4.0, 5.0]);

        assert!(!async_iter.is_empty());
        assert_eq!(async_iter.next(), Some(4.0));
        async_iter.reset();
        assert_eq!(async_iter.next(), Some(4.0));
    }
}

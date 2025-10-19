//! Messages - Message passing infrastructure for Pregel computation
//!
//! Provides the abstraction for message iteration, reduction, and delivery
//! in the Pregel Bulk Synchronous Parallel (BSP) model.

/// Iterator for messages in the Pregel computation.
///
/// Provides iteration over f64 values with optional sender tracking.
/// This is the primitive iterator that backs the `Messages` collection.
pub trait MessageIterator: Iterator<Item = f64> {
    /// Check if the message collection is empty.
    fn is_empty(&self) -> bool;

    /// Reset the iterator to start from the beginning.
    fn reset(&mut self);

    /// Get the sender of the current message, if sender tracking is enabled.
    ///
    /// # Returns
    ///
    /// - `Some(node_id)` if sender tracking is enabled and available
    /// - `None` if sender tracking is disabled or not available
    ///
    /// # Note
    ///
    /// Requires the config's `track_sender()` to return true.
    fn sender(&self) -> Option<u64> {
        None
    }
}

/// Collection of messages received by a node during a Pregel superstep.
///
/// Provides iteration over message values received from neighboring nodes.
/// Messages are the primary communication mechanism in Pregel, allowing
/// vertices to exchange information between supersteps.
///
/// # Example (Conceptual)
///
/// ```ignore
/// fn compute(&mut self, context: &mut ComputeContext<Self::Config>, messages: Messages) {
///     // Iterate over messages
///     for message_value in messages {
///         // Process each message
///     }
///     
///     // Check if empty
///     if messages.is_empty() {
///         // No messages received
///     }
///     
///     // Get sender (if tracking enabled)
///     if let Some(sender_id) = messages.sender() {
///         // Use sender information
///     }
/// }
/// ```
pub struct Messages<I: MessageIterator> {
    iterator: I,
}

impl<I: MessageIterator> Messages<I> {
    /// Create a new Messages instance wrapping a message iterator.
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }

    /// Get the raw message iterator for primitive iteration.
    ///
    /// Returns the underlying iterator for direct access to f64 values.
    pub fn double_iterator(&self) -> &I {
        &self.iterator
    }

    /// Get a mutable reference to the raw message iterator.
    pub fn double_iterator_mut(&mut self) -> &mut I {
        &mut self.iterator
    }

    /// Check if there are any messages.
    pub fn is_empty(&self) -> bool {
        self.iterator.is_empty()
    }

    /// If the computation defined a `MessageReducer`, this method will
    /// return the sender of the aggregated message.
    ///
    /// Depending on the reducer implementation, the sender is deterministically
    /// defined by the reducer (e.g., for Max or Min). In any other case,
    /// the sender will be one of the node IDs that sent messages to that node.
    ///
    /// # Note
    ///
    /// `track_sender()` must return true to enable sender tracking.
    ///
    /// # Returns
    ///
    /// - `Some(node_id)` if a reducer is defined and sender tracking is enabled
    /// - `None` if no reducer is defined or sender tracking is disabled
    pub fn sender(&self) -> Option<u64> {
        self.iterator.sender()
    }
}

impl<I: MessageIterator> Iterator for Messages<I> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

/// Interface for message passing between nodes in a Pregel computation.
///
/// Handles sending messages between nodes and providing iterators for received messages.
/// This is the core infrastructure that enables BSP-style message passing in Pregel.
///
/// # Type Parameters
///
/// - `ITERATOR`: The type of message iterator this messenger produces
///
/// # Lifecycle
///
/// 1. `init_iteration()` - Prepare for a new superstep
/// 2. `send_to()` - Send messages during compute phase (called many times)
/// 3. `message_iterator()` - Get an iterator for message delivery
/// 4. `init_message_iterator()` - Initialize iterator for a specific node
/// 5. `release()` - Clean up resources after computation
pub trait Messenger<ITERATOR: MessageIterator>: Send + Sync {
    /// Initialize the messenger for a new iteration/superstep.
    ///
    /// This is called at the start of each superstep to prepare message buffers
    /// and swap message queues (current iteration reads from previous iteration's messages).
    ///
    /// # Interior Mutability
    ///
    /// Takes `&self` to allow usage through `Arc`. Implementations must use interior
    /// mutability (e.g., `RwLock`, `Mutex`, `RefCell`) for queue management.
    fn init_iteration(&self, iteration: usize);

    /// Send a message from a source node to a target node.
    ///
    /// Messages are buffered and will be delivered in the next superstep.
    ///
    /// # Parameters
    ///
    /// - `source_node_id`: The ID of the node sending the message
    /// - `target_node_id`: The ID of the node receiving the message
    /// - `message`: The message value to send (f64)
    ///
    /// # Interior Mutability
    ///
    /// Takes `&self` to allow usage through `Arc`. Implementations must use interior
    /// mutability (e.g., `RwLock`, `Mutex`, `RefCell`) for queue mutations.
    fn send_to(&self, source_node_id: u64, target_node_id: u64, message: f64);

    /// Get a message iterator for reuse.
    ///
    /// Returns a reusable iterator instance that can be initialized
    /// for different nodes via `init_message_iterator()`.
    fn message_iterator(&self) -> ITERATOR;

    /// Initialize a message iterator for a specific node.
    ///
    /// Prepares the iterator to deliver messages for the given node.
    ///
    /// # Parameters
    ///
    /// - `message_iterator`: The iterator to initialize
    /// - `node_id`: The node ID to get messages for
    /// - `is_first_iteration`: Whether this is the first iteration (superstep 0)
    fn init_message_iterator(
        &self,
        message_iterator: &mut ITERATOR,
        node_id: u64,
        is_first_iteration: bool,
    );

    /// Get the sender of the latest message for a node, if sender tracking is enabled.
    ///
    /// # Parameters
    ///
    /// - `node_id`: The ID of the node to get the sender for
    ///
    /// # Returns
    ///
    /// - `Some(sender_id)` if sender tracking is enabled
    /// - `None` if sender tracking is disabled or not available
    fn sender(&self, _node_id: u64) -> Option<u64> {
        None
    }

    /// Release resources used by this messenger.
    ///
    /// Called after the Pregel computation completes to free memory.
    ///
    /// # Interior Mutability
    ///
    /// Takes `&self` to allow usage through `Arc`. Implementations must use interior
    /// mutability if cleanup requires mutation.
    fn release(&self);
}

/// Message reducer for combining multiple messages to the same target.
///
/// A reducer can significantly reduce memory consumption and improve performance
/// by combining messages before delivery. Common use cases:
/// - **Sum**: Add all message values together
/// - **Min/Max**: Keep only the minimum/maximum value
/// - **Count**: Count the number of messages
///
/// # Type Parameters
///
/// - `M`: The message type to reduce (typically f64 for Pregel)
///
/// # Example
///
/// ```ignore
/// struct SumReducer;
///
/// impl MessageReducer<f64> for SumReducer {
///     fn reduce(&self, a: f64, b: f64) -> f64 {
///         a + b
///     }
///
///     fn identity(&self) -> f64 {
///         0.0
///     }
/// }
/// ```
pub trait MessageReducer<M>: Send + Sync {
    /// Combine two messages into one.
    ///
    /// This is called repeatedly to reduce multiple messages:
    /// `reduce(reduce(m1, m2), m3)` etc.
    fn reduce(&self, a: M, b: M) -> M;

    /// Identity element for the reduction (neutral value).
    ///
    /// The identity must satisfy: `reduce(identity(), x) == x`
    fn identity(&self) -> M;
}

/// Empty message iterator (used when no messages are received).
pub struct EmptyMessageIterator;

impl Iterator for EmptyMessageIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl MessageIterator for EmptyMessageIterator {
    fn is_empty(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        // Nothing to reset
    }
}

/// Empty messages collection (used when no messages are received).
pub type EmptyMessages = Messages<EmptyMessageIterator>;

/// Constructor for EmptyMessages.
pub fn empty_messages() -> EmptyMessages {
    Messages::new(EmptyMessageIterator)
}

impl Default for Messages<EmptyMessageIterator> {
    fn default() -> Self {
        empty_messages()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMessageIterator {
        values: Vec<f64>,
        index: usize,
    }

    impl TestMessageIterator {
        fn new(values: Vec<f64>) -> Self {
            Self { values, index: 0 }
        }
    }

    impl Iterator for TestMessageIterator {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    impl MessageIterator for TestMessageIterator {
        fn is_empty(&self) -> bool {
            self.values.is_empty()
        }

        fn reset(&mut self) {
            self.index = 0;
        }
    }

    #[test]
    fn test_messages_iteration() {
        let iter = TestMessageIterator::new(vec![1.0, 2.0, 3.0]);
        let mut messages = Messages::new(iter);

        assert!(!messages.is_empty());
        assert_eq!(messages.next(), Some(1.0));
        assert_eq!(messages.next(), Some(2.0));
        assert_eq!(messages.next(), Some(3.0));
        assert_eq!(messages.next(), None);
    }

    #[test]
    fn test_empty_messages() {
        let messages = empty_messages();
        assert!(messages.is_empty());
        assert_eq!(messages.sender(), None);
    }

    #[test]
    fn test_message_iterator_reset() {
        let mut iter = TestMessageIterator::new(vec![1.0, 2.0]);
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next(), None);

        iter.reset();
        assert_eq!(iter.next(), Some(1.0));
    }

    struct SumReducer;

    impl MessageReducer<f64> for SumReducer {
        fn reduce(&self, a: f64, b: f64) -> f64 {
            a + b
        }

        fn identity(&self) -> f64 {
            0.0
        }
    }

    #[test]
    fn test_message_reducer() {
        let reducer = SumReducer;
        assert_eq!(reducer.reduce(1.0, 2.0), 3.0);
        assert_eq!(reducer.reduce(reducer.identity(), 5.0), 5.0);
        assert_eq!(reducer.identity(), 0.0);
    }
}

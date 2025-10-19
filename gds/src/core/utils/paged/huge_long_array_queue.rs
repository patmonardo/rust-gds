use crate::collections::HugeLongArray;

/// Circular FIFO queue backed by `HugeLongArray`.
///
/// Essential for graph algorithms requiring queue semantics:
/// - Breadth-first search traversal
/// - Level-order traversal in trees
/// - Producer-consumer patterns in parallel graph processing
/// - Stream processing with bounded memory
pub struct HugeLongArrayQueue {
    array: HugeLongArray,
    capacity: usize,
    head: usize,
    tail: usize,
}

impl HugeLongArrayQueue {
    /// Creates a new queue with the specified capacity.
    ///
    /// Note: Actual storage is capacity + 1 to distinguish full vs empty
    /// (classic circular buffer optimization).
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::core::utils::paged::HugeLongArrayQueue;
    ///
    /// let mut queue = HugeLongArrayQueue::new(1000);
    /// queue.add(42);
    /// assert_eq!(queue.remove(), 42);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            array: HugeLongArray::new(capacity + 1),
            capacity: capacity + 1,
            head: 0,
            tail: 0,
        }
    }

    /// Adds an element to the rear of the queue.
    ///
    /// # Panics
    ///
    /// Panics if the queue is at capacity.
    pub fn add(&mut self, value: i64) {
        let new_tail = (self.tail + 1) % self.capacity;
        assert!(new_tail != self.head, "Queue is full");
        self.array.set(self.tail, value);
        self.tail = new_tail;
    }

    /// Removes and returns the element from the front of the queue.
    ///
    /// # Panics
    ///
    /// Panics if the queue is empty.
    pub fn remove(&mut self) -> i64 {
        assert!(!self.is_empty(), "Queue is empty");
        let removed = self.array.get(self.head);
        self.head = (self.head + 1) % self.capacity;
        removed
    }

    /// Returns the front element without removing it.
    ///
    /// # Panics
    ///
    /// Panics if the queue is empty.
    pub fn peek(&self) -> i64 {
        assert!(!self.is_empty(), "Queue is empty");
        self.array.get(self.head)
    }

    /// Returns the current number of elements in the queue.
    pub fn size(&self) -> usize {
        let mut diff = self.tail as isize - self.head as isize;
        if diff < 0 {
            diff += self.capacity as isize;
        }
        diff as usize
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Checks if the queue is at capacity.
    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.capacity == self.head
    }

    /// Returns the remaining capacity.
    pub fn remaining_capacity(&self) -> usize {
        self.capacity - 1 - self.size()
    }

    /// Clears the queue without deallocating storage.
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
    }

    /// Converts the queue to a vector (front to back order).
    pub fn to_vec(&self) -> Vec<i64> {
        let queue_size = self.size();
        let mut result = Vec::with_capacity(queue_size);
        let mut current = self.head;

        for _ in 0..queue_size {
            result.push(self.array.get(current));
            current = (current + 1) % self.capacity;
        }
        result
    }

    /// Adds multiple values to the queue.
    ///
    /// # Panics
    ///
    /// Panics if there is insufficient capacity.
    pub fn add_all(&mut self, values: &[i64]) {
        assert!(
            values.len() <= self.remaining_capacity(),
            "Batch add would exceed capacity: {} > {}",
            values.len(),
            self.remaining_capacity()
        );

        for &value in values {
            self.add(value);
        }
    }

    /// Removes multiple values from the queue.
    ///
    /// Returns values in removal order (FIFO).
    ///
    /// # Panics
    ///
    /// Panics if there are fewer than `count` elements.
    pub fn remove_all(&mut self, count: usize) -> Vec<i64> {
        assert!(
            count <= self.size(),
            "Cannot remove {} elements: only {} available",
            count,
            self.size()
        );

        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.remove());
        }
        result
    }

    /// Drains all elements from the queue.
    ///
    /// Returns values in FIFO order, queue becomes empty.
    pub fn drain(&mut self) -> Vec<i64> {
        let result = self.to_vec();
        self.clear();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::HugeLongArrayQueue;

    #[test]
    fn add_remove_round_trip() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.add(1);
        queue.add(2);
        queue.add(3);

        assert_eq!(queue.remove(), 1);
        assert_eq!(queue.remove(), 2);
        assert_eq!(queue.remove(), 3);
        assert!(queue.is_empty());
    }

    #[test]
    fn peek_does_not_remove() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.add(42);
        assert_eq!(queue.peek(), 42);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.remove(), 42);
    }

    #[test]
    fn circular_wraparound_works() {
        let mut queue = HugeLongArrayQueue::new(3);
        queue.add(1);
        queue.add(2);
        assert_eq!(queue.remove(), 1);
        queue.add(3);
        queue.add(4);
        assert_eq!(queue.remove(), 2);
        assert_eq!(queue.remove(), 3);
        assert_eq!(queue.remove(), 4);
        assert!(queue.is_empty());
    }

    #[test]
    fn size_calculation_handles_wraparound() {
        let mut queue = HugeLongArrayQueue::new(5);
        queue.add(1);
        queue.add(2);
        queue.add(3);
        assert_eq!(queue.size(), 3);

        queue.remove();
        assert_eq!(queue.size(), 2);

        queue.add(4);
        queue.add(5);
        assert_eq!(queue.size(), 4);
    }

    #[test]
    fn clear_resets_state() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.add(1);
        queue.add(2);
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.remaining_capacity(), 10);
    }

    #[test]
    fn batch_operations() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.add_all(&[1, 2, 3, 4]);
        assert_eq!(queue.size(), 4);

        let removed = queue.remove_all(2);
        assert_eq!(removed, vec![1, 2]);
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn drain_empties_queue() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.add_all(&[1, 2, 3]);

        let all = queue.drain();
        assert_eq!(all, vec![1, 2, 3]);
        assert!(queue.is_empty());
    }

    #[test]
    fn to_vec_preserves_order() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.add_all(&[10, 20, 30, 40]);

        let vec = queue.to_vec();
        assert_eq!(vec, vec![10, 20, 30, 40]);
        assert_eq!(queue.size(), 4);
    }

    #[test]
    #[should_panic(expected = "Queue is full")]
    fn add_beyond_capacity_panics() {
        let mut queue = HugeLongArrayQueue::new(2);
        queue.add(1);
        queue.add(2);
        queue.add(3);
    }

    #[test]
    #[should_panic(expected = "Queue is empty")]
    fn remove_empty_panics() {
        let mut queue = HugeLongArrayQueue::new(10);
        queue.remove();
    }

    #[test]
    fn full_detection_works() {
        let mut queue = HugeLongArrayQueue::new(2);
        assert!(!queue.is_full());
        queue.add(1);
        assert!(!queue.is_full());
        queue.add(2);
        assert!(queue.is_full());
    }
}

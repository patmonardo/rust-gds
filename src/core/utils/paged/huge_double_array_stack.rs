use crate::collections::HugeDoubleArray;

/// LIFO stack backed by `HugeDoubleArray`.
///
/// Essential for graph algorithms requiring stack semantics with floating-point values:
/// - DFS traversal with edge weights
/// - Backtracking algorithms with scores
/// - PageRank convergence tracking
/// - Feature propagation in graph neural networks
pub struct HugeDoubleArrayStack {
    array: HugeDoubleArray,
    capacity: usize,
    size: usize,
}

impl HugeDoubleArrayStack {
    /// Creates a new stack with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::core::utils::paged::HugeDoubleArrayStack;
    ///
    /// let stack = HugeDoubleArrayStack::new(1000);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            array: HugeDoubleArray::new(capacity),
            capacity,
            size: 0,
        }
    }

    /// Pushes a value onto the stack.
    ///
    /// # Panics
    ///
    /// Panics if the stack is at capacity.
    pub fn push(&mut self, value: f64) {
        assert!(self.size < self.capacity, "Stack is full");
        self.array.set(self.size, value);
        self.size += 1;
    }

    /// Pops and returns the top value from the stack.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty.
    pub fn pop(&mut self) -> f64 {
        assert!(!self.is_empty(), "Stack is empty");
        self.size -= 1;
        self.array.get(self.size)
    }

    /// Returns the top value without removing it.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty.
    pub fn peek(&self) -> f64 {
        assert!(!self.is_empty(), "Stack is empty");
        self.array.get(self.size - 1)
    }

    /// Returns the current number of elements in the stack.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Checks if the stack is at capacity.
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// Returns the remaining capacity.
    pub fn remaining_capacity(&self) -> usize {
        self.capacity - self.size
    }

    /// Clears the stack without deallocating storage.
    pub fn clear(&mut self) {
        self.size = 0;
    }

    /// Converts the stack to a vector (bottom to top order).
    pub fn to_vec(&self) -> Vec<f64> {
        let mut result = Vec::with_capacity(self.size);
        for i in 0..self.size {
            result.push(self.array.get(i));
        }
        result
    }

    /// Pushes multiple values onto the stack.
    ///
    /// # Panics
    ///
    /// Panics if there is insufficient capacity.
    pub fn push_all(&mut self, values: &[f64]) {
        assert!(
            self.size + values.len() <= self.capacity,
            "Batch push would exceed capacity: {} > {}",
            self.size + values.len(),
            self.capacity
        );

        for &value in values {
            self.array.set(self.size, value);
            self.size += 1;
        }
    }

    /// Pops multiple values from the stack.
    ///
    /// Returns values in pop order (most recent first).
    ///
    /// # Panics
    ///
    /// Panics if there are fewer than `count` elements.
    pub fn pop_all(&mut self, count: usize) -> Vec<f64> {
        assert!(
            count <= self.size,
            "Cannot pop {} elements: only {} available",
            count,
            self.size
        );

        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            self.size -= 1;
            result.push(self.array.get(self.size));
        }
        result
    }

    /// Drains all elements from the stack.
    ///
    /// Returns values in pop order (LIFO), stack becomes empty.
    pub fn drain(&mut self) -> Vec<f64> {
        let count = self.size;
        self.pop_all(count)
    }
}

#[cfg(test)]
mod tests {
    use super::HugeDoubleArrayStack;

    #[test]
    fn push_pop_round_trip() {
        let mut stack = HugeDoubleArrayStack::new(10);
        stack.push(1.0);
        stack.push(2.5);
        stack.push(3.7);

        assert_eq!(stack.pop(), 3.7);
        assert_eq!(stack.pop(), 2.5);
        assert_eq!(stack.pop(), 1.0);
        assert!(stack.is_empty());
    }

    #[test]
    fn peek_does_not_remove() {
        let mut stack = HugeDoubleArrayStack::new(10);
        stack.push(42.5);
        assert_eq!(stack.peek(), 42.5);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), 42.5);
    }

    #[test]
    fn clear_resets_size() {
        let mut stack = HugeDoubleArrayStack::new(10);
        stack.push(1.0);
        stack.push(2.0);
        stack.clear();
        assert!(stack.is_empty());
        assert_eq!(stack.remaining_capacity(), 10);
    }

    #[test]
    fn batch_operations() {
        let mut stack = HugeDoubleArrayStack::new(10);
        stack.push_all(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(stack.size(), 4);

        let popped = stack.pop_all(2);
        assert_eq!(popped, vec![4.0, 3.0]);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn drain_empties_stack() {
        let mut stack = HugeDoubleArrayStack::new(10);
        stack.push_all(&[1.0, 2.0, 3.0]);

        let all = stack.drain();
        assert_eq!(all, vec![3.0, 2.0, 1.0]);
        assert!(stack.is_empty());
    }

    #[test]
    #[should_panic(expected = "Stack is full")]
    fn push_beyond_capacity_panics() {
        let mut stack = HugeDoubleArrayStack::new(2);
        stack.push(1.0);
        stack.push(2.0);
        stack.push(3.0);
    }

    #[test]
    #[should_panic(expected = "Stack is empty")]
    fn pop_empty_panics() {
        let mut stack = HugeDoubleArrayStack::new(10);
        stack.pop();
    }
}

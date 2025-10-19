use crate::collections::{HugeDoubleArray, HugeLongArray};
use crate::mem::Estimate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueueOrder {
    Min,
    Max,
}

impl QueueOrder {
    fn less_than(self, values: &HugeDoubleArray, a: usize, b: usize) -> bool {
        debug_assert!(a < values.size());
        debug_assert!(b < values.size());
        let cost_a = values.get(a);
        let cost_b = values.get(b);
        match self {
            QueueOrder::Min => cost_a < cost_b,
            QueueOrder::Max => cost_a > cost_b,
        }
    }
}

/// Heap-based priority queue storing `usize` elements with `f64` priorities.
pub struct HugeLongPriorityQueue {
    capacity: usize,
    heap: HugeLongArray,
    map_index_to: HugeLongArray,
    cost_values: HugeDoubleArray,
    size: usize,
    order: QueueOrder,
}

impl HugeLongPriorityQueue {
    /// Creates a queue that keeps the smallest priorities (lower cost is better).
    pub fn min(capacity: usize) -> Self {
        Self::new(capacity, QueueOrder::Min)
    }

    /// Creates a queue that keeps the largest priorities (higher cost is better).
    pub fn max(capacity: usize) -> Self {
        Self::new(capacity, QueueOrder::Max)
    }

    fn new(capacity: usize, order: QueueOrder) -> Self {
        let heap_size = Self::heap_size(capacity);
        Self {
            capacity,
            heap: HugeLongArray::new(heap_size),
            map_index_to: HugeLongArray::new(heap_size),
            cost_values: HugeDoubleArray::new(capacity),
            size: 0,
            order,
        }
    }

    /// Estimates the memory usage, in bytes, for a queue with the provided capacity.
    pub fn memory_estimation(capacity: usize) -> usize {
        let heap_size = Self::heap_size(capacity);
        Estimate::size_of_long_array(heap_size)
            + Estimate::size_of_long_array(heap_size)
            + Estimate::size_of_double_array(capacity)
    }

    /// Adds an element with the given cost to the queue.
    pub fn add(&mut self, element: usize, cost: f64) {
        self.assert_element_in_range(element);
        assert!(
            self.size < self.capacity,
            "Queue is full (capacity: {})",
            self.capacity
        );

        self.add_cost(element, cost);
        self.size += 1;
        self.place_element(self.size, element);
        self.up_heap(self.size);
    }

    /// Sets the cost of an element, inserting it if not already present.
    pub fn set(&mut self, element: usize, cost: f64) {
        self.assert_element_in_range(element);
        let existed = self.add_cost(element, cost);
        if existed {
            self.update(element);
        } else {
            assert!(
                self.size < self.capacity,
                "Queue is full (capacity: {})",
                self.capacity
            );
            self.size += 1;
            self.place_element(self.size, element);
            self.up_heap(self.size);
        }
    }

    /// Returns the cost associated with an element.
    pub fn cost(&self, element: usize) -> f64 {
        if element < self.capacity {
            self.cost_values.get(element)
        } else {
            0.0
        }
    }

    /// Checks whether the queue currently contains the element.
    pub fn contains(&self, element: usize) -> bool {
        element < self.capacity && self.map_index_to.get(element) > 0
    }

    /// Returns the element at the top of the queue without removing it.
    pub fn top(&self) -> usize {
        assert!(self.size > 0, "Priority queue is empty");
        self.get_heap_element(1)
    }

    /// Removes and returns the top element. Returns `-1` when the queue is empty.
    pub fn pop(&mut self) -> i64 {
        if self.size == 0 {
            return -1;
        }

        let result = self.get_heap_element(1);
        let last = self.get_heap_element(self.size);
        self.place_element(1, last);
        self.size -= 1;

        if self.size > 0 {
            self.down_heap(1);
        }

        self.remove_cost(result);
        result as i64
    }

    /// Returns the number of stored elements.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns `true` when the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Clears the queue so it can be reused.
    pub fn clear(&mut self) {
        self.size = 0;
        self.map_index_to.fill(0);
    }

    /// Releases the underlying storage. Queue methods should not be used after calling this.
    pub fn release(&mut self) {
        self.size = 0;
        self.capacity = 0;
        self.heap = HugeLongArray::new(0);
        self.map_index_to = HugeLongArray::new(0);
        self.cost_values = HugeDoubleArray::new(0);
    }

    /// Returns the element at the given zero-based position inside the heap.
    pub fn ith(&self, index: usize) -> usize {
        self.get_heap_element(index + 1)
    }

    /// Returns an iterator over the current heap contents (heap order, not priority order).
    pub fn iter(&self) -> QueueIter<'_> {
        QueueIter {
            queue: self,
            position: 1,
        }
    }

    fn heap_size(capacity: usize) -> usize {
        if capacity == 0 {
            2
        } else {
            capacity + 1
        }
    }

    fn add_cost(&mut self, element: usize, cost: f64) -> bool {
        let exists = self.map_index_to.get(element) > 0;
        self.cost_values.set(element, cost);
        exists
    }

    fn remove_cost(&mut self, element: usize) {
        if element < self.map_index_to.size() {
            self.map_index_to.set(element, 0);
        }
    }

    fn update(&mut self, element: usize) {
        let position = self.find_element_position(element);
        if position == 0 {
            return;
        }

        if !self.up_heap(position) && position <= self.size {
            self.down_heap(position);
        }
    }

    fn up_heap(&mut self, mut position: usize) -> bool {
        let node = self.get_heap_element(position);
        let mut moved = false;
        let mut parent = position / 2;

        while parent > 0 {
            let parent_element = self.get_heap_element(parent);
            if !self.less_than(node, parent_element) {
                break;
            }
            self.place_element(position, parent_element);
            position = parent;
            parent = position / 2;
            moved = true;
        }

        self.place_element(position, node);
        moved
    }

    fn down_heap(&mut self, mut position: usize) {
        let node = self.get_heap_element(position);
        loop {
            let left = position * 2;
            if left > self.size {
                break;
            }

            let mut best = left;
            if left < self.size {
                let right = left + 1;
                let left_element = self.get_heap_element(left);
                let right_element = self.get_heap_element(right);
                if self.less_than(right_element, left_element) {
                    best = right;
                }
            }

            let best_element = self.get_heap_element(best);
            if !self.less_than(best_element, node) {
                break;
            }

            self.place_element(position, best_element);
            position = best;
        }

        self.place_element(position, node);
    }

    fn less_than(&self, a: usize, b: usize) -> bool {
        self.order.less_than(&self.cost_values, a, b)
    }

    fn place_element(&mut self, position: usize, element: usize) {
        debug_assert!(position < self.heap.size());
        debug_assert!(element < self.map_index_to.size());
        self.heap.set(position, element as i64);
        self.map_index_to.set(element, position as i64);
    }

    fn find_element_position(&self, element: usize) -> usize {
        if element >= self.map_index_to.size() {
            return 0;
        }
        self.map_index_to.get(element) as usize
    }

    fn get_heap_element(&self, position: usize) -> usize {
        self.heap.get(position) as usize
    }

    fn assert_element_in_range(&self, element: usize) {
        assert!(
            element < self.capacity,
            "Element {} exceeds capacity {}",
            element,
            self.capacity
        );
    }
}

pub struct QueueIter<'a> {
    queue: &'a HugeLongPriorityQueue,
    position: usize,
}

impl<'a> Iterator for QueueIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position <= self.queue.size {
            let value = self.queue.get_heap_element(self.position);
            self.position += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a HugeLongPriorityQueue {
    type Item = usize;
    type IntoIter = QueueIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::HugeLongPriorityQueue;

    #[test]
    fn min_queue_orders_costs() {
        let mut queue = HugeLongPriorityQueue::min(4);
        queue.add(0, 0.9);
        queue.add(1, 0.2);
        queue.add(2, 0.5);
        assert_eq!(queue.top(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 0);
        assert_eq!(queue.pop(), -1);
    }

    #[test]
    fn max_queue_orders_costs() {
        let mut queue = HugeLongPriorityQueue::max(3);
        queue.add(0, 0.9);
        queue.add(1, 0.2);
        queue.add(2, 0.5);
        assert_eq!(queue.top(), 0);
        assert_eq!(queue.pop(), 0);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 1);
        assert!(queue.is_empty());
    }

    #[test]
    fn set_updates_existing_element() {
        let mut queue = HugeLongPriorityQueue::min(3);
        queue.add(0, 0.9);
        queue.add(1, 0.8);
        queue.set(1, 0.1);
        queue.set(2, 0.2);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 0);
    }

    #[test]
    fn clear_resets_queue() {
        let mut queue = HugeLongPriorityQueue::min(2);
        queue.add(0, 0.4);
        queue.add(1, 0.2);
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.pop(), -1);
        queue.add(0, 0.3);
        assert_eq!(queue.top(), 0);
    }

    #[test]
    fn iterates_over_heap_contents() {
        let mut queue = HugeLongPriorityQueue::min(3);
        queue.add(0, 0.3);
        queue.add(1, 0.1);
        queue.add(2, 0.2);
        let collected: Vec<_> = queue.iter().collect();
        assert_eq!(collected.len(), 3);
        assert!(collected.contains(&0));
        assert!(collected.contains(&1));
        assert!(collected.contains(&2));
    }

    #[test]
    fn memory_estimation_accounts_for_heap_map_and_costs() {
        let estimated = HugeLongPriorityQueue::memory_estimation(10);
        assert!(estimated > 0);
    }
}

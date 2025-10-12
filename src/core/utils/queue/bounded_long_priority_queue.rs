use std::cmp::Ordering;

/// A bounded priority queue retaining the best `i64` elements by `f64` priority.
#[derive(Debug)]
pub struct BoundedLongPriorityQueue {
    bound: usize,
    min_value: f64,
    elements: Vec<i64>,
    priorities: Vec<f64>,
    element_count: usize,
    order: QueueOrder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueueOrder {
    Min,
    Max,
}

impl BoundedLongPriorityQueue {
    /// Creates a queue that keeps the smallest priorities (lower is better).
    pub fn min(bound: usize) -> Self {
        Self::new(bound, QueueOrder::Min)
    }

    /// Creates a queue that keeps the largest priorities (higher is better).
    pub fn max(bound: usize) -> Self {
        Self::new(bound, QueueOrder::Max)
    }

    fn new(bound: usize, order: QueueOrder) -> Self {
        assert!(bound > 0, "queue bound must be positive");
        Self {
            bound,
            min_value: f64::NAN,
            elements: vec![0; bound],
            priorities: vec![0.0; bound],
            element_count: 0,
            order,
        }
    }

    /// Tries to add an element, returning `true` if the element was accepted.
    pub fn offer(&mut self, element: i64, priority: f64) -> bool {
        match self.order {
            QueueOrder::Min => self.add(element, priority),
            QueueOrder::Max => self.add(element, -priority),
        }
    }

    /// Iterates over stored elements in priority order.
    pub fn for_each<F>(&self, mut consumer: F)
    where
        F: FnMut(i64, f64),
    {
        match self.order {
            QueueOrder::Min => {
                for i in 0..self.element_count {
                    consumer(self.elements[i], self.priorities[i]);
                }
            }
            QueueOrder::Max => {
                for i in 0..self.element_count {
                    consumer(self.elements[i], -self.priorities[i]);
                }
            }
        }
    }

    /// Returns the stored elements in order of priority.
    pub fn elements(&self) -> Vec<i64> {
        self.elements[..self.element_count].to_vec()
    }

    /// Returns the stored priorities in order.
    pub fn priorities(&self) -> Vec<f64> {
        match self.order {
            QueueOrder::Min => {
                if self.min_value.is_nan() {
                    Vec::new()
                } else {
                    self.priorities[..self.element_count].to_vec()
                }
            }
            QueueOrder::Max => {
                if self.min_value.is_nan() {
                    Vec::new()
                } else {
                    self.priorities[..self.element_count]
                        .iter()
                        .map(|p| -*p)
                        .collect()
                }
            }
        }
    }

    /// Returns the current number of elements in the queue.
    pub fn size(&self) -> usize {
        self.element_count
    }

    /// Checks if the queue already contains the element.
    pub fn contains(&self, element: i64) -> bool {
        self.elements[..self.element_count].contains(&element)
    }

    /// Returns the element at the given index (priority order).
    pub fn element_at(&self, index: usize) -> i64 {
        assert!(index < self.element_count, "index out of range");
        self.elements[index]
    }

    /// Updates the element at the given index.
    pub fn update_element_at(&mut self, index: usize, new_element: i64) {
        assert!(index < self.element_count, "index out of range");
        self.elements[index] = new_element;
    }

    fn add(&mut self, element: i64, priority: f64) -> bool {
        if self.element_count < self.bound || self.min_value.is_nan() || priority < self.min_value {
            let idx = self.binary_search(0, self.element_count, priority);
            let mut idx = if idx < 0 {
                (-idx) as usize
            } else {
                idx as usize + 1
            };

            if idx == 0 {
                idx = 1;
            }

            if idx < self.bound {
                let max = self.element_count.min(self.bound - 1);
                for pos in (idx..=max).rev() {
                    self.priorities[pos] = self.priorities[pos - 1];
                    self.elements[pos] = self.elements[pos - 1];
                }
            }

            self.priorities[idx - 1] = priority;
            self.elements[idx - 1] = element;

            if self.element_count < self.bound {
                self.element_count += 1;
            }

            if self.element_count > 0 {
                self.min_value = self.priorities[self.element_count - 1];
            }

            true
        } else {
            false
        }
    }

    fn binary_search(&self, from: usize, to: usize, key: f64) -> isize {
        if from >= to {
            return -(from as isize) - 1;
        }

        let slice = &self.priorities[from..to];
        match slice.binary_search_by(|value| match value.partial_cmp(&key) {
            Some(order) => order,
            None => Ordering::Greater,
        }) {
            Ok(idx) => (from + idx) as isize,
            Err(insert) => -((from + insert) as isize) - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BoundedLongPriorityQueue;

    #[test]
    fn min_queue_orders_by_priority() {
        let mut queue = BoundedLongPriorityQueue::min(3);
        assert!(queue.offer(1, 0.9));
        assert!(queue.offer(2, 0.1));
        assert!(queue.offer(3, 0.5));
        assert_eq!(queue.elements(), vec![2, 3, 1]);
        assert_eq!(queue.priorities(), vec![0.1, 0.5, 0.9]);
    }

    #[test]
    fn min_queue_rejects_worse_when_full() {
        let mut queue = BoundedLongPriorityQueue::min(2);
        assert!(queue.offer(1, 0.2));
        assert!(queue.offer(2, 0.3));
        assert!(!queue.offer(3, 0.4));
        assert_eq!(queue.elements(), vec![1, 2]);
    }

    #[test]
    fn max_queue_keeps_largest_priorities() {
        let mut queue = BoundedLongPriorityQueue::max(2);
        assert!(queue.offer(1, 0.2));
        assert!(queue.offer(2, 0.5));
        assert!(queue.offer(3, 0.7));
        assert_eq!(queue.elements(), vec![3, 2]);
        assert_eq!(queue.priorities(), vec![0.7, 0.5]);
    }
}

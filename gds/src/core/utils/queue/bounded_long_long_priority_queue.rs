use std::cmp::Ordering;

/// A bounded priority queue retaining the best `(i64, i64)` pairs by `f64` priority.
#[derive(Debug)]
pub struct BoundedLongLongPriorityQueue {
    bound: usize,
    min_value: f64,
    elements1: Vec<i64>,
    elements2: Vec<i64>,
    priorities: Vec<f64>,
    element_count: usize,
    order: QueueOrder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueueOrder {
    Min,
    Max,
}

impl BoundedLongLongPriorityQueue {
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
            elements1: vec![0; bound],
            elements2: vec![0; bound],
            priorities: vec![0.0; bound],
            element_count: 0,
            order,
        }
    }

    /// Tries to add an element pair with the given priority.
    pub fn offer(&mut self, element1: i64, element2: i64, priority: f64) -> bool {
        match self.order {
            QueueOrder::Min => self.add(element1, element2, priority),
            QueueOrder::Max => self.add(element1, element2, -priority),
        }
    }

    /// Iterates over stored pairs in priority order.
    pub fn for_each<F>(&self, mut consumer: F)
    where
        F: FnMut(i64, i64, f64),
    {
        match self.order {
            QueueOrder::Min => {
                for i in 0..self.element_count {
                    consumer(self.elements1[i], self.elements2[i], self.priorities[i]);
                }
            }
            QueueOrder::Max => {
                for i in 0..self.element_count {
                    consumer(self.elements1[i], self.elements2[i], -self.priorities[i]);
                }
            }
        }
    }

    /// Returns the first components in priority order.
    pub fn elements1(&self) -> Vec<i64> {
        self.elements1[..self.element_count].to_vec()
    }

    /// Returns the second components in priority order.
    pub fn elements2(&self) -> Vec<i64> {
        self.elements2[..self.element_count].to_vec()
    }

    /// Returns the priorities in priority order.
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

    /// Returns the number of stored pairs.
    pub fn size(&self) -> usize {
        self.element_count
    }

    fn add(&mut self, element1: i64, element2: i64, priority: f64) -> bool {
        if self.element_count < self.bound || self.min_value.is_nan() || priority < self.min_value {
            let idx_result = self.binary_search(0, self.element_count, priority);
            let mut idx = if idx_result < 0 {
                (-idx_result) as usize
            } else {
                idx_result as usize + 1
            };

            if idx == 0 {
                idx = 1;
            }

            if idx < self.bound {
                let max = self.element_count.min(self.bound - 1);
                for pos in (idx..=max).rev() {
                    self.priorities[pos] = self.priorities[pos - 1];
                    self.elements1[pos] = self.elements1[pos - 1];
                    self.elements2[pos] = self.elements2[pos - 1];
                }
            }

            self.priorities[idx - 1] = priority;
            self.elements1[idx - 1] = element1;
            self.elements2[idx - 1] = element2;

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
    use super::BoundedLongLongPriorityQueue;

    #[test]
    fn min_queue_orders_pairs() {
        let mut queue = BoundedLongLongPriorityQueue::min(3);
        assert!(queue.offer(1, 10, 0.9));
        assert!(queue.offer(2, 20, 0.1));
        assert!(queue.offer(3, 30, 0.5));
        assert_eq!(queue.elements1(), vec![2, 3, 1]);
        assert_eq!(queue.elements2(), vec![20, 30, 10]);
        assert_eq!(queue.priorities(), vec![0.1, 0.5, 0.9]);
    }

    #[test]
    fn max_queue_keeps_largest() {
        let mut queue = BoundedLongLongPriorityQueue::max(2);
        assert!(queue.offer(1, 10, 0.2));
        assert!(queue.offer(2, 20, 0.5));
        assert!(queue.offer(3, 30, 0.7));
        assert_eq!(queue.elements1(), vec![3, 2]);
        assert_eq!(queue.elements2(), vec![30, 20]);
        assert_eq!(queue.priorities(), vec![0.7, 0.5]);
    }
}

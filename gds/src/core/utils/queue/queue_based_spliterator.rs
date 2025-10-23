use crate::concurrency::TerminationFlag;
use std::sync::Arc;
use std::time::Duration;

/// Error returned by blocking queue operations.
#[derive(Debug)]
pub enum BlockingQueueError {
    Interrupted,
}

/// Blocking queue abstraction used by the queue-based spliterator.
pub trait BlockingQueue<T>: Send + Sync + 'static {
    fn poll(&self, timeout: Duration) -> Result<Option<T>, BlockingQueueError>;
}

/// Characteristics flags mirroring Java's `Spliterator` constants.
pub struct SpliteratorCharacteristics;

impl SpliteratorCharacteristics {
    pub const ORDERED: u32 = 0x0000_0010;
    pub const DISTINCT: u32 = 0x0000_0001;
    pub const SORTED: u32 = 0x0000_0004;
    pub const SIZED: u32 = 0x0000_0040;
    pub const NONNULL: u32 = 0x0000_0100;
    pub const IMMUTABLE: u32 = 0x0000_0400;
    pub const CONCURRENT: u32 = 0x0000_1000;
    pub const SUBSIZED: u32 = 0x0000_4000;
}

/// Queue-backed iterator that stops once a tombstone element is observed or the queue times out.
pub struct QueueBasedSpliterator<T, Q>
where
    T: PartialEq + Send + 'static,
    Q: BlockingQueue<T>,
{
    queue: Arc<Q>,
    tombstone: T,
    termination_guard: TerminationFlag,
    timeout: Duration,
    entry: Option<T>,
}

impl<T, Q> QueueBasedSpliterator<T, Q>
where
    T: PartialEq + Send + 'static,
    Q: BlockingQueue<T>,
{
    pub fn new(
        queue: Arc<Q>,
        tombstone: T,
        termination_guard: TerminationFlag,
        timeout: Duration,
    ) -> Self {
        let entry = Self::poll_initial(&queue, timeout);
        Self {
            queue,
            tombstone,
            termination_guard,
            timeout,
            entry,
        }
    }

    pub fn try_advance<F>(&mut self, mut action: F) -> bool
    where
        F: FnMut(T),
    {
        if self.is_end() {
            return false;
        }
        self.termination_guard.assert_running();
        let current = match self.entry.take() {
            Some(value) => value,
            None => return false,
        };
        action(current);
        self.entry = self.poll_next();
        !self.is_end()
    }

    pub fn estimate_size(&self) -> u64 {
        u64::MAX
    }

    pub fn characteristics(&self) -> u32 {
        SpliteratorCharacteristics::NONNULL
    }

    fn is_end(&self) -> bool {
        match &self.entry {
            None => true,
            Some(value) => value == &self.tombstone,
        }
    }

    fn poll_next(&self) -> Option<T> {
        self.queue.poll(self.timeout).unwrap_or_default()
    }

    fn poll_initial(queue: &Arc<Q>, timeout: Duration) -> Option<T> {
        queue.poll(timeout).unwrap_or_default()
    }
}

impl<T, Q> Iterator for QueueBasedSpliterator<T, Q>
where
    T: PartialEq + Send + 'static,
    Q: BlockingQueue<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }
        self.termination_guard.assert_running();
        let current = self.entry.take();
        self.entry = self.poll_next();
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::TerminationFlag;
    use std::collections::VecDeque;
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::{Duration, Instant};

    struct InMemoryBlockingQueue<T> {
        state: Mutex<VecDeque<T>>,
        available: Condvar,
    }

    impl<T> InMemoryBlockingQueue<T> {
        fn new() -> Self {
            Self {
                state: Mutex::new(VecDeque::new()),
                available: Condvar::new(),
            }
        }

        fn offer(&self, item: T) {
            let mut guard = self.state.lock().expect("queue mutex poisoned");
            guard.push_back(item);
            self.available.notify_one();
        }
    }

    impl<T> BlockingQueue<T> for InMemoryBlockingQueue<T>
    where
        T: Send + 'static,
    {
        fn poll(&self, timeout: Duration) -> Result<Option<T>, BlockingQueueError> {
            let mut guard = self.state.lock().expect("queue mutex poisoned");
            if guard.is_empty() {
                let deadline = Instant::now() + timeout;
                let mut remaining = timeout;
                while guard.is_empty() {
                    if remaining.is_zero() {
                        return Ok(None);
                    }
                    let (next_guard, wait_result) = self
                        .available
                        .wait_timeout(guard, remaining)
                        .expect("queue condvar poisoned");
                    guard = next_guard;
                    if !guard.is_empty() {
                        break;
                    }
                    if wait_result.timed_out() {
                        return Ok(None);
                    }
                    let now = Instant::now();
                    if now >= deadline {
                        return Ok(None);
                    }
                    remaining = deadline.saturating_duration_since(now);
                }
            }
            Ok(guard.pop_front())
        }
    }

    #[test]
    fn streams_until_tombstone() {
        let queue = Arc::new(InMemoryBlockingQueue::new());
        queue.offer(1);
        queue.offer(2);
        queue.offer(-1);

        let flag = TerminationFlag::running_true();
        let mut iter = QueueBasedSpliterator::new(queue, -1, flag, Duration::from_millis(10));

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn try_advance_consumes_elements() {
        let queue = Arc::new(InMemoryBlockingQueue::new());
        queue.offer(5);
        queue.offer(7);
        queue.offer(-1);

        let flag = TerminationFlag::running_true();
        let mut spliterator =
            QueueBasedSpliterator::new(queue, -1, flag, Duration::from_millis(10));

        let mut collected = Vec::new();
        while spliterator.try_advance(|value| collected.push(value)) {}
        assert_eq!(collected, vec![5, 7]);
    }

    #[test]
    fn timeout_results_in_end() {
        let queue = Arc::new(InMemoryBlockingQueue::new());
        let flag = TerminationFlag::running_true();
        let mut spliterator = QueueBasedSpliterator::new(queue, -1, flag, Duration::from_millis(1));
        assert_eq!(spliterator.next(), None);
    }
}

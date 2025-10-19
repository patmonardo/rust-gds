//! Computation Runtime for Sum Aggregation
//!
//! This module implements the **Subtle pole** of the Functor machinery.
//! It represents ephemeral computation (accumulation in memory).

/// Computation Runtime for Sum Aggregation
///
/// This is the **Subtle pole** - ephemeral accumulation.
/// It knows how to accumulate values in-memory using a running sum.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent PropertyValues in storage
/// - **Computation Runtime** (Subtle) = ephemeral accumulation values
/// - **Functor** = the mapping between them
#[derive(Debug, Clone, Copy)]
pub struct SumComputationRuntime {
    /// Running accumulator for sum
    sum: f64,
    /// Count of values processed
    count: usize,
}

impl SumComputationRuntime {
    /// Create a new computation runtime
    ///
    /// Initializes with zero sum and zero count.
    pub fn new() -> Self {
        Self { sum: 0.0, count: 0 }
    }

    /// Add a value to the sum
    ///
    /// This is the core operation of the Subtle pole.
    /// Values coming from PropertyValues (Gross) are accumulated here.
    pub fn add_value(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }

    /// Get the current sum
    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Get the count of values processed
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get the average (if any values were processed)
    pub fn average(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
}

impl Default for SumComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_computation_new() {
        let runtime = SumComputationRuntime::new();
        assert_eq!(runtime.sum(), 0.0);
        assert_eq!(runtime.count(), 0);
    }

    #[test]
    fn test_sum_computation_add_value() {
        let mut runtime = SumComputationRuntime::new();
        runtime.add_value(1.0);
        runtime.add_value(2.0);
        runtime.add_value(3.0);

        assert_eq!(runtime.sum(), 6.0);
        assert_eq!(runtime.count(), 3);
    }

    #[test]
    fn test_sum_computation_average() {
        let mut runtime = SumComputationRuntime::new();
        assert_eq!(runtime.average(), None);

        runtime.add_value(1.0);
        runtime.add_value(2.0);
        runtime.add_value(3.0);

        assert_eq!(runtime.average(), Some(2.0));
    }

    #[test]
    fn test_sum_computation_default() {
        let runtime = SumComputationRuntime::default();
        assert_eq!(runtime.sum(), 0.0);
        assert_eq!(runtime.count(), 0);
    }
}

//! Memory budget validation utilities.
//!
//! Validates memory estimations against system memory constraints.

use super::MemoryEstimationResult;
use crate::mem::{Estimate, MemoryRange};

/// Validates memory estimations against system memory constraints.
///
/// # Example
///
/// ```rust,ignore
/// use gds::mem::memest::MemoryBudgetValidator;
///
/// let validator = MemoryBudgetValidator::new(8 * 1024 * 1024 * 1024); // 8 GiB
///
/// if validator.validate(&estimation) {
///     println!("Estimation fits within budget");
///     println!("Remaining: {}", Estimate::human_readable(validator.remaining(&estimation)));
/// } else {
///     println!("Insufficient memory!");
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MemoryBudgetValidator {
    memory_budget: usize,
}

impl MemoryBudgetValidator {
    /// Creates a new memory budget validator.
    ///
    /// # Arguments
    ///
    /// * `memory_budget` - The memory budget in bytes
    pub fn new(memory_budget: usize) -> Self {
        Self { memory_budget }
    }

    /// Validates if a memory estimation fits within the budget.
    ///
    /// Uses the minimum memory requirement from the estimation.
    pub fn validate(&self, estimation: &MemoryEstimationResult) -> bool {
        self.validate_range(estimation.memory_range())
    }

    /// Validates if a memory range fits within the budget.
    ///
    /// Uses the maximum value from the range for conservative validation.
    pub fn validate_range(&self, memory_range: &MemoryRange) -> bool {
        memory_range.max() <= self.memory_budget
    }

    /// Returns the memory budget.
    pub fn budget(&self) -> usize {
        self.memory_budget
    }

    /// Returns the remaining memory after subtracting the estimation.
    ///
    /// Returns 0 if the estimation exceeds the budget.
    pub fn remaining(&self, estimation: &MemoryEstimationResult) -> usize {
        let required = estimation.memory_usage();
        self.memory_budget.saturating_sub(required)
    }

    /// Returns the percentage of the budget that would be used by the estimation.
    ///
    /// Returns a value between 0.0 and 100.0 (capped at 100.0 even if over budget).
    pub fn percentage_used(&self, estimation: &MemoryEstimationResult) -> f64 {
        if self.memory_budget == 0 {
            return 0.0;
        }

        let required = estimation.memory_usage();
        let percentage = (required as f64 / self.memory_budget as f64) * 100.0;

        percentage.clamp(0.0, 100.0)
    }

    /// Formats the remaining memory in human-readable form.
    pub fn format_remaining(&self, estimation: &MemoryEstimationResult) -> String {
        Estimate::human_readable(self.remaining(estimation))
    }

    /// Checks if the estimation exceeds the budget.
    pub fn exceeds_budget(&self, estimation: &MemoryEstimationResult) -> bool {
        !self.validate(estimation)
    }

    /// Returns the deficit if estimation exceeds budget, 0 otherwise.
    pub fn deficit(&self, estimation: &MemoryEstimationResult) -> usize {
        let required = estimation.memory_usage();
        required.saturating_sub(self.memory_budget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::ConcreteGraphDimensions;
    use crate::mem::MemoryTree;

    fn create_estimation(memory_bytes: usize) -> MemoryEstimationResult {
        let dimensions = ConcreteGraphDimensions::of(1000, 5000);
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(memory_bytes));
        MemoryEstimationResult::new(dimensions, tree)
    }

    #[test]
    fn test_validate_within_budget() {
        let validator = MemoryBudgetValidator::new(1024 * 1024); // 1 MiB
        let estimation = create_estimation(512 * 1024); // 512 KiB

        assert!(validator.validate(&estimation));
    }

    #[test]
    fn test_validate_exact_budget() {
        let validator = MemoryBudgetValidator::new(1024);
        let estimation = create_estimation(1024);

        assert!(validator.validate(&estimation));
    }

    #[test]
    fn test_validate_exceeds_budget() {
        let validator = MemoryBudgetValidator::new(512);
        let estimation = create_estimation(1024);

        assert!(!validator.validate(&estimation));
        assert!(validator.exceeds_budget(&estimation));
    }

    #[test]
    fn test_remaining_memory() {
        let validator = MemoryBudgetValidator::new(1024 * 1024);
        let estimation = create_estimation(512 * 1024);

        assert_eq!(validator.remaining(&estimation), 512 * 1024);
    }

    #[test]
    fn test_remaining_memory_over_budget() {
        let validator = MemoryBudgetValidator::new(512);
        let estimation = create_estimation(1024);

        assert_eq!(validator.remaining(&estimation), 0);
    }

    #[test]
    fn test_percentage_used() {
        let validator = MemoryBudgetValidator::new(1000);
        let estimation = create_estimation(250);

        assert_eq!(validator.percentage_used(&estimation), 25.0);
    }

    #[test]
    fn test_percentage_used_over_budget() {
        let validator = MemoryBudgetValidator::new(100);
        let estimation = create_estimation(200);

        // Should cap at 100%
        assert_eq!(validator.percentage_used(&estimation), 100.0);
    }

    #[test]
    fn test_percentage_used_zero_budget() {
        let validator = MemoryBudgetValidator::new(0);
        let estimation = create_estimation(100);

        assert_eq!(validator.percentage_used(&estimation), 0.0);
    }

    #[test]
    fn test_deficit() {
        let validator = MemoryBudgetValidator::new(100);
        let estimation = create_estimation(150);

        assert_eq!(validator.deficit(&estimation), 50);
    }

    #[test]
    fn test_deficit_within_budget() {
        let validator = MemoryBudgetValidator::new(200);
        let estimation = create_estimation(150);

        assert_eq!(validator.deficit(&estimation), 0);
    }

    #[test]
    fn test_validate_range() {
        let validator = MemoryBudgetValidator::new(1000);
        let range = MemoryRange::of_range(500, 900);

        assert!(validator.validate_range(&range));
    }

    #[test]
    fn test_validate_range_exceeds() {
        let validator = MemoryBudgetValidator::new(1000);
        let range = MemoryRange::of_range(500, 1100);

        // Uses max value, so should fail
        assert!(!validator.validate_range(&range));
    }
}

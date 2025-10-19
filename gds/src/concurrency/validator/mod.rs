// Concurrency validation system
//
// This module provides validation for concurrency settings, primarily for
// license enforcement. The "enterprise" pattern exists to enforce limits
// based on license tiers.
//
// ## The Validation "System"
//
// Java made this complex because EJB. The actual logic is just:
//   if (requested > limit) throw error;
//
// But we package it nicely for license-based enforcement!

mod concurrency_validator;
mod open_gds_concurrency_validator;

pub use concurrency_validator::{ConcurrencyValidator, ConcurrencyValidatorError};
pub use open_gds_concurrency_validator::{
    OpenGdsConcurrencyValidator, OpenGdsConcurrencyValidatorBuilder,
};

// Re-export LicenseState from pool for convenience
pub use super::pool::LicenseState;

/// Service singleton for validator access (Java compatibility).
///
/// In Java GDS:
/// ```java
/// public final class ConcurrencyValidatorService {
///     private static ConcurrencyValidator instance = new OpenGdsConcurrencyValidator();
///     public static void validator(ConcurrencyValidator v) { instance = v; }
///     public static ConcurrencyValidator validator() { return instance; }
/// }
/// ```
///
/// In Rust: We provide the same API but recommend using the types directly.
///
/// # Examples
///
/// ```
/// use gds::concurrency::validator::ConcurrencyValidatorService;
///
/// let validator = ConcurrencyValidatorService::validator();
/// // validator.validate(8, "concurrency", 4) would return Err(...)
/// ```
pub struct ConcurrencyValidatorService;

impl ConcurrencyValidatorService {
    /// Default concurrency limit for open source version.
    pub const DEFAULT_CONCURRENCY_LIMIT: usize = 4;

    /// Get the global validator instance.
    ///
    /// Always returns OpenGdsConcurrencyValidator (no mutable global state!).
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::validator::ConcurrencyValidatorService;
    ///
    /// let validator = ConcurrencyValidatorService::validator();
    /// assert!(validator.validate(4, "concurrency", 4).is_ok());
    /// ```
    pub fn validator() -> OpenGdsConcurrencyValidator {
        OpenGdsConcurrencyValidator
    }

    /// Get the maximum allowed concurrency level.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::validator::ConcurrencyValidatorService;
    ///
    /// let max = ConcurrencyValidatorService::max_concurrency();
    /// assert_eq!(max, 4);
    /// ```
    pub const fn max_concurrency() -> usize {
        Self::DEFAULT_CONCURRENCY_LIMIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_validator() {
        let validator = ConcurrencyValidatorService::validator();
        assert!(validator.validate(4, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_service_max_concurrency() {
        assert_eq!(ConcurrencyValidatorService::max_concurrency(), 4);
    }

    #[test]
    fn test_service_default_limit() {
        assert_eq!(ConcurrencyValidatorService::DEFAULT_CONCURRENCY_LIMIT, 4);
    }
}

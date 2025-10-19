use super::{ConcurrencyValidator, ConcurrencyValidatorError, LicenseState};

/// Open GDS concurrency validator - default implementation.
///
/// Enforces the simple rule: requested <= limit.
/// That's it. That's the whole "enterprise validation system." 🎯
///
/// In Java GDS:
/// ```java
/// public class OpenGdsConcurrencyValidator implements ConcurrencyValidator {
///     @Override
///     public void validate(int requested, String key, int limit) {
///         if (requested > limit) {
///             throw new IllegalArgumentException(formatWithLocale(
///                 "Community users cannot exceed %1$s=%2$d (you configured %1$s=%3$d)",
///                 key, limit, requested
///             ));
///         }
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use gds::concurrency::validator::{ConcurrencyValidator, OpenGdsConcurrencyValidator};
///
/// let validator = OpenGdsConcurrencyValidator;
///
/// // Valid: within limit
/// assert!(validator.validate(4, "concurrency", 4).is_ok());
///
/// // Invalid: exceeds limit
/// assert!(validator.validate(8, "concurrency", 4).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OpenGdsConcurrencyValidator;

impl ConcurrencyValidator for OpenGdsConcurrencyValidator {
    fn validate(
        &self,
        requested_concurrency: usize,
        config_key: &str,
        concurrency_limitation: usize,
    ) -> Result<(), ConcurrencyValidatorError> {
        if requested_concurrency > concurrency_limitation {
            Err(ConcurrencyValidatorError::new(
                requested_concurrency,
                concurrency_limitation,
                config_key,
            ))
        } else {
            Ok(())
        }
    }
}

/// Builder for OpenGdsConcurrencyValidator (enterprise pattern compatibility).
///
/// In Java GDS:
/// ```java
/// @ServiceProvider
/// public class OpenGdsConcurrencyValidatorBuilder implements ConcurrencyValidatorBuilder {
///     @Override public ConcurrencyValidator build(LicenseState s) {
///         return new OpenGdsConcurrencyValidator();
///     }
///     @Override public int priority() { return Integer.MIN_VALUE; }
/// }
/// ```
///
/// In Rust: We skip the @ServiceProvider madness, but provide the same API.
///
/// # Examples
///
/// ```
/// use gds::concurrency::validator::{OpenGdsConcurrencyValidatorBuilder, LicenseState};
///
/// let builder = OpenGdsConcurrencyValidatorBuilder;
/// let validator = builder.build(LicenseState::Open);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct OpenGdsConcurrencyValidatorBuilder;

impl OpenGdsConcurrencyValidatorBuilder {
    /// Build a validator for the given license state.
    ///
    /// Currently always returns OpenGdsConcurrencyValidator regardless of license.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::validator::{ConcurrencyValidator, OpenGdsConcurrencyValidatorBuilder, LicenseState};
    ///
    /// let builder = OpenGdsConcurrencyValidatorBuilder;
    /// let validator = builder.build(LicenseState::Open);
    /// assert!(validator.validate(4, "concurrency", 4).is_ok());
    /// ```
    pub fn build(&self, _license_state: LicenseState) -> OpenGdsConcurrencyValidator {
        OpenGdsConcurrencyValidator
    }

    /// Returns priority for this builder (Java compatibility).
    ///
    /// Returns i32::MIN to match Java's Integer.MIN_VALUE.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::validator::OpenGdsConcurrencyValidatorBuilder;
    ///
    /// let builder = OpenGdsConcurrencyValidatorBuilder;
    /// assert_eq!(builder.priority(), i32::MIN);
    /// ```
    pub const fn priority(&self) -> i32 {
        i32::MIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_within_limit() {
        let validator = OpenGdsConcurrencyValidator;
        assert!(validator.validate(4, "concurrency", 4).is_ok());
        assert!(validator.validate(1, "concurrency", 4).is_ok());
        assert!(validator.validate(3, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_validate_exceeds_limit() {
        let validator = OpenGdsConcurrencyValidator;
        let result = validator.validate(8, "concurrency", 4);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.requested, 8);
        assert_eq!(err.limit, 4);
        assert_eq!(err.config_key, "concurrency");
    }

    #[test]
    fn test_validate_exactly_at_limit() {
        let validator = OpenGdsConcurrencyValidator;
        assert!(validator.validate(4, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_validate_zero() {
        let validator = OpenGdsConcurrencyValidator;
        // Zero is technically valid if limit allows it
        assert!(validator.validate(0, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_default() {
        let validator = OpenGdsConcurrencyValidator::default();
        assert!(validator.validate(4, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_clone_copy() {
        let v1 = OpenGdsConcurrencyValidator;
        let v2 = v1; // Copy
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_builder_open_license() {
        let builder = OpenGdsConcurrencyValidatorBuilder;
        let validator = builder.build(LicenseState::Open);
        assert!(validator.validate(4, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_builder_enterprise_license() {
        let builder = OpenGdsConcurrencyValidatorBuilder;
        // Currently returns same validator for enterprise
        let validator = builder.build(LicenseState::Enterprise);
        assert!(validator.validate(4, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_builder_priority() {
        let builder = OpenGdsConcurrencyValidatorBuilder;
        assert_eq!(builder.priority(), i32::MIN);
    }

    #[test]
    fn test_builder_default() {
        let builder = OpenGdsConcurrencyValidatorBuilder::default();
        let validator = builder.build(LicenseState::Open);
        assert!(validator.validate(4, "concurrency", 4).is_ok());
    }

    #[test]
    fn test_as_trait_object() {
        let validator: Box<dyn ConcurrencyValidator> = Box::new(OpenGdsConcurrencyValidator);
        assert!(validator.validate(4, "concurrency", 4).is_ok());
        assert!(validator.validate(8, "concurrency", 4).is_err());
    }

    #[test]
    fn test_error_message_format() {
        let validator = OpenGdsConcurrencyValidator;
        let result = validator.validate(8, "writeConcurrency", 4);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("writeConcurrency=4"));
        assert!(msg.contains("writeConcurrency=8"));
    }
}

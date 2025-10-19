use super::OpenGdsPoolSizes;

/// License state for pool size determination.
///
/// Simplified version for Rust - no EJB service provider framework needed!
/// This can be extended with actual license checking when needed.
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{LicenseState, PoolSizesProvider};
///
/// let license = LicenseState::Open;
/// let provider = PoolSizesProvider;
/// let pool = provider.get(license);
/// assert_eq!(pool.core_pool_size(), 4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LicenseState {
    /// Open source license
    Open,
    /// Enterprise license (future extension)
    Enterprise,
}

/// Pool sizes provider - determines pool sizes based on license state.
///
/// This is a **radically simplified** version of Java's service provider pattern.
/// No annotations, no service loading, no MIN_VALUE priorities - just clean code.
///
/// In Java GDS:
/// ```java
/// @ServiceProvider
/// public class OpenGdsPoolSizesProvider implements PoolSizesProvider {
///     @Override public PoolSizes get(LicenseState licenseState) {
///         return new OpenGdsPoolSizes();
///     }
///     @Override public int priority() { return Integer.MIN_VALUE; }
/// }
/// ```
///
/// In Rust: **Just call a function.** No service framework needed!
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, PoolSizesProvider, LicenseState};
///
/// let provider = PoolSizesProvider;
/// let pool = provider.get(LicenseState::Open);
/// assert_eq!(pool.core_pool_size(), 4);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct PoolSizesProvider;

impl PoolSizesProvider {
    /// Get pool sizes based on license state.
    ///
    /// Currently always returns OpenGdsPoolSizes (4/4).
    /// Can be extended for license-aware sizing when needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::pool::{PoolSizes, PoolSizesProvider, LicenseState};
    ///
    /// let provider = PoolSizesProvider;
    /// let pool = provider.get(LicenseState::Open);
    /// assert_eq!(pool.core_pool_size(), 4);
    /// ```
    pub fn get(&self, _license_state: LicenseState) -> OpenGdsPoolSizes {
        OpenGdsPoolSizes
    }

    /// Returns priority for this provider.
    ///
    /// This mirrors Java's priority system, but without the service loading.
    /// Lower numbers = lower priority. Returns i32::MIN for base implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::pool::PoolSizesProvider;
    ///
    /// let provider = PoolSizesProvider;
    /// assert_eq!(provider.priority(), i32::MIN);
    /// ```
    pub const fn priority(&self) -> i32 {
        i32::MIN
    }
}

/// Global singleton for pool sizes service (Java compatibility).
///
/// In Java GDS:
/// ```java
/// public final class PoolSizesService {
///     private static PoolSizes instance = new OpenGdsPoolSizes();
///     public static void poolSizes(PoolSizes poolSizes) { instance = poolSizes; }
///     public static PoolSizes poolSizes() { return Objects.requireNonNull(instance); }
/// }
/// ```
///
/// In Rust: We provide the same API but recommend just using the types directly.
/// This exists primarily for Java/TS compatibility.
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, PoolSizesService};
///
/// let pool = PoolSizesService::pool_sizes();
/// assert_eq!(pool.core_pool_size(), 4);
/// ```
pub struct PoolSizesService;

impl PoolSizesService {
    /// Get the global pool sizes instance.
    ///
    /// Always returns OpenGdsPoolSizes (no mutable global state in Rust!).
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::pool::{PoolSizes, PoolSizesService};
    ///
    /// let pool = PoolSizesService::pool_sizes();
    /// assert_eq!(pool.core_pool_size(), 4);
    /// ```
    pub fn pool_sizes() -> OpenGdsPoolSizes {
        OpenGdsPoolSizes
    }

    // Note: We don't provide set_pool_sizes() because mutable global state
    // is an anti-pattern in Rust. Use dependency injection instead!
}

#[cfg(test)]
mod tests {
    use super::super::PoolSizes;
    use super::*;

    #[test]
    fn test_license_state_variants() {
        let open = LicenseState::Open;
        let enterprise = LicenseState::Enterprise;
        assert_ne!(open, enterprise);
    }

    #[test]
    fn test_provider_get_open() {
        let provider = PoolSizesProvider;
        let pool = provider.get(LicenseState::Open);
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }

    #[test]
    fn test_provider_get_enterprise() {
        let provider = PoolSizesProvider;
        // Currently returns same for enterprise
        let pool = provider.get(LicenseState::Enterprise);
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }

    #[test]
    fn test_provider_priority() {
        let provider = PoolSizesProvider;
        assert_eq!(provider.priority(), i32::MIN);
    }

    #[test]
    fn test_provider_default() {
        let provider = PoolSizesProvider::default();
        let pool = provider.get(LicenseState::Open);
        assert_eq!(pool.core_pool_size(), 4);
    }

    #[test]
    fn test_service_pool_sizes() {
        let pool = PoolSizesService::pool_sizes();
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }

    #[test]
    fn test_service_multiple_calls() {
        // Should always return the same default
        let pool1 = PoolSizesService::pool_sizes();
        let pool2 = PoolSizesService::pool_sizes();
        assert_eq!(pool1.core_pool_size(), pool2.core_pool_size());
    }
}

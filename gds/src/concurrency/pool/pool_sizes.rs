/// Thread pool size configuration trait.
///
/// Defines the core and maximum pool sizes for thread pool management.
/// This trait mirrors the Java GDS PoolSizes interface, providing
/// configuration for thread pool dimensions.
///
/// # Design Philosophy
///
/// Rust doesn't need Java's PoolSizesProvider/Service pattern - we use traits!
/// The trait itself is the abstraction, and implementations can be swapped
/// at compile time or runtime via trait objects.
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, default, fixed};
///
/// let pool = default();
/// assert_eq!(pool.core_pool_size(), 4);
/// assert_eq!(pool.max_pool_size(), 4);
///
/// let fixed_pool = fixed(8);
/// assert_eq!(fixed_pool.core_pool_size(), 8);
/// assert_eq!(fixed_pool.max_pool_size(), 8);
/// ```
pub trait PoolSizes {
    /// The core number of threads to keep in the pool.
    ///
    /// This is the minimum number of threads that will be maintained,
    /// even if they are idle.
    fn core_pool_size(&self) -> usize;

    /// The maximum number of threads allowed in the pool.
    ///
    /// The pool can grow up to this size under load.
    fn max_pool_size(&self) -> usize;
}

// =============================================================================
// FACTORY FUNCTIONS - Module-level constructors
// =============================================================================

/// Returns the default pool sizes (4 core, 4 max).
///
/// Conservative default suitable for most workloads.
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, default};
///
/// let pool = default();
/// assert_eq!(pool.core_pool_size(), 4);
/// ```
pub fn default() -> DefaultPoolSizes {
    DefaultPoolSizes
}

/// Creates a fixed-size pool with the same core and max size.
///
/// # Arguments
///
/// * `size` - Number of threads for both core and max pool size
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, fixed};
///
/// let pool = fixed(8);
/// assert_eq!(pool.core_pool_size(), 8);
/// assert_eq!(pool.max_pool_size(), 8);
/// ```
pub fn fixed(size: usize) -> FixedPoolSizes {
    FixedPoolSizes { size }
}

/// Creates a single-threaded pool for debugging and testing.
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, single_threaded};
///
/// let pool = single_threaded();
/// assert_eq!(pool.core_pool_size(), 1);
/// assert_eq!(pool.max_pool_size(), 1);
/// ```
pub fn single_threaded() -> FixedPoolSizes {
    FixedPoolSizes { size: 1 }
}

/// Creates pool sizes based on available CPU cores.
///
/// # Arguments
///
/// * `factor` - Multiplier for CPU count (default 1.0)
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, from_cpu_cores};
///
/// // 1:1 mapping to CPU cores
/// let pool = from_cpu_cores(1.0);
///
/// // 2x CPU cores for I/O-bound workloads
/// let io_pool = from_cpu_cores(2.0);
/// ```
pub fn from_cpu_cores(factor: f64) -> FixedPoolSizes {
    let cores = num_cpus::get();
    let size = ((cores as f64) * factor).max(1.0) as usize;
    FixedPoolSizes { size }
}

/// Creates a custom pool with different core and max sizes.
///
/// # Arguments
///
/// * `core_size` - Core pool size
/// * `max_size` - Maximum pool size
///
/// # Examples
///
/// ```
/// use gds::concurrency::pool::{PoolSizes, custom};
///
/// let pool = custom(4, 16);
/// assert_eq!(pool.core_pool_size(), 4);
/// assert_eq!(pool.max_pool_size(), 16);
/// ```
pub fn custom(core_size: usize, max_size: usize) -> CustomPoolSizes {
    CustomPoolSizes {
        core_size,
        max_size,
    }
}

// =============================================================================
// IMPLEMENTATIONS
// =============================================================================

/// Default pool sizes implementation (4 core, 4 max).
///
/// Conservative defaults suitable for most workloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefaultPoolSizes;

impl PoolSizes for DefaultPoolSizes {
    fn core_pool_size(&self) -> usize {
        4
    }

    fn max_pool_size(&self) -> usize {
        4
    }
}

impl Default for DefaultPoolSizes {
    fn default() -> Self {
        DefaultPoolSizes
    }
}

/// Fixed-size pool with same core and max size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedPoolSizes {
    size: usize,
}

impl PoolSizes for FixedPoolSizes {
    fn core_pool_size(&self) -> usize {
        self.size
    }

    fn max_pool_size(&self) -> usize {
        self.size
    }
}

/// Custom pool sizes with different core and max values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomPoolSizes {
    core_size: usize,
    max_size: usize,
}

impl PoolSizes for CustomPoolSizes {
    fn core_pool_size(&self) -> usize {
        self.core_size
    }

    fn max_pool_size(&self) -> usize {
        self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pool_sizes() {
        let pool = default();
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }

    #[test]
    fn test_fixed_pool_sizes() {
        let pool = fixed(8);
        assert_eq!(pool.core_pool_size(), 8);
        assert_eq!(pool.max_pool_size(), 8);
    }

    #[test]
    fn test_single_threaded() {
        let pool = single_threaded();
        assert_eq!(pool.core_pool_size(), 1);
        assert_eq!(pool.max_pool_size(), 1);
    }

    #[test]
    fn test_from_cpu_cores_factor_1() {
        let pool = from_cpu_cores(1.0);
        let expected = num_cpus::get();
        assert_eq!(pool.core_pool_size(), expected);
        assert_eq!(pool.max_pool_size(), expected);
    }

    #[test]
    fn test_from_cpu_cores_factor_2() {
        let pool = from_cpu_cores(2.0);
        let expected = num_cpus::get() * 2;
        assert_eq!(pool.core_pool_size(), expected);
        assert_eq!(pool.max_pool_size(), expected);
    }

    #[test]
    fn test_from_cpu_cores_half() {
        let pool = from_cpu_cores(0.5);
        let expected = (num_cpus::get() as f64 * 0.5).max(1.0) as usize;
        assert_eq!(pool.core_pool_size(), expected);
        assert_eq!(pool.max_pool_size(), expected);
    }

    #[test]
    fn test_custom_pool_sizes() {
        let pool = custom(4, 16);
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 16);
    }

    #[test]
    fn test_trait_object() {
        let pool: Box<dyn PoolSizes> = Box::new(fixed(8));
        assert_eq!(pool.core_pool_size(), 8);
        assert_eq!(pool.max_pool_size(), 8);
    }

    #[test]
    fn test_equality() {
        let pool1 = FixedPoolSizes { size: 8 };
        let pool2 = fixed(8);
        assert_eq!(pool1, pool2);
    }

    #[test]
    fn test_clone() {
        let pool1 = custom(4, 16);
        let pool2 = pool1;
        assert_eq!(pool1.core_pool_size(), pool2.core_pool_size());
        assert_eq!(pool1.max_pool_size(), pool2.max_pool_size());
    }

    #[test]
    fn test_debug() {
        let pool = fixed(8);
        let debug = format!("{:?}", pool);
        assert!(debug.contains("FixedPoolSizes"));
        assert!(debug.contains("8"));
    }
}

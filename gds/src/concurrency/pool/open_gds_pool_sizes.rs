use super::PoolSizes;

/// Open GDS pool sizes - default implementation.
///
/// Conservative default implementation using the official factory pattern.
/// This is the "official" default for Open GDS, matching the Java/TS behavior.
///
/// In Java GDS:
/// ```java
/// // TODO: think how to make the magic numbers less magic
/// public class OpenGdsPoolSizes implements PoolSizes {
///     @Override public int corePoolSize() { return 4; }
///     @Override public int maxPoolSize() { return 4; }
/// }
/// ```
///
/// **They're not magic anymore - they're the official defaults!** 🎉
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::pool::{PoolSizes, OpenGdsPoolSizes};
///
/// let pool = OpenGdsPoolSizes;
/// assert_eq!(pool.core_pool_size(), 4);
/// assert_eq!(pool.max_pool_size(), 4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OpenGdsPoolSizes;

impl PoolSizes for OpenGdsPoolSizes {
    fn core_pool_size(&self) -> usize {
        4
    }

    fn max_pool_size(&self) -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_gds_pool_sizes() {
        let pool = OpenGdsPoolSizes;
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }

    #[test]
    fn test_default() {
        let pool = OpenGdsPoolSizes::default();
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }

    #[test]
    fn test_clone_copy() {
        let pool1 = OpenGdsPoolSizes;
        let pool2 = pool1; // Copy
        assert_eq!(pool1.core_pool_size(), pool2.core_pool_size());
    }

    #[test]
    fn test_equality() {
        let pool1 = OpenGdsPoolSizes;
        let pool2 = OpenGdsPoolSizes;
        assert_eq!(pool1, pool2);
    }

    #[test]
    fn test_as_trait_object() {
        let pool: Box<dyn PoolSizes> = Box::new(OpenGdsPoolSizes);
        assert_eq!(pool.core_pool_size(), 4);
        assert_eq!(pool.max_pool_size(), 4);
    }
}

//! Factory interface for creating UserLogRegistry instances.

use crate::core::utils::warnings::UserLogRegistry;

/// Factory for creating UserLogRegistry instances.
///
/// Provides a standard interface for creating registries,
/// allowing for different implementations (empty, per-user, etc.).
pub trait UserLogRegistryFactory: Send + Sync {
    /// Creates a new UserLogRegistry instance.
    fn new_instance(&self) -> UserLogRegistry;
}

//! Empty user log registry factory.

use crate::core::utils::warnings::{EmptyUserLogStore, UserLogRegistry, UserLogRegistryFactory};

/// Factory that creates UserLogRegistry instances which discard all warnings.
///
/// This is used for testing and scenarios where logging is disabled.
#[derive(Debug, Clone, Copy)]
pub struct EmptyUserLogRegistryFactory;

impl EmptyUserLogRegistryFactory {
    /// Creates a new empty factory instance.
    pub const fn new() -> Self {
        Self
    }

    /// Returns a static reference to a shared empty factory instance.
    pub fn instance() -> &'static Self {
        &EMPTY_USER_LOG_REGISTRY_FACTORY_INSTANCE
    }
}

impl Default for EmptyUserLogRegistryFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl UserLogRegistryFactory for EmptyUserLogRegistryFactory {
    fn new_instance(&self) -> UserLogRegistry {
        UserLogRegistry::new(String::new(), Box::new(EmptyUserLogStore::new()))
    }
}

/// Singleton instance of EmptyUserLogRegistryFactory.
pub static EMPTY_USER_LOG_REGISTRY_FACTORY_INSTANCE: EmptyUserLogRegistryFactory =
    EmptyUserLogRegistryFactory;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_factory_creates_registry() {
        let factory = EmptyUserLogRegistryFactory::new();
        let registry = factory.new_instance();

        assert_eq!(registry.username(), "");
    }

    #[test]
    fn test_empty_factory_instance() {
        let factory = EmptyUserLogRegistryFactory::instance();
        let registry = factory.new_instance();

        assert_eq!(registry.username(), "");
    }
}

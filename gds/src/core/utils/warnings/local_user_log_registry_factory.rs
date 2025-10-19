//! Local user log registry factory for specific users.

use crate::core::utils::warnings::{UserLogRegistry, UserLogRegistryFactory, UserLogStore};
use std::sync::Arc;

/// Factory that creates UserLogRegistry instances for a specific user.
///
/// Each registry created by this factory will use the same username
/// and user log store provided during construction.
pub struct LocalUserLogRegistryFactory {
    username: String,
    user_log_store: Arc<dyn UserLogStore>,
}

impl LocalUserLogRegistryFactory {
    /// Creates a new local user log registry factory.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which logs will be registered
    /// * `user_log_store` - The store to be used by created registries
    pub fn new(username: String, user_log_store: Arc<dyn UserLogStore>) -> Self {
        Self {
            username,
            user_log_store,
        }
    }

    /// Returns the username associated with this factory.
    pub fn username(&self) -> &str {
        &self.username
    }
}

impl UserLogRegistryFactory for LocalUserLogRegistryFactory {
    fn new_instance(&self) -> UserLogRegistry {
        UserLogRegistry::new(
            self.username.clone(),
            Box::new(ArcUserLogStoreWrapper {
                store: Arc::clone(&self.user_log_store),
            }),
        )
    }
}

/// Wrapper to allow Arc<dyn UserLogStore> to be used as Box<dyn UserLogStore>.
struct ArcUserLogStoreWrapper {
    store: Arc<dyn UserLogStore>,
}

impl UserLogStore for ArcUserLogStoreWrapper {
    fn add_user_log_message(
        &self,
        username: &str,
        task: &crate::core::utils::progress::Task,
        message: String,
    ) {
        self.store.add_user_log_message(username, task, message);
    }

    fn query(&self, username: &str) -> Vec<crate::core::utils::warnings::UserLogEntry> {
        self.store.query(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::warnings::EmptyUserLogStore;

    #[test]
    fn test_local_factory_creation() {
        let store = Arc::new(EmptyUserLogStore::new());
        let factory = LocalUserLogRegistryFactory::new("test_user".to_string(), store);

        assert_eq!(factory.username(), "test_user");
    }

    #[test]
    fn test_local_factory_creates_registry() {
        let store = Arc::new(EmptyUserLogStore::new());
        let factory = LocalUserLogRegistryFactory::new("test_user".to_string(), store);
        let registry = factory.new_instance();

        assert_eq!(registry.username(), "test_user");
    }

    #[test]
    fn test_multiple_registries_from_same_factory() {
        let store = Arc::new(EmptyUserLogStore::new());
        let factory = LocalUserLogRegistryFactory::new("test_user".to_string(), store);

        let registry1 = factory.new_instance();
        let registry2 = factory.new_instance();

        assert_eq!(registry1.username(), "test_user");
        assert_eq!(registry2.username(), "test_user");
    }
}

use crate::core::basic_context::{BasicContext, BasicUser};
use crate::core::context::{ExecutionConfig, LogLevel};

/// Factory for creating different types of execution contexts.
pub struct ContextFactory;

impl ContextFactory {
    /// Create an anonymous context (no user authentication).
    pub fn anonymous(config: Option<ExecutionConfig>) -> BasicContext {
        BasicContext::new().with_config(config.unwrap_or_default())
    }

    /// Create a context for a specific user.
    pub fn for_user(
        username: impl Into<String>,
        roles: Vec<String>,
        config: Option<ExecutionConfig>,
    ) -> BasicContext {
        let mut user = BasicUser::new(username);
        user.roles = roles.clone();
        user.permissions = roles; // Simple permission model

        BasicContext::new()
            .with_user(user)
            .with_config(config.unwrap_or_default())
    }

    /// Create a context with session-based authentication.
    pub fn with_session(
        _session_user: impl Into<String>,
        config: Option<ExecutionConfig>,
    ) -> BasicContext {
        let mut ctx = BasicContext::new();
        ctx = ctx.with_config(config.unwrap_or_default());
        // Session user would be set here if BasicContext supported it directly
        ctx
    }

    /// Create a debug context with enhanced logging.
    pub fn debug(username: Option<String>) -> BasicContext {
        let debug_config = ExecutionConfig::new()
            .with_log_level(LogLevel::Debug)
            .with_debug(true);

        match username {
            Some(name) => Self::for_user(
                name,
                vec!["user".to_string(), "debug".to_string()],
                Some(debug_config),
            ),
            None => Self::anonymous(Some(debug_config)),
        }
    }

    /// Create a high-performance context with specific resource allocations.
    pub fn high_performance(
        username: impl Into<String>,
        concurrency: usize,
        max_memory: usize,
    ) -> BasicContext {
        let config = ExecutionConfig::new()
            .with_concurrency(concurrency)
            .with_max_memory(max_memory)
            .with_log_level(LogLevel::Warn); // Reduce logging overhead

        Self::for_user(
            username,
            vec!["user".to_string(), "power-user".to_string()],
            Some(config),
        )
    }

    /// Create a context for Neo4j interop (future use).
    pub fn neo4j_interop(username: impl Into<String>, _database_name: String) -> BasicContext {
        let config = ExecutionConfig::new();
        // Would set execution mode and database name when supported
        Self::for_user(username, vec!["neo4j-user".to_string()], Some(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::context::Context;

    #[test]
    fn test_anonymous_context() {
        let ctx = ContextFactory::anonymous(None);
        assert!(!ctx.security_context().is_authenticated());
    }

    #[test]
    fn test_for_user() {
        let ctx = ContextFactory::for_user("alice", vec!["admin".to_string()], None);
        assert!(ctx.security_context().is_authenticated());
        assert_eq!(ctx.session_user(), Some("alice".to_string()));
    }

    #[test]
    fn test_debug_context() {
        let ctx = ContextFactory::debug(Some("bob".to_string()));
        assert!(ctx.is_debug_enabled());
        assert_eq!(ctx.log_level(), LogLevel::Debug);
    }
}

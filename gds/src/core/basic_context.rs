use crate::core::context::{
    AuthSubject, Context, ExecutionConfig, ExecutionMetrics, ExecutionMode, LogLevel,
    SecurityContext, User,
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Basic implementation of User for standalone contexts.
#[derive(Clone, Debug)]
pub struct BasicUser {
    pub username: String,
    pub roles: Vec<String>,
    pub is_authenticated: bool,
    pub permissions: Vec<String>,
}

impl BasicUser {
    pub fn new(username: impl Into<String>) -> Self {
        BasicUser {
            username: username.into(),
            roles: vec!["user".to_string()],
            is_authenticated: true,
            permissions: vec![],
        }
    }

    pub fn anonymous() -> Self {
        BasicUser {
            username: "anonymous".to_string(),
            roles: vec![],
            is_authenticated: false,
            permissions: vec![],
        }
    }
}

impl User for BasicUser {
    fn username(&self) -> &str {
        &self.username
    }

    fn roles(&self) -> &[String] {
        &self.roles
    }

    fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }

    fn permissions(&self) -> &[String] {
        &self.permissions
    }
}

/// Basic implementation of AuthSubject.
#[derive(Clone, Debug)]
pub struct BasicAuthSubject {
    username: String,
    authenticated: bool,
}

impl BasicAuthSubject {
    fn new(user: Option<&BasicUser>) -> Self {
        match user {
            Some(u) => BasicAuthSubject {
                username: u.username.clone(),
                authenticated: u.is_authenticated,
            },
            None => BasicAuthSubject {
                username: "anonymous".to_string(),
                authenticated: false,
            },
        }
    }
}

impl AuthSubject for BasicAuthSubject {
    fn executing_user(&self) -> String {
        self.username.clone()
    }

    fn authenticated_user(&self) -> String {
        self.username.clone()
    }

    fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    fn has_username(&self, username: &str) -> bool {
        self.username == username
    }
}

/// Basic implementation of SecurityContext.
#[derive(Clone, Debug)]
pub struct BasicSecurityContext {
    subject: BasicAuthSubject,
    roles: Vec<String>,
}

impl BasicSecurityContext {
    fn new(user: Option<&BasicUser>) -> Self {
        let subject = BasicAuthSubject::new(user);
        let roles = user.map(|u| u.roles.clone()).unwrap_or_default();

        BasicSecurityContext { subject, roles }
    }
}

impl SecurityContext for BasicSecurityContext {
    fn subject(&self) -> &dyn AuthSubject {
        &self.subject
    }

    fn is_authenticated(&self) -> bool {
        self.subject.is_authenticated()
    }

    fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role) || self.roles.contains(&"admin".to_string())
    }

    fn roles(&self) -> Vec<String> {
        self.roles.clone()
    }
}

/// Basic concrete implementation of Context for standalone use.
/// Matches the TypeScript BasicContext pattern.
#[derive(Clone, Debug)]
pub struct BasicContext {
    user: Option<BasicUser>,
    session_user: Option<String>,
    security_context: BasicSecurityContext,
    parameters: HashMap<String, String>,
    config: ExecutionConfig,
    execution_mode: ExecutionMode,
    allocated_memory: usize,
    max_memory: usize,
    concurrency: usize,
    start_time: u64,
}

impl BasicContext {
    pub fn new() -> Self {
        let user = None;
        let security_context = BasicSecurityContext::new(user.as_ref());
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        BasicContext {
            user,
            session_user: None,
            security_context,
            parameters: HashMap::new(),
            config: ExecutionConfig::new(),
            execution_mode: ExecutionMode::Standalone,
            allocated_memory: 0,
            max_memory: usize::MAX,
            concurrency: num_cpus::get(),
            start_time,
        }
    }

    pub fn with_user(mut self, user: BasicUser) -> Self {
        self.session_user = Some(user.username.clone());
        self.security_context = BasicSecurityContext::new(Some(&user));
        self.user = Some(user);
        self
    }

    pub fn with_config(mut self, config: ExecutionConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_parameters(mut self, parameters: HashMap<String, String>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_execution_mode(mut self, mode: ExecutionMode) -> Self {
        self.execution_mode = mode;
        self
    }
}

impl Default for BasicContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Context for BasicContext {
    fn user(&self) -> Option<&dyn User> {
        self.user.as_ref().map(|u| u as &dyn User)
    }

    fn session_user(&self) -> Option<String> {
        self.session_user.clone()
    }

    fn security_context(&self) -> &dyn SecurityContext {
        &self.security_context
    }

    fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    fn config(&self) -> &ExecutionConfig {
        &self.config
    }

    fn execution_mode(&self) -> ExecutionMode {
        self.execution_mode
    }

    fn allocated_memory(&self) -> usize {
        self.allocated_memory
    }

    fn max_memory(&self) -> usize {
        self.max_memory
    }

    fn concurrency(&self) -> usize {
        self.concurrency
    }

    fn log_level(&self) -> LogLevel {
        self.config.log_level.unwrap_or(LogLevel::Info)
    }

    fn is_debug_enabled(&self) -> bool {
        self.config.debug_enabled.unwrap_or(false) || self.log_level() == LogLevel::Debug
    }

    fn metrics(&self) -> ExecutionMetrics {
        ExecutionMetrics::new().with_elapsed_time(self.start_time)
    }

    fn database_name(&self) -> Option<String> {
        Some("rust-gds".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_context_creation() {
        let ctx = BasicContext::new();
        assert_eq!(ctx.execution_mode(), ExecutionMode::Standalone);
        assert!(!ctx.security_context().is_authenticated());
        assert_eq!(ctx.database_name(), Some("rust-gds".to_string()));
    }

    #[test]
    fn test_context_with_user() {
        let user = BasicUser::new("alice");
        let ctx = BasicContext::new().with_user(user);

        assert_eq!(ctx.session_user(), Some("alice".to_string()));
        assert!(ctx.security_context().is_authenticated());
        assert_eq!(ctx.security_context().subject().executing_user(), "alice");
    }

    #[test]
    fn test_security_context_roles() {
        let mut user = BasicUser::new("bob");
        user.roles.push("admin".to_string());

        let ctx = BasicContext::new().with_user(user);
        let sec_ctx = ctx.security_context();

        assert!(sec_ctx.has_role("admin"));
        assert!(sec_ctx.has_role("unknown")); // admin has all roles
    }

    #[test]
    fn test_metrics_timing() {
        let ctx = BasicContext::new();
        std::thread::sleep(std::time::Duration::from_millis(10));

        let metrics = ctx.metrics();
        assert!(metrics.cpu_time >= 10);
    }
}

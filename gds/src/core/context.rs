use std::collections::HashMap;

/// Core execution context for graph operations.
/// Provides user authentication, security context, and execution environment.
pub trait Context {
    fn user(&self) -> Option<&dyn User>;
    fn session_user(&self) -> Option<String>;
    fn security_context(&self) -> &dyn SecurityContext;
    fn parameters(&self) -> &HashMap<String, String>;
    fn config(&self) -> &ExecutionConfig;
    fn execution_mode(&self) -> ExecutionMode;
    fn allocated_memory(&self) -> usize;
    fn max_memory(&self) -> usize;
    fn concurrency(&self) -> usize;
    fn log_level(&self) -> LogLevel;
    fn is_debug_enabled(&self) -> bool;
    fn metrics(&self) -> ExecutionMetrics;
    fn transaction(&self) -> Option<&dyn Transaction> {
        None
    }
    fn database_name(&self) -> Option<String> {
        None
    }
}

/// Security context providing user authentication and authorization.
pub trait SecurityContext {
    fn subject(&self) -> &dyn AuthSubject;
    fn is_authenticated(&self) -> bool;
    fn has_role(&self, role: &str) -> bool;
    fn roles(&self) -> Vec<String>;
}

/// Authentication subject representing the executing user.
pub trait AuthSubject {
    fn executing_user(&self) -> String;
    fn authenticated_user(&self) -> String;
    fn is_authenticated(&self) -> bool;
    fn has_username(&self, username: &str) -> bool;
}

/// User information with roles and permissions.
pub trait User {
    fn username(&self) -> &str;
    fn roles(&self) -> &[String];
    fn is_authenticated(&self) -> bool;
    fn permissions(&self) -> &[String];
}

/// Execution configuration for algorithms and operations.
#[derive(Clone, Debug)]
pub struct ExecutionConfig {
    pub concurrency: Option<usize>,
    pub max_memory: Option<usize>,
    pub log_level: Option<LogLevel>,
    pub debug_enabled: Option<bool>,
    pub additional: HashMap<String, String>,
}

impl ExecutionConfig {
    pub fn new() -> Self {
        ExecutionConfig {
            concurrency: None,
            max_memory: None,
            log_level: None,
            debug_enabled: None,
            additional: HashMap::new(),
        }
    }

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn with_max_memory(mut self, max_memory: usize) -> Self {
        self.max_memory = Some(max_memory);
        self
    }

    pub fn with_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = Some(log_level);
        self
    }

    pub fn with_debug(mut self, enabled: bool) -> Self {
        self.debug_enabled = Some(enabled);
        self
    }
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution mode for different runtime environments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExecutionMode {
    Standalone,
    Neo4jEmbedded,
    Neo4jFabric,
    Distributed,
}

/// Log levels for execution context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Execution metrics and performance data.
#[derive(Clone, Debug)]
pub struct ExecutionMetrics {
    pub start_time: u64,
    pub memory_used: usize,
    pub cpu_time: u64,
    pub tasks_completed: usize,
    pub errors_count: usize,
}

impl ExecutionMetrics {
    pub fn new() -> Self {
        ExecutionMetrics {
            start_time: 0,
            memory_used: 0,
            cpu_time: 0,
            tasks_completed: 0,
            errors_count: 0,
        }
    }

    pub fn with_elapsed_time(mut self, start_time: u64) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        self.start_time = start_time;
        self.cpu_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| (d.as_millis() as u64).saturating_sub(start_time))
            .unwrap_or(0);
        self
    }
}

impl Default for ExecutionMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction context for database operations.
pub trait Transaction {
    fn id(&self) -> &str;
    fn start_time(&self) -> u64;
    fn is_open(&self) -> bool;
    fn commit(&self) -> Result<(), String>;
    fn rollback(&self) -> Result<(), String>;
}

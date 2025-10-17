//! Logging interface for rust-gds.
//!
//! Provides a simple, adaptable logging interface matching Neo4j's Log pattern.

use std::fmt;

/// Simple logging interface.
///
/// Enables independent evolution and running without external dependencies.
/// Mimics Neo4j's Log interface for compatibility.
pub trait Log: Send + Sync {
    /// Log an info message.
    fn info(&self, message: &str);

    /// Log a formatted info message.
    fn info_fmt(&self, format: &str, args: &[&dyn fmt::Display]);

    /// Log a warning message.
    fn warn(&self, message: &str);

    /// Log a warning with error.
    fn warn_with_error(&self, message: &str, error: &dyn std::error::Error);

    /// Log an error message.
    fn error(&self, message: &str);

    /// Log an error with error object.
    fn error_with_error(&self, message: &str, error: &dyn std::error::Error);

    /// Log a debug message.
    fn debug(&self, format: &str, args: &[&dyn fmt::Display]);

    /// Check if debug logging is enabled.
    fn is_debug_enabled(&self) -> bool;
}

/// No-operation logger for testing.
pub struct NoOpLog;

impl Log for NoOpLog {
    fn info(&self, _message: &str) {}
    fn info_fmt(&self, _format: &str, _args: &[&dyn fmt::Display]) {}
    fn warn(&self, _message: &str) {}
    fn warn_with_error(&self, _message: &str, _error: &dyn std::error::Error) {}
    fn error(&self, _message: &str) {}
    fn error_with_error(&self, _message: &str, _error: &dyn std::error::Error) {}
    fn debug(&self, _format: &str, _args: &[&dyn fmt::Display]) {}
    fn is_debug_enabled(&self) -> bool {
        false
    }
}

/// Console-based logger for development.
pub struct ConsoleLog {
    debug_enabled: bool,
}

impl ConsoleLog {
    pub fn new() -> Self {
        Self {
            debug_enabled: std::env::var("RUST_LOG").is_ok()
                || std::env::var("DEBUG").is_ok_and(|v| v == "true"),
        }
    }

    fn format_message(&self, format: &str, args: &[&dyn fmt::Display]) -> String {
        let mut result = format.to_string();
        for arg in args {
            if let Some(pos) = result.find("%s") {
                result.replace_range(pos..pos + 2, &format!("{}", arg));
            }
        }
        result
    }
}

impl Default for ConsoleLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Log for ConsoleLog {
    fn info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    fn info_fmt(&self, format: &str, args: &[&dyn fmt::Display]) {
        println!("[INFO] {}", self.format_message(format, args));
    }

    fn warn(&self, message: &str) {
        eprintln!("[WARN] {}", message);
    }

    fn warn_with_error(&self, message: &str, error: &dyn std::error::Error) {
        eprintln!("[WARN] {}: {}", message, error);
    }

    fn error(&self, message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    fn error_with_error(&self, message: &str, error: &dyn std::error::Error) {
        eprintln!("[ERROR] {}: {}", message, error);
    }

    fn debug(&self, format: &str, args: &[&dyn fmt::Display]) {
        if self.is_debug_enabled() {
            println!("[DEBUG] {}", self.format_message(format, args));
        }
    }

    fn is_debug_enabled(&self) -> bool {
        self.debug_enabled
    }
}

/// Logger with prefix for all messages.
pub struct PrefixedLog {
    prefix: String,
    base_log: Box<dyn Log>,
}

impl PrefixedLog {
    pub fn new(prefix: String, base_log: Box<dyn Log>) -> Self {
        Self { prefix, base_log }
    }
}

impl Log for PrefixedLog {
    fn info(&self, message: &str) {
        self.base_log
            .info(&format!("[{}] {}", self.prefix, message));
    }

    fn info_fmt(&self, format: &str, args: &[&dyn fmt::Display]) {
        self.base_log
            .info_fmt(&format!("[{}] {}", self.prefix, format), args);
    }

    fn warn(&self, message: &str) {
        self.base_log
            .warn(&format!("[{}] {}", self.prefix, message));
    }

    fn warn_with_error(&self, message: &str, error: &dyn std::error::Error) {
        self.base_log
            .warn_with_error(&format!("[{}] {}", self.prefix, message), error);
    }

    fn error(&self, message: &str) {
        self.base_log
            .error(&format!("[{}] {}", self.prefix, message));
    }

    fn error_with_error(&self, message: &str, error: &dyn std::error::Error) {
        self.base_log
            .error_with_error(&format!("[{}] {}", self.prefix, message), error);
    }

    fn debug(&self, format: &str, args: &[&dyn fmt::Display]) {
        self.base_log
            .debug(&format!("[{}] {}", self.prefix, format), args);
    }

    fn is_debug_enabled(&self) -> bool {
        self.base_log.is_debug_enabled()
    }
}

/// Factory functions for creating loggers.
pub mod factory {
    use super::*;

    /// Create a no-operation logger.
    pub fn no_op() -> Box<dyn Log> {
        Box::new(NoOpLog)
    }

    /// Create a console logger.
    pub fn console() -> Box<dyn Log> {
        Box::new(ConsoleLog::new())
    }

    /// Create a prefixed logger.
    pub fn with_prefix(prefix: impl Into<String>, base_log: Box<dyn Log>) -> Box<dyn Log> {
        Box::new(PrefixedLog::new(prefix.into(), base_log))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_op_log() {
        let log = NoOpLog;
        log.info("test");
        log.warn("test");
        log.error("test");
        assert!(!log.is_debug_enabled());
    }

    #[test]
    fn test_console_log() {
        let log = ConsoleLog::new();
        log.info("info message");
        log.warn("warn message");
        log.error("error message");
    }

    #[test]
    fn test_prefixed_log() {
        let base = factory::console();
        let prefixed = factory::with_prefix("TEST", base);
        prefixed.info("test message");
    }
}

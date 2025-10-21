//! Progress Tracking - Task progress and logging for algorithms
//!
//! **Translation Source**: `org.neo4j.gds.core.utils.progress.*` classes
//! **Key Features**: Progress tracking, task management, logging integration
//!
//! This module provides progress tracking capabilities for algorithms, allowing
//! users to monitor execution progress and get detailed logging.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Progress tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressConfig {
    /// Whether to enable progress logging
    pub enable_logging: bool,
    /// Log level for progress messages
    pub log_level: LogLevel,
    /// Progress update interval (milliseconds)
    pub update_interval_ms: u64,
    /// Whether to show percentage progress
    pub show_percentage: bool,
    /// Whether to show estimated time remaining
    pub show_eta: bool,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_level: LogLevel::Info,
            update_interval_ms: 1000,
            show_percentage: true,
            show_eta: true,
        }
    }
}

/// Log levels for progress tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Progress tracker for algorithm execution
pub struct ProgressTracker {
    /// Total work units
    total_work: u64,
    /// Completed work units
    completed_work: Arc<AtomicU64>,
    /// Start time
    start_time: Instant,
    /// Last update time
    last_update: Arc<AtomicU64>,
    /// Configuration
    config: ProgressConfig,
    /// Task name
    task_name: String,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new(task_name: String, total_work: u64, config: ProgressConfig) -> Self {
        Self {
            total_work,
            completed_work: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
            last_update: Arc::new(AtomicU64::new(0)),
            config,
            task_name,
        }
    }

    /// Create a progress tracker with default configuration
    pub fn new_default(task_name: String, total_work: u64) -> Self {
        Self::new(task_name, total_work, ProgressConfig::default())
    }

    /// Update progress by adding completed work
    pub fn update_progress(&self, work_completed: u64) {
        let new_completed = self.completed_work.fetch_add(work_completed, Ordering::Relaxed) + work_completed;
        
        if self.config.enable_logging {
            self.maybe_log_progress(new_completed);
        }
    }

    /// Set progress to a specific value
    pub fn set_progress(&self, completed: u64) {
        self.completed_work.store(completed, Ordering::Relaxed);
        
        if self.config.enable_logging {
            self.maybe_log_progress(completed);
        }
    }

    /// Get current progress percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.total_work == 0 {
            100.0
        } else {
            (self.completed_work.load(Ordering::Relaxed) as f64 / self.total_work as f64) * 100.0
        }
    }

    /// Get elapsed time
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get estimated time remaining
    pub fn estimated_time_remaining(&self) -> Option<Duration> {
        let completed = self.completed_work.load(Ordering::Relaxed);
        if completed == 0 || completed >= self.total_work {
            return None;
        }

        let elapsed = self.elapsed_time();
        let rate = completed as f64 / elapsed.as_secs_f64();
        let remaining_work = self.total_work - completed;
        let remaining_seconds = remaining_work as f64 / rate;

        Some(Duration::from_secs_f64(remaining_seconds))
    }

    /// Check if task is complete
    pub fn is_complete(&self) -> bool {
        self.completed_work.load(Ordering::Relaxed) >= self.total_work
    }

    /// Get a thread-safe reference for parallel updates
    pub fn thread_safe_ref(&self) -> ThreadSafeProgressTracker {
        ThreadSafeProgressTracker {
            completed_work: self.completed_work.clone(),
            last_update: self.last_update.clone(),
            config: self.config.clone(),
            task_name: self.task_name.clone(),
            start_time: self.start_time,
            total_work: self.total_work,
        }
    }

    /// Maybe log progress based on update interval
    fn maybe_log_progress(&self, completed: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let last_update = self.last_update.load(Ordering::Relaxed);
        if now - last_update >= self.config.update_interval_ms {
            if self.last_update.compare_exchange(
                last_update,
                now,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ).is_ok() {
                self.log_progress(completed);
            }
        }
    }

    /// Log current progress
    fn log_progress(&self, completed: u64) {
        let percentage = self.progress_percentage();
        let elapsed = self.elapsed_time();
        
        let mut message = format!(
            "{}: {}/{} ({:.1}%) - Elapsed: {:.1}s",
            self.task_name,
            completed,
            self.total_work,
            percentage,
            elapsed.as_secs_f64()
        );

        if self.config.show_eta {
            if let Some(eta) = self.estimated_time_remaining() {
                message.push_str(&format!(" - ETA: {:.1}s", eta.as_secs_f64()));
            }
        }

        // In a real implementation, this would use the logging framework
        println!("[{}] {}", self.config.log_level.as_str(), message);
    }
}

/// Thread-safe progress tracker for parallel algorithms
#[derive(Clone)]
pub struct ThreadSafeProgressTracker {
    completed_work: Arc<AtomicU64>,
    last_update: Arc<AtomicU64>,
    config: ProgressConfig,
    task_name: String,
    start_time: Instant,
    total_work: u64,
}

impl ThreadSafeProgressTracker {
    /// Update progress from a parallel thread
    pub fn update_progress(&self, work_completed: u64) {
        let new_completed = self.completed_work.fetch_add(work_completed, Ordering::Relaxed) + work_completed;
        
        if self.config.enable_logging {
            self.maybe_log_progress(new_completed);
        }
    }

    /// Set progress to a specific value
    pub fn set_progress(&self, completed: u64) {
        self.completed_work.store(completed, Ordering::Relaxed);
        
        if self.config.enable_logging {
            self.maybe_log_progress(completed);
        }
    }

    /// Get current progress percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.total_work == 0 {
            100.0
        } else {
            (self.completed_work.load(Ordering::Relaxed) as f64 / self.total_work as f64) * 100.0
        }
    }

    /// Check if task is complete
    pub fn is_complete(&self) -> bool {
        self.completed_work.load(Ordering::Relaxed) >= self.total_work
    }

    /// Maybe log progress based on update interval
    fn maybe_log_progress(&self, completed: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let last_update = self.last_update.load(Ordering::Relaxed);
        if now - last_update >= self.config.update_interval_ms {
            if self.last_update.compare_exchange(
                last_update,
                now,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ).is_ok() {
                self.log_progress(completed);
            }
        }
    }

    /// Log current progress
    fn log_progress(&self, completed: u64) {
        let percentage = self.progress_percentage();
        let elapsed = self.start_time.elapsed();
        
        let mut message = format!(
            "{}: {}/{} ({:.1}%) - Elapsed: {:.1}s",
            self.task_name,
            completed,
            self.total_work,
            percentage,
            elapsed.as_secs_f64()
        );

        if self.config.show_eta {
            let rate = completed as f64 / elapsed.as_secs_f64();
            if rate > 0.0 {
                let remaining_work = self.total_work - completed;
                let remaining_seconds = remaining_work as f64 / rate;
                message.push_str(&format!(" - ETA: {:.1}s", remaining_seconds));
            }
        }

        // In a real implementation, this would use the logging framework
        println!("[{}] {}", self.config.log_level.as_str(), message);
    }
}

impl LogLevel {
    /// Convert log level to string
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Progress tracking error
#[derive(Debug, thiserror::Error)]
pub enum ProgressTrackingError {
    #[error("Invalid progress configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Progress tracking failed: {0}")]
    TrackingFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 100);
        assert_eq!(tracker.progress_percentage(), 0.0);
        assert!(!tracker.is_complete());
    }

    #[test]
    fn test_progress_update() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 100);
        
        tracker.update_progress(25);
        assert_eq!(tracker.progress_percentage(), 25.0);
        
        tracker.update_progress(25);
        assert_eq!(tracker.progress_percentage(), 50.0);
        
        tracker.update_progress(50);
        assert_eq!(tracker.progress_percentage(), 100.0);
        assert!(tracker.is_complete());
    }

    #[test]
    fn test_progress_set() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 100);
        
        tracker.set_progress(75);
        assert_eq!(tracker.progress_percentage(), 75.0);
        
        tracker.set_progress(100);
        assert_eq!(tracker.progress_percentage(), 100.0);
        assert!(tracker.is_complete());
    }

    #[test]
    fn test_thread_safe_progress() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 100);
        let thread_safe = tracker.thread_safe_ref();
        
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let tracker = thread_safe.clone();
                thread::spawn(move || {
                    for _ in 0..25 {
                        tracker.update_progress(1);
                        thread::sleep(Duration::from_millis(1));
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.progress_percentage(), 100.0);
        assert!(tracker.is_complete());
    }

    #[test]
    fn test_elapsed_time() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 100);
        
        thread::sleep(Duration::from_millis(100));
        let elapsed = tracker.elapsed_time();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn test_eta_calculation() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 100);
        
        // No ETA when no progress
        assert!(tracker.estimated_time_remaining().is_none());
        
        // Update progress
        tracker.update_progress(25);
        thread::sleep(Duration::from_millis(100));
        
        // Should have ETA now
        let eta = tracker.estimated_time_remaining();
        assert!(eta.is_some());
        assert!(eta.unwrap() > Duration::from_secs(0));
    }

    #[test]
    fn test_zero_total_work() {
        let tracker = ProgressTracker::new_default("test_task".to_string(), 0);
        assert_eq!(tracker.progress_percentage(), 100.0);
        assert!(tracker.is_complete());
    }
}

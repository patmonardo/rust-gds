// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! A timer that measures elapsed time and calls a callback when stopped.
//!
//! This timer is useful for tracking the duration of operations in graph algorithms
//! and can automatically report timing information when it goes out of scope.
//!
//! # Examples
//!
//! ```
//! use gds::core::utils::ProgressTimer;
//!
//! // Basic usage - measure and print
//! let timer = ProgressTimer::start(|duration| {
//!     println!("Operation took {} ms", duration);
//! });
//! // ... do work ...
//! timer.stop();
//!
//! // Or use RAII pattern (automatic stop on drop)
//! {
//!     let _timer = ProgressTimer::start(|d| println!("Took {} ms", d));
//!     // ... do work ...
//! } // Automatically stops here
//!
//! // Without callback
//! let timer = ProgressTimer::start_simple();
//! // ... do work ...
//! timer.stop();
//! println!("Duration: {} ms", timer.get_duration());
//! ```

use std::time::Instant;

/// A timer that measures elapsed time and optionally calls a callback when stopped.
///
/// Implements Drop to automatically stop the timer when it goes out of scope,
/// providing RAII-style resource management for timing measurements.
pub struct ProgressTimer {
    on_stop: Box<dyn FnOnce(u128) + Send>,
    start_time: Instant,
    duration: u128,
    stopped: bool,
}

impl ProgressTimer {
    /// Creates and starts a new timer with the specified callback.
    ///
    /// The callback will be invoked with the elapsed duration in milliseconds
    /// when the timer is stopped (either explicitly or via Drop).
    ///
    /// # Arguments
    /// * `on_stop` - Callback that receives the duration in milliseconds
    ///
    /// # Examples
    /// ```
    /// use gds::core::utils::ProgressTimer;
    ///
    /// let timer = ProgressTimer::start(|duration| {
    ///     println!("Graph algorithm completed in {} ms", duration);
    /// });
    /// // ... execute algorithm ...
    /// timer.stop();
    /// ```
    pub fn start<F>(on_stop: F) -> Self
    where
        F: FnOnce(u128) + Send + 'static,
    {
        Self {
            on_stop: Box::new(on_stop),
            start_time: Instant::now(),
            duration: 0,
            stopped: false,
        }
    }

    /// Creates and starts a new timer without a callback.
    ///
    /// Useful when you just want to measure time without automatic reporting.
    /// Use `get_duration()` to retrieve the measured time after stopping.
    ///
    /// # Examples
    /// ```
    /// use gds::core::utils::ProgressTimer;
    ///
    /// let timer = ProgressTimer::start_simple();
    /// // ... do work ...
    /// timer.stop();
    /// println!("Elapsed: {} ms", timer.get_duration());
    /// ```
    pub fn start_simple() -> Self {
        Self::start(|_| {})
    }

    /// Stops the timer and invokes the callback with the measured duration.
    ///
    /// If the timer has already been stopped, this method does nothing.
    /// Returns self for method chaining.
    ///
    /// # Examples
    /// ```
    /// use gds::core::utils::ProgressTimer;
    ///
    /// let timer = ProgressTimer::start_simple();
    /// // ... work ...
    /// let duration = timer.stop().get_duration();
    /// ```
    pub fn stop(mut self) -> Self {
        if !self.stopped {
            self.duration = self.start_time.elapsed().as_millis();
            // Take ownership of the callback and call it
            let callback = std::mem::replace(&mut self.on_stop, Box::new(|_| {}));
            callback(self.duration);
            self.stopped = true;
        }
        self
    }

    /// Gets the measured duration in milliseconds.
    ///
    /// Returns 0 if the timer hasn't been stopped yet.
    ///
    /// # Examples
    /// ```
    /// use gds::core::utils::ProgressTimer;
    ///
    /// let timer = ProgressTimer::start_simple();
    /// // ... work ...
    /// timer.stop();
    /// assert!(timer.get_duration() > 0);
    /// ```
    pub fn get_duration(&self) -> u128 {
        self.duration
    }
}

impl Drop for ProgressTimer {
    /// Automatically stops the timer when it goes out of scope.
    ///
    /// This enables RAII-style automatic timing with the following pattern:
    /// ```
    /// use gds::core::utils::ProgressTimer;
    ///
    /// {
    ///     let _timer = ProgressTimer::start(|d| println!("Took {} ms", d));
    ///     // ... algorithm executes ...
    /// } // Timer automatically stops and reports here
    /// ```
    fn drop(&mut self) {
        if !self.stopped {
            self.duration = self.start_time.elapsed().as_millis();
            let callback = std::mem::replace(&mut self.on_stop, Box::new(|_| {}));
            callback(self.duration);
            self.stopped = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timer_measures_duration() {
        let timer = ProgressTimer::start_simple();
        thread::sleep(Duration::from_millis(10));
        let timer = timer.stop();
        assert!(timer.get_duration() >= 10);
    }

    #[test]
    fn test_timer_callback_invoked() {
        let duration_holder = Arc::new(Mutex::new(0u128));
        let duration_clone = Arc::clone(&duration_holder);

        let timer = ProgressTimer::start(move |duration| {
            *duration_clone.lock().unwrap() = duration;
        });

        thread::sleep(Duration::from_millis(10));
        let _timer = timer.stop();

        let captured_duration = *duration_holder.lock().unwrap();
        assert!(captured_duration >= 10);
    }

    #[test]
    fn test_timer_auto_stop_on_drop() {
        let duration_holder = Arc::new(Mutex::new(0u128));
        let duration_clone = Arc::clone(&duration_holder);

        {
            let _timer = ProgressTimer::start(move |duration| {
                *duration_clone.lock().unwrap() = duration;
            });
            thread::sleep(Duration::from_millis(10));
        } // Timer should auto-stop here

        let captured_duration = *duration_holder.lock().unwrap();
        assert!(captured_duration >= 10);
    }

    #[test]
    fn test_timer_multiple_stop_calls_safe() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        let timer = ProgressTimer::start(move |_| {
            *counter_clone.lock().unwrap() += 1;
        });

        let _timer = timer.stop();
        // Callback should only be called once
        assert_eq!(*counter.lock().unwrap(), 1);
    }

    #[test]
    fn test_timer_duration_before_stop() {
        let timer = ProgressTimer::start_simple();
        assert_eq!(timer.get_duration(), 0);
    }
}

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

//! Progress value representing task completion state.

use std::fmt;

/// Marker for unknown volume.
pub const UNKNOWN_VOLUME: usize = usize::MAX;

/// Immutable progress value combining current progress and total volume.
///
/// Represents the completion state of a task with lazy calculation
/// of relative progress percentage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    progress: usize,
    volume: usize,
}

impl Progress {
    /// Create a new Progress value.
    pub fn of(progress: usize, volume: usize) -> Self {
        Self { progress, volume }
    }

    /// Create zero progress with given volume.
    pub fn zero(volume: usize) -> Self {
        Self::of(0, volume)
    }

    /// Create completed progress.
    pub fn completed(volume: usize) -> Self {
        Self::of(volume, volume)
    }

    /// Create progress with unknown volume.
    pub fn unknown(progress: usize) -> Self {
        Self::of(progress, UNKNOWN_VOLUME)
    }

    /// Get current progress value.
    pub fn progress(&self) -> usize {
        self.progress
    }

    /// Get total volume.
    pub fn volume(&self) -> usize {
        self.volume
    }

    /// Calculate relative progress (0.0 to 1.0, or UNKNOWN_VOLUME as f64).
    pub fn relative_progress(&self) -> f64 {
        if self.volume == UNKNOWN_VOLUME {
            return UNKNOWN_VOLUME as f64;
        }

        // Progress can be larger if volume was estimated too low
        if self.progress >= self.volume {
            1.0
        } else {
            self.progress as f64 / self.volume as f64
        }
    }

    /// Get progress as percentage (0.0 to 100.0).
    pub fn percentage(&self) -> f64 {
        let relative = self.relative_progress();
        if relative == UNKNOWN_VOLUME as f64 {
            return UNKNOWN_VOLUME as f64;
        }
        (relative * 100.0).min(100.0)
    }

    /// Check if task is complete.
    pub fn is_complete(&self) -> bool {
        self.volume != UNKNOWN_VOLUME && self.progress >= self.volume
    }

    /// Check if volume is unknown.
    pub fn has_unknown_volume(&self) -> bool {
        self.volume == UNKNOWN_VOLUME
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_unknown_volume() {
            write!(f, "{} / UNKNOWN", self.progress)
        } else {
            write!(
                f,
                "{} / {} ({:.1}%)",
                self.progress,
                self.volume,
                self.percentage()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_creation() {
        let p = Progress::of(50, 100);
        assert_eq!(p.progress(), 50);
        assert_eq!(p.volume(), 100);
    }

    #[test]
    fn test_relative_progress() {
        let p = Progress::of(50, 100);
        assert!((p.relative_progress() - 0.5).abs() < 0.001);

        let complete = Progress::of(100, 100);
        assert!((complete.relative_progress() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_over_completion() {
        // Volume estimated too low
        let p = Progress::of(150, 100);
        assert!((p.relative_progress() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_unknown_volume() {
        let p = Progress::unknown(42);
        assert!(p.has_unknown_volume());
        assert_eq!(p.relative_progress(), UNKNOWN_VOLUME as f64);
    }

    #[test]
    fn test_zero_and_completed() {
        let zero = Progress::zero(100);
        assert_eq!(zero.progress(), 0);
        assert!(!zero.is_complete());

        let done = Progress::completed(100);
        assert!(done.is_complete());
        assert_eq!(done.percentage(), 100.0);
    }

    #[test]
    fn test_percentage() {
        let p = Progress::of(25, 100);
        assert!((p.percentage() - 25.0).abs() < 0.1);
    }
}

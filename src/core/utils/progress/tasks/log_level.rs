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

//! Log level enumeration for task progress tracking.

use std::fmt;

/// Log level for progress tracking messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default)]
pub enum LogLevel {
    Debug = 1,
    #[default]
    Info = 2,
    Warning = 3,
}

impl LogLevel {
    /// Get numeric priority (higher = more important).
    pub fn priority(self) -> u8 {
        self as u8
    }

    /// Check if this level should be logged at the given threshold.
    pub fn should_log(self, threshold: LogLevel) -> bool {
        self.priority() >= threshold.priority()
    }
}


impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_priority() {
        assert!(LogLevel::Warning.priority() > LogLevel::Info.priority());
        assert!(LogLevel::Info.priority() > LogLevel::Debug.priority());
    }

    #[test]
    fn test_should_log() {
        assert!(LogLevel::Warning.should_log(LogLevel::Debug));
        assert!(LogLevel::Info.should_log(LogLevel::Debug));
        assert!(!LogLevel::Debug.should_log(LogLevel::Info));
    }

    #[test]
    fn test_default() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }
}

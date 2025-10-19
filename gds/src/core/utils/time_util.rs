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

use std::time::SystemTime;

/// Represents a time zone identifier, typically an IANA time zone name.
/// Examples: "UTC", "America/New_York", "Europe/Berlin".
pub type ZoneId = String;

/// Time utility functions.
/// This is a Rust translation of GDS's org.neo4j.gds.core.utils.TimeUtil.
pub struct TimeUtil;

impl TimeUtil {
    /// Returns a SystemTime representing the current moment in time.
    ///
    /// The original Java method `ZonedDateTime.now(ZoneId zoneId)` returns a `ZonedDateTime`
    /// object, which encapsulates both an instant and a specific time zone.
    /// Rust's `SystemTime` represents an instant in time (internally UTC)
    /// and does not carry explicit time zone information in the same way.
    ///
    /// This method returns `SystemTime::now()`, which captures the current instant.
    /// The `zone_id` parameter from the original Java method is included in the signature
    /// for consistency but is not directly used to alter the returned `SystemTime` value.
    /// If operations specific to the `zone_id` (e.g., getting the wall-clock hour in that zone)
    /// are required, a date-time library with time zone support (like `chrono`) would be necessary.
    ///
    /// The Java implementation uses `Clock.system(zoneId)` which refers to the actual
    /// system clock, not a potentially mocked clock from `ClockService`. Thus, `SystemTime::now()`
    /// aligns with this intent of getting the true current time.
    pub fn now(_zone_id: Option<ZoneId>) -> SystemTime {
        SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let time1 = TimeUtil::now(None);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let time2 = TimeUtil::now(Some("UTC".to_string()));
        assert!(time2 > time1);
    }
}

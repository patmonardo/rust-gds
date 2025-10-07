//! DatabaseInfo - Information about a database location and credentials.

use super::DatabaseId;
use std::fmt;

/// Location information for a database.
///
/// Currently supports remote locations with host, port, and optional credentials.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DatabaseLocation {
    /// Remote database location
    Remote {
        /// Host address
        host: String,
        /// Port number
        port: u16,
        /// Optional username
        username: Option<String>,
        /// Optional password
        password: Option<String>,
    },
}

impl DatabaseLocation {
    /// Creates a new remote database location.
    ///
    /// # Arguments
    /// * `host` - The host address
    /// * `port` - The port number
    /// * `username` - Optional username
    /// * `password` - Optional password
    pub fn remote<H: Into<String>>(
        host: H,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self::Remote {
            host: host.into(),
            port,
            username,
            password,
        }
    }
}

impl fmt::Display for DatabaseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Remote {
                host,
                port,
                username,
                ..
            } => {
                if let Some(user) = username {
                    write!(f, "{}@{}:{}", user, host, port)
                } else {
                    write!(f, "{}:{}", host, port)
                }
            }
        }
    }
}

/// Complete information about a database.
///
/// DatabaseInfo encapsulates the database identifier and its location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabaseInfo {
    database_id: DatabaseId,
    location: DatabaseLocation,
}

impl DatabaseInfo {
    /// Creates a new DatabaseInfo.
    ///
    /// # Arguments
    /// * `database_id` - The database identifier
    /// * `location` - The database location
    ///
    pub fn new(database_id: DatabaseId, location: DatabaseLocation) -> Self {
        Self {
            database_id,
            location,
        }
    }

    /// Returns the database ID.
    pub fn database_id(&self) -> &DatabaseId {
        &self.database_id
    }

    /// Returns the database location.
    pub fn location(&self) -> &DatabaseLocation {
        &self.location
    }
}

impl fmt::Display for DatabaseInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.database_id, self.location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_location() {
        let loc = DatabaseLocation::remote("localhost", 7687, None, None);
        assert_eq!(format!("{}", loc), "localhost:7687");
    }

    #[test]
    fn test_remote_location_with_username() {
        let loc = DatabaseLocation::remote(
            "localhost",
            7687,
            Some("neo4j".to_string()),
            Some("password".to_string()),
        );
        assert_eq!(format!("{}", loc), "neo4j@localhost:7687");
    }

    #[test]
    fn test_database_info() {
        let db_id = DatabaseId::new("test-db");
        let location = DatabaseLocation::remote("localhost", 7687, None, None);
        let info = DatabaseInfo::new(db_id, location);

        assert_eq!(info.database_id().value(), "test-db");
        assert_eq!(format!("{}", info.location()), "localhost:7687");
    }

    #[test]
    fn test_database_info_display() {
        let db_id = DatabaseId::new("neo4j");
        let location = DatabaseLocation::remote(
            "localhost",
            7687,
            Some("admin".to_string()),
            Some("pass".to_string()),
        );
        let info = DatabaseInfo::new(db_id, location);

        assert_eq!(format!("{}", info), "neo4j@admin@localhost:7687");
    }
}

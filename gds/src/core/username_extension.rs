use crate::core::context::Context;
use crate::core::username::Username;

/// Simulates the registration of a Username provider in a global registry.
/// This is a Rust adaptation of the Neo4j extension pattern.
pub struct UsernameExtension;

impl UsernameExtension {
    pub const EXTENSION_TYPE: &str = "GLOBAL";
    pub const NAME: &str = "gds.username";

    /// Factory method to create a Username from a context.
    pub fn create_username(context: &dyn Context) -> Username {
        let subject = context.security_context().subject();
        let username = subject.executing_user();
        Username::of(username)
    }
}

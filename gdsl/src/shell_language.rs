//! Shell Language - The GDSL shell language implementation
//!
//! This module implements the GDSL shell language that provides
//! the command interface between Kernel and UserLand.

use crate::messaging::{Message, MessageType};
use crate::ts_json::TSJsonInterface;

/// The GDSL shell language interpreter
pub struct ShellLanguage {
    ts_json_interface: TSJsonInterface,
}

impl ShellLanguage {
    /// Create a new GDSL shell language interpreter
    pub fn new() -> Self {
        Self {
            ts_json_interface: TSJsonInterface::new(),
        }
    }
    
    /// Execute a GDSL command
    pub fn execute(&self, command: &str) -> Result<String, String> {
        // Parse and execute GDSL shell commands
        match command {
            "translate" => Ok("Translation command executed".to_string()),
            "message" => Ok("Message command executed".to_string()),
            "form" => Ok("Form command executed".to_string()),
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
    
    /// Send command to Kernel
    pub fn send_to_kernel(&self, command: &str) -> Result<String, String> {
        let message = Message::new(
            MessageType::PureForm,
            serde_json::Value::String(command.to_string()),
        );
        self.ts_json_interface.send_to_kernel(&message)
    }
    
    /// Receive response from Kernel
    pub fn receive_from_kernel(&self, response: &str) -> Result<Message, String> {
        self.ts_json_interface.receive_from_kernel(response)
    }
}

impl Default for ShellLanguage {
    fn default() -> Self {
        Self::new()
    }
}

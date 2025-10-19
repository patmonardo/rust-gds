//! UserLand Interface - Interface to UserLand (Logic package)
//!
//! This module provides the interface that connects GDSL to UserLand
//! through the Logic package (Given Form Processor).

use crate::messaging::{Message, MessageType};
use crate::ts_json::TSJsonInterface;

/// Interface to UserLand (Logic package)
pub struct UserLandInterface {
    ts_json_interface: TSJsonInterface,
}

impl UserLandInterface {
    /// Create a new UserLand interface
    pub fn new() -> Self {
        Self {
            ts_json_interface: TSJsonInterface::new(),
        }
    }
    
    /// Send a Given Form to UserLand
    pub fn send_given_form(&self, given_form: &str) -> Result<String, String> {
        let message = Message::new(
            MessageType::GivenForm,
            serde_json::Value::String(given_form.to_string()),
        );
        self.ts_json_interface.send_to_kernel(&message)
    }
    
    /// Receive a Given Form from UserLand
    pub fn receive_given_form(&self, ts_json: &str) -> Result<Message, String> {
        self.ts_json_interface.receive_from_kernel(ts_json)
    }
    
    /// Process UserLand request
    pub fn process_userland_request(&self, request: &str) -> Result<String, String> {
        // Process request from UserLand (Logic package)
        Ok(format!("Processed UserLand request: {}", request))
    }
}

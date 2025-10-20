//! Model Interface - Interface to Model package (DSDL)
//!
//! This module provides the interface that connects GDSL to the Model package
//! which implements DSDL (Domain-Specific DSL) for View/Representations.

use crate::messaging::{Message, MessageType};
use crate::ts_json::TSJsonInterface;
use crate::sdsl_interface::SDSLInterface;

/// Interface to Model package (SDSL)
pub struct ModelInterface {
    ts_json_interface: TSJsonInterface,
    sdsl_interface: SDSLInterface,
}

impl ModelInterface {
    /// Create a new Model interface
    pub fn new() -> Self {
        Self {
            ts_json_interface: TSJsonInterface::new(),
            sdsl_interface: SDSLInterface::new(),
        }
    }
    
    /// Send a message to Model package
    pub fn send_to_model(&self, message: &str) -> Result<String, String> {
        let message = Message::new(
            MessageType::Translation,
            serde_json::Value::String(message.to_string()),
        );
        self.ts_json_interface.send_to_kernel(&message)
    }
    
    /// Receive a message from Model package
    pub fn receive_from_model(&self, ts_json: &str) -> Result<Message, String> {
        self.ts_json_interface.receive_from_kernel(ts_json)
    }
    
    /// Process Model request
    pub fn process_model_request(&self, request: &str) -> Result<String, String> {
        // Process request from Model package (DSDL)
        Ok(format!("Processed Model request: {}", request))
    }
    
    /// Get SDSL interface
    pub fn sdsl_interface(&self) -> &SDSLInterface {
        &self.sdsl_interface
    }
    
    /// Translate between Kernel and Model
    pub fn translate_kernel_to_model(&self, kernel_message: &str) -> Result<String, String> {
        // Translate Kernel message to Model (SDSL) representation
        self.sdsl_interface.translate_pure_to_sdsl(kernel_message)
    }
    
    /// Translate between Model and Kernel
    pub fn translate_model_to_kernel(&self, model_message: &str) -> Result<String, String> {
        // Translate Model (SDSL) representation to Kernel message
        self.sdsl_interface.translate_sdsl_to_pure(model_message)
    }
}

impl Default for ModelInterface {
    fn default() -> Self {
        Self::new()
    }
}

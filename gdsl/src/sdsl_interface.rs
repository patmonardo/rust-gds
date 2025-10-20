//! SDSL Interface - Species-Specific DSL for View/Representations
//!
//! This module provides the interface to the Model package (SDSL) which is
//! a middleware communication language for View/Representations that feeds
//! into the TaskProcessor.

use crate::messaging::{Message, MessageType};
use crate::ts_json::TSJsonInterface;

/// Interface to Model package (SDSL)
pub struct SDSLInterface {
    ts_json_interface: TSJsonInterface,
}

impl SDSLInterface {
    /// Create a new SDSL interface
    pub fn new() -> Self {
        Self {
            ts_json_interface: TSJsonInterface::new(),
        }
    }
    
    /// Send a View representation to DSDL
    pub fn send_view_representation(&self, view: &str) -> Result<String, String> {
        let message = Message::new(
            MessageType::GivenForm,
            serde_json::Value::String(view.to_string()),
        );
        self.ts_json_interface.send_to_kernel(&message)
    }
    
    /// Receive a View representation from DSDL
    pub fn receive_view_representation(&self, ts_json: &str) -> Result<Message, String> {
        self.ts_json_interface.receive_from_kernel(ts_json)
    }
    
    /// Process SDSL representation
    pub fn process_sdsl_representation(&self, representation: &str) -> Result<String, String> {
        // Process SDSL representation for View/Representations
        Ok(format!("Processed SDSL representation: {}", representation))
    }
    
    /// Translate Pure Form to SDSL representation
    pub fn translate_pure_to_sdsl(&self, pure_form: &str) -> Result<String, String> {
        // Translate Pure Form (from Kernel) to SDSL representation
        Ok(format!("SDSL: {}", pure_form))
    }
    
    /// Translate SDSL representation to Pure Form
    pub fn translate_sdsl_to_pure(&self, sdsl_representation: &str) -> Result<String, String> {
        // Translate SDSL representation to Pure Form (for Kernel)
        Ok(format!("PureForm: {}", sdsl_representation))
    }
}

impl Default for SDSLInterface {
    fn default() -> Self {
        Self::new()
    }
}

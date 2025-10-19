//! Messaging - Core messaging protocol between Kernel and UserLand
//!
//! This module implements the messaging protocol that allows GDSL to
//! communicate between the Kernel (Pure Form Processor) and UserLand
//! (Given Form Processor).

use serde::{Deserialize, Serialize};

/// Message types for Kernel-UserLand communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Pure Form message from Kernel
    PureForm,
    /// Given Form message to UserLand
    GivenForm,
    /// Translation request
    Translation,
    /// Response message
    Response,
}

/// A message in the GDSL protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

impl Message {
    /// Create a new message
    pub fn new(message_type: MessageType, payload: serde_json::Value) -> Self {
        Self {
            message_type,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Serialize message to TS-JSON
    pub fn to_ts_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// Deserialize message from TS-JSON
    pub fn from_ts_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

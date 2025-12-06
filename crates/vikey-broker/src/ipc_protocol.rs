//! IPC Protocol
//!
//! Shared message types between TSF DLL and broker

use serde::{Deserialize, Serialize};

/// IPC Request from TSF DLL to broker
#[derive(Serialize, Deserialize, Debug)]
pub enum IpcRequest {
    /// Process a keystroke
    ProcessKey(char),
    /// Process backspace
    ProcessBackspace,
    /// Reset engine state
    Reset,
    /// Ping to check if broker is alive
    Ping,
}

/// IPC Response from broker to TSF DLL
#[derive(Serialize, Deserialize, Debug)]
pub enum IpcResponse {
    /// Action to execute
    Action(Action),
    /// Pong response
    Pong,
    /// Error occurred
    Error(String),
}

/// Action to execute in the application
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    /// Do nothing, pass through
    DoNothing,
    /// Replace text: delete N chars, insert string
    Replace { delete: usize, insert: String },
}

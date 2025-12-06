//! IPC Client for communicating with vikey-broker
//!
//! Uses Named Pipes to send keystrokes and receive actions

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::time::Duration;

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

/// IPC Client for Named Pipe communication
pub struct IpcClient {
    pipe_path: String,
    connected: bool,
}

impl IpcClient {
    /// Create new IPC client
    pub fn new() -> Self {
        Self {
            pipe_path: r"\\.\pipe\vikey-broker".to_string(),
            connected: false,
        }
    }
    
    /// Try to connect to broker
    pub fn connect(&mut self) -> Result<(), String> {
        #[cfg(debug_assertions)]
        {
            use std::io::Write;
            let log_path = std::env::temp_dir().join("vikey_tip.log");
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
            {
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let _ = writeln!(file, "[{}] IpcClient::connect() attempting...", timestamp);
            }
        }
        
        // Try to open pipe
        match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.pipe_path)
        {
            Ok(_) => {
                self.connected = true;
                
                #[cfg(debug_assertions)]
                {
                    use std::io::Write;
                    let log_path = std::env::temp_dir().join("vikey_tip.log");
                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_path)
                    {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                        let _ = writeln!(file, "[{}] IpcClient connected successfully", timestamp);
                    }
                }
                
                Ok(())
            }
            Err(e) => {
                self.connected = false;
                
                #[cfg(debug_assertions)]
                {
                    use std::io::Write;
                    let log_path = std::env::temp_dir().join("vikey_tip.log");
                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_path)
                    {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                        let _ = writeln!(file, "[{}] IpcClient connection failed: {}", timestamp, e);
                    }
                }
                
                Err(format!("Failed to connect: {}", e))
            }
        }
    }
    
    /// Send request and receive response
    fn send_request(&mut self, request: &IpcRequest) -> Result<IpcResponse, String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }
        
        // Open pipe for this request
        let mut pipe = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.pipe_path)
            .map_err(|e| {
                self.connected = false;
                format!("Pipe error: {}", e)
            })?;
        
        // Serialize request
        let request_bytes = bincode::serialize(request)
            .map_err(|e| format!("Serialize error: {}", e))?;
        
        // Send request
        pipe.write_all(&request_bytes)
            .map_err(|e| format!("Write error: {}", e))?;
        
        pipe.flush()
            .map_err(|e| format!("Flush error: {}", e))?;
        
        // Read response
        let mut response_bytes = vec![0u8; 4096];
        let n = pipe.read(&mut response_bytes)
            .map_err(|e| format!("Read error: {}", e))?;
        
        if n == 0 {
            return Err("Empty response".to_string());
        }
        
        // Deserialize response
        let response: IpcResponse = bincode::deserialize(&response_bytes[..n])
            .map_err(|e| format!("Deserialize error: {}", e))?;
        
        Ok(response)
    }
    
    /// Process a keystroke
    pub fn process_key(&mut self, ch: char) -> Result<Action, String> {
        #[cfg(debug_assertions)]
        {
            use std::io::Write;
            let log_path = std::env::temp_dir().join("vikey_tip.log");
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
            {
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let _ = writeln!(file, "[{}] IpcClient::process_key('{}')", timestamp, ch);
            }
        }
        
        let request = IpcRequest::ProcessKey(ch);
        
        match self.send_request(&request) {
            Ok(IpcResponse::Action(action)) => {
                #[cfg(debug_assertions)]
                {
                    use std::io::Write;
                    let log_path = std::env::temp_dir().join("vikey_tip.log");
                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_path)
                    {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                        let _ = writeln!(file, "[{}] IpcClient received action: {:?}", timestamp, action);
                    }
                }
                
                Ok(action)
            }
            Ok(IpcResponse::Error(err)) => Err(err),
            Ok(_) => Err("Unexpected response".to_string()),
            Err(e) => Err(e),
        }
    }
    
    /// Process backspace
    pub fn process_backspace(&mut self) -> Result<Action, String> {
        let request = IpcRequest::ProcessBackspace;
        
        match self.send_request(&request) {
            Ok(IpcResponse::Action(action)) => Ok(action),
            Ok(IpcResponse::Error(err)) => Err(err),
            Ok(_) => Err("Unexpected response".to_string()),
            Err(e) => Err(e),
        }
    }
    
    /// Reset engine state
    pub fn reset(&mut self) -> Result<(), String> {
        let request = IpcRequest::Reset;
        
        match self.send_request(&request) {
            Ok(IpcResponse::Action(_)) => Ok(()),
            Ok(IpcResponse::Error(err)) => Err(err),
            Ok(_) => Err("Unexpected response".to_string()),
            Err(e) => Err(e),
        }
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

impl Default for IpcClient {
    fn default() -> Self {
        Self::new()
    }
}

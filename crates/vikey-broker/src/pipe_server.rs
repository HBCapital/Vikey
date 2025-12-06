//! Named Pipe Server - Simplified Implementation
//!
//! Uses std library for cross-platform compatibility

use crate::ipc_protocol::{IpcRequest, IpcResponse, Action};
use vikey_core::{Engine, Action as EngineAction};
use anyhow::{Result, Context};
use tracing::{info, warn, error, debug};
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::os::windows::fs::OpenOptionsExt;

const PIPE_NAME: &str = r"\\.\pipe\vikey-broker";
const BUFFER_SIZE: usize = 4096;

/// Run the Named Pipe server
pub fn run_server(mut engine: Engine) -> Result<()> {
    info!("Starting Named Pipe server: {}", PIPE_NAME);
    info!("Note: Using simplified std implementation");
    
    loop {
        info!("Creating pipe instance...");
        
        // Open pipe for reading/writing
        // This will block until a client connects
        let mut pipe = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(0x00000003) // FILE_FLAG_OVERLAPPED | FILE_FLAG_WRITE_THROUGH
            .open(PIPE_NAME)
            .context("Failed to open pipe")?;
        
        info!("Client connected!");
        
        // Handle client
        if let Err(e) = handle_client(&mut pipe, &mut engine) {
            error!("Client error: {:?}", e);
        }
        
        info!("Client disconnected");
    }
}

/// Handle client connection
fn handle_client(pipe: &mut std::fs::File, engine: &mut Engine) -> Result<()> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    
    loop {
        // Read request
        let bytes_read = match pipe.read(&mut buffer) {
            Ok(0) => {
                debug!("Client disconnected (0 bytes)");
                break;
            }
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                debug!("Client disconnected (broken pipe)");
                break;
            }
            Err(e) => {
                return Err(e).context("Read failed");
            }
        };
        
        debug!("Received {} bytes", bytes_read);
        
        // Deserialize request
        let request: IpcRequest = match bincode::deserialize(&buffer[..bytes_read]) {
            Ok(req) => req,
            Err(e) => {
                warn!("Failed to deserialize: {}", e);
                continue;
            }
        };
        
        debug!("Request: {:?}", request);
        
        // Process request
        let response = process_request(request, engine);
        
        debug!("Response: {:?}", response);
        
        // Serialize response
        let response_bytes = bincode::serialize(&response)
            .context("Failed to serialize response")?;
        
        // Write response
        pipe.write_all(&response_bytes)
            .context("Write failed")?;
        
        pipe.flush()
            .context("Flush failed")?;
        
        debug!("Sent {} bytes", response_bytes.len());
    }
    
    Ok(())
}

/// Process IPC request
fn process_request(request: IpcRequest, engine: &mut Engine) -> IpcResponse {
    match request {
        IpcRequest::ProcessKey(ch) => {
            info!("Processing key: '{}'", ch);
            
            match engine.process(ch) {
                EngineAction::Replace { backspace_count, text } => {
                    info!("→ Replace(delete={}, insert='{}')", backspace_count, text);
                    IpcResponse::Action(Action::Replace { 
                        delete: backspace_count, 
                        insert: text 
                    })
                }
                EngineAction::DoNothing => {
                    debug!("→ DoNothing");
                    IpcResponse::Action(Action::DoNothing)
                }
                EngineAction::Commit(text) => {
                    info!("→ Commit('{}')", text);
                    IpcResponse::Action(Action::Replace { 
                        delete: 0, 
                        insert: text 
                    })
                }
            }
        }
        IpcRequest::ProcessBackspace => {
            info!("Processing backspace");
            
            match engine.process_backspace() {
                EngineAction::Replace { backspace_count, text } => {
                    info!("→ Replace(delete={}, insert='{}')", backspace_count, text);
                    IpcResponse::Action(Action::Replace { 
                        delete: backspace_count, 
                        insert: text 
                    })
                }
                EngineAction::DoNothing => {
                    debug!("→ DoNothing");
                    IpcResponse::Action(Action::DoNothing)
                }
                EngineAction::Commit(text) => {
                    info!("→ Commit('{}')", text);
                    IpcResponse::Action(Action::Replace { 
                        delete: 0, 
                        insert: text 
                    })
                }
            }
        }
        IpcRequest::Reset => {
            info!("Resetting engine");
            engine.reset();
            IpcResponse::Action(Action::DoNothing)
        }
        IpcRequest::Ping => {
            debug!("Ping → Pong");
            IpcResponse::Pong
        }
    }
}

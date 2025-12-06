//! Vikey Broker Service
//!
//! Named Pipe server that processes keystrokes using vikey-core

mod ipc_protocol;
mod pipe_server;

use anyhow::Result;
use vikey_core::Engine;
use vikey_vietnamese::VietnamesePlugin;
use tracing::{info, error};

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("Vikey Broker starting...");
    
    // Create engine
    let mut engine = Engine::new();
    
    // Register Vietnamese plugin
    let vietnamese_plugin = VietnamesePlugin::new();
    engine.register(Box::new(vietnamese_plugin))?;
    
    // Set language and input method
    engine.set_language("vietnamese")?;  // Use plugin ID, not language code
    engine.set_input_method("telex")?;
    
    info!("Engine initialized with Vietnamese Telex");
    
    // Start Named Pipe server
    info!("Starting Named Pipe server on \\\\.\\pipe\\vikey-broker");
    
    match pipe_server::run_server(engine) {
        Ok(()) => {
            info!("Server stopped gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Server error: {:?}", e);
            Err(e)
        }
    }
}

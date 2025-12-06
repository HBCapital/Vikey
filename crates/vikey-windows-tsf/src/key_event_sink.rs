//! Key Event Sink
//!
//! Implements ITfKeyEventSink to intercept and process keystrokes

use windows::core::*;
use windows::Win32::UI::TextServices::*;
use windows::Win32::Foundation::*;
use std::sync::Mutex;
use crate::ipc::{IpcClient, Action};
use crate::text_ops;

/// Key Event Sink for Vikey
#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    ipc_client: Mutex<IpcClient>,
    client_id: Mutex<u32>,
}

impl KeyEventSink {
    pub fn new() -> Self {
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
                let _ = writeln!(file, "[{}] KeyEventSink::new()", timestamp);
            }
        }
        
        // Create IPC client and try to connect
        let mut client = IpcClient::new();
        let _ = client.connect(); // Ignore error, will retry later
        
        Self {
            ipc_client: Mutex::new(client),
            client_id: Mutex::new(0),
        }
    }
    
    /// Set client ID (called from TextService)
    pub fn set_client_id(&self, client_id: u32) {
        *self.client_id.lock().unwrap() = client_id;
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink {
    fn OnSetFocus(&self, fforeground: BOOL) -> Result<()> {
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
                let _ = writeln!(file, "[{}] OnSetFocus(foreground={})", timestamp, fforeground.as_bool());
            }
        }
        
        Ok(())
    }
    
    fn OnTestKeyDown(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        let vkey = wparam.0 as u32;
        
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
                let _ = writeln!(file, "[{}] OnTestKeyDown(vkey=0x{:X}, lparam=0x{:X})", 
                    timestamp, vkey, lparam.0);
            }
        }
        
        // Check if this is a printable character
        if is_printable_key(vkey) {
            // We want to handle this key
            Ok(BOOL(1))
        } else {
            // Pass through
            Ok(BOOL(0))
        }
    }
    
    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        let vkey = wparam.0 as u32;
        let ch = vkey_to_char(vkey);
        
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
                let _ = writeln!(file, "[{}] OnKeyDown(vkey=0x{:X}, char='{}', lparam=0x{:X})", 
                    timestamp, vkey, ch, lparam.0);
            }
        }
        
        // Try to process with IPC
        let mut client = self.ipc_client.lock().unwrap();
        
        // If not connected, try to reconnect
        if !client.is_connected() {
            let _ = client.connect();
        }
        
        // If still not connected, pass through
        if !client.is_connected() {
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
                    let _ = writeln!(file, "[{}] IPC not connected, passing through", timestamp);
                }
            }
            return Ok(BOOL(0)); // Pass through
        }
        
        // Process keystroke via IPC
        match client.process_key(ch) {
            Ok(Action::Replace { delete, insert }) => {
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
                        let _ = writeln!(file, "[{}] Action::Replace(delete={}, insert='{}')", 
                            timestamp, delete, insert);
                    }
                }
                
                // Execute text manipulation
                if let Some(context) = pic {
                    let client_id = *self.client_id.lock().unwrap();
                    
                    unsafe {
                        match text_ops::execute_action(context, client_id, delete, &insert) {
                            Ok(()) => {
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
                                        let _ = writeln!(file, "[{}] Text manipulation successful", timestamp);
                                    }
                                }
                                
                                // Return TRUE to indicate we handled the key
                                Ok(BOOL(1))
                            }
                            Err(e) => {
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
                                        let _ = writeln!(file, "[{}] Text manipulation failed: {:?}", timestamp, e);
                                    }
                                }
                                
                                // On error, pass through
                                Ok(BOOL(0))
                            }
                        }
                    }
                } else {
                    // No context, pass through
                    Ok(BOOL(0))
                }
            }
            Ok(Action::DoNothing) => {
                // Pass through
                Ok(BOOL(0))
            }
            Err(e) => {
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
                        let _ = writeln!(file, "[{}] IPC error: {}", timestamp, e);
                    }
                }
                
                // On error, pass through
                Ok(BOOL(0))
            }
        }
    }
    
    fn OnTestKeyUp(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        let vkey = wparam.0 as u32;
        
        // We don't handle key up events
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
                let _ = writeln!(file, "[{}] OnTestKeyUp(vkey=0x{:X})", timestamp, vkey);
            }
        }
        
        Ok(BOOL(0))
    }
    
    fn OnKeyUp(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        let vkey = wparam.0 as u32;
        
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
                let _ = writeln!(file, "[{}] OnKeyUp(vkey=0x{:X})", timestamp, vkey);
            }
        }
        
        Ok(BOOL(0))
    }
    
    fn OnPreservedKey(
        &self,
        _pic: Option<&ITfContext>,
        _rguid: *const GUID,
    ) -> Result<BOOL> {
        // We don't use preserved keys
        Ok(BOOL(0))
    }
}

/// Check if a virtual key is a printable character
fn is_printable_key(vkey: u32) -> bool {
    match vkey {
        // A-Z
        0x41..=0x5A => true,
        // 0-9
        0x30..=0x39 => true,
        // Space
        0x20 => true,
        // OEM keys (punctuation, etc.)
        0xBA..=0xC0 => true,
        0xDB..=0xDF => true,
        _ => false,
    }
}

/// Convert virtual key to character
fn vkey_to_char(vkey: u32) -> char {
    match vkey {
        // A-Z
        0x41..=0x5A => ((vkey - 0x41) as u8 + b'a') as char,
        // 0-9
        0x30..=0x39 => ((vkey - 0x30) as u8 + b'0') as char,
        // Space
        0x20 => ' ',
        _ => '?',
    }
}

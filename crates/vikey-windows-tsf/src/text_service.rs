//! Vikey Text Service
//! 
//! Implements ITfTextInputProcessor for TSF integration

use windows::core::*;
use windows::Win32::UI::TextServices::*;
use windows::Win32::Foundation::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use crate::com::{dll_add_ref, dll_release};
use crate::key_event_sink::KeyEventSink;

/// Vikey Text Service
#[implement(ITfTextInputProcessor)]
pub struct TextService {
    ref_count: AtomicU32,
    
    // Thread manager and client ID
    thread_mgr: Mutex<Option<ITfThreadMgr>>,
    client_id: Mutex<u32>,
    
    // Activated state
    activated: Mutex<bool>,
    
    // Key event sink
    key_event_sink: Mutex<Option<ITfKeyEventSink>>,
    key_event_sink_cookie: Mutex<u32>,
}

impl TextService {
    pub fn new() -> Self {
        dll_add_ref();
        
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
                let _ = writeln!(file, "[{}] TextService::new()", timestamp);
            }
        }
        
        Self {
            ref_count: AtomicU32::new(1),
            thread_mgr: Mutex::new(None),
            client_id: Mutex::new(0),
            activated: Mutex::new(false),
            key_event_sink: Mutex::new(None),
            key_event_sink_cookie: Mutex::new(0),
        }
    }
}

impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(
        &self,
        ptim: Option<&ITfThreadMgr>,
        tid: u32,
    ) -> Result<()> {
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
                let _ = writeln!(file, "[{}] TextService::Activate(tid={})", timestamp, tid);
            }
        }
        
        // Check if already activated
        let mut activated = self.activated.lock().unwrap();
        if *activated {
            return Err(E_UNEXPECTED.into());
        }
        
        // Store thread manager and client ID
        if let Some(thread_mgr) = ptim {
            *self.thread_mgr.lock().unwrap() = Some(thread_mgr.clone());
            *self.client_id.lock().unwrap() = tid;
            *activated = true;
            
            // Register KeyEventSink
            unsafe {
                match self.register_key_event_sink(thread_mgr, tid) {
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
                                let _ = writeln!(file, "[{}] KeyEventSink registered successfully", timestamp);
                            }
                        }
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
                                let _ = writeln!(file, "[{}] Failed to register KeyEventSink: {:?}", timestamp, e);
                            }
                        }
                        // Don't fail activation if sink registration fails
                    }
                }
            }
            
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
                    let _ = writeln!(file, "[{}] TextService activated successfully", timestamp);
                }
            }
            
            Ok(())
        } else {
            Err(E_INVALIDARG.into())
        }
    }
    
    fn Deactivate(&self) -> Result<()> {
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
                let _ = writeln!(file, "[{}] TextService::Deactivate()", timestamp);
            }
        }
        
        // Check if activated
        let mut activated = self.activated.lock().unwrap();
        if !*activated {
            return Err(E_UNEXPECTED.into());
        }
        
        // Unregister KeyEventSink
        unsafe {
            if let Err(e) = self.unregister_key_event_sink() {
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
                        let _ = writeln!(file, "[{}] Failed to unregister KeyEventSink: {:?}", timestamp, e);
                    }
                }
            }
        }
        
        // Clean up
        *self.thread_mgr.lock().unwrap() = None;
        *self.client_id.lock().unwrap() = 0;
        *activated = false;
        
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
                let _ = writeln!(file, "[{}] TextService deactivated successfully", timestamp);
            }
        }
        
        Ok(())
    }
}

impl TextService {
    /// Register KeyEventSink with keystroke manager
    unsafe fn register_key_event_sink(
        &self,
        thread_mgr: &ITfThreadMgr,
        tid: u32,
    ) -> Result<()> {
        // Get keystroke manager
        let keystroke_mgr: ITfKeystrokeMgr = thread_mgr.cast()?;
        
        // Create key event sink
        let sink: ITfKeyEventSink = KeyEventSink::new().into();
        
        // Advise sink
        let mut cookie: u32 = 0;
        keystroke_mgr.AdviseKeyEventSink(tid, &sink, TRUE)?;
        
        // Store sink and cookie
        *self.key_event_sink.lock().unwrap() = Some(sink);
        *self.key_event_sink_cookie.lock().unwrap() = cookie;
        
        Ok(())
    }
    
    /// Unregister KeyEventSink
    unsafe fn unregister_key_event_sink(&self) -> Result<()> {
        let cookie = *self.key_event_sink_cookie.lock().unwrap();
        
        if cookie != 0 {
            if let Some(thread_mgr) = self.thread_mgr.lock().unwrap().as_ref() {
                let keystroke_mgr: ITfKeystrokeMgr = thread_mgr.cast()?;
                keystroke_mgr.UnadviseKeyEventSink(cookie)?;
            }
        }
        
        // Clear sink
        *self.key_event_sink.lock().unwrap() = None;
        *self.key_event_sink_cookie.lock().unwrap() = 0;
        
        Ok(())
    }
}

impl Drop for TextService {
    fn drop(&mut self) {
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
                let _ = writeln!(file, "[{}] TextService::drop()", timestamp);
            }
        }
        
        dll_release();
    }
}

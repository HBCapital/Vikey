//! Vikey Windows TSF Text Service
//!
//! This DLL implements a Windows Text Services Framework (TSF) input method
//! for Vietnamese language input.

#![cfg(windows)]
#![allow(non_snake_case)]

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;

mod com;
mod factory;
mod text_service;
mod registration;
mod key_event_sink;
mod ipc;
mod text_ops;

use factory::ClassFactory;
use registration::{register_server, unregister_server, get_dll_path};

// CLSID for Vikey Text Service
// TODO: Generate proper GUID with uuidgen
// For now using placeholder
pub const CLSID_VIKEY_TEXT_SERVICE: GUID = GUID::from_u128(0x12345678_1234_1234_1234_123456789ABC);

/// DLL entry point
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: u32,
    _lpv_reserved: *const core::ffi::c_void,
) -> BOOL {
    const DLL_PROCESS_ATTACH: u32 = 1;
    const DLL_PROCESS_DETACH: u32 = 0;
    
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            // Initialize logging
            #[cfg(debug_assertions)]
            {
                log_debug("DLL_PROCESS_ATTACH");
            }
        }
        DLL_PROCESS_DETACH => {
            #[cfg(debug_assertions)]
            {
                log_debug("DLL_PROCESS_DETACH");
            }
        }
        _ => {}
    }
    
    BOOL(1) // TRUE
}

/// Get class object for COM
#[no_mangle]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut core::ffi::c_void,
) -> HRESULT {
    unsafe {
        if ppv.is_null() {
            return E_POINTER;
        }
        
        *ppv = std::ptr::null_mut();
        
        if rclsid.is_null() || riid.is_null() {
            return E_INVALIDARG;
        }
        
        // Check if requesting our CLSID
        if *rclsid != CLSID_VIKEY_TEXT_SERVICE {
            return CLASS_E_CLASSNOTAVAILABLE;
        }
        
        // Create and return class factory
        let factory: IClassFactory = ClassFactory::new().into();
        
        factory.query(riid, ppv as *mut _)
    }
}

/// Check if DLL can be unloaded
#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    if com::dll_can_unload() {
        S_OK
    } else {
        S_FALSE
    }
}

/// Register the TSF service
#[no_mangle]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    #[cfg(debug_assertions)]
    log_debug("DllRegisterServer called");
    
    match get_dll_path() {
        Ok(dll_path) => {
            match register_server(&dll_path) {
                Ok(()) => {
                    #[cfg(debug_assertions)]
                    log_debug(&format!("Registered: {:?}", dll_path));
                    
                    S_OK
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    log_debug(&format!("Registration failed: {:?}", e));
                    
                    E_FAIL
                }
            }
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            log_debug(&format!("Failed to get DLL path: {:?}", e));
            
            E_FAIL
        }
    }
}

/// Unregister the TSF service
#[no_mangle]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    #[cfg(debug_assertions)]
    log_debug("DllUnregisterServer called");
    
    match unregister_server() {
        Ok(()) => S_OK,
        Err(e) => {
            #[cfg(debug_assertions)]
            log_debug(&format!("Unregistration failed: {:?}", e));
            E_FAIL
        }
    }
}

#[cfg(debug_assertions)]
fn log_debug(msg: &str) {
    use std::io::Write;
    let log_path = std::env::temp_dir().join("vikey_tip.log");
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
    {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clsid() {
        // Just verify CLSID is valid
        assert_ne!(CLSID_VIKEY_TEXT_SERVICE, GUID::zeroed());
    }
    
    #[test]
    fn test_ref_counting() {
        assert_eq!(com::dll_get_ref_count(), 0);
        com::dll_add_ref();
        assert_eq!(com::dll_get_ref_count(), 1);
        com::dll_release();
        assert_eq!(com::dll_get_ref_count(), 0);
    }
}

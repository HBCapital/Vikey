//! Registration Module
//!
//! Handles COM server and TSF service registration

use anyhow::{Result, Context};
use std::path::PathBuf;
use winreg::RegKey;
use winreg::enums::*;

// GUIDs for Vikey TSF
pub const CLSID_VIKEY_TEXT_SERVICE: &str = "{E6B8A6C0-1234-5678-9ABC-DEF012345678}";
pub const GUID_PROFILE: &str = "{E6B8A6C1-1234-5678-9ABC-DEF012345678}";

// Vietnamese language ID (0x042A)
const LANGID_VIETNAMESE: u32 = 0x042A;

/// Register COM server
pub fn register_com_server(dll_path: &PathBuf) -> Result<()> {
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
            let _ = writeln!(file, "[{}] register_com_server({})", timestamp, dll_path.display());
        }
    }
    
    // HKEY_LOCAL_MACHINE\SOFTWARE\Classes\CLSID\{GUID}
    // Using HKLM instead of HKCR for better reliability
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let clsid_path = format!("SOFTWARE\\Classes\\CLSID\\{}", CLSID_VIKEY_TEXT_SERVICE);
    
    let (clsid_key, _) = hklm.create_subkey(&clsid_path)
        .context("Failed to create CLSID key")?;
    
    clsid_key.set_value("", &"Vikey Vietnamese Input")
        .context("Failed to set CLSID description")?;
    
    // InprocServer32
    let (inproc_key, _) = clsid_key.create_subkey("InprocServer32")
        .context("Failed to create InprocServer32 key")?;
    
    let dll_path_str = dll_path.to_string_lossy().to_string();
    inproc_key.set_value("", &dll_path_str)
        .context("Failed to set DLL path")?;
    
    inproc_key.set_value("ThreadingModel", &"Apartment")
        .context("Failed to set threading model")?;
    
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
            let _ = writeln!(file, "[{}] COM server registered successfully", timestamp);
        }
    }
    
    Ok(())
}

/// Unregister COM server
pub fn unregister_com_server() -> Result<()> {
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
            let _ = writeln!(file, "[{}] unregister_com_server()", timestamp);
        }
    }
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let clsid_path = format!("SOFTWARE\\Classes\\CLSID\\{}", CLSID_VIKEY_TEXT_SERVICE);
    
    match hklm.delete_subkey_all(&clsid_path) {
        Ok(_) => {
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
                    let _ = writeln!(file, "[{}] COM server unregistered successfully", timestamp);
                }
            }
            Ok(())
        }
        Err(e) => {
            // Key might not exist, that's ok
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
                    let _ = writeln!(file, "[{}] COM server key not found (already unregistered?): {}", timestamp, e);
                }
            }
            Ok(())
        }
    }
}

/// Register TSF service
pub fn register_tsf_service(dll_path: &PathBuf) -> Result<()> {
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
            let _ = writeln!(file, "[{}] register_tsf_service({})", timestamp, dll_path.display());
        }
    }
    
    // HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\CTF\TIP\{CLSID}
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let tip_path = format!("SOFTWARE\\Microsoft\\CTF\\TIP\\{}", CLSID_VIKEY_TEXT_SERVICE);
    
    let (tip_key, _) = hklm.create_subkey(&tip_path)
        .context("Failed to create TIP key")?;
    
    // LanguageProfile\{LANGID}\{PROFILE_GUID}
    let profile_path = format!("LanguageProfile\\0x{:08X}\\{}", LANGID_VIETNAMESE, GUID_PROFILE);
    let (profile_key, _) = tip_key.create_subkey(&profile_path)
        .context("Failed to create language profile key")?;
    
    profile_key.set_value("Description", &"Vikey - Vietnamese Input")
        .context("Failed to set description")?;
    
    let dll_path_str = dll_path.to_string_lossy().to_string();
    profile_key.set_value("IconFile", &dll_path_str)
        .context("Failed to set icon file")?;
    
    profile_key.set_value("IconIndex", &0u32)
        .context("Failed to set icon index")?;
    
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
            let _ = writeln!(file, "[{}] TSF service registered successfully", timestamp);
        }
    }
    
    Ok(())
}

/// Unregister TSF service
pub fn unregister_tsf_service() -> Result<()> {
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
            let _ = writeln!(file, "[{}] unregister_tsf_service()", timestamp);
        }
    }
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let tip_path = format!("SOFTWARE\\Microsoft\\CTF\\TIP\\{}", CLSID_VIKEY_TEXT_SERVICE);
    
    match hklm.delete_subkey_all(&tip_path) {
        Ok(_) => {
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
                    let _ = writeln!(file, "[{}] TSF service unregistered successfully", timestamp);
                }
            }
            Ok(())
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
                    let _ = writeln!(file, "[{}] TSF service key not found (already unregistered?): {}", timestamp, e);
                }
            }
            Ok(())
        }
    }
}

/// Register server (called by DllRegisterServer)
pub fn register_server(dll_path: &PathBuf) -> Result<()> {
    register_com_server(dll_path)?;
    register_tsf_service(dll_path)?;
    Ok(())
}

/// Unregister server (called by DllUnregisterServer)
pub fn unregister_server() -> Result<()> {
    unregister_tsf_service()?;
    unregister_com_server()?;
    Ok(())
}

/// Get DLL path
pub fn get_dll_path() -> Result<PathBuf> {
    use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
    use windows::Win32::Foundation::HMODULE;
    
    unsafe {
        let mut buffer = vec![0u16; 260];
        let len = GetModuleFileNameW(HMODULE(0), &mut buffer);
        
        if len == 0 {
            anyhow::bail!("Failed to get module file name");
        }
        
        let path = String::from_utf16_lossy(&buffer[..len as usize]);
        Ok(PathBuf::from(path))
    }
}

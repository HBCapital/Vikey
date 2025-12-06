use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;

type DllRegisterServerFn = unsafe extern "system" fn() -> HRESULT;

fn main() {
    let dll_path = r"target\release\vikey_windows_tsf.dll";
    
    println!("Loading DLL: {}", dll_path);
    
    unsafe {
        // Load DLL
        let wide_path: Vec<u16> = OsStr::new(dll_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        let hmodule = LoadLibraryW(windows::core::PCWSTR(wide_path.as_ptr()));
        
        if hmodule.is_invalid() {
            println!("Failed to load DLL!");
            return;
        }
        
        println!("DLL loaded successfully");
        
        // Get DllRegisterServer
        let proc_name = b"DllRegisterServer\0";
        let proc_addr = GetProcAddress(hmodule, windows::core::PCSTR(proc_name.as_ptr()));
        
        if proc_addr.is_none() {
            println!("DllRegisterServer not found!");
            FreeLibrary(hmodule);
            return;
        }
        
        println!("DllRegisterServer found");
        
        // Call it
        let register_fn: DllRegisterServerFn = std::mem::transmute(proc_addr);
        let hr = register_fn();
        
        println!("DllRegisterServer returned: 0x{:08X}", hr.0);
        
        if hr == S_OK {
            println!("✅ Registration successful!");
        } else {
            println!("❌ Registration failed!");
        }
        
        FreeLibrary(hmodule);
    }
}
